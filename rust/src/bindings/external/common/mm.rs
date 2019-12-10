use super::*;

/// Constructs a new matrix map builder.
///
/// Returns a handle to a matrix map builder with no detectors attached to it
/// yet. Use `dqcs_mmb_add_*()` to do that. The detectors are queried in the
/// order in which they are added, so be sure to add more specific gates first.
/// Then construct the matrix map object itself using `dqcs_mm_new()`.
#[no_mangle]
pub extern "C" fn dqcs_mmb_new() -> dqcs_handle_t {
    api_return(0, || {
        // TODO
        err("not yet implemented")
    })
}

/// Adds a set of default gate matrix detectors to the given matrix map
/// builder.
///
/// `mmb` must be a handle to a matrix map builder object (`dqcs_mmb_new()`).
/// `version` is reserved and should be set to zero; if the default set of
/// gates is changed in a future version of DQCsim, the different defaults will
/// be disambiguated with this numbed. `epsilon` specifies the maximum
/// element-wise root-mean-square error between the incoming matrix and the to
/// be detected matrix that results in a positive match. `ignore_phase`
/// specifies whether the aforementioned check should ignore global phase or
/// not.
///
/// The current (version 0) defaults are equivalent to calling:
///
/// ```C
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_PAULI_I, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_PAULI_X, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_PAULI_Y, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_PAULI_Z, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_H, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_S, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_S_DAG, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_T, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_T_DAG, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_RX_90, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_RX_M90, epsilon, ignore_gphase, 0);
/// if (!ignore_gphase) {
///   dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_RX_180, epsilon, ignore_gphase, 0);
/// }
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_RY_90, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_RY_M90, epsilon, ignore_gphase, 0);
/// if (!ignore_gphase) {
///   dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_RY_180, epsilon, ignore_gphase, 0);
///   dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_RZ_90, epsilon, ignore_gphase, 0);
///   dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_RZ_M90, epsilon, ignore_gphase, 0);
///   dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_RZ_180, epsilon, ignore_gphase, 0);
/// }
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_RX, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_RY, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_RZ, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_R, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_SWAP, epsilon, ignore_gphase, 0);
/// dqcs_mmb_add_internal(mmb, NULL, NULL, DQCS_GATE_SQRT_SWAP, epsilon, ignore_gphase, 0);
/// ```
#[no_mangle]
pub extern "C" fn dqcs_mmb_add_defaults(
    mmb: dqcs_handle_t,
    version: usize,
    epsilon: c_double,
    ignore_gphase: bool,
) -> dqcs_return_t {
    api_return_none(|| {
        // TODO
        err("not yet implemented")
    })
}

/// Adds a gate matrix detector for the given DQCsim-defined gate to the given
/// matrix map builder.
///
/// `mmb` must be a handle to a matrix map builder object (`dqcs_mmb_new()`).
/// `key_data` is a user-specified value that is returned as part of the match
/// function result when this detector is the first to match. `key_free` is an
/// optional callback function used to free `key_data` when the matrix map
/// (builder) is destroyed, or when this function fails. `gate` defines
/// which gate to detect. Some of the detectable gates are parameterized. Note
/// that there are no controlled gates, because they are normally encoded
/// separately in the DQCsim protocol and therefore not part of the matrix; to
/// detect gate matrices with explicit control qubits, preprocess the gates
/// with `dqcs_gate_reduce_control()` first. `epsilon` specifies the maximum
/// element-wise root-mean-square error between the incoming matrix and the to
/// be detected matrix that results in a positive match. `ignore_phase`
/// specifies whether the aforementioned check should ignore global phase or
/// not. `param_proto` optionally specifies an `ArbData` handle (consumed by
/// this function) used as a prototype for the gate parameter data returned by
/// the match function. The gate detectors will push the following parameters
/// as binary strings on top of any binary strings in the prototype object:
///
///  - all gate detectors push an unsigned 32-bit integer with the value of the
///    `dqcs_internal_gate_t` enum.
///  - `DQCS_GATE_RX`, `DQCS_GATE_RY`, and `DQCS_GATE_RZ` push a 64-bit double
///    floating point with the angle in addition to the enum.
///  - `DQCS_GATE_RK` pushes a 32-bit integer with the k parameter in addition
///    to the enum.
///  - `DQCS_GATE_R` pushes a the three double floating point angles in
///    addition to the enum (ordered theta, phi, lambda).
///  - `DQCS_GATE_U` pushes the entire matrix as a single argument consisting
///    of 2**N * 2**N * 2 doubles, in real-first row-major format (same as the
///    other matrix definitions in DQCsim).
#[no_mangle]
pub extern "C" fn dqcs_mmb_add_internal(
    mmb: dqcs_handle_t,
    key_free: Option<extern "C" fn(user_data: *mut c_void)>,
    key_data: *mut c_void,
    gate: dqcs_internal_gate_t,
    epsilon: c_double,
    ignore_gphase: bool,
    param_proto: dqcs_handle_t,
) -> dqcs_return_t {
    api_return_none(|| {
        // TODO
        err("not yet implemented")
    })
}

/// Adds a gate matrix detector for the given gate matrix to the given
/// matrix map builder.
///
/// `mmb` must be a handle to a matrix map builder object (`dqcs_mmb_new()`).
/// `key_data` is a user-specified value that is returned as part of the match
/// function result when this detector is the first to match. `key_free` is an
/// optional callback function used to free `key_data` when the matrix map
/// (builder) is destroyed, or when this function fails. `matrix` must
/// point to an appropriately sized array of doubles, representing the unitary
/// matrix that is to be detected. The matrix is specified in row-major form,
/// using pairs of doubles for the real vs. imaginary component of each entry.
/// `matrix_len` must be set to the number of complex numbers in the matrix
/// (4 for a one-qubit gate, 16 for a two-qubit gate, 256 for a three-qubit
/// gate and so on). `epsilon` specifies the maximum element-wise
/// root-mean-square error between the incoming matrix and the to be detected
/// matrix that results in a positive match. `ignore_phase` specifies whether
/// the aforementioned check should ignore global phase or not. `param_data`
/// optionally specifies an `ArbData` handle (consumed by this function) of
/// which a copy will be returned by the detector as the gate parameter data
/// object when a positive match occurs.
#[no_mangle]
pub extern "C" fn dqcs_mmb_add_fixed(
    mmb: dqcs_handle_t,
    key_free: Option<extern "C" fn(key_data: *mut c_void)>,
    key_data: *mut c_void,
    matrix: *const c_double,
    matrix_len: size_t,
    epsilon: c_double,
    ignore_gphase: bool,
    param_data: dqcs_handle_t,
) -> dqcs_return_t {
    api_return_none(|| {
        // TODO
        err("not yet implemented")
    })
}

/// Adds a custom gate matrix detector to the given matrix map builder.
///
/// `mmb` must be a handle to a matrix map builder object (`dqcs_mmb_new()`).
/// `key_data` is a user-specified value that is returned as part of the match
/// function result when this detector is the first to match. `key_free` is an
/// optional callback function used to free `key_data` when the matrix map
/// (builder) is destroyed, or when this function fails.
///
/// The callback receives a matrix for the user to match. If the gate matches,
/// the function must return `DQCS_TRUE`. If it doesn't match, it must return
/// `DQCS_FALSE`. If an error occurs, it must call `dqcs_error_set()` with the
/// error message and return `DQCS_BOOL_FAILURE`.
///
/// When the gate matches, the function can return a gate parameter data object
/// by way of assigning `param_data` to a new `ArbData` handle. Ownership of
/// this handle is passed to the callee regardless of the return value (that is,
/// even if no match is reported, the callee will ensure that the `ArbData`
/// handle passed here, if any, is freed). `param_data` will always point to 0
/// when the function is called.
///
/// It is up to the user how to do this matching, but the function is assumed
/// to always return the same value for the same input. Otherwise, the caching
/// behavior of the `GateMap` will make the results inconsistent.
#[no_mangle]
pub extern "C" fn dqcs_mmb_add_user(
    mmb: dqcs_handle_t,
    key_free: Option<extern "C" fn(key_data: *mut c_void)>,
    key_data: *mut c_void,
    callback: extern "C" fn(
        user_data: *const c_void,
        matrix: *const c_double,
        matrix_len: size_t,
        param_data: *mut dqcs_handle_t,
    ) -> dqcs_bool_return_t,
    user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        // TODO
        err("not yet implemented")
    })
}

/// Finalizes a matrix map builder object into a matrix map object.
///
/// `mmb` must be a handle to a matrix map builder object (`dqcs_mmb_new()`).
/// This handle is consumed if the function succeeds.
///
/// This function returns a new handle to a matrix map object, which can then
/// be used to map matrices to user-specified keys.
#[no_mangle]
pub extern "C" fn dqcs_mm_new(mmb: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        // TODO
        err("not yet implemented")
    })
}

/// Uses a matrix map object to map the given matrix to some key representing
/// the gate type.
///
/// `mm` must be a handle to a matrix map object (`dqcs_mm_new()`). `matrix`
/// must point to an appropriately sized array of doubles, representing the
/// unitary matrix that is to be matched. The matrix is specified in row-major
/// form, using pairs of doubles for the real vs. imaginary component of each
/// entry. `matrix_len` must be set to the number of complex numbers in the
/// matrix (4 for a one-qubit gate, 16 for a two-qubit gate, 256 for a
/// three-qubit gate and so on). `key_data` serves as an optional return
/// value; if non-NULL and a match is found, the `key_data` specified
/// when the respective matcher was added is returned here as a `const void *`.
/// If no match is found, `key_data` is not written to. `param_data` also
/// serves as an optional return value; if non-NULL and a match is found,
/// it is set to a handle to a new `ArbData` object representing the gate's
/// parameters. Ownership of this object is passed to the user, so it is up
/// to the user to eventually delete this object. If no match is found,
/// `param_data` is set to 0. This function returns `DQCS_TRUE` if a match was
/// found, `DQCS_FALSE` if no match was found, or `DQCS_BOOL_FAILURE` if an
/// error occurs.
#[no_mangle]
pub extern "C" fn dqcs_mm_map_matrix(
    mm: dqcs_handle_t,
    matrix: *const c_double,
    matrix_len: size_t,
    key_data: *mut *const c_void,
    param_data: *mut dqcs_handle_t,
) -> dqcs_bool_return_t {
    api_return_bool(|| {
        // TODO
        err("not yet implemented")
    })
}

/// Uses a matrix map object to map the matrix of the given gate to some key
/// representing the gate type.
///
/// `mm` must be a handle to a matrix map object (`dqcs_mm_new()`). `gate` must
/// be a handle to a gate that has a matrix (that is, either a unitary gate, or
/// a custom gate with a matrix). `key_data` serves as an optional return
/// value; if non-NULL and a match is found, the `key_data` specified
/// when the respective matcher was added is returned here as a `const void *`.
/// If no match is found, `key_data` is not written to. `param_data` also
/// serves as an optional return value; if non-NULL and a match is found,
/// it is set to a handle to a new `ArbData` object representing the gate's
/// parameters. Ownership of this object is passed to the user, so it is up
/// to the user to eventually delete this object. If no match is found,
/// `param_data` is set to 0. This function returns `DQCS_TRUE` if a match was
/// found, `DQCS_FALSE` if no match was found, or `DQCS_BOOL_FAILURE` if an
/// error occurs.
#[no_mangle]
pub extern "C" fn dqcs_mm_map_gate(
    mm: dqcs_handle_t,
    gate: dqcs_handle_t,
    key_data: *mut *const c_void,
    param_data: *mut dqcs_handle_t,
) -> dqcs_bool_return_t {
    api_return_bool(|| {
        // TODO
        err("not yet implemented")
    })
}

/// Clears the cache of a matrix map object.
///
/// Matrix map objects internally cache detected matrices to increase detection
/// speed when a matrix is received a second time. This function clears the
/// cache.
#[no_mangle]
pub extern "C" fn dqcs_mm_clear_cache(mm: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        // TODO
        err("not yet implemented")
    })
}
