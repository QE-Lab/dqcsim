use serde::{Deserialize, Serialize};
use std::os::raw::c_int;

#[derive(Serialize, Deserialize)]
pub enum Log {
    Record(String),
}

#[derive(Serialize, Deserialize)]
pub enum Control {
    Start,
    Stop(bool),
}

#[derive(Serialize, Deserialize)]
pub enum Reply {
    Yes,
    No,
}

/// Enum with supported signal variants.
#[derive(Serialize, Deserialize)]
pub enum Signal {
    SIGTERM,
    SIGINT,
    SIGKILL,
    SIGQUIT,
    Other(c_int),
}

// use serde_json::Value;
//
// #[derive(Serialize, Deserialize)]
// pub enum PluginControl {
//     Command(ArbCmd),
//     Init(D2Pinit),
//     Abort,
// }
//
// /// Generic exception response, sent in response to a command that fails.
// #[derive(Serialize, Deserialize)]
// pub struct Exception {
//     msg: String,
// }
//
// /// Generic success response, sent in response to a command that succeeds but
// /// does not return anything.
// #[derive(Serialize, Deserialize)]
// pub struct Success;
//
// #[derive(Serialize, Deserialize)]
// pub enum P2Dunion {
//     Exception(Exception),
//     ArbData(ArbData),
//     P2Dinit(P2Dinit),
//     /// Termination response. This is sent in response to a termination request,
//     /// just before the plugin shuts itself down.
//     P2Dfini,
//     P2Drun(P2Drun),
// }
//
// #[derive(Serialize, Deserialize)]
// pub enum D2Punion {
//     ArbCmd(ArbCmd),
//     D2Pinit(D2Pinit),
//     D2Pfini(D2Pfini),
//     D2Prun(D2Prun),
// }
//
// /// Loglevel enumeration
// #[derive(Serialize, Deserialize)]
// pub enum LogLevel {
//     Trace = 0,
//     Debug = 1,
//     Info = 2,
//     Warn = 3,
//     Err = 4,
//     Critical = 5,
//     Off = 6,
// }
//
// /// Initialization request. This is always the first thing sent by DQCsim.
// /// The upstream connection is the gatestream connection for operators and
// /// back-ends, or the host connection for plugins.
// #[derive(Serialize, Deserialize)]
// pub struct D2Pinit {
//     /// URI for downstream link to downstream plugin (front-end and operator
//     /// plugins only). The addressed plugin should connect to this to send
//     /// GateStreamDown messages.
//     pub down_push_uri: String,
//     /// URI for upstream link from downstream plugin (front-end and operator
//     /// plugins only). The addressed plugin should connect to this to receive
//     /// GateStreamUp messages.
//     pub down_pull_uri: String,
//     /// Arbitrary data for customizing plugin instantiation.
//     pub arb_cmds: Vec<ArbCmd>,
//     /// Logger name prefix for the plugin.
//     pub logger_prefix: String,
//     /// Loglevel for this plugin.
//     pub log_level: LogLevel,
// }
//
// /// Initialization response. This is sent in response to the initialization
// /// message from DQCsim to inform DQCsim of the URI that the next plugin
// /// should connect to, and/or to inform DQCsim that the plugin is ready to
// /// receive commands from the upstream connection.
// #[derive(Serialize, Deserialize)]
// pub struct P2Dinit {
//     /// URI for downstream link from upstream plugin (back-end and operator
//     /// plugins only). The upstream plugin should connect to this to send
//     /// GateStreamDown messages.
//     up_pull_uri: String,
//     /// URI for upstream link from downstream plugin (back-end and operator
//     /// plugins only). The upstream plugin should connect to this to receive
//     /// GateStreamUp messages.
//     up_push_uri: String,
// }
//
// /// Termination request. This is sent when DQCsim wants to shut the plugin
// /// down.
// #[derive(Serialize, Deserialize)]
// pub struct D2Pfini {
//     /// Whether this is a graceful shutdown request. When true, a normal shutdown
//     /// is requested; when false, a shutdown is requested due to a failure
//     /// elsewhere in the system. In the latter case, the plugin should refrain
//     /// from any further communication aside from the response to this message
//     /// to avoid blocking.
//     pub graceful: bool,
// }
//
// /// Frontend execution request. Passes control to the timed simulation of the
// /// accelerator, which then runs until it completes or blocks.
// #[derive(Serialize, Deserialize)]
// pub struct D2Prun {
//     /// Accelerator call start marker (queued by DQCsim::start()).
//     start: bool,
//     /// Arbitrary commands/data to pass to the accelerator start function.
//     start_data: Vec<ArbCmd>,
//     /// Messages to push into the host-to-accelerator queue (queued by
//     /// DQCsim::send()).
//     queue: Vec<ArbData>,
// }
//
// /// Frontend execution response. Contains the results of the accelerator
// /// execution, if any.
// #[derive(Serialize, Deserialize)]
// pub struct P2Drun {
//     /// Accelerator call completion marker (unblocks DQCsim::wait()).
//     exited: bool,
//     /// Exit code, set only if exited is set.
//     exit_code: Option<i32>,
//     /// Messages to push into the accelerator-to-host queue (unblocks
//     /// DQCsim::recv()).
//     queue: Vec<ArbData>,
// }
//
// #[derive(Serialize, Deserialize)]
// pub struct PluginToDQCsim {
//     response: P2Dunion,
// }
//
// #[derive(Serialize, Deserialize)]
// pub struct DQCsimToPlugin {
//     pub command: D2Punion,
// }
//
// /// Table for passing arbitrary data between two endpoints without the channel
// /// knowing what the data looks like.
// #[derive(Serialize, Deserialize)]
// pub struct ArbData {
//     /// JSON object.
//     json: Value,
//     /// Optional unstructured data.
//     args: Option<Vec<Vec<u8>>>,
// }
//
// /// Table for sending an arbitrary command from one endpoint to another.
// #[derive(Serialize, Deserialize)]
// pub struct ArbCmd {
//     /// Identifies the interface that this command addresses. If an endpoint
//     /// receives a command for an unsupported interface, it should treat the
//     /// command as no-op.
//     interface_identifier: String,
//     /// Identifies the name of the command within the specified interface.
//     /// If the interface is recognized but the operation is not, an error
//     /// should be thrown.
//     operation_identifier: String,
//     /// Arbitrary data to go along with the command.
//     data: ArbData,
// }
