use crate::{
    common::{
        channel::IpcChannel,
        error::{err, ErrorKind, Result},
        log::{stdio::proxy_stdio, thread::LogThread},
        protocol::{PluginToSimulator, SimulatorToPlugin},
        types::{ArbCmd, PluginType},
    },
    host::{
        configuration::{
            EnvMod, PluginLogConfiguration, PluginProcessConfiguration, StreamCaptureMode, Timeout,
        },
        plugin::Plugin,
    },
    info, trace, warn,
};
use ipc_channel::ipc;
use std::{process, sync, thread, time};

/// A Plugin running in a child process.
///
/// PluginProcess implements the [`Plugin`] trait to enable the [`Simulator`]
/// to spawn and connect the plugin.
/// A PluginProcess is defined by its [`PluginProcessConfiguration`].
#[derive(Debug)]
pub struct PluginProcess {
    /// The complete plugin configuration.
    configuration: PluginProcessConfiguration,
    /// A handle to the child process which runs the plugin.
    /// Wrapped in an option, which is None until the [`Simulator`] spawns the
    /// plugin.
    child: Option<process::Child>,
    /// The SimulatorChannel is populated by the spawn method of the Plugin
    /// trait.
    channel: Option<IpcChannel<SimulatorToPlugin, PluginToSimulator>>,
}

impl PluginProcess {
    /// Constructs a new PluginProcess based on a PluginProcessConfiguration.
    /// Returns the constructed PluginProcess. The child process is not spawned
    /// at construction. The [`Plugin`] trait's [`spawn`] method spawns the
    /// child process.
    pub fn new(configuration: PluginProcessConfiguration) -> PluginProcess {
        PluginProcess {
            configuration,
            child: None,
            channel: None,
        }
    }
}

impl Plugin for PluginProcess {
    /// Spawn the child process based on the plugin configuration.
    /// The simulator address is passed as the first argument to the child
    /// process, or as the 2nd argument to the interpreter when the
    /// configuration specifies a script.
    fn spawn(&mut self, logger: &LogThread) -> Result<()> {
        // Setup connection channel
        let (server, server_name) = ipc::IpcOneShotServer::new()?;

        // Construct the child process
        let mut command = process::Command::new(&self.configuration.specification.executable);

        // Script
        if let Some(script) = &self.configuration.specification.script {
            command.arg(script);
        }

        command
            // TODO: matthijs remove this
            .arg(&self.configuration.name)
            // Pass simulator address
            .arg(server_name)
            // Set working directory
            .current_dir(&self.configuration.functional.work)
            // Stderr capture mode
            .stderr(&self.configuration.nonfunctional.stderr_mode)
            // Stdout capture mode
            .stdout(&self.configuration.nonfunctional.stdout_mode);

        // Environment
        self.configuration
            .functional
            .env
            .iter()
            .for_each(|env_mod| match env_mod {
                EnvMod::Set { key, value } => {
                    command.env(key, value);
                }
                EnvMod::Remove { key } => {
                    command.env_remove(key);
                }
            });

        // Spawn child process
        self.child = Some(command.spawn()?);

        // Connect and get channel from child process
        match self.configuration.nonfunctional.accept_timeout {
            Timeout::Infinite => {
                let (_, channel) = server.accept()?;
                self.channel = Some(channel);
            }
            Timeout::Duration(timeout) => {
                #[cfg_attr(feature = "cargo-clippy", allow(clippy::mutex_atomic))]
                let pair = sync::Arc::new((sync::Mutex::new(false), sync::Condvar::new()));
                let pair2 = pair.clone();
                let handle: thread::JoinHandle<
                    Result<IpcChannel<SimulatorToPlugin, PluginToSimulator>>,
                > = thread::spawn(move || {
                    {
                        let &(ref lock, _) = &*pair2;
                        let mut started = lock.lock().expect("Unable to aquire lock");
                        *started = true;
                    }
                    let (_, channel) = server.accept()?;
                    {
                        let &(_, ref cvar) = &*pair2;
                        cvar.notify_one();
                    }
                    Ok(channel)
                });
                let &(ref lock, ref cvar) = &*pair;
                let (started, wait_result) = cvar
                    .wait_timeout(
                        lock.lock()
                            .expect("Plugin IPC connection start lock poisoned"),
                        timeout,
                    )
                    .expect("Plugin IPC connection start lock poisoned");
                if *started && !wait_result.timed_out() {
                    self.channel = handle
                        .join()
                        .map_err(|_| {
                            ErrorKind::Other("Plugin IPC connection thread failed".to_string())
                        })?
                        .ok();
                } else {
                    err("plugin did not connect within specified timeout")?
                }
            }
        }

        // Setup pipes
        if let StreamCaptureMode::Capture(level) = self.configuration.nonfunctional.stderr_mode {
            proxy_stdio(
                format!("{}::stderr", self.configuration.name),
                Box::new(self.child.as_mut().unwrap().stderr.take().expect("stderr")),
                logger.get_sender(),
                level,
            );
        }
        if let StreamCaptureMode::Capture(level) = self.configuration.nonfunctional.stdout_mode {
            proxy_stdio(
                format!("{}::stdout", self.configuration.name),
                Box::new(self.child.as_mut().unwrap().stdout.take().expect("stdout")),
                logger.get_sender(),
                level,
            );
        }

        Ok(())
    }

    fn plugin_type(&self) -> PluginType {
        self.configuration.specification.typ
    }

    fn init_cmds(&self) -> Vec<ArbCmd> {
        self.configuration.functional.init.clone()
    }

    fn log_configuration(&self) -> PluginLogConfiguration {
        PluginLogConfiguration::from(&self.configuration)
    }

    fn rpc(&mut self, msg: SimulatorToPlugin) -> Result<PluginToSimulator> {
        self.channel.as_ref().unwrap().0.send(msg)?;
        Ok(self.channel.as_ref().unwrap().1.recv()?)
    }
}

impl Drop for PluginProcess {
    fn drop(&mut self) {
        trace!("Dropping PluginProcess");

        if self
            .child
            .as_mut()
            .expect("Child process")
            .try_wait()
            .expect("PluginProcess failed")
            .is_none()
        {
            trace!(
                "Aborting PluginProcess (timeout: {:?})",
                self.configuration.nonfunctional.shutdown_timeout
            );
            self.channel
                .as_ref()
                .unwrap()
                .0
                .send(SimulatorToPlugin::Abort)
                .expect("Failed to abort PluginProcess");

            if let Timeout::Duration(duration) = self.configuration.nonfunctional.shutdown_timeout {
                let now = time::Instant::now();
                loop {
                    if now.elapsed() < duration {
                        match self.channel.as_ref().unwrap().1.try_recv() {
                            Ok(PluginToSimulator::Success) => break,
                            Ok(_) | Err(_) => {
                                std::thread::sleep(std::time::Duration::from_millis(10));
                            }
                        }
                    } else {
                        // At this point we're going to kill.
                        trace!("Killing PluginProcess");
                        self.child
                            .as_mut()
                            .unwrap()
                            .kill()
                            .expect("Failed to kill PluginProcess");
                        break;
                    }
                }
            }
        }

        // At this point the process should be shutting down or already down.
        let status = self
            .child
            .as_mut()
            .unwrap()
            .wait()
            .expect("Failed to get exit status");

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
