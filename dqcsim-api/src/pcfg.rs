use super::*;
use dqcsim::log::tee_file::TeeFile;
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
    with_state(
        || 0,
        |mut state| {
            let spec = receive_str(spec)?;
            if spec.is_empty() {
                return Err(
                    APIError::Generic("Plugin specification must not be empty".to_string()).into(),
                );
            }
            Ok(
                state.push(Object::PluginConfiguration(PluginConfiguration::new(
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
    with_state(
        || 0,
        |mut state| {
            let executable = receive_str(executable)?;
            if executable.is_empty() {
                return Err(
                    APIError::Generic("Plugin executable must not be empty".to_string()).into(),
                );
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
                state.push(Object::PluginConfiguration(PluginConfiguration::new(
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
    with_state(
        || dqcs_return_t::DQCS_FAILURE,
        |mut state| match state.objects.remove(&cmd_handle) {
            Some(Object::ArbCmd(cmd_ob)) => match state.objects.get_mut(&handle) {
                Some(Object::PluginConfiguration(pcfg)) => {
                    pcfg.functional.init.push(cmd_ob);
                    Ok(dqcs_return_t::DQCS_SUCCESS)
                }
                Some(_) => {
                    state.objects.insert(cmd_handle, Object::ArbCmd(cmd_ob));
                    Err(APIError::UnsupportedHandle(handle).into())
                }
                None => {
                    state.objects.insert(cmd_handle, Object::ArbCmd(cmd_ob));
                    Err(APIError::InvalidHandle(handle).into())
                }
            },
            Some(ob) => {
                state.objects.insert(cmd_handle, ob);
                Err(APIError::UnsupportedHandle(cmd_handle).into())
            }
            None => Err(APIError::InvalidHandle(cmd_handle).into()),
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
