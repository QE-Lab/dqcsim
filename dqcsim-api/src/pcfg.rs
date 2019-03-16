use super::*;
use failure::Error;
use std::ptr::null;

/// Convenience function for writing functions that operate on
/// `PluginConfiguration`s.
fn with_pcfg<T>(
    handle: dqcs_handle_t,
    error: impl FnOnce() -> T,
    call: impl FnOnce(&mut PluginConfiguration) -> Result<T, Error>,
) -> T {
    with_state(error, |mut state| match state.objects.get_mut(&handle) {
        Some(Object::PluginConfiguration(x)) => call(x),
        Some(_) => Err(APIError::UnsupportedHandle(handle).into()),
        None => Err(APIError::InvalidHandle(handle).into()),
    })
}

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

/*
/// Appends an `ArbCmd` to the list of initialization commands of a plugin.
///
/// The `ArbCmd` handle is consumed by this function, and is thus invalidated.
/// This may happen even if the function fails - so don't make it fail.
#[no_mangle]
pub extern "C" fn dqcs_pcfg_init_arb(handle: dqcs_handle_t, cmd: dqcs_handle_t) -> dqcs_return_t {
    // TODO
    dqcs_return_t::DQCS_FAILURE
}
*/
