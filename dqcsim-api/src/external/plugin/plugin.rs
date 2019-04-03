use super::*;

// TODO: add function to run plugin in the current thread

/// Sends a log message using the specified plugin's logger.
///
/// `message` and `level` are mandatory, the rest of the entries can be `NULL`
/// or 0 if they are unknown.
#[no_mangle]
pub extern "C" fn dqcs_plugin_log(
    plugin: dqcs_plugin_state_t,
    message: *const c_char,
    level: dqcs_loglevel_t,
    module: *const c_char,
    file: *const c_char,
    line_nr: u32,
) -> dqcs_return_t {
    api_return_none(|| {
        let plugin = plugin.resolve()?;
        let line_nr = if line_nr == 0 { None } else { Some(line_nr) };
        plugin.log(
            receive_str(message)?,
            level.into_loglevel()?,
            receive_optional_str(module)?,
            receive_optional_str(file)?,
            line_nr,
        );
        Ok(())
    })
}

// TODO: add functions for the other APIs
