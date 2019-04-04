use super::*;
use dqcsim::log;

/// Primitive API for sending a log message using the current logger.
///
/// Returns `DQCS_SUCCESS` if logging was successful, or `DQCS_FAILURE` if no
/// logger is available in the current thread or one of the arguments could not
/// be converted. Loggers are available in the simulation host thread and in
/// threads running plugins.
///
/// ## Formatting and fallback to stderr
///
/// As an alternative to this function, you can also use `dqcs_log_format()`.
/// This function differs from `dqcs_log_raw()` in two ways:
///
///  - Instead of the `message` string, a printf-style format string and
///    associated varargs are passed to construct the message.
///  - When logging fails, this function falls back to writing to `stderr`
///    instead of returning the errors.
///
/// ## Macros
///
/// From C and C++, these functions are normally not called directly. Instead,
/// the following macros are used:
///
/// ```C
/// dqcs_log_trace("trace message!");
/// dqcs_log_debug("debug message!");
/// dqcs_log_info("info message!");
/// dqcs_log_note("notice!");
/// dqcs_log_warn("warning!");
/// dqcs_log_error("error!");
/// dqcs_log_fatal("fatal error!");
/// ```
///
/// These macros automatically set `file` to the C source filename and `line`
/// to the line number. `module` is hardcoded to "C" or "CPP" depending on
/// source file language. They use `dqcs_log_format()`, so they also support
/// printf-style formatting. For instance:
///
/// ```C
/// dqcs_note("answer to %s: %d", "ultimate question", 42);
/// ```
#[no_mangle]
pub extern "C" fn dqcs_log_raw(
    level: dqcs_loglevel_t,
    module: *const c_char,
    file: *const c_char,
    line_nr: u32,
    message: *const c_char,
) -> dqcs_return_t {
    api_return_none(|| {
        let message = receive_str(message)?;
        let module = receive_optional_str(module)?.unwrap_or_else(|| "unknown");
        let file = receive_optional_str(file)?.unwrap_or_else(|| "unknown");
        let level = level.into_loglevel()?;
        if log!(?
            target: module,
            location: (file, line_nr),
            level, "{}", message
        ) {
            Ok(())
        } else {
            inv_op("no logger available")
        }
    })
}
