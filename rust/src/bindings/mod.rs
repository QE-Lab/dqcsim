//! This module provides a C interface to the DQCsim simulator.
//!
//! Refer to [the generated API docs](https://mbrobbel.github.io/dqcsim/c-api/c-api.apigen.html)
//! for more information.

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
