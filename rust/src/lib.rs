//! DQCsim: the Delft Quantum & Classical simulator
//!
//! For general information, refer to [the readme file on github](https://github.com/mbrobbel/dqcsim-rs/blob/master/README.md).

mod core;
pub use crate::core::*;

#[cfg(feature = "bindings")]
pub mod bindings;
