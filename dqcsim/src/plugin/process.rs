use crate::{
    plugin::PluginError,
    protocol::channel::{setup, SimulatorChannel},
    util::log::Record,
};
use crossbeam_channel::Sender;
use failure::Error;
use ipc_channel::router::ROUTER;
use std::{
    process::{Child, Command},
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
    pub fn connect(
        mut self,
        sender: Sender<Record>,
        ipc_connect_timeout: Option<Duration>,
    ) -> Result<PluginProcess, Error> {
        let command = self.command.take().ok_or(PluginError::ProcessError(
            "Process in broken state.".to_string(),
        ))?;
        let (child, mut channel) = setup(command, ipc_connect_timeout)?;
        ROUTER.route_ipc_receiver_to_crossbeam_sender(
            channel.log().expect("Unable to get log channel"),
            sender,
        );
        self.child = Some(child);
        self.channel = Some(channel);
        Ok(self)
    }
    pub fn kill(&mut self) -> Result<(), std::io::Error> {
        self.child.as_mut().unwrap().kill()
    }
}

// TODO: pipestream reader which dumps lines as they come in.
//                 // https://gist.github.com/ArtemGr/db40ae04b431a95f2b78

impl Drop for PluginProcess {
    fn drop(&mut self) {
        // Wait for child.
        let status = self
            .child
            .take()
            .expect("Process in broken state")
            .wait()
            .expect("Child process failed");
        match status.code() {
            Some(code) => log::info!("Exited with status code: {}", code),
            None => log::error!("Process terminated by signal"),
        }
    }
}
