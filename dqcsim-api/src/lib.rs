//! The `dqcsim-api` library crate provides a C interface to the DQCsim
//! simulator.
//!
//! # Handles
//!
//! The API is based upon a handle system for referring to simulator data.
//! Handles are like cross-language references or pointers: they point to a
//! piece of data in the simulator, but don't contain it.
//!
//! The usage of handles implies that the complex data structures contained
//! within the simulator never actually leave the simulator. Instead, when the
//! simulator needs to pass data to you, it returns a handle, which you can use
//! to access the contents of the referred structure through a number of API
//! calls. Similarly, when you need to pass something to the simulator, you
//! construct an object through a number of API calls, and then pass the handle
//! to the simulator.
//!
//! # Operating on handles
//!
//! Handles can represent a number of different object types. Based on the type
//! of object the handle represents, different interfaces are supported. For
//! instance, `ArbCmd` objects support `handle`, `arb`, and `cmd`, while
//! `ArbData` objects only support `handle` and `arb`. Note that all handles
//! support the `handle` interface.
//!
//! The name of the API functions directly corresponds with the name of the
//! interface it requires the primary handle it operates on to have: the
//! functions have the form `dqcs_<interface>_*`.
//!
//! Refer to the documentation of `dqcs_handle_type_t` for more information.
//!
//! # Memory management
//!
//! To prevent memory leaks, pay close attention to the documentation of the
//! API calls you make. Most importantly, strings returned by DQCsim almost
//! always have to be deallocated by you through `free()` (the only exception
//! is `dqcs_explain()`). You should also make sure that you delete handles
//! that you no longer need through `dqcs_handle_delete()`, though most of the
//! time DQCsim does this for you when you use a handle.
//!
//! # Error handling
//!
//! Almost all API calls can fail, for instance because an invalid handle is
//! supplied. Since C does not support any kind of exceptions, such failures
//! are reported through the return value. Which value is used to indicate an
//! error depends on the return type:
//!
//!  - no return data: -1 for failure, 0 for success.
//!  - booleans and timestamps: -1 for failure, the value otherwise.
//!  - handles and qubit references: 0 for failure, the (positive) handle
//!    otherwise.
//!  - pointers: `NULL` for failure, the pointer otherwise.
//!
//! When you receive a failure code, use `dqcs_explain()` to get an error
//! message.
//!
//! # Thread-safety
//!
//! The global state that the API calls operate on is purely *thread-local*.
//! That means that you can't exchange objects between threads.
//!
//! The API will however call some callback functions provided by you from a
//! different, DQCsim-maintained thread. This means that you cannot call API
//! functions from that thread! Such instances are clearly marked in the
//! documentation.

use dqcsim::configuration::*;
use failure::{Error, Fail};
use libc::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr::null;

// Type definitions shared between rust and C.
mod ctypes;
pub use ctypes::*;

// dqcs_handle_* functions, for operating on any handle.
mod handle;
pub use handle::*;

// dqcs_arb_* functions, for operating on `ArbData` objects and objects
// containing/using a single `ArbData`.
mod arb;
pub use arb::*;

// dqcs_cmd_* functions, for operating on `ArbCmd` objects and objects
// containing/using a single `ArbCmd`.
mod cmd;
pub use cmd::*;

// dqcs_pcfg_* functions, for constructing `PluginConfiguration` objects.
mod pcfg;
pub use pcfg::*;

// dqcs_scfg_* functions, for constructing `SimulatorConfiguration` objects.
mod scfg;
pub use scfg::*;

mod test_callbacks;
pub use test_callbacks::*;

/// Enumeration of all objects that can be associated with an handle, including
/// the object data.
#[derive(Debug)]
#[allow(dead_code)]
enum Object {
    /// ArbData object.
    ArbData(ArbData),

    /// ArbData object.
    ArbCmd(ArbCmd),

    /// PluginConfiguration object.
    PluginConfiguration(PluginConfiguration),

    /// SimulatorConfiguration object.
    SimulatorConfiguration(SimulatorConfiguration),
}

/// Thread-local state storage structure.
struct ThreadState {
    /// Mapping from handle to object.
    ///
    /// This contains all API-managed data.
    pub objects: HashMap<dqcs_handle_t, Object>,

    /// This variable stores the handle value that will be used for the next
    /// allocation.
    pub handle_counter: dqcs_handle_t,

    /// This variable records the error message associated with the latest
    /// failure.
    pub last_error: Option<CString>,
}

impl ThreadState {
    /// Sets the `last_error` field in the `ThreadState` structure and returns
    /// `DQCS_FAILURE` for convenience.
    fn fail(&mut self, msg: impl AsRef<str>) -> dqcs_return_t {
        self.last_error =
            Some(CString::new(msg.as_ref()).unwrap_or_else(|_| CString::new("<UNKNOWN>").unwrap()));
        dqcs_return_t::DQCS_FAILURE
    }

    /// Stuffs the given object into the API-managed storage. Returns the
    /// handle created for it.
    fn push(&mut self, object: Object) -> dqcs_handle_t {
        let handle = self.handle_counter;
        self.objects.insert(handle, object);
        self.handle_counter = handle + 1;
        handle
    }
}

/// Thread-local state storage. Be careful not to call user callback functions
/// while holding a reference to the state: those callbacks can and probably
/// will claim the reference mutably at some point. Basically, once user
/// callbacks need to become "callable" from the Rust world, for instance when
/// a configuration object is consumed into the object it configures, they
/// should move out of this state.
thread_local! {
    static STATE: RefCell<ThreadState> = RefCell::new(ThreadState {
        objects: HashMap::new(),
        handle_counter: 1,
        last_error: None,
    });
}

/// Error structure used for reporting generic API errors.
#[derive(Debug, Fail, PartialEq)]
enum APIError {
    #[fail(display = "{}", 0)]
    Generic(String),
    #[fail(display = "Handle {} is invalid", 0)]
    InvalidHandle(dqcs_handle_t),
    #[fail(display = "Handle {} does not implement the requisite interface", 0)]
    UnsupportedHandle(dqcs_handle_t),
    #[fail(display = "Index {} out of range", 0)]
    IndexError(ssize_t),
}

/// Convenience function for operating on the thread-local state object.
fn with_state<T>(
    error: impl FnOnce() -> T,
    call: impl FnOnce(std::cell::RefMut<ThreadState>) -> Result<T, Error>,
) -> T {
    STATE.with(|state| match call(state.borrow_mut()) {
        Ok(r) => r,
        Err(e) => {
            state.borrow_mut().fail(e.to_string());
            error()
        }
    })
}

/// Convenience function for converting a C string to a Rust `str`.
fn receive_str<'a>(s: *const c_char) -> Result<&'a str, Error> {
    if s.is_null() {
        Err(APIError::Generic("Received NULL string".to_string()).into())
    } else {
        Ok(unsafe { CStr::from_ptr(s) }.to_str()?)
    }
}

/// Convenience function for returning an owned string to C land.
///
/// On success, this **returns a newly allocated string. It must be freed
/// with `free()` by the caller.**
fn return_string(s: impl AsRef<str>) -> Result<*const c_char, Error> {
    let s = CString::new(s.as_ref())?;
    let s = unsafe { strdup(s.as_ptr()) };
    if s.is_null() {
        Err(APIError::Generic("Failed to allocate return value".to_string()).into())
    } else {
        Ok(s)
    }
}

/// Convenience function for converting a C const buffer to a Rust `&[u8]`.
fn receive_raw<'a>(obj: *const c_void, obj_size: usize) -> Result<&'a [u8], Error> {
    if obj_size == 0 {
        Ok(&[])
    } else if obj.is_null() {
        Err(APIError::Generic("Received NULL data".to_string()).into())
    } else {
        Ok(unsafe { std::slice::from_raw_parts(obj as *const u8, obj_size) })
    }
}

/// Convenience function for converting a C const buffer to a Rust `&[u8]`.
fn return_raw(obj_in: &[u8], obj_out: *mut c_void, obj_size: usize) -> Result<ssize_t, Error> {
    if obj_size > 0 && obj_out.is_null() {
        Err(APIError::Generic("Received NULL buffer".to_string()).into())
    } else {
        let actual_size = obj_in.len();
        let copy_size = std::cmp::min(actual_size, obj_size);
        if copy_size > 0 {
            unsafe {
                memcpy(obj_out, obj_in.as_ptr() as *const c_void, copy_size);
            }
        }
        Ok(actual_size as ssize_t)
    }
}

/// Converts an index Pythonically and checks bounds.
///
/// By "Pythonically" we mean that the list length is added to the index if it
/// is negative, allowing -1 to be used for the end of the list, -2 for the
/// penultimate item, and so on.
///
/// `len` specifies the size of the list, `index` is the index to convert, and
/// `insert` selects whether index == len is okay (it is for the `insert()`
/// function, but isn't for anything else).
fn receive_index(len: size_t, index: ssize_t, insert: bool) -> Result<size_t, Error> {
    let converted_index = if index < 0 {
        index + (len as ssize_t)
    } else {
        index
    };
    let mut ok = true;
    if converted_index < 0 || converted_index as size_t > len {
        ok = false;
    } else if converted_index as size_t == len {
        ok = insert;
    }
    if ok {
        Ok(converted_index as size_t)
    } else {
        Err(APIError::IndexError(index).into())
    }
}

/// Returns a pointer to the latest error message.
///
/// Call this to get extra information when another function returns a failure
/// code. The returned pointer is temporary and therefore should **NOT** be
/// `free()`d; it will become invalid when a new error occurs.
#[no_mangle]
pub extern "C" fn dqcs_explain() -> *const c_char {
    STATE.with(|state| {
        let state = state.borrow();
        match &state.last_error {
            Some(msg) => msg.as_ptr(),
            None => null(),
        }
    })
}

/// Sets the latest error message string.
///
/// This must be called by callback functions when an error occurs within the
/// callback, otherwise the upstream result for `dqcs_explain()` will be
/// undefined.
#[no_mangle]
pub extern "C" fn dqcs_set_error(msg: *const c_char) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if msg.is_null() {
            state.last_error = None
        } else {
            state.last_error = Some(unsafe { CStr::from_ptr(msg) }.to_owned())
        }
    })
}

/// User data structure for callbacks.
///
/// All callbacks carry a user-defined `void*` with them, which is passed to
/// the callback as the first argument whenever it's called. This can be used
/// for closure data or for calling C++ class member functions. In the case of
/// a closure though, the ownership of the closure data is logically the
/// closure itself, which is moved from the user language into the Rust domain.
/// To avoid leaking this data, a second function pointer is optionally
/// provided by the user that is called when the closure is deleted, allowing
/// the user to clean up.
///
/// For Python, the user data is always the callable PyObject pointer, and the
/// `user_free` callbacks all point to the same function, which just decrements
/// the callable's refcount. This behavior is unfortunately not at *all*
/// supported by SWIG, so it's implemented in the `add_swig_directives.py`
/// script.
///
/// To turn an API callback into a Rust closure when it is installed, be sure
/// to construct this object outside of the closure and then move it into the
/// closure! For example:
///
/// ```rust
/// let data = CallbackUserData::new(user_free, user_data);
/// let cb = move || callback(data.data());
/// ```
struct CallbackUserData {
    user_free: Option<extern "C" fn(*mut c_void)>,
    data: *mut c_void,
}

impl Drop for CallbackUserData {
    fn drop(&mut self) {
        if let Some(user_free) = self.user_free {
            user_free(self.data);
        }
    }
}

impl CallbackUserData {
    /// Constructs a `CallbackUserData` object.
    pub fn new(
        user_free: Option<extern "C" fn(*mut c_void)>,
        data: *mut c_void,
    ) -> CallbackUserData {
        CallbackUserData { user_free, data }
    }

    /// Returns the user data pointer.
    pub fn data(&self) -> *mut c_void {
        self.data
    }
}
