use crate::{
    plugin::PluginError,
    protocol::ipc::SimulatorChannel,
    util::{
        ipc::setup,
        log::{stdio_to_log, Record},
    },
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
        let command = self
            .command
            .take()
            .ok_or_else(|| PluginError::ProcessError("Process in broken state.".to_string()))?;
        let (mut child, mut channel) = setup(command, ipc_connect_timeout)?;
        ROUTER.route_ipc_receiver_to_crossbeam_sender(
            channel.log().expect("Unable to get log channel"),
            sender.clone(),
        );

        // Log piped stdout/stderr
        stdio_to_log(
            Box::new(child.stderr.take().expect("stderr")),
            sender.clone(),
            log::Level::Error,
        );
        stdio_to_log(
            Box::new(child.stdout.take().expect("stdout")),
            sender,
            log::Level::Info,
        );

        self.child = Some(child);
        self.channel = Some(channel);
        Ok(self)
    }
    pub fn init(&self) {}
    // pub fn kill(&mut self) -> Result<(), std::io::Error> {
    //     self.child.as_mut().unwrap().kill()
    // }
}

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
            Some(code) => {
                let msg = format!("Exited with status code: {}", code);
                if code > 0 {
                    log::warn!("{}", msg)
                } else {
                    log::info!("{}", msg)
                }
            }
            None => log::error!("Process terminated by signal"),
        };
    }
}
