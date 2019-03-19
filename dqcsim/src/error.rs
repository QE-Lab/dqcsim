//! Error-handling.
//!
//! Types for error-handling in this crate, based on the [`failure`] crate.
//!
//! [`Error`] is the wrapper which implements [`Fail`] and containers the inner
//! [`ErrorKind`] and its [`Context`].
//!
//! [`failure`]: ../../failure/index.html
//! [`Error`]: ./struct.Error.html
//! [`ErrorKind`]: ./enum.ErrorKind.html
//! [`Fail`]: ../../failure/trait.Fail.html
//! [`Context`]: ../../failure/struct.Context.html

use failure::{Backtrace, Context, Fail};
use std::{fmt, fmt::Display, result};

/// Internal [`Result`] type which uses the crate's [`Error`] type.
///
/// [`Error`]: ./struct.Error.html
pub type Result<T> = result::Result<T, Error>;

/// Re-export the [`ResultExt`] trait which adds the [`Context`] methods to
/// [`Result`].
///
/// [`ResultExt`]: ../../failure/trait.ResultExt.html
pub use failure::ResultExt;

/// [`Error`] type for this crate.
///
/// Implements [`Fail`].
///
/// [`Error`]: ./struct.Error.html
/// [`Fail`]: ../../failure/trait.Fail.html
#[derive(Debug)]
pub struct Error {
    /// [`Context`] which contains the [`ErrorKind`].
    ///
    /// [`Context`]: ../../failure/struct.Context.html
    /// [`ErrorKind`]: ./enum.ErrorKind.html
    ctx: Context<ErrorKind>,
}

#[derive(PartialEq, Debug, Fail)]
pub enum ErrorKind {
    // TODO: remove
    #[fail(display = "{}", _0)]
    SimulatorConfigurationError(SimulatorConfigurationError),

    /// LogError
    #[fail(display = "Log error: {}", _0)]
    LogError(String),

    /// Generic invalid argument: use when a function is called in a way it
    /// shouldn't be.
    #[fail(display = "Invalid argument: {}", _0)]
    InvalidArgument(String),

    /// Generic invalid operation: use when a function is called while it
    /// shouldn't be.
    #[fail(display = "Invalid operation: {}", _0)]
    InvalidOperation(String),

    /// Generic error: use when an error doesn't fit in the above categories
    /// and you're too lazy to define one properly.
    #[fail(display = "Error: {}", _0)]
    Other(String),

    /// Wraps multiple errors that occurred asynchronously.
    #[fail(display = "Multiple errors occurred. Check the log.")]
    Multiple(Vec<Box<ErrorKind>>),

    /// For propagating crossbeam_channel errors.
    #[fail(display = "Inter-thread communication error: {}", _0)]
    ITCError(String),

    /// For propagating ipc_channel errors.
    #[fail(display = "Interprocess communication error: {}", _0)]
    IPCError(String),

    /// For propagating std::io::Error errors.
    #[fail(display = "I/O error: {}", _0)]
    IoError(String, std::io::ErrorKind),

    /// For propagating term::Error errors.
    #[fail(display = "Terminal error: {}", _0)]
    TermError(String, term::Error),
}

/// Shorthand for producing a LogError.
pub fn log_err<T>(s: impl Into<String>) -> Result<T> {
    Err(ErrorKind::LogError(s.into()).into())
}

/// Shorthand for producing an invalid argument error.
pub fn inv_arg<T>(s: impl Into<String>) -> Result<T> {
    Err(ErrorKind::InvalidArgument(s.into()).into())
}

/// Shorthand for producing an invalid operation error.
pub fn inv_op<T>(s: impl Into<String>) -> Result<T> {
    Err(ErrorKind::InvalidOperation(s.into()).into())
}

/// Shorthand for producing an error that does not fit in any of the ErrorKind
/// classes.
pub fn err<T>(s: impl Into<String>) -> Result<T> {
    Err(ErrorKind::Other(s.into()).into())
}

// TODO: remove
#[derive(Debug, Clone, Fail, PartialEq, Eq)]
pub enum SimulatorConfigurationError {
    #[fail(display = "Duplicate frontend plugin")]
    DuplicateFrontend,
    #[fail(display = "Duplicate backend plugin")]
    DuplicateBackend,
    #[fail(display = "Missing frontend plugin")]
    MissingFrontend,
    #[fail(display = "Missing backend plugin")]
    MissingBackend,
    #[fail(display = "Duplicate plugin name '{}'", 0)]
    DuplicateName(String),
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ctx.fmt(f)
    }
}

impl From<ErrorKind> for Error {
    fn from(ctx: ErrorKind) -> Error {
        Error {
            ctx: Context::new(ctx),
        }
    }
}

impl From<Context<String>> for Error {
    fn from(ctx: Context<String>) -> Error {
        Error {
            ctx: ctx.map(ErrorKind::Other),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Error {
        Error { ctx }
    }
}

impl From<SimulatorConfigurationError> for Error {
    fn from(error: SimulatorConfigurationError) -> Error {
        Error {
            ctx: Context::new(ErrorKind::SimulatorConfigurationError(error)),
        }
    }
}

/// Custom errors

/// std::io::Error
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        let msg = error.to_string();
        Error {
            ctx: Context::new(ErrorKind::IoError(msg, error.kind())),
        }
    }
}

/// term::Error
impl From<term::Error> for Error {
    fn from(error: term::Error) -> Error {
        let msg = error.to_string();
        Error {
            ctx: Context::new(ErrorKind::TermError(msg, error)),
        }
    }
}

impl<T> From<crossbeam_channel::SendError<T>> for Error {
    fn from(error: crossbeam_channel::SendError<T>) -> Error {
        let msg = error.to_string();
        Error {
            ctx: Context::new(ErrorKind::ITCError(msg)),
        }
    }
}

impl From<ipc_channel::Error> for Error {
    fn from(error: ipc_channel::Error) -> Error {
        let msg = error.to_string();
        Error {
            ctx: Context::new(ErrorKind::IPCError(msg)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn add() {
        assert_eq!(2 + 2, 4);
    }
}
