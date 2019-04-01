use crate::{common::error::Result, host::{configuration::PluginConfiguration, plugin::Plugin}};
use std::thread;

pub struct PluginThread {
    thread: Option<Box<dyn Fn(String) -> () + Send>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl PluginThread {
    pub fn new(thread: impl Fn(String) -> () + Send + 'static) -> PluginThread {
        PluginThread {
            thread: Some(Box::new(thread)),
            handle: None,
        }
    }
}

impl Plugin for PluginThread {
    fn configuration(&self) -> &PluginConfiguration {
        &PluginConfiguration::default()
    }
    fn spawn(&mut self, simulator: String) -> Result<()> {
        let thread = self.thread.take().unwrap();
        self.handle = Some(thread::spawn(move || thread(simulator)));
        Ok(())
    }
}

impl Drop for PluginThread {
    fn drop(&mut self) {}
}
