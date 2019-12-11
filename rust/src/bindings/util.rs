use super::*;
use crate::common::util::log_2;

/// Convenience function for converting a C string to a Rust `str`.
pub fn receive_str<'a>(s: *const c_char) -> Result<&'a str> {
    if s.is_null() {
        inv_arg("unexpected NULL string")
    } else {
        Ok(unsafe { CStr::from_ptr(s) }.to_str()?)
    }
}

/// Convenience function for converting a C string to an optional Rust `str`.
pub fn receive_optional_str<'a>(s: *const c_char) -> Result<Option<&'a str>> {
    if s.is_null() {
        Ok(None)
    } else {
        Ok(Some(unsafe { CStr::from_ptr(s) }.to_str()?))
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

/// Convenience function for converting a C array of doubles representing a
/// unitary matrix to its Rust representation.
pub fn receive_matrix(
    ptr: *const c_double,
    matrix_len: size_t,
    num_qubits: usize,
) -> Result<Option<Vec<Complex64>>> {
    if matrix_len == 0 {
        Ok(None)
    } else if num_qubits == 0 {
        inv_arg("cannot read matrix for 0 qubits")
    } else {
        let num_entries = 2usize.pow(2 * num_qubits as u32);
        if matrix_len != num_entries {
            inv_arg("matrix has the wrong number of entries")
        } else if ptr.is_null() {
            inv_arg("matrix pointer is null")
        } else {
            let mut vec = Vec::with_capacity(num_entries);
            for i in 0..num_entries {
                let re: f64 = unsafe { *ptr.add(i * 2) };
                let im: f64 = unsafe { *ptr.add(i * 2 + 1) };
                vec.push(Complex64::new(re, im));
            }
            Ok(Some(vec))
        }
    }
}

/// Version of receive_matrix with implicit number of qubits.
pub fn receive_matrix_raw(
    ptr: *const c_double,
    matrix_len: size_t,
) -> Result<Option<Vec<Complex64>>> {
    receive_matrix(
        ptr,
        matrix_len,
        if matrix_len % 2 != 0 {
            inv_arg("invalid matrix size")
        } else {
            log_2(matrix_len / 2).ok_or_else(oe_inv_arg("invalid matrix size"))
        }?,
    )
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
/// ```ignore
/// let data = UserData::new(user_free, user_data);
/// let cb = move || callback(data.data());
/// ```
pub struct UserData {
    user_free: Option<extern "C" fn(*mut c_void)>,
    data: *mut c_void,
}

unsafe impl Send for UserData {}

impl Drop for UserData {
    fn drop(&mut self) {
        if let Some(user_free) = self.user_free {
            user_free(self.data);
        }
    }
}

impl UserData {
    /// Constructs a `UserData` object.
    pub fn new(user_free: Option<extern "C" fn(*mut c_void)>, data: *mut c_void) -> UserData {
        UserData { user_free, data }
    }

    /// Returns the user data pointer.
    pub fn data(&self) -> *mut c_void {
        self.data
    }
}
