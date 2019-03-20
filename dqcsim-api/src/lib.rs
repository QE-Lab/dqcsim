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
//! ## Operating on handles
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
//! DQCsim plugins use callback functions to let you define the behavior of
//! the plugin. When *your* behavior wants to return an error, the same
//! handshake is done, but the other way around: you set the error string
//! using `dqcs_set_error()` and then you return the failure code.
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
use dqcsim::error::*;
use libc::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr::null;

// Type definitions shared between rust and C.
mod ctypes;
pub use ctypes::*;

// Utility functions and auxiliary data structures.
mod util;
use util::*;

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

// dqcs_accel_* functions, for talking to the simulator from a host
// perspective.
mod accel;
pub use accel::*;

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

thread_local! {
    /// Thread-local state storage. Be careful not to call user callback functions
    /// while holding a reference to the state: those callbacks can and probably
    /// will claim the reference mutably at some point. Basically, once user
    /// callbacks need to become "callable" from the Rust world, for instance when
    /// a configuration object is consumed into the object it configures, they
    /// should move out of this state.
    static STATE: RefCell<ThreadState> = RefCell::new(ThreadState {
        objects: HashMap::new(),
        handle_counter: 1,
        last_error: None,
    });
}

/// Convenience function for operating on the thread-local state object.
fn with_state<T>(
    error: impl FnOnce() -> T,
    call: impl FnOnce(std::cell::RefMut<ThreadState>) -> Result<T>,
) -> T {
    STATE.with(|state| match call(state.borrow_mut()) {
        Ok(r) => r,
        Err(e) => {
            state.borrow_mut().fail(e.to_string());
            error()
        }
    })
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
