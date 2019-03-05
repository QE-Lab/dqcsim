use std::{str::FromStr, string::ParseError};
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct PluginConfig {
    /// Plugin name.
    #[structopt(short = "n", long = "name")]
    pub name: String,
    /// Set logging verbosity to <loglevel>
    /// [OFF, ERROR, WARN, INFO, DEBUG, TRACE]
    #[structopt(short = "l", long = "loglevel")]
    pub loglevel: Option<log::LevelFilter>,
    /// Simulator server address.
    #[structopt(short = "s", long = "server")]
    pub server: Option<String>,
}

impl FromStr for PluginConfig {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<PluginConfig, ParseError> {
        Ok(PluginConfig {
            name: s.to_owned(),
            loglevel: Some(log::LevelFilter::Trace),
            server: None,
        })
    }
}
