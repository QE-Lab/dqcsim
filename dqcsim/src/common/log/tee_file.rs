use crate::common::log::{Log, LogRecord, Loglevel, LoglevelFilter};
use failure::Fail;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::PathBuf};

/// Error structure used for reporting TeeFile errors.
#[derive(Debug, Fail, PartialEq)]
pub enum TeeFileError {
    #[fail(display = "{}", 0)]
    ParseError(String),
}

/// Represents a tee file for the logging system.
#[derive(Debug, Deserialize, Serialize)]
pub struct TeeFile {
    pub filter: LoglevelFilter,
    pub file: PathBuf,
    #[serde(skip)]
    buffer: Option<File>,
}

impl Clone for TeeFile {
    fn clone(&self) -> TeeFile {
        TeeFile::new(self.filter, self.file.clone())
    }
}

impl PartialEq for TeeFile {
    fn eq(&self, other: &TeeFile) -> bool {
        self.filter == other.filter && self.file == other.file
    }
}

impl TeeFile {
    /// Convenience method for building a TeeFile. This is the combinatin of build + create.
    pub fn new(filter: impl Into<LoglevelFilter>, file: impl Into<PathBuf>) -> TeeFile {
        let file = file.into();
        TeeFile {
            filter: filter.into(),
            file: file.clone(),
            buffer: Some(File::create(file).unwrap()),
        }
    }
    /// Create the File.
    pub fn create(mut self) -> TeeFile {
        self.buffer = Some(File::create(self.file.clone()).unwrap());
        self
    }
    /// Build the TeeFile, but do not create the inner File.
    pub fn build(filter: impl Into<LoglevelFilter>, file: impl Into<PathBuf>) -> TeeFile {
        TeeFile {
            filter: filter.into(),
            file: file.into(),
            buffer: None,
        }
    }
}

impl Log for TeeFile {
    fn name(&self) -> &str {
        self.file.to_str().unwrap()
    }
    fn enabled(&self, level: Loglevel) -> bool {
        LoglevelFilter::from(level) <= self.filter
    }
    fn log(&self, record: &LogRecord) {
        if let Some(mut buffer) = self.buffer.as_ref() {
            writeln!(buffer, "{}", record).expect("Failed to write to file");
        }
    }
}

impl ::std::str::FromStr for TeeFile {
    type Err = failure::Error;

    /// Constructs a TeeFile from its string representation, which is of the
    /// form <filter>:<file>. <filter> is parsed by
    /// `LoglevelFilter::from_str()` and thus supports abbreviations.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitter = s.splitn(2, ':');
        let filter = LoglevelFilter::from_str(splitter.next().unwrap())?;
        let file: PathBuf = splitter
            .next()
            .ok_or_else(|| {
                TeeFileError::ParseError("Expected a colon in tee file description.".to_string())
            })?
            .into();
        Ok(TeeFile::build(filter, file))
    }
}

impl ::std::fmt::Display for TeeFile {
    /// Turns the TeeFile object into a string representation that can be
    /// parsed by `from_str()`.
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}:{}", self.filter, self.file.to_str().unwrap())
    }
}

#[cfg(test)]
mod test {

    use super::super::LoglevelFilter;
    use super::TeeFile;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        assert_eq!(
            TeeFile::from_str("info:hello:/there").unwrap(),
            TeeFile::build(LoglevelFilter::Info, "hello:/there"),
        );
    }

    #[test]
    fn to_str() {
        assert_eq!(
            TeeFile::build(LoglevelFilter::Info, "hello:/there").to_string(),
            "Info:hello:/there",
        );
    }

}
