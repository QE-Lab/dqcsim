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

/// Macro for generating the boilerplate code for the plugin callback setters.
macro_rules! define_callback {
    (
        // Docstring.
        $(#[$doc:meta])*

        // Name of the API function and name of the callback in the
        // PluginDefinition structure:
        $setter:ident::$name:ident

        // Argument list, supplying first the type name in the Rust domain and
        // then the type in the foreign domain:
        ($($ai:ident: $atr:ty as $atf:ty),*)

        // Return value (excluding Result<>), again first the type in the Rust
        // domain and then the type in the foreign domain:
        -> $rtr:ty as $rtf:ty

        // Closure going from Rust to foreign. Needs to start with `callback,
        // state, data` due to macro hygiene rules, followed by the contents of
        // the closure.
        = $callback:ident, $state:ident, $data:ident $contents:tt
    ) => {
        $(#[$doc])*
        #[no_mangle]
        pub extern "C" fn $setter(
            pdef: dqcs_handle_t,
            callback: Option<
                extern "C" fn(user_data: *mut c_void, state: dqcs_plugin_state_t, $($ai: $atf),*) -> $rtf,
            >,
            user_free: Option<extern "C" fn(user_data: *mut c_void)>,
            user_data: *mut c_void,
        ) -> dqcs_return_t {
            api_return_none(|| {
                let $callback = callback.ok_or_else(oe_inv_arg("callback cannot be null"))?;
                resolve!(pdef as &mut PluginDefinition);
                let $data = CallbackUserData::new(user_free, user_data);
                pdef.$name = Box::new(
                    move |$state: &mut PluginState, $($ai: $atr),*| -> Result<$rtr> $contents,
                );
                Ok(())
            })
        }
    }
}

define_callback!(
    /// Sets the user logic initialization callback.
    ///
    /// This is always called before any of the other callbacks are run. The
    /// downstream plugin has already been initialized at this stage, so it is
    /// legal to send it commands.
    ///
    /// The default behavior is no-op.
    ///
    /// Besides the common arguments, the callback receives a handle to an
    /// `ArbCmd` queue (`dqcs_cq_*`, `dqcs_cmd_*`, and `dqcs_arb_*` interfaces)
    /// containing user-defined initialization commands. This is a borrowed
    /// handle; the caller will delete it.
    ///
    /// The callback can return an error by setting an error message using
    /// `dqcs_error_set()` and returning `DQCS_FAILURE`. Otherwise, it should
    /// return `DQCS_SUCCESS`.
    dqcs_pdef_set_initialize_cb::initialize(
        init_cmds: Vec<ArbCmd> as dqcs_handle_t
    ) -> () as dqcs_return_t = callback, state, data {
        let init_cmds: ArbCmdQueue = init_cmds.into_iter().collect();
        let init_cmds = insert(init_cmds);
        let result = cb_return_none(callback(data.data(), state.into(), init_cmds));
        delete!(init_cmds);
        result
    }
);

define_callback!(
    /// Sets the user logic drop/cleanup callback.
    ///
    /// This is called when a plugin is gracefully terminated. It is not
    /// recommended to execute any downstream instructions at this time, but it
    /// is supported in case this is really necessary.
    ///
    /// The default behavior is no-op.
    ///
    /// The callback can return an error by setting an error message using
    /// `dqcs_error_set()` and returning `DQCS_FAILURE`. Otherwise, it should
    /// return `DQCS_SUCCESS`.
    dqcs_pdef_set_drop_cb::drop() -> () as dqcs_return_t = callback, state, data {
        cb_return_none(callback(data.data(), state.into()))
    }
);

define_callback!(
    /// Sets the run callback for frontends.
    ///
    /// This is called in response to a `start()` host API call. The return
    /// value is returned through the `wait()` host API call.
    ///
    /// The default behavior is to fail with a "not implemented" error;
    /// frontends backends should always override this. This callback is never
    /// called for operator or backend plugins.
    ///
    /// Besides the common arguments, the callback receives a handle to an
    /// `ArbData` object containing the data that the host passed to `start()`.
    /// This is a borrowed handle; the caller will delete it.
    ///
    /// When the run callback is successful, it should return a valid `ArbData`
    /// handle. This can be the same as the argument, but it can also be a new
    /// object. This `ArbData` is returned to the host through `wait()`. This
    /// `ArbData` object is deleted after the callback completes.
    ///
    /// The callback can return an error by setting an error message using
    /// `dqcs_error_set()` and returning 0. Otherwise, it should return a
    /// valid `ArbData` handle.
    dqcs_pdef_set_run_cb::run(
        args: ArbData as dqcs_handle_t
    ) -> ArbData as dqcs_handle_t = callback, state, data {
        let args = insert(args);
        let result = cb_return(0, callback(data.data(), state.into(), args)).and_then(|arb| {
            take!(arb as ArbData);
            Ok(arb)
        });
        delete!(args);
        result
    }
);

define_callback!(
    /// Sets the qubit allocation callback for operators and backends.
    ///
    /// The default for operators is to pass through to `state.allocate()`. The
    /// default for backends is no-op. This callback is never called for
    /// frontend plugins.
    ///
    /// Besides the common arguments, the callback receives a handle to a qubit
    /// set containing the references that are to be used for the
    /// to-be-allocated qubits and an `ArbCmd` queue containing user-defined
    /// commands to optionally augment the behavior of the qubits. These are
    /// borrowed handles; the caller will delete them.
    ///
    /// The callback can return an error by setting an error message using
    /// `dqcs_error_set()` and returning `DQCS_FAILURE`. Otherwise, it should
    /// return `DQCS_SUCCESS`.
    dqcs_pdef_set_allocate_cb::allocate(
        qubits: Vec<QubitRef> as dqcs_handle_t,
        alloc_cmds: Vec<ArbCmd> as dqcs_handle_t
    ) -> () as dqcs_return_t = callback, state, data {
        let qubits: QubitReferenceSet = qubits.into_iter().collect();
        let qubits = insert(qubits);
        let alloc_cmds: ArbCmdQueue = alloc_cmds.into_iter().collect();
        let alloc_cmds = insert(alloc_cmds);
        let result = cb_return_none(callback(data.data(), state.into(), qubits, alloc_cmds));
        delete!(qubits);
        delete!(alloc_cmds);
        result
    }
);

define_callback!(
    /// Sets the qubit deallocation callback for operators and backends.
    ///
    /// The default for operators is to pass through to `state.free()`. The
    /// default for backends is no-op. This callback is never called for
    /// frontend plugins.
    ///
    /// Besides the common arguments, the callback receives a handle to a qubit
    /// set containing the qubits that are to be freed. This is a borrowed
    /// handle; the caller will delete it.
    ///
    /// The callback can return an error by setting an error message using
    /// `dqcs_error_set()` and returning `DQCS_FAILURE`. Otherwise, it should
    /// return `DQCS_SUCCESS`.
    dqcs_pdef_set_free_cb::free(
        qubits: Vec<QubitRef> as dqcs_handle_t
    ) -> () as dqcs_return_t = callback, state, data {
        let qubits: QubitReferenceSet = qubits.into_iter().collect();
        let qubits = insert(qubits);
        let result = cb_return_none(callback(data.data(), state.into(), qubits));
        delete!(qubits);
        result
    }
);
