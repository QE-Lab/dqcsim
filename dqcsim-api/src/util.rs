use super::*;

/// Shorthand for producing an unsupported handle error.
pub fn unsup_handle<T>(handle: dqcs_handle_t, iface: impl AsRef<str>) -> Result<T> {
    inv_arg(format!(
        "handle {} does not support the {} interface",
        handle,
        iface.as_ref()
    ))
}

/// Shorthand for producing an invalid handle error.
pub fn inv_handle<T>(handle: dqcs_handle_t) -> Result<T> {
    inv_arg(format!("handle {} is invalid", handle))
}

/// Convenience function for converting a C string to a Rust `str`.
pub fn receive_str<'a>(s: *const c_char) -> Result<&'a str> {
    if s.is_null() {
        inv_arg("unexpected NULL string")
    } else {
        Ok(unsafe { CStr::from_ptr(s) }.to_str()?)
    }
}

/// Convenience function for returning an owned string to C land.
///
/// On success, this **returns a newly allocated string. It must be freed
/// with `free()` by the caller.**
pub fn return_string(s: impl AsRef<str>) -> Result<*mut c_char> {
    let s = CString::new(s.as_ref())?;
    let s = unsafe { strdup(s.as_ptr()) as *mut c_char };
    if s.is_null() {
        err("failed to allocate return value")
    } else {
        Ok(s)
    }
}

/// Convenience function for converting a C const buffer to a Rust `&[u8]`.
pub fn receive_raw<'a>(obj: *const c_void, obj_size: usize) -> Result<&'a [u8]> {
    if obj_size == 0 {
        Ok(&[])
    } else if obj.is_null() {
        inv_arg("unexpected NULL data")
    } else {
        Ok(unsafe { std::slice::from_raw_parts(obj as *const u8, obj_size) })
    }
}

/// Convenience function for converting a C const buffer to a Rust `&[u8]`.
pub fn return_raw(obj_in: &[u8], obj_out: *mut c_void, obj_size: usize) -> Result<ssize_t> {
    if obj_size > 0 && obj_out.is_null() {
        inv_arg("unexpected NULL buffer")
    } else {
        let actual_size = obj_in.len();
        let copy_size = std::cmp::min(actual_size, obj_size);
        if copy_size > 0 {
            unsafe {
                memcpy(obj_out, obj_in.as_ptr() as *const c_void, copy_size);
            }
        }
        Ok(actual_size as ssize_t)
    }
}

/// Converts an index Pythonically and checks bounds.
///
/// By "Pythonically" we mean that the list length is added to the index if it
/// is negative, allowing -1 to be used for the end of the list, -2 for the
/// penultimate item, and so on.
///
/// `len` specifies the size of the list, `index` is the index to convert, and
/// `insert` selects whether index == len is okay (it is for the `insert()`
/// function, but isn't for anything else).
pub fn receive_index(len: size_t, index: ssize_t, insert: bool) -> Result<size_t> {
    let converted_index = if index < 0 {
        if insert {
            index + (len as ssize_t) + 1
        } else {
            index + (len as ssize_t)
        }
    } else {
        index
    };
    let mut ok = true;
    if converted_index < 0 || converted_index as size_t > len {
        ok = false;
    } else if converted_index as size_t == len {
        ok = insert;
    }
    if ok {
        Ok(converted_index as size_t)
    } else {
        inv_arg(format!("index out of range: {}", index))
    }
}

/// User data structure for callbacks.
///
/// All callbacks carry a user-defined `void*` with them, which is passed to
/// the callback as the first argument whenever it's called. This can be used
/// for closure data or for calling C++ class member functions. In the case of
/// a closure though, the ownership of the closure data is logically the
/// closure itself, which is moved from the user language into the Rust domain.
/// To avoid leaking this data, a second function pointer is optionally
/// provided by the user that is called when the closure is deleted, allowing
/// the user to clean up.
///
/// For Python, the user data is always the callable PyObject pointer, and the
/// `user_free` callbacks all point to the same function, which just decrements
/// the callable's refcount. This behavior is unfortunately not at *all*
/// supported by SWIG, so it's implemented in the `add_swig_directives.py`
/// script.
///
/// To turn an API callback into a Rust closure when it is installed, be sure
/// to construct this object outside of the closure and then move it into the
/// closure! For example:
///
/// ```rust
/// let data = CallbackUserData::new(user_free, user_data);
/// let cb = move || callback(data.data());
/// ```
pub struct CallbackUserData {
    user_free: Option<extern "C" fn(*mut c_void)>,
    data: *mut c_void,
}

unsafe impl Send for CallbackUserData {}

impl Drop for CallbackUserData {
    fn drop(&mut self) {
        if let Some(user_free) = self.user_free {
            user_free(self.data);
        }
    }
}

impl CallbackUserData {
    /// Constructs a `CallbackUserData` object.
    pub fn new(
        user_free: Option<extern "C" fn(*mut c_void)>,
        data: *mut c_void,
    ) -> CallbackUserData {
        CallbackUserData { user_free, data }
    }

    /// Returns the user data pointer.
    pub fn data(&self) -> *mut c_void {
        self.data
    }
}

/// Convenience function for writing functions that operate on `ArbData`s.
pub fn with_arb<T>(
    handle: dqcs_handle_t,
    error: impl FnOnce() -> T,
    call: impl FnOnce(&mut ArbData) -> Result<T>,
) -> T {
    with_api_state(error, |mut state| match state.objects.get_mut(&handle) {
        Some(APIObject::ArbData(x)) => call(x),
        Some(APIObject::ArbCmd(x)) => call(x.data_mut()),
        Some(_) => unsup_handle(handle, "arb"),
        None => inv_handle(handle),
    })
}

/// Convenience function for taking an ArbData from the object store in the
/// context of the accelerator or plugin interface layer.
pub fn take_arb<T>(
    handle: dqcs_handle_t,
    call: impl FnOnce(&mut ArbData) -> Result<T>,
) -> Result<T> {
    // Take the ArbData from the object store.
    let mut maybe_ob = API_STATE.with(|state| state.borrow_mut().objects.remove(&handle));

    // Call the callback.
    let ret = match maybe_ob.as_mut() {
        Some(APIObject::ArbData(x)) => call(x),
        Some(APIObject::ArbCmd(x)) => call(x.data_mut()),
        Some(_) => unsup_handle(handle, "arb"),
        None => inv_handle(handle),
    };

    // If the callback of handle loading failed but we did get an object,
    // insert it back into the handle store.
    if let Some(ob) = maybe_ob {
        if ret.is_err() {
            API_STATE.with(|state| state.borrow_mut().objects.insert(handle, ob));
        }
    }

    ret
}

/// Convenience function for writing functions that operate on `ArbCmd`s.
pub fn with_cmd<T>(
    handle: dqcs_handle_t,
    error: impl FnOnce() -> T,
    call: impl FnOnce(&mut ArbCmd) -> Result<T>,
) -> T {
    with_api_state(error, |mut state| match state.objects.get_mut(&handle) {
        Some(APIObject::ArbCmd(x)) => call(x),
        Some(_) => unsup_handle(handle, "cmd"),
        None => inv_handle(handle),
    })
}

/// Convenience function for taking an ArbCmd from the object store in the
/// context of the accelerator or plugin interface layer.
pub fn take_cmd<T>(
    handle: dqcs_handle_t,
    call: impl FnOnce(&mut ArbCmd) -> Result<T>,
) -> Result<T> {
    // Take the ArbCmd from the object store.
    let mut maybe_ob = API_STATE.with(|state| state.borrow_mut().objects.remove(&handle));

    // Call the callback.
    let ret = match maybe_ob.as_mut() {
        Some(APIObject::ArbCmd(x)) => call(x),
        Some(_) => unsup_handle(handle, "cmd"),
        None => inv_handle(handle),
    };

    // If the callback of handle loading failed but we did get an object,
    // insert it back into the handle store.
    if let Some(ob) = maybe_ob {
        if ret.is_err() {
            API_STATE.with(|state| state.borrow_mut().objects.insert(handle, ob));
        }
    }

    ret
}

/// Convenience function for writing functions that operate on
/// `PluginConfiguration`s.
pub fn with_pcfg<T>(
    handle: dqcs_handle_t,
    error: impl FnOnce() -> T,
    call: impl FnOnce(&mut PluginConfiguration) -> Result<T>,
) -> T {
    with_api_state(error, |mut state| match state.objects.get_mut(&handle) {
        Some(APIObject::PluginConfiguration(x)) => call(x),
        Some(_) => unsup_handle(handle, "pcfg"),
        None => inv_handle(handle),
    })
}

/// Convenience function for writing functions that operate on
/// `SimulatorConfiguration`s.
pub fn with_scfg<T>(
    handle: dqcs_handle_t,
    error: impl FnOnce() -> T,
    call: impl FnOnce(&mut SimulatorConfiguration) -> Result<T>,
) -> T {
    with_api_state(error, |mut state| match state.objects.get_mut(&handle) {
        Some(APIObject::SimulatorConfiguration(x)) => call(x),
        Some(_) => unsup_handle(handle, "scfg"),
        None => inv_handle(handle),
    })
}
