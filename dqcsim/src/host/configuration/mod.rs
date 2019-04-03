//! Configuration structures for the plugins and simulator.

mod env_mod;
pub use env_mod::EnvMod;

mod stream_capture_mode;
pub use stream_capture_mode::StreamCaptureMode;

mod seed;
pub use seed::Seed;

mod timeout;
pub use timeout::Timeout;

mod plugin;
pub use plugin::{
    log::PluginLogConfiguration,
    process::{
        PluginProcessConfiguration, PluginProcessFunctionalConfiguration,
        PluginProcessNonfunctionalConfiguration, PluginProcessSpecification,
    },
    thread::PluginThreadConfiguration,
    PluginConfiguration,
};

mod simulator;
pub use simulator::SimulatorConfiguration;
