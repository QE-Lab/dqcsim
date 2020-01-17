//! This module provides a C interface to the DQCsim simulator.
//!
//! Refer to [the generated API docs](https://mbrobbel.github.io/dqcsim/c-api/c-api.apigen.html)
//! for more information.

use super::*;
use crate::{
    common::{converter::*, error::*, log::*, types::*},
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

// Module containing utility functions and auxiliary data structures.
mod util;
use util::*;
