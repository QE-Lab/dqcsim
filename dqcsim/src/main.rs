use dqcsim_core::plugin;
use log::debug;
use slog::{Drain, Level};
use std::error::Error;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug)]
pub struct ParseLevelError;
impl std::fmt::Display for ParseLevelError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ParseLevelError {
    fn description(&self) -> &str {
        "invalid log level. [Off, Critical, Error, Warning, Info, Debug, Trace]"
    }
}

fn parse_filterlevel(arg: &str) -> Result<Level, ParseLevelError> {
    match Level::from_str(arg) {
        Ok(level) => Ok(level),
        Err(_) => match usize::from_str(arg) {
            Ok(level) => match Level::from_usize(level) {
                Some(level) => Ok(level),
                None => Err(ParseLevelError),
            },
            Err(_) => Err(ParseLevelError),
        },
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// Set logging verbosity to <loglevel>
    /// [Off, Critical, Error, Warning, Info, Debug, Trace]
    #[structopt(
        short = "l",
        long = "loglevel",
        parse(try_from_str = "parse_filterlevel")
    )]
    loglevel: Option<Level>,

    /// Plugin configurations.
    #[structopt(raw(required = "true", min_values = "1"))]
    plugins: Vec<plugin::config::PluginConfig>,
}

fn main() -> Result<(), ()> {
    // Parse arguments
    let opt = Opt::from_args();

    // Setup logger
    let logger = slog::Logger::root(
        slog_async::Async::new(
            slog_term::CompactFormat::new(slog_term::TermDecorator::new().build())
                .build()
                .fuse(),
        )
        .build()
        // Default to Trace logging for now
        .filter_level(opt.loglevel.unwrap_or(slog::Level::Trace))
        .fuse(),
        slog::slog_o!("name" => env!("CARGO_PKG_NAME"), "version" => env!("CARGO_PKG_VERSION")),
    );
    let _scope_guard = slog_scope::set_global_logger(logger);
    let _log_guard = slog_stdlog::init().unwrap();

    // Debug message with parsed Opt struct
    debug!("Parsed arguments: {:#?}", &opt);

    // Create plugins from PluginConfigs
    let plugins: Vec<plugin::Plugin> = opt
        .plugins
        .into_iter()
        .map(|config| plugin::Plugin::from(config))
        .collect();

    plugins
        .into_iter()
        .for_each(|mut plugin| plugin.init().expect("Plugin init failed."));

    Ok(())
}
