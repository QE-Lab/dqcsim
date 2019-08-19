//! Contains structs that manage the lifetime and connections of a single
//! plugin.

pub mod process;
pub mod thread;

use crate::{
    common::{
        error::{err, Result},
        log::thread::LogThread,
        protocol::{
            PluginAcceptUpstreamRequest, PluginInitializeRequest, PluginInitializeResponse,
            PluginToSimulator, PluginUserInitializeRequest, SimulatorToPlugin,
        },
        types::{ArbCmd, ArbData, PluginType},
    },
    host::configuration::PluginLogConfiguration,
};
use std::fmt::Debug;

#[macro_export]
macro_rules! checked_rpc {
    ($plugin:expr, $message:expr, expect $t:ident) => {
        match $plugin.rpc($message.into()) {
            Ok(PluginToSimulator::$t(x)) => Ok(x),
            Ok(PluginToSimulator::Failure(e)) => err(e),
            Ok(_) => err("Protocol error: unexpected response from plugin"),
            Err(e) => Err(e),
        }
    };
    ($plugin:expr, $message:expr) => {
        match $plugin.rpc($message.into()) {
            Ok(PluginToSimulator::Success) => Ok(()),
            Ok(PluginToSimulator::Failure(e)) => err(e),
            Ok(_) => err("Protocol error: unexpected response from plugin"),
            Err(e) => Err(e),
        }
    };
}

/// The Plugin trait, implemented by all Plugins used in a Simulation.
pub trait Plugin: Debug {
    /// Spawn the Plugin. The Plugin should spawn the actual plugin code and
    /// prepare the communication channel. After spawning the [`rpc`] method
    /// should be available. The simulator will continue to send initialization
    /// requests via the commmunication channel.
    ///
    /// The logger is provided by the simulator, and plugins can use the
    /// reference to the log thread to make the neccesary copies of log senders
    /// to use with their log proxies.
    fn spawn(&mut self, logger: &LogThread) -> Result<()>;

    /// Returns the PluginType of this plugin.
    fn plugin_type(&self) -> PluginType;

    /// Returns the vector of `ArbCmd`s that are to be passed to the plugin's
    /// `initialize()` callback.
    fn init_cmds(&self) -> Vec<ArbCmd>;

    /// Returns the logging configuration for this plugin.
    fn log_configuration(&self) -> PluginLogConfiguration;

    /// Send the SimulatorToPlugin message to the plugin.
    fn rpc(&mut self, msg: SimulatorToPlugin) -> Result<PluginToSimulator>;
}

impl dyn Plugin {
    /// Returns the name of this plugin from its logging configuration.
    pub fn name(&self) -> String {
        self.log_configuration().name
    }

    /// Sends an `PluginInitializeRequest` to this plugin.
    pub fn initialize(
        &mut self,
        logger: &LogThread,
        downstream: &Option<String>,
        seed: u64,
    ) -> Result<PluginInitializeResponse> {
        checked_rpc!(
            self,
            PluginInitializeRequest {
                downstream: downstream.clone(),
                plugin_type: self.plugin_type(),
                seed,
                log_configuration: self.log_configuration(),
                log_channel: logger.get_ipc_sender(),
            },
            expect Initialized
        )
    }

    /// Requests that the plugin waits for the upstream plugin to connect and
    /// establishes the connection.
    pub fn accept_upstream(&mut self) -> Result<()> {
        checked_rpc!(self, PluginAcceptUpstreamRequest)
    }

    /// Send user initialize request to the plugin. This invokes the initialize
    /// callback with the user initialize commands.
    pub fn user_initialize(&mut self) -> Result<()> {
        checked_rpc!(
            self,
            PluginUserInitializeRequest {
                init_cmds: self.init_cmds()
            }
        )
    }

    /// Sends an `ArbCmd` message to this plugin.
    pub fn arb(&mut self, cmd: impl Into<ArbCmd>) -> Result<ArbData> {
        checked_rpc!(
            self,
            cmd.into(),
            expect ArbResponse
        )
    }
}
