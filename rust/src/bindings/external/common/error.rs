use super::*;

/// Returns a pointer to the latest error message.
///
/// Call this to get extra information when another function returns a failure
/// code. The returned pointer is temporary and therefore should **NOT** be
/// `free()`d. It will become invalid when a new error occurs.
#[no_mangle]
pub extern "C" fn dqcs_error_get() -> *const c_char {
    API_STATE.with(|state| {
        let state = state.borrow();
        match &state.last_error {
            Some(msg) => msg.as_ptr(),
            None => null(),
        }
    })
}

/// Sets the latest error message string.
///
/// This must be called by callback functions when an error occurs within the
/// callback, otherwise the upstream result for `dqcs_error_get()` will be
/// undefined.
///
/// If `msg` is set to `NULL`, the error string is cleared instead.
#[no_mangle]
pub extern "C" fn dqcs_error_set(msg: *const c_char) {
    API_STATE.with(|state| {
        let mut state = state.borrow_mut();
        if msg.is_null() {
            state.last_error = None
        } else {
            state.last_error = Some(unsafe { CStr::from_ptr(msg) }.to_owned())
        }
    })
}
