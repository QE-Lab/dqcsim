use crate::common::protocol::{PluginToSimulator, SimulatorToPlugin};
use ipc_channel::ipc::{IpcReceiver, IpcSender};
use serde::{Deserialize, Serialize};

/// The Simulator side of a Simulator to Plugin channel.
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulatorChannel {
    pub request: IpcSender<SimulatorToPlugin>,
    pub response: IpcReceiver<PluginToSimulator>,
}

impl SimulatorChannel {
    /// Returns a SimulatorChannel wrapper.
    pub fn new(
        request: IpcSender<SimulatorToPlugin>,
        response: IpcReceiver<PluginToSimulator>,
    ) -> SimulatorChannel {
        SimulatorChannel { request, response }
    }
}
