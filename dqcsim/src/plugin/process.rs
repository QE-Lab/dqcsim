use crate::{
    error,
    error::Result,
    info,
    ipc::{simulator::start, SimulatorChannel},
    log::{router::route, stdio::proxy_stdio, Loglevel, Record},
    protocol::message::{Request, Response},
    trace, warn,
};
use crossbeam_channel::Sender;
use std::process::{Child, Command};

#[derive(Debug)]
pub struct PluginProcess {
    child: Child,
    channel: SimulatorChannel,
}

impl PluginProcess {
    pub fn new(command: &mut Command, sender: Sender<Record>) -> Result<PluginProcess> {
        trace!("Constructing PluginProcess: {:?}", command);

        let (mut child, mut channel) = start(command, None)?;

        route(channel.log().unwrap(), sender.clone());

        // Log piped stdout/stderr
        proxy_stdio(
            Box::new(child.stderr.take().expect("stderr")),
            sender.clone(),
            Loglevel::Error,
        );

        proxy_stdio(
            Box::new(child.stdout.take().expect("stdout")),
            sender,
            Loglevel::Info,
        );

        Ok(PluginProcess { child, channel })
    }

    pub fn request(&self, request: Request) -> Result<()> {
        self.channel.request.send(request)?;
        Ok(())
    }

    pub fn wait_for_reply(&self) -> Response {
        self.channel.response.recv().unwrap()
    }
}

impl Drop for PluginProcess {
    fn drop(&mut self) {
        trace!("Dropping PluginProcess");

        if self
            .child
            .try_wait()
            .expect("PluginProcess failed")
            .is_none()
        {
            trace!("Aborting PluginProcess");
            self.request(Request::Abort)
                .expect("Failed to abort PluginProcess");

            // No timeout support for ipc-channel, so we wait.
            std::thread::sleep(std::time::Duration::from_millis(100));
            match self.channel.response.try_recv() {
                Ok(Response::Success) => {}
                _ => {
                    trace!("Killing PluginProcess");
                    self.child.kill().expect("Failed to kill PluginProcess");
                }
            }
        }

        // At this point the process should be shutting down.
        let status = self.child.wait().expect("Failed to get exit status");

        match status.code() {
            Some(code) => {
                let msg = format!("PluginProcess exited with status code: {}", code);
                if code > 0 {
                    warn!("{}", msg)
                } else {
                    info!("{}", msg)
                }
            }
            None => error!("PluginProcess terminated by signal"),
        }
    }
}
