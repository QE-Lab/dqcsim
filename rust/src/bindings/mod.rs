//! This module provides a C interface to the DQCsim simulator.
//!
//! Refer to [the generated API docs](https://mbrobbel.github.io/dqcsim-rs/c-api/c-api.apigen.html)
//! for more information.
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
//! # Callbacks
//!
//! In some places you can pass callbacks to the API. Depending on the
//! callback, it may be called from a different thread. This is clearly
//! documented along with the callback setter function; just keep it in mind.
//!
//! In order to support closures in higher-level languages, all callback
//! setters take an optional cleanup callback and a `void*` to a piece of user
//! data. The cleanup callback is intended for cleaning up this user data if
//! necessary; it is called when DQCsim drops all references to the primary
//! callback, so it is guaranteed that the primary callback is never called
//! again when the cleanup. It is also guaranteed that the cleanup callback
//! is executed exactly once (unless the process dies spectacularly, in which
//! case it may not be called). However, very few guarantees are made about
//! which thread the cleanup callback is called from! If you use it, make sure
//! that it is thread-safe.

use super::*;
use crate::{
    common::{error::*, log::*, types::*},
    host::{configuration::*, simulator::Simulator},
    plugin::{definition::*, state::*},
};
use libc::*;
use num_complex::Complex64;
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
