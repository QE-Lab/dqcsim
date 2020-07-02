use serde::{Deserialize, Serialize};
use std::{
    env::current_dir,
    io,
    path::{Path, PathBuf},
};
use strum_macros::{Display, EnumIter, EnumString};

/// Represents the style for storing paths in a reproduction file.
#[derive(Display, EnumIter, EnumString, Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum ReproductionPathStyle {
    /// Specifies that paths should be saved the same way they were specified
    /// on the command line.
    #[strum(to_string = "Keep", serialize = "keep", serialize = "k")]
    Keep,

    /// Specifies that all paths should be saved relative to DQCsim's working
    /// directory.
    #[strum(to_string = "Relative", serialize = "relative", serialize = "r")]
    Relative,

    /// Specifies that all paths should be saved canonically, i.e. relative to
    /// the root directory.
    #[strum(to_string = "Absolute", serialize = "absolute", serialize = "a")]
    Absolute,
}

impl ReproductionPathStyle {
    /// Converts a path as specified by the underlying `ReproductionPathStyle`.
    ///
    /// Calls `std::env::current_dir()` if the style is `Relative` to get the
    /// base for the relative path.
    pub fn convert_path(self, path: &Path) -> io::Result<PathBuf> {
        match self {
            ReproductionPathStyle::Keep => Ok(path.into()),
            ReproductionPathStyle::Relative => {
                let workdir = current_dir()?;
                let path =
                    pathdiff::diff_paths(&path.canonicalize()?, &workdir).ok_or_else(|| {
                        io::Error::new(io::ErrorKind::NotFound, "Cannot make path relative")
                    })?;
                if path.as_os_str().is_empty() {
                    Ok(PathBuf::from("."))
                } else {
                    Ok(path)
                }
            }
            ReproductionPathStyle::Absolute => Ok(path.canonicalize()?),
        }
    }

    /// Convenience function that applies `convert_path()` on the contents of
    /// an `Option`.
    pub fn convert_path_option(self, path: &Option<PathBuf>) -> io::Result<Option<PathBuf>> {
        if let Some(path) = path.as_ref() {
            Ok(Some(self.convert_path(path)?))
        } else {
            Ok(None)
        }
    }
}
