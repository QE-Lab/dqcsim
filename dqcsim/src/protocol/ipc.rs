use crate::{
    protocol::message::{Control, Reply},
    util::log::Record,
};
use failure::Fail;
use ipc_channel::ipc::{IpcReceiver, IpcSender};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Fail)]
pub enum ChannelError {
    #[fail(display = "Ipc channel failed.")]
    IpcError(#[fail(cause)] ipc_channel::Error),
    #[fail(display = "I/O error.")]
    IoError(#[fail(cause)] io::Error),
    #[fail(display = "Ipc channel timeout")]
    Timeout,
}

#[derive(Serialize, Deserialize)]
pub struct SimulatorChannel {
    log: Option<IpcReceiver<Record>>,
    control: Option<IpcSender<Control>>,
    reply: Option<IpcReceiver<Reply>>,
}

#[derive(Serialize, Deserialize)]
pub struct PluginChannel {
    log: Option<IpcSender<Record>>,
    control: Option<IpcReceiver<Control>>,
    reply: Option<IpcSender<Reply>>,
}

impl SimulatorChannel {
    pub fn new(
        log: IpcReceiver<Record>,
        control: IpcSender<Control>,
        reply: IpcReceiver<Reply>,
    ) -> SimulatorChannel {
        SimulatorChannel {
            log: Some(log),
            control: Some(control),
            reply: Some(reply),
        }
    }
    pub fn log(&mut self) -> Option<IpcReceiver<Record>> {
        self.log.take()
    }
}

impl PluginChannel {
    pub fn new(
        log: IpcSender<Record>,
        control: IpcReceiver<Control>,
        reply: IpcSender<Reply>,
    ) -> PluginChannel {
        PluginChannel {
            log: Some(log),
            control: Some(control),
            reply: Some(reply),
        }
    }
    pub fn log(&mut self) -> Option<IpcSender<Record>> {
        self.log.take()
    }
}
