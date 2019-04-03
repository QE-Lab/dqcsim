use serde::{Deserialize, Serialize};

/// Enumeration of the three types of plugins.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum PluginType {
    Frontend,
    Operator,
    Backend,
}
