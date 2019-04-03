use crate::{common::error::Result, host::plugin::Plugin};

pub mod log;
pub mod process;
pub mod thread;

/// Trait for types of configurations. The only thing that needs to be
/// implemented is a way to instantiate a Plugin from it.
pub trait PluginConfiguration {
    /// Instantiates the plugin.
    fn instantiate(self) -> Result<Box<dyn Plugin>>;
}
