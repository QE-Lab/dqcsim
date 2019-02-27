use std::{str::FromStr, string::ParseError};

#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub name: String,
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
