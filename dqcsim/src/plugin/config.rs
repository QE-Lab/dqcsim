use crate::util::log::LevelFilter;
use std::{str::FromStr, string::ParseError};

#[derive(Debug)]
pub struct PluginConfig {
    /// Plugin name.
    pub name: String,
    /// Set logging verbosity to <loglevel>
    /// [OFF, ERROR, WARN, INFO, DEBUG, TRACE]
    pub loglevel: Option<LevelFilter>,
}

impl FromStr for PluginConfig {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<PluginConfig, ParseError> {
        Ok(PluginConfig {
            name: s.to_owned(),
            loglevel: Some(LevelFilter::Trace),
        })
    }
}
