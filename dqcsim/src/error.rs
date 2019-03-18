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

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    /// Simulator errors
    #[fail(display = "Simulator failed")]
    SimulatorError,

    /// Constructor error if constructor failed
    #[fail(display = "constructor failed: {}", _0)]
    ConstructFailed(String),

    // LogError
    #[fail(display = "log error: {}", _0)]
    LogError(String),

    // Broken state
    #[fail(display = "simulator in broken state: {}", _0)]
    BrokenState(String),

    // Invalid operation
    #[fail(display = "invalid operation: {}", _0)]
    InvalidOperation(String),

    // crossbeam_channel erro
    #[fail(display = "channel operation failed: {}", _0)]
    ChannelError(String),

    // std::io::Error
    #[fail(display = "io error: {}", _0)]
    IoError(String),

    // term::Error
    #[fail(display = "term error: {}", _0)]
    TermError(String),

    // Other
    #[fail(display = "error: {}", _0)]
    Other(String),
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

/// Custom errors

/// std::io::Error
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error {
            ctx: Context::new(ErrorKind::IoError(format!("{}", error))),
        }
    }
}

/// term::Error
impl From<term::Error> for Error {
    fn from(error: term::Error) -> Error {
        Error {
            ctx: Context::new(ErrorKind::TermError(format!("{}", error))),
        }
    }
}

impl<T> From<crossbeam_channel::SendError<T>> for Error {
    fn from(error: crossbeam_channel::SendError<T>) -> Error {
        Error {
            ctx: Context::new(ErrorKind::ChannelError(format!("{}", error))),
        }
    }
}

impl From<ipc_channel::Error> for Error {
    fn from(error: ipc_channel::Error) -> Error {
        Error {
            ctx: Context::new(ErrorKind::ChannelError(format!("{}", error))),
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
