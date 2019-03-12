use dqcsim::util::{
    log,
    log::{debug, info, LevelFilter},
};
use dqcsim::{plugin, util::log::thread::LogThread, util::signal};
use failure::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Set logging verbosity to <loglevel>
    /// [OFF, ERROR, WARN, INFO, DEBUG, TRACE]
    #[structopt(short = "l", long = "loglevel")]
    loglevel: Option<LevelFilter>,

    /// Plugin configurations.
    #[structopt(raw(required = "true", min_values = "1"))]
    plugins: Vec<plugin::config::PluginConfig>,
}

fn main() -> Result<(), Error> {
    // Parse arguments
    let opt = Opt::from_args();

    // Setup logging
    let logger = LogThread::spawn(opt.loglevel.unwrap_or(LevelFilter::Trace));

    // Setup signal hook
    let _signal = signal::notify(&[
        signal_hook::SIGTERM,
        signal_hook::SIGINT,
        signal_hook::SIGQUIT,
    ])
    .expect("Failed to initialize signal hook");

    info!(
        "Starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    // Debug message with parsed Opt struct
    debug!("Parsed arguments: {:?}", &opt);

    log::fatal!("fatal");
    log::error!("error");
    log::warn!("warn");
    log::note!("note");
    log::info!("info");
    log::debug!("debug");
    log::trace!("trace");

    let _simulator = dqcsim::simulator::Simulation::new();

    // Create plugins from PluginConfigs
    let plugins: Vec<Result<plugin::Plugin, Error>> = opt
        .plugins
        .into_iter()
        .map(|config| plugin::Plugin::new(config, &logger, None))
        .collect();
    for plugin in plugins {
        plugin?.init().expect("init failed");
    }

    Ok(())
}
