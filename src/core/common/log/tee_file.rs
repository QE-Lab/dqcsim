use crate::common::{
    error::Result,
    log::{Log, LogRecord, Loglevel, LoglevelFilter},
};
use failure::Fail;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::PathBuf};

/// Error structure used for reporting TeeFile errors.
#[derive(Debug, Fail, PartialEq)]
pub enum TeeFileError {
    #[fail(display = "{}", 0)]
    ParseError(String),
}

/// Represents a tee file configuration for the logging system.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TeeFileConfiguration {
    pub filter: LoglevelFilter,
    pub file: PathBuf,
}

impl TeeFileConfiguration {
    /// Constructs a new TeeFileConfiguration with the provided log level
    /// filter and path.
    pub fn new(
        filter: impl Into<LoglevelFilter>,
        file: impl Into<PathBuf>,
    ) -> TeeFileConfiguration {
        TeeFileConfiguration {
            filter: filter.into(),
            file: file.into(),
        }
    }
}

/// TeeFile is the combination of a TeeFileConfiguration and a handle to the
/// tee file. TeeFile implements the Log trait and can write log records to
/// the file.
#[derive(Debug)]
pub struct TeeFile {
    /// The TeeFileConfiguration
    pub configuration: TeeFileConfiguration,
    /// The file handle, wrapper in an Option to allow implementation of the
    /// Log trait.
    buffer: Option<File>,
}

impl TeeFile {
    /// Constructs a new tee file. Consumes the provided configuration.
    pub fn new(configuration: TeeFileConfiguration) -> Result<TeeFile> {
        let buffer = Some(File::create(&configuration.file)?);
        Ok(TeeFile {
            buffer,
            configuration,
        })
    }
}

impl Log for TeeFile {
    fn name(&self) -> &str {
        self.configuration.file.to_str().unwrap()
    }
    fn enabled(&self, level: Loglevel) -> bool {
        LoglevelFilter::from(level) <= self.configuration.filter
    }
    fn log(&self, record: &LogRecord) {
        if let Some(mut buffer) = self.buffer.as_ref() {
            writeln!(buffer, "{}", record).expect("Failed to write to file");
        }
    }
}

impl ::std::str::FromStr for TeeFileConfiguration {
    type Err = failure::Error;

    /// Constructs a TeeFile from its string representation, which is of the
    /// form <filter>:<file>. <filter> is parsed by
    /// `LoglevelFilter::from_str()` and thus supports abbreviations.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut splitter = s.splitn(2, ':');
        let filter = LoglevelFilter::from_str(splitter.next().unwrap())?;
        let file: PathBuf = splitter
            .next()
            .ok_or_else(|| {
                TeeFileError::ParseError("expected a colon in tee file description".to_string())
            })?
            .into();
        Ok(TeeFileConfiguration { filter, file })
    }
}

impl ::std::fmt::Display for TeeFileConfiguration {
    /// Turns the TeeFile object into a string representation that can be
    /// parsed by `from_str()`.
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}:{}", self.filter, self.file.to_str().unwrap())
    }
}

#[cfg(test)]
mod test {

    use super::super::{Log, LogRecord, Loglevel, LoglevelFilter};
    use super::*;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        assert_eq!(
            TeeFileConfiguration::from_str("info:/tmp/hello:/there").unwrap(),
            TeeFileConfiguration::new(LoglevelFilter::Info, "/tmp/hello:/there"),
        );

        let tfc = TeeFileConfiguration::from_str("hello");
        assert!(tfc.is_err());
        assert_eq!(tfc.unwrap_err().to_string(), "hello is not a valid loglevel filter, valid values are off, fatal, error, warn, note, info, debug, or trace");

        let tfc = TeeFileConfiguration::from_str("info");
        assert!(tfc.is_err());
        assert_eq!(
            tfc.unwrap_err().to_string(),
            "expected a colon in tee file description"
        );
    }

    #[test]
    fn to_str() {
        assert_eq!(
            TeeFileConfiguration::new(LoglevelFilter::Info, "/tmp/hello:/there").to_string(),
            "Info:/tmp/hello:/there",
        );
    }

    #[test]
    fn debug() {
        let tf = TeeFileConfiguration::new(LoglevelFilter::Info, "hello:/there");
        assert_eq!(
            format!("{:?}", tf),
            "TeeFileConfiguration { filter: Info, file: \"hello:/there\" }"
        );

        let tfc = TeeFileConfiguration::new(LoglevelFilter::Trace, "/dev/zero");
        let tf = TeeFile::new(tfc);

        assert!(
            format!("{:?}", tf.unwrap()).starts_with("TeeFile { configuration: TeeFileConfiguration { filter: Trace, file: \"/dev/zero\" }, buffer: Some(File {")
        );
    }

    #[test]
    fn with_clone() {
        let tf = TeeFileConfiguration::new(LoglevelFilter::Info, "/tmp/log.info");
        assert_eq!(
            format!("{:?}", tf),
            "TeeFileConfiguration { filter: Info, file: \"/tmp/log.info\" }"
        );
        let _ = tf.clone();
    }

    #[test]
    fn pub_fields() {
        let tf = TeeFileConfiguration::new(LoglevelFilter::Info, "/tmp/log.info");
        assert_eq!(tf.filter, LoglevelFilter::Info);
        assert_eq!(tf.file, PathBuf::from("/tmp/log.info"));
    }

    #[test]
    fn log() {
        let tfc = TeeFileConfiguration::new(LoglevelFilter::Info, "/tmp/log.info");
        let tf = TeeFile::new(tfc);
        assert!(tf.is_ok());
        let tf = tf.unwrap();
        assert_eq!(tf.name(), "/tmp/log.info");
        assert!(tf.enabled(Loglevel::Info));
        assert!(!tf.enabled(Loglevel::Trace));
        let record = LogRecord::new(
            "logger",
            "message",
            Loglevel::Trace,
            "path",
            "file",
            1234u32,
            1u32,
            1u64,
        );
        tf.log(&record);
    }

}
