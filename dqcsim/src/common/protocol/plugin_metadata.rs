use serde::{Deserialize, Serialize};
use std::fmt;

/// Contains information about a plugin implementation.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// The name of the plugin implementation.
    name: String,

    /// The author of the plugin.
    author: String,

    /// The plugin version.
    version: String,
}

impl PluginMetadata {
    /// Constructs a new plugin metadata record.
    pub fn new(
        name: impl Into<String>,
        author: impl Into<String>,
        version: impl Into<String>,
    ) -> PluginMetadata {
        PluginMetadata {
            name: name.into(),
            author: author.into(),
            version: version.into(),
        }
    }

    /// Returns the name of the plugin implementation.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the author of the plugin.
    pub fn get_author(&self) -> &str {
        &self.author
    }

    /// Returns the plugin version.
    pub fn get_version(&self) -> &str {
        &self.version
    }
}

impl fmt::Display for PluginMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} version {} by {}",
            self.name, self.author, self.version
        )
    }
}
