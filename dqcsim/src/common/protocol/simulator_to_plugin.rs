use crate::{
    common::{
        log::LogRecord,
        types::{ArbCmd, ArbData, PluginType},
    },
    host::configuration::PluginLogConfiguration,
};
use ipc_channel::ipc::IpcSender;
use serde::{Deserialize, Serialize};

/// Simulator/host to plugin requests.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

    /// Request to complete the connection with the upstream plugin.
    ///
    /// This is always the second message sent by DQCsim for operators and
    /// backends. It is called after the upstream plugin has been successfully
    /// initialized. In response, the plugin must wait for the upstream plugin
    /// to connect and finish setting up the connection.
    ///
    /// The valid responses to this message are:
    ///
    ///  - success: `PluginToSimulator::Success`
    ///  - failure: `PluginToSimulator::Failure`
    AcceptUpstream,

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

impl Into<SimulatorToPlugin> for ArbCmd {
    fn into(self) -> SimulatorToPlugin {
        SimulatorToPlugin::ArbRequest(self)
    }
}

/// Plugin initialization request. See `SimulatorToPlugin::Initialize`.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginInitializeRequest {
    /// Gatestream endpoint for the downstream plugin to connect to.
    ///
    /// Must be specified for frontends and operators, must not be specified
    /// for backends.
    pub downstream: Option<String>,

    /// The expected plugin type.
    pub plugin_type: PluginType,

    /// Vector of `ArbCmd` to supply to the plugin's `init()` function.
    pub init_cmds: Vec<ArbCmd>,

    /// Configuration for the logging subsystem of the plugin.
    pub log_configuration: PluginLogConfiguration,

    /// Sender side of the log channel. Can be used by a Plugin to send log
    /// records to the simulator.
    pub log_channel: IpcSender<LogRecord>,
}

impl Into<SimulatorToPlugin> for PluginInitializeRequest {
    fn into(self) -> SimulatorToPlugin {
        SimulatorToPlugin::Initialize(Box::new(self))
    }
}

impl PartialEq for PluginInitializeRequest {
    fn eq(&self, other: &PluginInitializeRequest) -> bool {
        self.downstream == other.downstream
            && self.plugin_type == other.plugin_type
            && self.init_cmds == other.init_cmds
            && self.log_configuration == other.log_configuration
    }
}

pub struct PluginAcceptUpstreamRequest;

impl Into<SimulatorToPlugin> for PluginAcceptUpstreamRequest {
    fn into(self) -> SimulatorToPlugin {
        SimulatorToPlugin::AcceptUpstream
    }
}

/// Frontend run request. See `SimulatorToPlugin::RunRequest`.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FrontendRunRequest {
    /// When specified, the frontend's `run()` callback must be called with the
    /// contained `ArbData` as argument.
    pub start: Option<ArbData>,

    /// Messages queued up through the host's `send()` function, to be consumed
    /// by the plugin's `recv()` function.
    pub messages: Vec<ArbData>,
}

impl Into<SimulatorToPlugin> for FrontendRunRequest {
    fn into(self) -> SimulatorToPlugin {
        SimulatorToPlugin::RunRequest(self)
    }
}
