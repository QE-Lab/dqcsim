//! IPC channel setup functionality.

use crate::common::{
    error::{ErrorKind, Result},
    log::Record,
    protocol::{GatestreamDown, GatestreamUp, PluginToSimulator, SimulatorToPlugin},
};
use ipc_channel::ipc::{IpcReceiver, IpcSender};
use serde::{Deserialize, Serialize};

/// The Plugin side of a Simulator and Plugin channel.
/// Constructed and owned by a [`Connection`] instance.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginChannel {
    pub log: Option<IpcSender<Record>>,
    pub request: IpcReceiver<SimulatorToPlugin>,
    pub response: IpcSender<PluginToSimulator>,
}

impl PluginChannel {
    /// Returns a PluginChannel wrapper.
    pub fn new(
        log: IpcSender<Record>,
        request: IpcReceiver<SimulatorToPlugin>,
        response: IpcSender<PluginToSimulator>,
    ) -> PluginChannel {
        PluginChannel {
            log: Some(log),
            request,
            response,
        }
    }
    /// Take log channel out the channel wrapper.
    pub fn log(&mut self) -> Option<IpcSender<Record>> {
        self.log.take()
    }
}

/// Channel between plugins. This side sends GateStream messages to a downstream plugin.
/// Constructed and owned by a [`Connection`] instance.
#[derive(Debug, Serialize, Deserialize)]
pub struct DownstreamChannel {
    pub tx: Option<IpcSender<GatestreamDown>>,
    pub rx: Option<IpcReceiver<GatestreamUp>>,
}

impl DownstreamChannel {
    /// Construct a new DownstreamChannel.
    pub fn new(tx: IpcSender<GatestreamDown>, rx: IpcReceiver<GatestreamUp>) -> DownstreamChannel {
        DownstreamChannel {
            tx: Some(tx),
            rx: Some(rx),
        }
    }
    pub fn rx_ref(&self) -> Result<&IpcReceiver<GatestreamUp>> {
        match &self.rx {
            Some(rx) => Ok(rx),
            None => Err(ErrorKind::IPCError(
                "Downstream receiver side not available".to_string(),
            ))?,
        }
    }
    pub fn tx_ref(&self) -> Result<&IpcSender<GatestreamDown>> {
        match &self.tx {
            Some(tx) => Ok(tx),
            None => Err(ErrorKind::IPCError(
                "Downstream sender side not available".to_string(),
            ))?,
        }
    }

    /// Take receiver out the channel wrapper.
    pub fn rx(&mut self) -> Option<IpcReceiver<GatestreamUp>> {
        self.rx.take()
    }
    /// Take sender out the channel wrapper.
    pub fn tx(&mut self) -> Option<IpcSender<GatestreamDown>> {
        self.tx.take()
    }
}

/// Channel between plugins. This side receives GateStream messages from an upstream plugin.
/// Constructed and owned by a [`Connection`] instance.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpstreamChannel {
    tx: Option<IpcSender<GatestreamUp>>,
    rx: Option<IpcReceiver<GatestreamDown>>,
}

impl UpstreamChannel {
    /// Construct a new UpstreamChannel.
    pub fn new(tx: IpcSender<GatestreamUp>, rx: IpcReceiver<GatestreamDown>) -> UpstreamChannel {
        UpstreamChannel {
            tx: Some(tx),
            rx: Some(rx),
        }
    }
    /// Take receiver out the channel wrapper.
    pub fn rx(&mut self) -> Option<IpcReceiver<GatestreamDown>> {
        self.rx.take()
    }
    /// Take sender out the channel wrapper.
    pub fn tx(&mut self) -> Option<IpcSender<GatestreamUp>> {
        self.tx.take()
    }
}
