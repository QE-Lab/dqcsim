use dqcsim_core::plugin;
use log::{debug, error, info, trace, warn, Level};
use slog::Drain;
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
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, slog::slog_o!("version" => env!("CARGO_PKG_VERSION")));
    // let log = slog::Logger::root(drain, slog::o!());
    let _scope_guard = slog_scope::set_global_logger(logger);
    let _log_guard = slog_stdlog::init().unwrap();

    // slog::info!(logger, "Logging ready!");

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
