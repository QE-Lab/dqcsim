use std::{str::FromStr, string::ParseError};

#[derive(Debug)]
pub struct PluginConfig {
    pub name: String,
}

impl FromStr for PluginConfig {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<PluginConfig, ParseError> {
        Ok(PluginConfig { name: s.to_owned() })
    }
}
