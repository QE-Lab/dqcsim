pub mod process;
// pub mod thread;

use crate::{
    common::{
        error::{err, inv_op, Result},
        log::thread::LogThread,
        protocol::{
            PluginInitializeRequest, PluginInitializeResponse, PluginToSimulator, SimulatorToPlugin,
        },
        types::{ArbCmd, ArbData},
    },
    host::configuration::{PluginConfiguration, PluginType},
};

/// The Plugin trait, implemented by all Plugins used in a Simulation.
pub trait Plugin {
    /// Spawn the Plugin. The Plugin should spawn the actual plugin code and
    /// prepare the communication channel. After spawning both the [`send`] and
    /// [`receive`] functions should be available. The simulator will continue
    /// to send initialization requests via the commmunication channel.
    ///
    /// The logger is provided by the simulator, and plugins can use the
    /// reference to the log thread to make the neccesary copies of log senders
    /// to use with their log proxies.k
    fn spawn(&mut self, logger: &LogThread) -> Result<()>;

    fn configuration(&self) -> PluginConfiguration;

    fn send(&mut self, msg: SimulatorToPlugin) -> Result<()>;
    fn recv(&mut self) -> Result<PluginToSimulator>;
}

impl Plugin {
    pub fn name(&self) -> String {
        self.configuration().name
    }
    pub fn plugin_type(&self) -> PluginType {
        self.configuration().specification.typ
    }

    /// Sends an `PluginInitializeRequest` to this plugin.
    pub fn init(
        &mut self,
        logger: &LogThread,
        downstream: Option<String>,
    ) -> Result<PluginInitializeResponse> {
        self.send(SimulatorToPlugin::Initialize(Box::new(
            PluginInitializeRequest {
                downstream,
                configuration: self.configuration(),
                log: logger.get_ipc_sender(),
            },
        )))?;

        match self.recv()? {
            PluginToSimulator::Initialized(response) => Ok(response),
            PluginToSimulator::Failure(data) => err(data),
            _ => inv_op("Unexpected response from plugin"),
        }
    }

    /// Sends an `ArbCmd` message to this plugin.
    pub fn arb(&mut self, cmd: impl Into<ArbCmd>) -> Result<ArbData> {
        self.send(SimulatorToPlugin::ArbRequest(cmd.into()))?;
        match self.recv()? {
            PluginToSimulator::ArbResponse(data) => Ok(data),
            PluginToSimulator::Failure(data) => err(data),
            _ => inv_op("Unexpected response from plugin"),
        }
    }
}

// pub trait FrontEnd {}?
