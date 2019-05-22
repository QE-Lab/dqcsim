//! Implementation of the plugin trait for running the plugin within a thread
//! inside DQCsim.

use crate::{
    common::{
        channel::SimulatorChannel,
        error::Result,
        log::thread::LogThread,
        protocol::{PluginToSimulator, SimulatorToPlugin},
        types::{ArbCmd, PluginType},
    },
    error, fatal,
    host::{
        configuration::{PluginLogConfiguration, PluginThreadConfiguration},
        plugin::Plugin,
    },
    trace,
};
use ipc_channel::ipc;
use std::{fmt, thread};

pub type PluginThreadClosure = Box<dyn Fn(String) -> () + Send>;

pub struct PluginThread {
    thread: Option<PluginThreadClosure>,
    handle: Option<thread::JoinHandle<()>>,
    channel: Option<SimulatorChannel>,
    plugin_type: PluginType,
    init_cmds: Vec<ArbCmd>,
    log_configuration: PluginLogConfiguration,
}

impl fmt::Debug for PluginThread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PluginThread")
            .field("thread", &"...".to_string())
            .field("handle", &self.handle)
            .field("channel", &self.channel)
            .finish()
    }
}

impl PluginThread {
    /// Constructs a plugin thread from a plugin definition and configuration.
    pub fn new(configuration: PluginThreadConfiguration) -> PluginThread {
        PluginThread {
            thread: Some(configuration.closure),
            handle: None,
            channel: None,
            plugin_type: configuration.plugin_type,
            init_cmds: configuration.init_cmds,
            log_configuration: configuration.log_configuration,
        }
    }
}

impl Plugin for PluginThread {
    fn spawn(&mut self, _: &LogThread) -> Result<()> {
        let thread = self.thread.take().unwrap();
        // Setup connection channel
        let (server, server_name) = ipc::IpcOneShotServer::new()?;

        // Spawn the thread.
        self.handle = Some(thread::spawn(move || {
            // Set a custom panic hook which generates a fatal log record for
            // panics including a stack trace.
            std::panic::set_hook(Box::new(|info| {
                let backtrace = backtrace::Backtrace::new();
                for line in format!("{}", info)
                    .split('\n')
                    .chain(format!("{:?}", backtrace).split('\n'))
                {
                    fatal!("{}", line);
                }
            }));

            // Start the thread function.
            thread(server_name)
        }));

        // Wait for the thread to connect.
        let (_, channel) = server.accept()?;

        self.channel = Some(channel);
        Ok(())
    }

    fn rpc(&mut self, msg: SimulatorToPlugin) -> Result<PluginToSimulator> {
        self.channel.as_ref().unwrap().0.send(msg)?;
        Ok(self.channel.as_ref().unwrap().1.recv()?)
    }

    fn plugin_type(&self) -> PluginType {
        self.plugin_type
    }

    fn init_cmds(&self) -> Vec<ArbCmd> {
        self.init_cmds.clone()
    }

    fn log_configuration(&self) -> PluginLogConfiguration {
        self.log_configuration.clone()
    }
}

fn join_thread(handle: thread::JoinHandle<()>, name: impl fmt::Display) {
    match handle.join() {
        Ok(_) => trace!("Thread joined"),
        Err(e) => {
            let msg = if let Some(e) = e.downcast_ref::<&'static str>() {
                e
            } else if let Some(e) = e.downcast_ref::<String>() {
                e
            } else {
                "unknown"
            };
            fatal!("Thread {} panicked:", name);
            for line in msg.split('\n') {
                fatal!("{}", line);
            }
        }
    }
}

impl Drop for PluginThread {
    fn drop(&mut self) {
        trace!("Dropping PluginThread");

        // Attempt to send Abort request.
        if self.handle.is_some() && self.channel.is_some() {
            match self.rpc(SimulatorToPlugin::Abort) {
                Ok(PluginToSimulator::Success) => {
                    join_thread(self.handle.take().unwrap(), Plugin::name(self));
                }
                Ok(PluginToSimulator::Failure(error)) => {
                    error!("Thread {} failed to abort: {}", Plugin::name(self), error);
                }
                Ok(_) => {
                    error!("Unexected reply from {}", Plugin::name(self));
                }
                Err(error) => {
                    error!("Failed to send Abort to {}: {}", Plugin::name(self), error);
                    join_thread(self.handle.take().unwrap(), Plugin::name(self));
                }
            }
        } else if self.handle.is_none() {
            error!("Thread handle for {} already dropped", Plugin::name(self));
        } else {
            error!("Channel to {} already closed", Plugin::name(self));
        }

        // Close the simulator channel.
        self.channel.take();

        // Drop the thread handle
        self.handle.take();
    }
}
