use super::*;

/// Returns the type of object associated with the given handle.
#[no_mangle]
pub extern "C" fn dqcs_handle_type(handle: dqcs_handle_t) -> dqcs_handle_type_t {
    API_STATE.with(|state| {
        let state = state.borrow();
        match &state.objects.get(&handle) {
            None => dqcs_handle_type_t::DQCS_HTYPE_INVALID,
            Some(APIObject::ArbData(_)) => dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA,
            Some(APIObject::ArbCmd(_)) => dqcs_handle_type_t::DQCS_HTYPE_ARB_CMD,
            Some(APIObject::ArbCmdQueue(_)) => dqcs_handle_type_t::DQCS_HTYPE_ARB_CMD_QUEUE,
            Some(APIObject::QubitReferenceSet(_)) => dqcs_handle_type_t::DQCS_HTYPE_QUBIT_SET,
            Some(APIObject::Gate(_)) => dqcs_handle_type_t::DQCS_HTYPE_GATE,
            Some(APIObject::QubitMeasurementResult(_)) => dqcs_handle_type_t::DQCS_HTYPE_MEAS,
            Some(APIObject::PluginConfiguration(x)) => match x.specification.typ {
                PluginType::Frontend => dqcs_handle_type_t::DQCS_HTYPE_FRONT_CONFIG,
                PluginType::Operator => dqcs_handle_type_t::DQCS_HTYPE_OPER_CONFIG,
                PluginType::Backend => dqcs_handle_type_t::DQCS_HTYPE_BACK_CONFIG,
            },
            Some(APIObject::SimulatorConfiguration(_)) => dqcs_handle_type_t::DQCS_HTYPE_SIM_CONFIG,
            Some(APIObject::Simulator(_)) => dqcs_handle_type_t::DQCS_HTYPE_SIM,
            Some(APIObject::PluginDefinition(x)) => match x.get_type() {
                PluginType::Frontend => dqcs_handle_type_t::DQCS_HTYPE_FRONT_DEF,
                PluginType::Operator => dqcs_handle_type_t::DQCS_HTYPE_OPER_DEF,
                PluginType::Backend => dqcs_handle_type_t::DQCS_HTYPE_BACK_DEF,
            },
        }
    })
}

/// Returns a debug dump of the object associated with the given handle.
///
/// On success, this **returns a newly allocated string containing the
/// description. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_handle_dump(handle: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(handle as &APIObject);
        Ok(format!("{:#?}", handle))
    })
}

/// Destroys the object associated with a handle.
///
/// Returns 0 when successful, -1 otherwise.
#[no_mangle]
pub extern "C" fn dqcs_handle_delete(handle: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        take!(handle as APIObject);
        let _ = handle;
        Ok(())
    })
}
