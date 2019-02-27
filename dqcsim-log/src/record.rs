use serde::{Deserialize, Serialize};
use std::fmt;

// #[derive(Serialize, Deserialize)]
// struct Metadata {
//     level: Level,
//     target: String,
// }

#[derive(Serialize, Deserialize)]
pub struct Record {
    level: log::Level,
    args: String,
    target: String,
    thread_id: u32,
    // module_path: String,
    // file: String,
    // line: u32,
}

impl Record {
    pub fn level(&self) -> &log::Level {
        &self.level
    }
    pub fn target(&self) -> &str {
        &self.target
    }
    pub fn thread_id(&self) -> u32 {
        self.thread_id
    }
}

impl<'a> From<&log::Record<'a>> for Record {
    fn from(log: &log::Record<'a>) -> Record {
        Record {
            level: log.level(),
            args: log.args().to_string(),
            target: std::thread::current()
                .name()
                .unwrap_or_default()
                .to_string(),
            thread_id: std::process::id(),
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.args)
    }
}
