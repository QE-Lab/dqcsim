use crate::{
    common::{
        channel::SimulatorChannel,
        error::Result,
        log::thread::LogThread,
        protocol::{PluginToSimulator, SimulatorToPlugin},
        types::{ArbCmd, PluginType},
    },
    fatal,
    host::{
        configuration::{PluginLogConfiguration, PluginThreadConfiguration},
        plugin::Plugin,
    },
    plugin::state::PluginState,
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
        let definition = configuration.definition;
        let plugin_type = definition.get_type();
        PluginThread::new_raw(
            move |server| {
                PluginState::run(&definition, server).unwrap();
                trace!("$");
            },
            plugin_type,
            configuration.init_cmds,
            configuration.log_configuration,
        )
    }

    /// Construct a plugin thread given the actual function that runs in the
    /// spawned thread and the configuration. This is to be used for testing
    /// only.
    fn new_raw(
        thread: impl Fn(String) -> () + Send + 'static,
        plugin_type: PluginType,
        init_cmds: Vec<ArbCmd>,
        log_configuration: PluginLogConfiguration,
    ) -> PluginThread {
        PluginThread {
            thread: Some(Box::new(thread)),
            handle: None,
            channel: None,
            plugin_type,
            init_cmds,
            log_configuration,
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
            // Set a custom panic hook.
            std::panic::set_hook(Box::new(|info| {
                fatal!("{}", info);
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

impl Drop for PluginThread {
    fn drop(&mut self) {}
}
