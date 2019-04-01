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
//! is `dqcs_error_get()`). You should also make sure that you delete handles
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
//! When you receive a failure code, use `dqcs_error_get()` to get an error
//! message.
//!
//! DQCsim plugins use callback functions to let you define the behavior of
//! the plugin. When *your* behavior wants to return an error, the same
//! handshake is done, but the other way around: you set the error string
//! using `dqcs_error_set()` and then you return the failure code.
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

use dqcsim::{
    common::{error::*, types::*},
    host::configuration::*,
};
use libc::*;
use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::{CStr, CString},
    ptr::null,
};

// Module containing type definitions shared between rust and C.
mod ctypes;
pub use ctypes::*;

// Module containing all the external functions exposed to the library user.
mod external;
pub use external::*;

// Module containing the thread-local API state object. This object owns all
// the objects that are directly manipulable by the API and are not otherwise
// owned by any DQCsim instance.
mod api_state;
use api_state::*;

// Module containing the thread-local state of whatever DQCsim structure is
// being represented by the API user, which may be one of the following or
// nothing:
//  - an "accelerator" a.k.a. a DQCsim simulation;
//  - a frontend plugin;
//  - an operator plugin;
//  - a backend plugin;
// Even when none of those things is active, API functions operating only on
// the API_STATE object will still work just fine, and are in fact needed to
// initialize the above four things.
//
// The reason for the separation between the API and DQCsim state has to do
// with the fact that objects owned by the DQCsim state are allowed to call
// back into the user code, which is in turn allowed to modify the API state.
// That is, DQCSIM_STATE owns closures that may take a mutable reference to
// API_STATE. Unless we move the closures out of DQCSIM_STATE before calling
// them, this necessarily means that API_STATE and DQCSIM_STATE cannot be
// contained by the same RefCell. Hence the need for them to be separated.
//
// Note that the above means that user callback functions are NOT allowed to
// call any API function that takes a reference to DQCSIM_STATE. Fortunately,
// there is (currently!) no need for them to ever do that.
//mod dqcsim_state;
//use dqcsim_state::*;

// Module containing utility functions and auxiliary data structures.
mod util;
use util::*;
