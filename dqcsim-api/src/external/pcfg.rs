use super::*;
use dqcsim::common::log::tee_file::TeeFile;
use std::ptr::null;

/// Creates a new `PluginConfiguration` object using sugared syntax.
///
/// `typ` specifies the type of plugin. `name` specifies the name used to refer
/// to the plugin later, which much be unique within a simulation; if it is
/// empty or `NULL`, auto-naming will be performed: "front" for the frontend,
/// "oper<i>" for the operators (indices starting at 1 from frontend to
/// backend), and "back" for the backend. `spec` specifies which plugin to use,
/// using the same syntax that the `dqcsim` command line interface uses.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_new(
    typ: dqcs_plugin_type_t,
    name: *const c_char,
    spec: *const c_char,
) -> dqcs_handle_t {
    with_api_state(
        || 0,
        |mut state| {
            let spec = receive_str(spec)?;
            if spec.is_empty() {
                return inv_arg("plugin specification must not be empty");
            }
            Ok(
                state.push(APIObject::PluginConfiguration(PluginConfiguration::new(
                    receive_str(name)?,
                    PluginSpecification::from_sugar(spec, typ.into())?,
                ))),
            )
        },
    )
}

/// Creates a new `PluginConfiguration` object using raw paths.
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
    with_api_state(
        || 0,
        |mut state| {
            let executable = receive_str(executable)?;
            if executable.is_empty() {
                return inv_arg("plugin executable must not be empty");
            }
            let script_path;
            if script.is_null() {
                script_path = None;
            } else {
                let script = receive_str(script)?;
                if script.is_empty() {
                    script_path = None;
                } else {
                    script_path = Some(script);
                }
            }
            Ok(
                state.push(APIObject::PluginConfiguration(PluginConfiguration::new(
                    receive_str(name)?,
                    PluginSpecification::new(executable, script_path, typ),
                ))),
            )
        },
    )
}

/// Returns the type of the given plugin configuration.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_type(handle: dqcs_handle_t) -> dqcs_plugin_type_t {
    with_pcfg(
        handle,
        || dqcs_plugin_type_t::DQCS_PTYPE_INVALID,
        |plugin| Ok(plugin.specification.typ.into()),
    )
}

/// Returns the configured name for the given plugin.
///
/// On success, this **returns a newly allocated string containing the
/// name. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_name(handle: dqcs_handle_t) -> *const c_char {
    with_pcfg(handle, null, |plugin| return_string(&plugin.name))
}

/// Returns the configured executable path for the given plugin.
///
/// On success, this **returns a newly allocated string containing the
/// executable path. Free it with `free()` when you're done with it to avoid
/// memory leaks.** On failure (i.e., the handle is invalid) this returns
/// `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_executable(handle: dqcs_handle_t) -> *const c_char {
    with_pcfg(handle, null, |plugin| {
        return_string(plugin.specification.executable.to_string_lossy())
    })
}

/// Returns the configured script path for the given plugin.
///
/// On success, this **returns a newly allocated string containing the
/// script path. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`. An
/// empty string will be returned if no script is configured to distinguish it
/// from failure.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_script(handle: dqcs_handle_t) -> *const c_char {
    with_pcfg(handle, null, |plugin| {
        if let Some(script) = plugin.specification.script.as_ref() {
            return_string(script.to_string_lossy())
        } else {
            return_string("")
        }
    })
}

/// Appends an `ArbCmd` to the list of initialization commands of a plugin.
///
/// The `ArbCmd` handle is consumed by this function, and is thus invalidated,
/// if and only if it is successful.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_init_arb(
    handle: dqcs_handle_t,
    cmd_handle: dqcs_handle_t,
) -> dqcs_return_t {
    with_api_state(
        || dqcs_return_t::DQCS_FAILURE,
        |mut state| match state.objects.remove(&cmd_handle) {
            Some(APIObject::ArbCmd(cmd_ob)) => match state.objects.get_mut(&handle) {
                Some(APIObject::PluginConfiguration(pcfg)) => {
                    pcfg.functional.init.push(cmd_ob);
                    Ok(dqcs_return_t::DQCS_SUCCESS)
                }
                Some(_) => {
                    state.objects.insert(cmd_handle, APIObject::ArbCmd(cmd_ob));
                    unsup_handle(handle, "pcfg")
                }
                None => {
                    state.objects.insert(cmd_handle, APIObject::ArbCmd(cmd_ob));
                    inv_handle(handle)
                }
            },
            Some(ob) => {
                state.objects.insert(cmd_handle, ob);
                unsup_handle(cmd_handle, "cmd")
            }
            None => inv_handle(cmd_handle),
        },
    )
}

/// Overrides an environment variable for the plugin process.
///
/// The environment variable `key` is set to `value` regardless of whether it
/// exists in the parent environment variable scope.
///
/// If value is `NULL`, the environment variable `key` is unset instead.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_env_set(
    handle: dqcs_handle_t,
    key: *const c_char,
    value: *const c_char,
) -> dqcs_return_t {
    with_pcfg(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |plugin| {
            if value.is_null() {
                plugin
                    .functional
                    .env
                    .push(EnvMod::remove(receive_str(key)?));
            } else {
                plugin
                    .functional
                    .env
                    .push(EnvMod::set(receive_str(key)?, receive_str(value)?));
            }
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Removes/unsets an environment variable for the plugin process.
///
/// The environment variable `key` is unset regardless of whether it exists in
/// the parent environment variable scope.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_env_unset(handle: dqcs_handle_t, key: *const c_char) -> dqcs_return_t {
    dqcs_pcfg_env_set(handle, key, null())
}

/// Overrides the working directory for the plugin process.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_work_set(handle: dqcs_handle_t, work: *const c_char) -> dqcs_return_t {
    with_pcfg(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |plugin| {
            plugin.functional.work = receive_str(work)?.into();
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Returns the configured working directory for the given plugin.
///
/// On success, this **returns a newly allocated string containing the
/// working directory. Free it with `free()` when you're done with it to avoid
/// memory leaks.** On failure (i.e., the handle is invalid) this returns
/// `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_work_get(handle: dqcs_handle_t) -> *const c_char {
    with_pcfg(handle, null, |plugin| {
        return_string(plugin.functional.work.to_string_lossy())
    })
}

/// Configures the logging verbosity for the given plugin.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_verbosity_set(
    handle: dqcs_handle_t,
    level: dqcs_loglevel_t,
) -> dqcs_return_t {
    with_pcfg(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |plugin| {
            plugin.nonfunctional.verbosity = level.into_loglevel_filter()?;
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Returns the configured verbosity for the given plugin.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_verbosity_get(handle: dqcs_handle_t) -> dqcs_loglevel_t {
    with_pcfg(
        handle,
        || dqcs_loglevel_t::DQCS_LOG_INVALID,
        |plugin| Ok(plugin.nonfunctional.verbosity.into()),
    )
}

/// Configures a plugin to also output its log messages to a file.
///
/// `verbosity` configures the verbosity level for the file only.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_tee(
    handle: dqcs_handle_t,
    verbosity: dqcs_loglevel_t,
    filename: *const c_char,
) -> dqcs_return_t {
    with_pcfg(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |plugin| {
            plugin.nonfunctional.tee_files.push(TeeFile::new(
                verbosity.into_loglevel_filter()?,
                receive_str(filename)?,
            ));
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Configures the capture mode for the stdout stream of the specified plugin.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_stdout_mode_set(
    handle: dqcs_handle_t,
    level: dqcs_loglevel_t,
) -> dqcs_return_t {
    with_pcfg(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |plugin| {
            plugin.nonfunctional.stdout_mode = level.into();
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Returns the configured stdout capture mode for the specified plugin.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_stdout_mode_get(handle: dqcs_handle_t) -> dqcs_loglevel_t {
    with_pcfg(
        handle,
        || dqcs_loglevel_t::DQCS_LOG_INVALID,
        |plugin| Ok(plugin.nonfunctional.stdout_mode.clone().into()),
    )
}

/// Configures the capture mode for the stderr stream of the specified plugin.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_stderr_mode_set(
    handle: dqcs_handle_t,
    level: dqcs_loglevel_t,
) -> dqcs_return_t {
    with_pcfg(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |plugin| {
            plugin.nonfunctional.stderr_mode = level.into();
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Returns the configured stderr capture mode for the specified plugin.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_stderr_mode_get(handle: dqcs_handle_t) -> dqcs_loglevel_t {
    with_pcfg(
        handle,
        || dqcs_loglevel_t::DQCS_LOG_INVALID,
        |plugin| Ok(plugin.nonfunctional.stderr_mode.clone().into()),
    )
}

/// Configures the timeout for the plugin to connect to DQCsim.
///
/// The default is 5 seconds, so you should normally be able to leave this
/// alone.
///
/// The time unit is seconds. Use IEEE positive infinity to specify an infinite
/// timeout.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_accept_timeout_set(
    handle: dqcs_handle_t,
    timeout: f64,
) -> dqcs_return_t {
    with_pcfg(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |plugin| {
            plugin.nonfunctional.accept_timeout = Timeout::try_from_double(timeout)?;
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Returns the configured timeout for the plugin to connect to DQCsim.
///
/// The time unit is in seconds. Returns positive inifinity for an infinite
/// timeout. Returns -1 when the function fails.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_accept_timeout_get(handle: dqcs_handle_t) -> f64 {
    with_pcfg(
        handle,
        || -1.0,
        |plugin| Ok(plugin.nonfunctional.accept_timeout.to_double()),
    )
}

/// Configures the timeout for the plugin to shut down gracefully.
///
/// The default is 5 seconds, so you should normally be able to leave this
/// alone.
///
/// The time unit is seconds. Use IEEE positive infinity to specify an infinite
/// timeout.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_shutdown_timeout_set(
    handle: dqcs_handle_t,
    timeout: f64,
) -> dqcs_return_t {
    with_pcfg(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |plugin| {
            plugin.nonfunctional.shutdown_timeout = Timeout::try_from_double(timeout)?;
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Returns the configured timeout for the plugin to shut down gracefully.
///
/// The time unit is in seconds. Returns positive inifinity for an infinite
/// timeout. Returns -1 when the function fails.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_shutdown_timeout_get(handle: dqcs_handle_t) -> f64 {
    with_pcfg(
        handle,
        || -1.0,
        |plugin| Ok(plugin.nonfunctional.shutdown_timeout.to_double()),
    )
}
