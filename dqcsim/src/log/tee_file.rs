use crate::log::LoglevelFilter;
use failure::Fail;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Error structure used for reporting TeeFile errors.
#[derive(Debug, Fail, PartialEq)]
pub enum TeeFileError {
    #[fail(display = "{}", 0)]
    ParseError(String),
}

/// Represents a tee file for the logging system.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TeeFile {
    pub filter: LoglevelFilter,
    pub file: PathBuf,
}

impl TeeFile {
    /// Convenience method for building a TeeFile.
    pub fn new(filter: impl Into<LoglevelFilter>, file: impl Into<PathBuf>) -> TeeFile {
        TeeFile {
            filter: filter.into(),
            file: file.into(),
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
        let file = splitter
            .next()
            .ok_or_else(|| {
                TeeFileError::ParseError("Expected a colon in tee file description.".to_string())
            })?
            .into();
        Ok(TeeFile { filter, file })
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
            TeeFile::new(LoglevelFilter::Info, "hello:/there"),
        );
    }

    #[test]
    fn to_str() {
        assert_eq!(
            TeeFile::new(LoglevelFilter::Info, "hello:/there").to_string(),
            "Info:hello:/there",
        );
    }

}
