use crate::{
    common::{
        channel::CrossbeamChannel,
        error::Result,
        log::{thread::LogThread, LogRecord},
        protocol::{PluginToSimulator, SimulatorToPlugin},
    },
    host::{configuration::PluginConfiguration, plugin::Plugin},
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
    pub fn new(
        thread: impl Fn(
                CrossbeamChannel<PluginToSimulator, SimulatorToPlugin>,
                crossbeam_channel::Sender<LogRecord>,
            ) -> ()
            + Send
            + 'static,
    ) -> PluginThread {
        PluginThread {
            thread: Some(Box::new(thread)),
            handle: None,
            channel: None,
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
    fn send(&mut self, msg: SimulatorToPlugin) -> Result<()> {
        self.channel.as_ref().unwrap().0.send(msg)?;
        Ok(())
    }

    /// Receive the next PluginToSimulator message.
    fn recv(&mut self) -> Result<PluginToSimulator> {
        Ok(self.channel.as_ref().unwrap().1.recv()?)
    }

    fn configuration(&self) -> PluginConfiguration {
        unimplemented!()
    }
}

impl Drop for PluginThread {
    fn drop(&mut self) {}
}

impl Into<Box<dyn Plugin>> for PluginThread {
    fn into(self) -> Box<dyn Plugin> {
        Box::new(self) as Box<dyn Plugin>
    }
}
