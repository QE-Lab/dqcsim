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

/// Thread-local state storage.
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
    IndexError(usize),
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
