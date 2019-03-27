//! Defines the protocols for all forms of communication.

// Requests from simulator to plugin.
mod simulator_to_plugin;
pub use simulator_to_plugin::{FrontendRunRequest, PluginInitializeRequest, SimulatorToPlugin};

// Responses from the plugin to the simulator.
mod plugin_to_simulator;
pub use plugin_to_simulator::{FrontendRunResponse, PluginInitializeResponse, PluginToSimulator};

// Messages from plugins to the logging thread (i.e. log messages).
mod plugin_to_log_thread;
pub use plugin_to_log_thread::PluginToLogThread;

// Gatestream request messages.
mod gatestream_down;
pub use gatestream_down::{GatestreamDown, PipelinedGatestreamDown};

// Gatestream response messages.
mod gatestream_up;
pub use gatestream_up::GatestreamUp;

// TODO: remove the structures below, replacing them with the structures
// defined in the modules above!

use crate::{
    common::{log::LoglevelFilter, types::ArbCmd},
    host::configuration::PluginConfiguration,
};
use serde::{Deserialize, Serialize};

/// Simulator to plugin requests.
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    /// Handshake the configuration for reference.
    Configuration(Box<PluginConfiguration>),
    /// Request to initialize the plugin.
    ///
    /// When requested, the plugin should connect to provided downstream and
    /// upstream plugin.
    Init(InitializeRequest),
    /// Request to abort the simulation and stop the plugin.
    Abort,
}

/// Plugin to simulator responses.
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    /// Initialization response.
    Init(InitializeResponse),
    /// Success response.
    Success,
}

/// Initialization request.
#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeRequest {
    /// Downstream plugin to connect to.
    pub downstream: Option<String>,
    /// Arbitrary commmands.
    pub arb_cmds: Vec<ArbCmd>,
    /// Prefix for logging.
    pub prefix: String,
    /// LoglevelFilter for logging.
    pub level: LoglevelFilter,
}

/// Initialization response.
#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeResponse {
    // Upstream endpoint.
    pub upstream: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GateStream {
    Hello(String),
    Bye(String),
}
