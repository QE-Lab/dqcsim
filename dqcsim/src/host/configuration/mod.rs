//! Configuration structures for the plugins and simulator.

mod env_mod;
pub use env_mod::EnvMod;

mod stream_capture_mode;
pub use stream_capture_mode::StreamCaptureMode;

mod seed;
pub use seed::Seed;

mod timeout;
pub use timeout::Timeout;

mod plugin_process;
pub use plugin_process::{
    PluginConfiguration, PluginFunctionalConfiguration, PluginNonfunctionalConfiguration,
    PluginSpecification,
};

mod plugin_log;
pub use plugin_log::PluginLogConfiguration;

mod simulator;
pub use simulator::SimulatorConfiguration;

// TODO: move me to common::types
use serde::{Deserialize, Serialize};
/// Enumeration of the three types of plugins.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum PluginType {
    Frontend,
    Operator,
    Backend,
}
