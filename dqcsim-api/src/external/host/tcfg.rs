use super::*;
use dqcsim::common::log::tee_file::TeeFileConfiguration;

/// Creates a new plugin thread configuration object from a plugin definition.
///
/// The plugin definition handle is consumed by this function.
#[no_mangle]
pub extern "C" fn dqcs_tcfg_new(pdef: dqcs_handle_t, name: *const c_char) -> dqcs_handle_t {
    api_return(0, || {
        take!(pdef as PluginDefinition);
        Ok(insert(PluginThreadConfiguration::new(
            pdef,
            PluginLogConfiguration::new(receive_str(name)?, LoglevelFilter::Trace),
        )))
    })
}

/// Returns the type of the given plugin thread configuration.
#[no_mangle]
pub extern "C" fn dqcs_tcfg_type(tcfg: dqcs_handle_t) -> dqcs_plugin_type_t {
    api_return(dqcs_plugin_type_t::DQCS_PTYPE_INVALID, || {
        resolve!(tcfg as &PluginThreadConfiguration);
        Ok(tcfg.definition.get_type().into())
    })
}

/// Returns the configured name for the given plugin thread.
///
/// On success, this **returns a newly allocated string containing the
/// name. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_tcfg_name(tcfg: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(tcfg as &PluginThreadConfiguration);
        Ok(tcfg.log_configuration.name.to_string())
    })
}

/// Appends an `ArbCmd` to the list of initialization commands of a plugin
/// thread.
///
/// The `ArbCmd` handle is consumed by this function, and is thus invalidated,
/// if and only if it is successful.
#[no_mangle]
pub extern "C" fn dqcs_tcfg_init_arb(tcfg: dqcs_handle_t, cmd: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(tcfg as &mut PluginThreadConfiguration);
        take!(cmd as ArbCmd);
        tcfg.init_cmds.push(cmd);
        Ok(())
    })
}

/// Configures the logging verbosity for the given plugin thread.
#[no_mangle]
pub extern "C" fn dqcs_tcfg_verbosity_set(
    tcfg: dqcs_handle_t,
    level: dqcs_loglevel_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(tcfg as &mut PluginThreadConfiguration);
        tcfg.log_configuration.verbosity = level.into_loglevel_filter()?;
        Ok(())
    })
}

/// Returns the configured verbosity for the given plugin thread.
#[no_mangle]
pub extern "C" fn dqcs_tcfg_verbosity_get(tcfg: dqcs_handle_t) -> dqcs_loglevel_t {
    api_return(dqcs_loglevel_t::DQCS_LOG_INVALID, || {
        resolve!(tcfg as &PluginThreadConfiguration);
        Ok(tcfg.log_configuration.verbosity.into())
    })
}

/// Configures a plugin thread to also output its log messages to a file.
///
/// `verbosity` configures the verbosity level for the file only.
#[no_mangle]
pub extern "C" fn dqcs_tcfg_tee(
    tcfg: dqcs_handle_t,
    verbosity: dqcs_loglevel_t,
    filename: *const c_char,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(tcfg as &mut PluginThreadConfiguration);
        tcfg.log_configuration
            .tee_files
            .push(TeeFileConfiguration::new(
                verbosity.into_loglevel_filter()?,
                receive_str(filename)?,
            ));
        Ok(())
    })
}
