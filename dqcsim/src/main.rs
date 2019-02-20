use dqcsim_core::{plugin, process};
use env_logger::{Builder, Env};
use log::{debug, error, info, trace, warn, Level};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Set logging verbosity to <loglevel>, which must be trace, debug,
    /// info, warn or error.
    #[structopt(short = "l", long = "loglevel", group = "log")]
    loglevel: Option<Level>,
    /// Plugin configurations.
    #[structopt(raw(required = "true", min_values = "1"), parse(try_from_str))]
    plugins: Vec<plugin::PluginConfig>,
}

fn main() -> Result<(), ()> {
    let opt = Opt::from_args();
    dbg!(&opt);

    // Setup logger
    Builder::from_env(
        Env::default().default_filter_or(opt.loglevel.unwrap_or(Level::Debug).to_string()),
    )
    .init();

    // Test log levels
    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");

    // Create plugins from PluginConfigs
    let plugins: Vec<plugin::Plugin> = opt
        .plugins
        .into_iter()
        .map(|config| plugin::Plugin::from(config))
        .collect();

    plugins.iter().for_each(|plugin| plugin.init());

    Ok(())
}
