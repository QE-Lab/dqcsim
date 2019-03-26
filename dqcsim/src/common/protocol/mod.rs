//! Defines the protocols for all forms of communication.

use crate::{common::log::LoglevelFilter, host::configuration::PluginConfiguration};
use serde::{Deserialize, Serialize};
use std::fmt;

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

// Modules containing data types used within the communication protocols.
mod arb_cmd;
pub use arb_cmd::ArbCmd;

mod arb_data;
pub use arb_data::ArbData;

mod gate;
pub use gate::Gate;

/// Represents a reference to a qubit.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QubitRef(usize);

impl fmt::Display for QubitRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a number of simulation cycles or the current simulation time.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cycles(u64);

impl fmt::Display for Cycles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a sequence number, used to identify pipelined gatestream
/// messages,
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SequenceNumber(u64);

impl fmt::Display for SequenceNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a qubit measurement result.
#[derive(Debug, Serialize, Deserialize)]
pub struct QubitMeasurement {
    /// The measured qubit.
    pub qubit: QubitRef,

    /// The measured value. true = 1, false = 0.
    pub value: bool,

    /// Implementation-specific additional data, such as the probability for
    /// this particular measurement outcome.
    pub data: ArbData,
}

/// Contains information about a plugin implementation.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// The name of the plugin implementation.
    name: String,

    /// The author of the plugin.
    author: String,

    /// The plugin version.
    version: String,
}

impl PluginMetadata {
    pub fn new(
        name: impl Into<String>,
        author: impl Into<String>,
        version: impl Into<String>,
    ) -> PluginMetadata {
        PluginMetadata {
            name: name.into(),
            author: author.into(),
            version: version.into(),
        }
    }

    /// Returns the name of the plugin implementation.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the author of the plugin.
    pub fn get_author(&self) -> &str {
        &self.author
    }

    /// Returns the plugin version.
    pub fn get_version(&self) -> &str {
        &self.version
    }
}

// TODO: remove the structures below, replacing them with the structures
// defined in the modules above!

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
