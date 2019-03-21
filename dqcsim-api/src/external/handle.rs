use super::*;
use std::ptr::null;

/// Returns the type of object associated with the given handle.
#[no_mangle]
pub extern "C" fn dqcs_handle_type(h: dqcs_handle_t) -> dqcs_handle_type_t {
    API_STATE.with(|state| {
        let state = state.borrow();
        match &state.objects.get(&h) {
            None => dqcs_handle_type_t::DQCS_HTYPE_INVALID,
            Some(APIObject::ArbData(_)) => dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA,
            Some(APIObject::ArbCmd(_)) => dqcs_handle_type_t::DQCS_HTYPE_ARB_CMD,
            Some(APIObject::PluginConfiguration(x)) => match x.specification.typ {
                PluginType::Frontend => dqcs_handle_type_t::DQCS_HTYPE_FRONT_CONFIG,
                PluginType::Operator => dqcs_handle_type_t::DQCS_HTYPE_OPER_CONFIG,
                PluginType::Backend => dqcs_handle_type_t::DQCS_HTYPE_BACK_CONFIG,
            },
            Some(APIObject::SimulatorConfiguration(_)) => dqcs_handle_type_t::DQCS_HTYPE_SIM_CONFIG,
        }
    })
}

/// Returns a debug dump of the object associated with the given handle.
///
/// On success, this **returns a newly allocated string containing the
/// description. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_handle_dump(h: dqcs_handle_t) -> *const c_char {
    API_STATE.with(|state| {
        let mut state = state.borrow_mut();
        let result = match &state.objects.get(&h) {
            None => inv_handle(h),
            Some(x) => return_string(format!("{:#?}", x)),
        };
        state.result_to_api(result, null)
    })
}

/// Destroys the object associated with a handle.
///
/// Returns 0 when successful, -1 otherwise.
#[no_mangle]
pub extern "C" fn dqcs_handle_delete(h: dqcs_handle_t) -> dqcs_return_t {
    API_STATE.with(|state| {
        let mut state = state.borrow_mut();
        let result = match &state.objects.remove_entry(&h) {
            None => inv_handle(h),
            Some(_) => Ok(dqcs_return_t::DQCS_SUCCESS),
        };
        state.result_to_api(result, || dqcs_return_t::DQCS_FAILURE)
    })
}
