use dqcsim_core::plugin;
use dqcsim_log::LogThread;
use log::debug;
use slog::Level;
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

    // Setup logging
    dqcsim_log::init(log::LevelFilter::Trace).expect("Failed to initialize logger.");
    let logger = LogThread::default();

    // Debug message with parsed Opt struct
    debug!("Parsed arguments: {:?}", &opt);

    // Create plugins from PluginConfigs
    let plugins: Vec<plugin::Plugin> = opt
        .plugins
        .into_iter()
        .map(|config| plugin::Plugin::new(config, &logger))
        .collect();
    for plugin in &plugins {
        plugin.init().expect("init failed");
    }

    Ok(())
}
