//! The `dqcsim-api` library crate provides a C interface to the DQCsim
//! simulator.
//!
//! The API is based upon a handle system for referring to simulator data. That
//! means that the complex data structures contained within the simulator never
//! actually leave the simulator directly. Instead, when the simulator needs to
//! pass data to you, it returns a handle, which you can use to access the
//! contents of the referred structure through a number of API calls.
//! Similarly, when you need to pass something to the simulator, you construct
//! an object through a number of API calls, and then pass the handle to the
//! object to the simulator.
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

mod arb;
pub use arb::*;

/// Object type for a handle.
///
/// Handles are like pointers into DQCsim's internal structures: all API calls
/// use these to refer to objects. Handles are always positive integers, and
/// they are not reused even after their object is deleted. Use
/// `dqcs_handle_type()` or `dqcs_handle_dump()` if you want information about
/// a mystery handle.
#[allow(non_camel_case_types)]
pub type dqcs_handle_t = c_longlong;

/// Enumeration of types that can be associated with a handle.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum dqcs_handle_type_t {
    /// Indicates that the given handle is invalid.
    ///
    /// This indicates one of the following:
    ///
    ///  - The handle value is invalid (zero or negative).
    ///  - The handle has not been used yet.
    ///  - The object associated with the handle was deleted.
    DQCS_INVALID = 0,

    /// Indicates that the given handle belongs to an ArbData object.
    DQCS_ARB_DATA,

    /// Indicates that the given handle belongs to a frontend plugin
    /// configuration object.
    DQCS_FRONT_CONFIG,

    /// Indicates that the given handle belongs to an operator plugin
    /// configuration object.
    DQCS_OPER_CONFIG,

    /// Indicates that the given handle belongs to a backend plugin
    /// configuration object.
    DQCS_BACK_CONFIG,

    /// Indicates that the given handle belongs to a simulator configuration
    /// object.
    DQCS_SIM_CONFIG,
}

/// Default return value for functions that don't need to return anything.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum dqcs_return_t {
    /// The function has failed. More information may be obtained through
    /// `dqcsim_explain()`.
    DQCS_FAILURE = -1,

    /// The function did what it was supposed to.
    DQCS_SUCCESS = 0,
}

/// Enumeration of all objects that can be associated with an handle, including
/// the object data.
#[derive(Debug)]
#[allow(dead_code)]
enum Object {
    /// ArbData object.
    ArbData(ArbData),

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
    #[fail(display = "Index {} out of range", 0)]
    IndexError(usize),
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

/// Returns the type of object associated with the given handle.
#[no_mangle]
pub extern "C" fn dqcs_handle_type(h: dqcs_handle_t) -> dqcs_handle_type_t {
    STATE.with(|state| {
        let state = state.borrow();
        match &state.objects.get(&h) {
            None => dqcs_handle_type_t::DQCS_INVALID,
            Some(Object::ArbData(_)) => dqcs_handle_type_t::DQCS_ARB_DATA,
            Some(Object::PluginConfiguration(x)) => match x.specification.typ {
                PluginType::Frontend => dqcs_handle_type_t::DQCS_FRONT_CONFIG,
                PluginType::Operator => dqcs_handle_type_t::DQCS_OPER_CONFIG,
                PluginType::Backend => dqcs_handle_type_t::DQCS_BACK_CONFIG,
            },
            Some(Object::SimulatorConfiguration(_)) => dqcs_handle_type_t::DQCS_SIM_CONFIG,
        }
    })
}

/// Returns a debug dump of the object associated with the given handle.
///
/// On success, this **returns a newly allocated string containing the
/// description. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_handle_dump(h: dqcs_handle_t) -> *const c_char {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        match &state.objects.get(&h) {
            None => {
                state.fail(format!("Handle {} is invalid", h));
                null()
            }
            Some(x) => match return_string(format!("{:?}", x)) {
                Ok(p) => p,
                Err(e) => {
                    state.fail(e.to_string());
                    null()
                }
            },
        }
    })
}

/// Destroys the object associated with a handle.
///
/// Returns 0 when successful, -1 otherwise.
#[no_mangle]
pub extern "C" fn dqcs_handle_delete(h: dqcs_handle_t) -> dqcs_return_t {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        match &state.objects.remove_entry(&h) {
            None => state.fail(format!("Handle {} is invalid", h)),
            Some(_) => dqcs_return_t::DQCS_SUCCESS,
        }
    })
}
