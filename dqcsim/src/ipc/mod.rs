use crate::{
    log::Record,
    protocol::message::{GateStream, Request, Response},
};
use ipc_channel::ipc::{IpcReceiver, IpcSender};
use serde::{Deserialize, Serialize};

pub mod connection;

/// Plugin IPC utility functions.
pub mod plugin;

/// Simulation IPC utility functions.
pub mod simulator;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulatorChannel {
    log: Option<IpcReceiver<Record>>,
    pub request: IpcSender<Request>,
    pub response: IpcReceiver<Response>,
}

#[derive(Serialize, Deserialize)]
pub struct PluginChannel {
    pub log: IpcSender<Record>,
    pub request: IpcReceiver<Request>,
    pub response: IpcSender<Response>,
}

#[derive(Serialize, Deserialize)]
pub struct DownstreamChannel {
    pub tx: IpcSender<GateStream>,
    pub rx: IpcReceiver<GateStream>,
}

#[derive(Serialize, Deserialize)]
pub struct UpstreamChannel {
    pub rx: IpcReceiver<GateStream>,
    pub tx: IpcSender<GateStream>,
}

impl DownstreamChannel {
    pub fn new(tx: IpcSender<GateStream>, rx: IpcReceiver<GateStream>) -> DownstreamChannel {
        DownstreamChannel { tx, rx }
    }
}

impl UpstreamChannel {
    pub fn new(rx: IpcReceiver<GateStream>, tx: IpcSender<GateStream>) -> UpstreamChannel {
        UpstreamChannel { rx, tx }
    }
}

impl SimulatorChannel {
    pub fn new(
        log: IpcReceiver<Record>,
        request: IpcSender<Request>,
        response: IpcReceiver<Response>,
    ) -> SimulatorChannel {
        SimulatorChannel {
            log: Some(log),
            request,
            response,
        }
    }
    pub fn log(&mut self) -> Option<IpcReceiver<Record>> {
        self.log.take()
    }
}
