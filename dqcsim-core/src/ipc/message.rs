use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum PluginControl {
    Start,
    Abort,
}
