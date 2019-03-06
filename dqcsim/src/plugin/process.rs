use crate::{
    plugin::PluginError,
    protocol::{
        channel::{setup, SimulatorChannel},
        message::{Control, Log, Reply},
    },
    util::log::{LogThread, Record},
};
use crossbeam_channel::Sender;
use failure::Error;
use ipc_channel::{
    ipc::{IpcOneShotServer, IpcReceiver, IpcSender},
    router::ROUTER,
};
use log::trace;
use serde::{Deserialize, Serialize};
use std::{
    process::{Child, Command},
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

pub struct PluginProcess {
    command: Option<Command>,
    child: Option<Child>,
    channel: Option<SimulatorChannel>,
}

impl PluginProcess {
    pub fn new(command: Command) -> PluginProcess {
        PluginProcess {
            command: Some(command),
            child: None,
            channel: None,
        }
    }
    pub fn connect(mut self, sender: Sender<Record>) -> Result<PluginProcess, Error> {
        let command = self.command.take().ok_or(PluginError::ProcessError(
            "Process in broken state.".to_string(),
        ))?;
        let (child, mut channel) = setup(command, None)?;
        ROUTER.route_ipc_receiver_to_crossbeam_sender(
            channel.log().expect("Unable to get log channel"),
            sender,
        );
        self.child = Some(child);
        self.channel = Some(channel);
        Ok(self)
    }
}
