use super::*;

// *** COMMON API ***

// dqcs_error_* functions, get getting and setting the last error message.
mod error;
pub use error::*;

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

// dqcs_cq_* functions, for operating on `ArbCmd` queues.
mod cq;
pub use cq::*;

// *** HOST API ***

// dqcs_pcfg_* functions, for constructing `PluginConfiguration` objects.
mod pcfg;
pub use pcfg::*;

// dqcs_scfg_* functions, for constructing `SimulatorConfiguration` objects.
mod scfg;
pub use scfg::*;

// dqcs_sim_* functions, for controlling a DQCsim simulator.
mod sim;
pub use sim::*;

// dqcs_accel_* functions, for talking to a generic accelerator.
mod accel;
pub use accel::*;

// *** PLUGIN API ***

// dqcs_pdef_* functions, for defining plugin implementations.
mod pdef;
pub use pdef::*;

// dqcs_plugin_* functions, for controlling plugins implementations.
mod plugin;
pub use plugin::*;
