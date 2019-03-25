use crate::{
    common::{
        error::{ErrorKind, Result},
        ipc::SimulatorChannel,
        log::{router::route, stdio::proxy_stdio, Record},
        protocol::{Request, Response},
    },
    host::{
        configuration::{PluginConfiguration, StreamCaptureMode, Timeout},
        ipc::start,
    },
    info, trace, warn,
};
use crossbeam_channel::Sender;
use std::{
    process::{Child, Command},
    time::Instant,
};

#[derive(Debug)]
pub struct PluginProcess {
    child: Child,
    channel: SimulatorChannel,
    shutdown_timeout: Timeout,
}

impl PluginProcess {
    pub fn new(
        configuration: &PluginConfiguration,
        command: &mut Command,
        sender: Sender<Record>,
    ) -> Result<PluginProcess> {
        trace!("Constructing PluginProcess: {:?}", command);

        let (mut child, mut channel) = start(
            command,
            &configuration.nonfunctional.accept_timeout,
            &configuration.nonfunctional.stderr_mode,
            &configuration.nonfunctional.stdout_mode,
        )?;

        // Handshake
        channel
            .request
            .send(Request::Configuration(Box::new(configuration.clone())))?;

        match channel.response.recv() {
            Ok(Response::Success) => {
                route(
                    configuration.name.as_str(),
                    configuration.nonfunctional.verbosity,
                    channel.log().unwrap(),
                    sender.clone(),
                );

                // Log piped stdout/stderr
                if let StreamCaptureMode::Capture(level) = configuration.nonfunctional.stderr_mode {
                    proxy_stdio(
                        format!("{}::stderr", configuration.name.as_str()),
                        Box::new(child.stderr.take().expect("stderr")),
                        sender.clone(),
                        level,
                    );
                }

                if let StreamCaptureMode::Capture(level) = configuration.nonfunctional.stdout_mode {
                    proxy_stdio(
                        format!("{}::stdout", configuration.name.as_str()),
                        Box::new(child.stdout.take().expect("stdout")),
                        sender,
                        level,
                    );
                }

                Ok(PluginProcess {
                    child,
                    channel,
                    shutdown_timeout: configuration.nonfunctional.shutdown_timeout,
                })
            }
            Ok(_) | Err(_) => Err(ErrorKind::Other("Handshake problem".to_string()))?,
        }
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
            trace!(
                "Aborting PluginProcess (timeout: {:?})",
                self.shutdown_timeout
            );
            self.request(Request::Abort)
                .expect("Failed to abort PluginProcess");

            if let Timeout::Duration(duration) = self.shutdown_timeout {
                let now = Instant::now();
                loop {
                    if now.elapsed() < duration {
                        match self.channel.response.try_recv() {
                            Ok(Response::Success) => break,
                            Ok(_) | Err(_) => {
                                std::thread::sleep(std::time::Duration::from_millis(10));
                            }
                        }
                    } else {
                        // At this point we're going to kill.
                        trace!("Killing PluginProcess");
                        self.child.kill().expect("Failed to kill PluginProcess");
                        break;
                    }
                }
            }
        }

        // At this point the process should be shutting down or already down.
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
            None => warn!("PluginProcess terminated by signal"),
        }
    }
}
