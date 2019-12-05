use super::*;
use crate::common::log::tee_file::TeeFileConfiguration;
use std::ptr::null;

/// Creates a new plugin process configuration object using sugared syntax.
///
/// `typ` specifies the type of plugin. `name` specifies the name used to refer
/// to the plugin later, which much be unique within a simulation; if it is
/// empty or `NULL`, auto-naming will be performed: "front" for the frontend,
/// "oper&lt;i&gt;" for the operators (indices starting at 1 from frontend to
/// backend), and "back" for the backend. `spec` specifies which plugin to use,
/// using the same syntax that the `dqcsim` command line interface uses.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_new(
    typ: dqcs_plugin_type_t,
    name: *const c_char,
    spec: *const c_char,
) -> dqcs_handle_t {
    api_return(0, || {
        let typ: Result<PluginType> = typ.into();
        let spec = receive_optional_str(spec)?.filter(|x| !x.is_empty());
        if let Some(spec) = spec {
            Ok(insert(PluginProcessConfiguration::new(
                receive_optional_str(name)?.unwrap_or(""),
                PluginProcessSpecification::from_sugar(spec, typ?)?,
            )))
        } else {
            inv_arg("plugin specification must not be empty")
        }
    })
}

/// Creates a new plugin process configuration object using raw paths.
///
/// This works the same as `dqcs_pcfg_new()`, but instead of the sugared,
/// command-line style specification you have to specify the path to the plugin
/// executable and (if applicable) the script it must execute directly. This is
/// useful when you have a specific executable in mind and you don't want the
/// somewhat heuristic desugaring algorithm from doing something unexpected.
///
/// Pass `NULL` or an empty string to `script` to specify a native plugin
/// executable that does not take a script argument.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_new_raw(
    typ: dqcs_plugin_type_t,
    name: *const c_char,
    executable: *const c_char,
    script: *const c_char,
) -> dqcs_handle_t {
    api_return(0, || {
        let typ: Result<PluginType> = typ.into();
        let executable = receive_optional_str(executable)?.filter(|x| !x.is_empty());
        let script = receive_optional_str(script)?.filter(|x| !x.is_empty());
        if let Some(executable) = executable {
            Ok(insert(PluginProcessConfiguration::new(
                receive_optional_str(name)?.unwrap_or(""),
                PluginProcessSpecification::new(executable, script, typ?),
            )))
        } else {
            inv_arg("plugin executable must not be empty")
        }
    })
}

/// Returns the type of the given plugin process configuration.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_type(pcfg: dqcs_handle_t) -> dqcs_plugin_type_t {
    api_return(dqcs_plugin_type_t::DQCS_PTYPE_INVALID, || {
        resolve!(pcfg as &PluginProcessConfiguration);
        Ok(pcfg.specification.typ.into())
    })
}

/// Returns the configured name for the given plugin process.
///
/// On success, this **returns a newly allocated string containing the
/// name. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_name(pcfg: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(pcfg as &PluginProcessConfiguration);
        Ok(pcfg.name.to_string())
    })
}

/// Returns the configured executable path for the given plugin process.
///
/// On success, this **returns a newly allocated string containing the
/// executable path. Free it with `free()` when you're done with it to avoid
/// memory leaks.** On failure (i.e., the handle is invalid) this returns
/// `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_executable(pcfg: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(pcfg as &PluginProcessConfiguration);
        Ok(pcfg.specification.executable.to_string_lossy().to_string())
    })
}

/// Returns the configured script path for the given plugin process.
///
/// On success, this **returns a newly allocated string containing the
/// script path. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`. An
/// empty string will be returned if no script is configured to distinguish it
/// from failure.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_script(pcfg: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(pcfg as &PluginProcessConfiguration);
        if let Some(script) = pcfg.specification.script.as_ref() {
            Ok(script.to_string_lossy().to_string())
        } else {
            Ok("".to_string())
        }
    })
}

/// Appends an `ArbCmd` to the list of initialization commands of a plugin
/// process.
///
/// The `ArbCmd` handle is consumed by this function, and is thus invalidated,
/// if and only if it is successful.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_init_cmd(pcfg: dqcs_handle_t, cmd: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(pcfg as &mut PluginProcessConfiguration);
        take!(cmd as ArbCmd);
        pcfg.functional.init.push(cmd);
        Ok(())
    })
}

/// Overrides an environment variable for the plugin process.
///
/// The environment variable `key` is set to `value` regardless of whether it
/// exists in the parent environment variable scope.
///
/// If value is `NULL`, the environment variable `key` is unset instead.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_env_set(
    pcfg: dqcs_handle_t,
    key: *const c_char,
    value: *const c_char,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(pcfg as &mut PluginProcessConfiguration);
        if value.is_null() {
            pcfg.functional.env.push(EnvMod::remove(receive_str(key)?));
        } else {
            pcfg.functional
                .env
                .push(EnvMod::set(receive_str(key)?, receive_str(value)?));
        }
        Ok(())
    })
}

/// Removes/unsets an environment variable for the plugin process.
///
/// The environment variable `key` is unset regardless of whether it exists in
/// the parent environment variable scope.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_env_unset(pcfg: dqcs_handle_t, key: *const c_char) -> dqcs_return_t {
    dqcs_pcfg_env_set(pcfg, key, null())
}

/// Overrides the working directory for the plugin process.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_work_set(pcfg: dqcs_handle_t, work: *const c_char) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(pcfg as &mut PluginProcessConfiguration);
        let work: std::path::PathBuf = receive_str(work)?.into();
        if !work.is_dir() {
            inv_arg("not a directory")
        } else {
            pcfg.functional.work = work;
            Ok(())
        }
    })
}

/// Returns the configured working directory for the given plugin process.
///
/// On success, this **returns a newly allocated string containing the
/// working directory. Free it with `free()` when you're done with it to avoid
/// memory leaks.** On failure (i.e., the handle is invalid) this returns
/// `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_work_get(pcfg: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(pcfg as &PluginProcessConfiguration);
        Ok(pcfg.functional.work.to_string_lossy().to_string())
    })
}

/// Configures the logging verbosity for the given plugin process.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_verbosity_set(
    pcfg: dqcs_handle_t,
    level: dqcs_loglevel_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(pcfg as &mut PluginProcessConfiguration);
        pcfg.nonfunctional.verbosity = level.into_loglevel_filter()?;
        Ok(())
    })
}

/// Returns the configured verbosity for the given plugin process.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_verbosity_get(pcfg: dqcs_handle_t) -> dqcs_loglevel_t {
    api_return(dqcs_loglevel_t::DQCS_LOG_INVALID, || {
        resolve!(pcfg as &PluginProcessConfiguration);
        Ok(pcfg.nonfunctional.verbosity.into())
    })
}

/// Configures a plugin process to also output its log messages to a file.
///
/// `verbosity` configures the verbosity level for the file only.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_tee(
    pcfg: dqcs_handle_t,
    verbosity: dqcs_loglevel_t,
    filename: *const c_char,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(pcfg as &mut PluginProcessConfiguration);
        pcfg.nonfunctional.tee_files.push(TeeFileConfiguration::new(
            verbosity.into_loglevel_filter()?,
            receive_str(filename)?,
        ));
        Ok(())
    })
}

/// Configures the capture mode for the stdout stream of the specified plugin
/// process.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_stdout_mode_set(
    pcfg: dqcs_handle_t,
    level: dqcs_loglevel_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(pcfg as &mut PluginProcessConfiguration);
        pcfg.nonfunctional.stdout_mode = level.into_capture_mode()?;
        Ok(())
    })
}

/// Returns the configured stdout capture mode for the specified plugin
/// process.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_stdout_mode_get(pcfg: dqcs_handle_t) -> dqcs_loglevel_t {
    api_return(dqcs_loglevel_t::DQCS_LOG_INVALID, || {
        resolve!(pcfg as &PluginProcessConfiguration);
        Ok(pcfg.nonfunctional.stdout_mode.clone().into())
    })
}

/// Configures the capture mode for the stderr stream of the specified plugin
/// process.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_stderr_mode_set(
    pcfg: dqcs_handle_t,
    level: dqcs_loglevel_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(pcfg as &mut PluginProcessConfiguration);
        pcfg.nonfunctional.stderr_mode = level.into_capture_mode()?;
        Ok(())
    })
}

/// Returns the configured stderr capture mode for the specified plugin
/// process.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_stderr_mode_get(pcfg: dqcs_handle_t) -> dqcs_loglevel_t {
    api_return(dqcs_loglevel_t::DQCS_LOG_INVALID, || {
        resolve!(pcfg as &PluginProcessConfiguration);
        Ok(pcfg.nonfunctional.stderr_mode.clone().into())
    })
}

/// Configures the timeout for the plugin process to connect to DQCsim.
///
/// The default is 5 seconds, so you should normally be able to leave this
/// alone.
///
/// The time unit is seconds. Use IEEE positive infinity to specify an infinite
/// timeout.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_accept_timeout_set(pcfg: dqcs_handle_t, timeout: f64) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(pcfg as &mut PluginProcessConfiguration);
        pcfg.nonfunctional.accept_timeout = Timeout::try_from_double(timeout)?;
        Ok(())
    })
}

/// Returns the configured timeout for the plugin process to connect to DQCsim.
///
/// The time unit is in seconds. Returns positive inifinity for an infinite
/// timeout. Returns -1 when the function fails.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_accept_timeout_get(pcfg: dqcs_handle_t) -> f64 {
    api_return(-1.0, || {
        resolve!(pcfg as &PluginProcessConfiguration);
        Ok(pcfg.nonfunctional.accept_timeout.to_double())
    })
}

/// Configures the timeout for the plugin process to shut down gracefully.
///
/// The default is 5 seconds, so you should normally be able to leave this
/// alone.
///
/// The time unit is seconds. Use IEEE positive infinity to specify an infinite
/// timeout.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_shutdown_timeout_set(
    pcfg: dqcs_handle_t,
    timeout: f64,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(pcfg as &mut PluginProcessConfiguration);
        pcfg.nonfunctional.shutdown_timeout = Timeout::try_from_double(timeout)?;
        Ok(())
    })
}

/// Returns the configured timeout for the plugin process to shut down
/// gracefully.
///
/// The time unit is in seconds. Returns positive inifinity for an infinite
/// timeout. Returns -1 when the function fails.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_shutdown_timeout_get(pcfg: dqcs_handle_t) -> f64 {
    api_return(-1.0, || {
        resolve!(pcfg as &PluginProcessConfiguration);
        Ok(pcfg.nonfunctional.shutdown_timeout.to_double())
    })
}
