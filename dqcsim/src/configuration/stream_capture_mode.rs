use enum_variants::EnumVariants;
use failure::Error;
use serde::{Deserialize, Serialize};

use crate::log::Loglevel;

/// All loglevel options plus pass and null, used to specify how a
/// stdout/stderr stream should be captured.
#[derive(EnumVariants, Debug, Copy, Clone, PartialEq)]
enum StreamCaptureOption {
    Pass,
    Null,
    Fatal,
    Error,
    Warn,
    Note,
    Info,
    Debug,
    Trace,
}

/// Stream capture mode.
///
/// Specifies how a plugin stdout/stderr stream should be captured.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum StreamCaptureMode {
    /// Don't capture the stream. That is, let it pass through to DQCsim's
    /// stdout/stderr stream unchecked.
    Pass,

    /// Disable the stream by piping it to /dev/null (or by emulating this
    /// behavior).
    Null,

    /// Capture the stream to turn each line into a log message with the
    /// specified level.
    Capture(Loglevel),
}

impl ::std::str::FromStr for StreamCaptureMode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match StreamCaptureOption::from_str(s)? {
            StreamCaptureOption::Pass => StreamCaptureMode::Pass,
            StreamCaptureOption::Null => StreamCaptureMode::Null,
            StreamCaptureOption::Fatal => StreamCaptureMode::Capture(Loglevel::Fatal),
            StreamCaptureOption::Error => StreamCaptureMode::Capture(Loglevel::Error),
            StreamCaptureOption::Warn => StreamCaptureMode::Capture(Loglevel::Warn),
            StreamCaptureOption::Note => StreamCaptureMode::Capture(Loglevel::Note),
            StreamCaptureOption::Info => StreamCaptureMode::Capture(Loglevel::Info),
            StreamCaptureOption::Debug => StreamCaptureMode::Capture(Loglevel::Debug),
            StreamCaptureOption::Trace => StreamCaptureMode::Capture(Loglevel::Trace),
        })
    }
}
