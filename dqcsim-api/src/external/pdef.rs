use super::*;

/// Creates a new `PluginDefinition` object.
///
/// Plugin definitions contain the callback functions/closures that define the
/// functionality of a plugin. They also contain some metadata to identify the
/// implementation, in the form of a name, author, and version string, that
/// must be specified when the definition is constructed. The callback
/// functions/closures are initialized to sane defaults for the requested
/// plugin type, but obviously one or more of these should be overridden to
/// make the plugin do something.
///
/// Once a definition object has been built, it can be used to spawn a plugin
/// thread or run a plugin in the main thread, given a DQCsim server URL for it
/// to connect to.
#[no_mangle]
pub extern "C" fn dqcs_pdef_new(
    typ: dqcs_plugin_type_t,
    name: *const c_char,
    author: *const c_char,
    version: *const c_char,
) -> dqcs_handle_t {
    api_return(0, || {
        Ok(insert(PluginDefinition::new(
            typ.into(),
            PluginMetadata::new(
                receive_str(name)?,
                receive_str(author)?,
                receive_str(version)?,
            ),
        )))
    })
}

/// Returns the plugin type for the given plugin definition object.
#[no_mangle]
pub extern "C" fn dqcs_pdef_get_type(pdef: dqcs_handle_t) -> dqcs_plugin_type_t {
    api_return(dqcs_plugin_type_t::DQCS_PTYPE_INVALID, || {
        resolve!(pdef as &PluginDefinition);
        Ok(pdef.get_type().into())
    })
}

/// Returns the plugin name for the given plugin definition object.
///
/// On success, this **returns a newly allocated string containing the JSON
/// string. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_pdef_get_name(pdef: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(pdef as &PluginDefinition);
        Ok(pdef.get_metadata().get_name().to_string())
    })
}

/// Returns the plugin author for the given plugin definition object.
///
/// On success, this **returns a newly allocated string containing the JSON
/// string. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_pdef_get_author(pdef: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(pdef as &PluginDefinition);
        Ok(pdef.get_metadata().get_author().to_string())
    })
}

/// Returns the plugin version for the given plugin definition object.
///
/// On success, this **returns a newly allocated string containing the JSON
/// string. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_pdef_get_version(pdef: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(pdef as &PluginDefinition);
        Ok(pdef.get_metadata().get_version().to_string())
    })
}

/// Overrides the default initialization behavior using a callback.
///
/// This is always called before any of the other callbacks are run. The
/// downstream plugin has already been initialized at this stage, so it is
/// legal to send it commands.
///
/// The default behavior is no-op.
///
/// The callback takes the following arguments:
///  - `void*`: user defined data.
///  - `dqcs_plugin_context_t`: plugin context, needed to call the
///    `dqcs_plugin_*` functions.
///  - `dqcs_handle_t`: handle to an `ArbCmd` queue (`dqcs_cq_*`, `dqcs_cmd_*`,
///     and `dqcs_arb_*` interfaces) containing user-defined initialization
///     commands. It is up to the callback to delete this handle.
///
/// The callback can return an error by setting an error message using
/// `dqcs_error_set()` and returning `DQCS_FAILURE`. Otherwise, it should
/// return `DQCS_SUCCESS`.
#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn dqcs_pdef_set_initialize_cb(
    pdef: dqcs_handle_t,
    callback: Option<
        extern "C" fn(*mut c_void, dqcs_plugin_context_t, dqcs_handle_t) -> dqcs_return_t,
    >,
    user_free: Option<extern "C" fn(*mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        let callback = callback.ok_or_else(oe_inv_arg("callback cannot be null"))?;
        resolve!(pdef as &mut PluginDefinition);
        let data = CallbackUserData::new(user_free, user_data);
        pdef.initialize = Box::new(
            move |ctxt: &mut PluginContext, init_cmds: Vec<ArbCmd>| -> Result<()> {
                let init_cmds: ArbCmdQueue = init_cmds.into_iter().collect();
                cb_return_none(callback(data.data(), ctxt.into(), insert(init_cmds)))
            },
        );
        Ok(())
    })
}

// TODO: all the other callbacks
