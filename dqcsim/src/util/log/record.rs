use log::Level;
use serde::{Deserialize, Serialize};
use std::fmt;

// I had to add this to overcome problems with the deserialization of log levels.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Level")]
enum LevelDef {
    Error = 1,
    Warn,
    Info,
    Debug,
    Trace,
}

// #[derive(Serialize)]
// #[serde(remote = "Metadata")]
// struct MetadataDef<'a> {
//     #[serde(getter = "Metadata::level")]
//     level: Level,
//     #[serde(getter = "Metadata::target")]
//     target: &'a str,
// }

// Provide a conversion to construct the remote type.
// impl<'a> From<MetadataDef<'a>> for Metadata<'a> {
//     fn from(def: MetadataDef<'a>) -> Metadata<'a> {
//         Metadata::builder()
//             .level(def.level)
//             .target(def.target)
//             .build()
//     }
// }

// fn get_format_args<'a, 'de, D>(deserializer: D) -> Result<fmt::Arguments<'a>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     Ok(format_args!("hello"))
// }

// #[derive(Serialize)]
// #[serde(remote = "log::Record")]
// struct RecordDef<'a> {
//     #[serde(with = "MetadataDef", getter = "Record::metadata")]
//     metadata: Metadata<'a>,
//     #[serde(getter = "Record::args")]
//     args: fmt::Arguments<'a>,
//     #[serde(getter = "Record::module_path")]
//     module_path: Option<&'a str>,
//     #[serde(getter = "Record::file")]
//     file: Option<&'a str>,
//     #[serde(getter = "Record::line")]
//     line: Option<u32>,
// }
//
// // Provide a conversion to construct the remote type.
// impl<'a> From<RecordDef<'a>> for log::Record<'a> {
//     fn from(def: RecordDef<'a>) -> log::Record<'a> {
//         Record::builder()
//             .metadata(def.metadata)
//             .args(def.args)
//             .module_path(def.module_path)
//             .file(def.file)
//             .line(def.line)
//             .build()
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(with = "LevelDef")]
    level: Level,
    args: String,
    target: String,
    thread: String,
    process: u32,
    // module_path: String,
    // file: String,
    // line: u32,
}

impl Record {
    pub fn level(&self) -> &Level {
        &self.level
    }
    pub fn target(&self) -> &str {
        &self.target
    }
    pub fn thread(&self) -> &str {
        &self.thread
    }
    pub fn process(&self) -> u32 {
        self.process
    }
}

impl<'a> From<&log::Record<'a>> for Record {
    fn from(log: &log::Record<'a>) -> Record {
        Record {
            level: log.level(),
            args: log.args().to_string(),
            target: log.target().to_string(),
            process: std::process::id(),
            thread: std::thread::current()
                .name()
                .unwrap_or_default()
                .to_string(),
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:5}:{:6} {}", self.process, self.thread, self.args)
    }
}
