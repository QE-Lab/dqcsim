use crate::{
    common::types::{ArbCmd, ArbData},
    host::configuration::PluginConfiguration,
};
use serde::{Deserialize, Serialize};

/// Simulator/host to plugin requests.
#[derive(Debug, Serialize, Deserialize)]
pub enum SimulatorToPlugin {
    /// Request to initialize the plugin.
    ///
    /// This is always the first message sent by DQCsim. In response, the
    /// plugin must:
    ///
    ///  - initialize its logging facilities (note that the tee files provided
    ///    by the copy of the plugin configuration are to be handled by the
    ///    plugin);
    ///  - verify with the user code that the plugin implementation is of the
    ///    expected type (frontend, operator, or backend);
    ///  - connect to the downstream plugin if the plugin is not a backend;
    ///  - run the user's initialization code;
    ///  - initialize an IPC endpoint for the upstream plugin to connect to if
    ///    the plugin is not a frontend;
    ///  - return the aforementioned URI to the simulator through a
    ///    `PluginToSimulator::Initialized` message.
    ///
    /// The valid responses to this message are:
    ///
    ///  - success: `PluginToSimulator::Initialized`
    ///  - failure: `PluginToSimulator::Failure`
    Initialize(Box<PluginInitializeRequest>),

    /// Request to abort the simulation and stop the plugin.
    ///
    /// The valid responses to this message are:
    ///
    ///  - success: `PluginToSimulator::Success`
    ///  - failure: `PluginToSimulator::Failure`
    Abort,

    /// Passes control from the host to the frontend plugin.
    ///
    /// This is only to be sent to frontends. In response, the frontend must:
    ///
    ///  - queue up any enclosed messages for reception through the plugin's
    ///    `recv()` function;
    ///  - if start is specified, call the user's implementation of the `run()`
    ///    callback (if this message was received while already executing
    ///    `run()`, return an error instead);
    ///  - if the user's implementation of `run()` terminates, put its return
    ///    value in the `result` field of the response;
    ///  - if the user's implementation of `run()` queued up messages through
    ///    `send()`, put them into the `messages` field of the response;
    ///  - send the `PluginToSimulator::RunResponse` message in response.
    ///
    /// The valid responses to this message are:
    ///
    ///  - success: `PluginToSimulator::RunResponse`
    ///  - failure: `PluginToSimulator::Failure`
    RunRequest(FrontendRunRequest),

    /// Requests execution of the given `ArbCmd` by the plugin.
    ///
    /// The valid responses to this message are:
    ///
    ///  - success: `PluginToSimulator::ArbResponse`
    ///  - failure: `PluginToSimulator::Failure`
    ArbRequest(ArbCmd),
}

/// Plugin initialization request. See `SimulatorToPlugin::Initialize`.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginInitializeRequest {
    /// Gatestream endpoint for the downstream plugin to connect to.
    ///
    /// Must be specified for frontends and operators, must not be specified
    /// for backends.
    pub downstream: Option<String>,

    /// Copy of the plugin configuration structure that DQCsim used to spawn
    /// the plugin process.
    ///
    /// Not all parts of this structure are necessarily relevant for the
    /// plugin, as DQCsim handles much of the configuration process. The plugin
    /// is expected to at least use the following parts:
    ///
    ///  - the plugin type is to be verified with the user code, to make sure
    ///    that the user code actually implements a plugin of the expected
    ///    type;
    ///  - the plugin name is to be used as a prefix for the logging system;
    ///  - tee files are to be handled by the plugin process for log system
    ///    performance reasons.
    ///
    /// Furthermore, while DQCsim performs loglevel filtering on the messages
    /// sent by the plugin, it is recommended that the plugin also does this
    /// filtering. This prevents log messages from being sent through the IPC
    /// connection (which costs performance) unnecessarily.
    pub configuration: PluginConfiguration,

    /// Arbitrary commmands sent to the user code of the plugin.
    pub arb_cmds: Vec<ArbCmd>,
}

/// Frontend run request. See `SimulatorToPlugin::RunRequest`.
#[derive(Debug, Serialize, Deserialize)]
pub struct FrontendRunRequest {
    /// When specified, the frontend's `run()` callback must be called with the
    /// contained `ArbData` as argument.
    pub start: Option<ArbData>,

    /// Messages queued up through the host's `send()` function, to be consumed
    /// by the plugin's `recv()` function.
    pub messages: Vec<ArbData>,
}
