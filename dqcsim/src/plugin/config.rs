use std::{str::FromStr, string::ParseError};

#[derive(Debug)]
pub struct PluginConfig {
    /// Plugin name.
    pub name: String,
    /// Set logging verbosity to <loglevel>
    /// [OFF, ERROR, WARN, INFO, DEBUG, TRACE]
    pub loglevel: Option<log::LevelFilter>,
}

impl FromStr for PluginConfig {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<PluginConfig, ParseError> {
        Ok(PluginConfig {
            name: s.to_owned(),
            loglevel: Some(log::LevelFilter::Trace),
        })
    }
}
