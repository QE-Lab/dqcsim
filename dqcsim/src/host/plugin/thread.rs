use crate::{
    common::{
        channel::CrossbeamChannel,
        error::Result,
        log::{thread::LogThread, LogRecord},
        protocol::{PluginToSimulator, SimulatorToPlugin},
        types::{ArbCmd, PluginType},
    },
    host::{
        configuration::{PluginLogConfiguration, PluginThreadConfiguration},
        plugin::Plugin,
    },
};
use std::{fmt, thread};

pub type PluginThreadClosure = Box<
    dyn Fn(
            CrossbeamChannel<PluginToSimulator, SimulatorToPlugin>,
            crossbeam_channel::Sender<LogRecord>,
        ) -> ()
        + Send,
>;

pub struct PluginThread {
    thread: Option<PluginThreadClosure>,
    handle: Option<thread::JoinHandle<()>>,
    channel: Option<CrossbeamChannel<SimulatorToPlugin, PluginToSimulator>>,
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
            move |_, _| {
                let _ = definition;
                // TODO start controller, stuff, etc
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
        thread: impl Fn(
                CrossbeamChannel<PluginToSimulator, SimulatorToPlugin>,
                crossbeam_channel::Sender<LogRecord>,
            ) -> ()
            + Send
            + 'static,
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
    fn spawn(&mut self, logger: &LogThread) -> Result<()> {
        let thread = self.thread.take().unwrap();
        let (request_tx, request) = crossbeam_channel::unbounded();
        let (response, response_rx) = crossbeam_channel::unbounded();
        let logger = logger.get_sender();
        self.handle = Some(thread::spawn(move || thread((response, request), logger)));
        self.channel = Some((request_tx, response_rx));
        Ok(())
    }

    /// Send the SimulatorToPlugin message to the plugin.
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
