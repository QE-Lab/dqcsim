use super::*;
use std::ptr::null;

/// Returns the type of object associated with the given handle.
#[no_mangle]
pub extern "C" fn dqcs_handle_type(h: dqcs_handle_t) -> dqcs_handle_type_t {
    STATE.with(|state| {
        let state = state.borrow();
        match &state.objects.get(&h) {
            None => dqcs_handle_type_t::DQCS_INVALID,
            Some(Object::ArbData(_)) => dqcs_handle_type_t::DQCS_ARB_DATA,
            Some(Object::ArbCmd(_)) => dqcs_handle_type_t::DQCS_ARB_CMD,
            Some(Object::PluginConfiguration(x)) => match x.specification.typ {
                PluginType::Frontend => dqcs_handle_type_t::DQCS_FRONT_CONFIG,
                PluginType::Operator => dqcs_handle_type_t::DQCS_OPER_CONFIG,
                PluginType::Backend => dqcs_handle_type_t::DQCS_BACK_CONFIG,
            },
            Some(Object::SimulatorConfiguration(_)) => dqcs_handle_type_t::DQCS_SIM_CONFIG,
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
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        match &state.objects.get(&h) {
            None => {
                state.fail(format!("Handle {} is invalid", h));
                null()
            }
            Some(x) => match return_string(format!("{:?}", x)) {
                Ok(p) => p,
                Err(e) => {
                    state.fail(e.to_string());
                    null()
                }
            },
        }
    })
}

/// Destroys the object associated with a handle.
///
/// Returns 0 when successful, -1 otherwise.
#[no_mangle]
pub extern "C" fn dqcs_handle_delete(h: dqcs_handle_t) -> dqcs_return_t {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        match &state.objects.remove_entry(&h) {
            None => state.fail(format!("Handle {} is invalid", h)),
            Some(_) => dqcs_return_t::DQCS_SUCCESS,
        }
    })
}
