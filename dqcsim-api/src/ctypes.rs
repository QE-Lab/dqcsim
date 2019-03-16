use libc::*;

/// Object type for a handle.
///
/// Handles are like pointers into DQCsim's internal structures: all API calls
/// use these to refer to objects. Besides the object, they contain type
/// information. This type can be retrieved using `dqcs_handle_type()`.
///
/// Handles are always positive integers, counting upwards from 1 upon
/// allocation, and they are not reused even after being deleted. Thus, every
/// subsequent object allocation returns a handle one greater than the
/// previous. This is guaranteed behavior that code can rely upon. The value
/// zero is reserved for invalid references or error propagation.
///
/// Note that the scope for handles is thread-local. That is, data referenced
/// by a handle cannot be shared or moved between threads.
///
/// The value zero is reserved for invalid references or error propagation.
#[allow(non_camel_case_types)]
pub type dqcs_handle_t = c_ulonglong;

/// Object type for a qubit reference.
///
/// Qubit references are exchanged between the frontend, operator, and backend
/// plugins to indicate which qubits a gate operates on. Note that this makes
/// them fundamentally different from handles, which are thread-local.
///
/// Qubit references are always positive integers, counting upwards from 1 upon
/// allocation, and they are not reused even after the qubit is deallocated.
/// Thus, every subsequent allocation returns a qubit reference one greater
/// than the previous. This is guaranteed behavior that code can rely upon. The
/// value zero is reserved for invalid references or error propagation.
#[allow(non_camel_case_types)]
pub type dqcs_qubit_t = c_ulonglong;

/// Object type for a simulation cycle timestamp.
///
/// Timestamps count upward from zero. The type is signed to allow usage of -1
/// for errors, and to allow numerical differences to be represented.
#[allow(non_camel_case_types)]
pub type dqcs_cycle_t = c_longlong;

/// Enumeration of types that can be associated with a handle.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum dqcs_handle_type_t {
    /// Indicates that the given handle is invalid.
    ///
    /// This indicates one of the following:
    ///
    ///  - The handle value is invalid (zero or negative).
    ///  - The handle has not been used yet.
    ///  - The object associated with the handle was deleted.
    DQCS_INVALID = 0,

    /// Indicates that the given handle belongs to an `ArbData` object.
    ///
    /// This means that the handle supports the `handle` and `arb` interfaces.
    DQCS_ARB_DATA = 1,

    /// Indicates that the given handle belongs to an `ArbCmd` object.
    ///
    /// This means that the handle supports the `handle`, `arb`, and `cmd`
    /// interfaces.
    DQCS_ARB_CMD = 2,

    /// Indicates that the given handle belongs to a `Gate` object.
    ///
    /// This means that the handle supports the `handle`, `arb`, and `gate`
    /// interfaces.
    DQCS_GATE = 3,

    /// Indicates that the given handle belongs to a frontend plugin
    /// configuration object.
    DQCS_FRONT_CONFIG = 4,

    /// Indicates that the given handle belongs to an operator plugin
    /// configuration object.
    DQCS_OPER_CONFIG = 5,

    /// Indicates that the given handle belongs to a backend plugin
    /// configuration object.
    DQCS_BACK_CONFIG = 6,

    /// Indicates that the given handle belongs to a simulator configuration
    /// object.
    DQCS_SIM_CONFIG = 7,
}

/// Enumeration of the different qubit sets associated with a gate.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum dqcs_qubit_set_type_t {
    /// The qubit list containing the target qubits.
    ///
    /// The target qubit list is the list of qubits that are affected by the
    /// gate matrix. Thus, the size of this list dictates the correct size of
    /// the gate matrix.
    DQCS_TARGET = 1,

    /// Set containing additional control qubits.
    ///
    /// These qubits are omitted from the gate matrix; the complete gate matrix
    /// including control qubits is inferred by the backend. Of course, it is
    /// also fine for operators or frontend to provide the complete matrix
    /// including control qubits, by putting the control qubits in the target
    /// list instead.
    DQCS_CONTROL = 2,

    /// Set containing all the qubit measurement registers affected by the
    /// associated gate.
    ///
    /// DQCsim uses this set to determine up to what point it needs to
    /// synchronize with the downstream plugins when a measurement register is
    /// read. Therefore, this set must be correctly specified regardless of
    /// whether the backend infers anything from it. For instance, a
    /// `measure_all` gate *must* include all qubits in this set.
    DQCS_MEASURE = 3,
}

/// Default return type for functions that don't need to return anything.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum dqcs_return_t {
    /// The function has failed. More information may be obtained through
    /// `dqcsim_explain()`.
    DQCS_FAILURE = -1,

    /// The function did what it was supposed to.
    DQCS_SUCCESS = 0,
}

/// Return type for functions that normally return a boolean but can also fail.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum dqcs_bool_return_t {
    /// The function has failed. More information may be obtained through
    /// `dqcsim_explain()`.
    DQCS_BOOL_FAILURE = -1,

    /// The function did what it was supposed to and returned false.
    DQCS_FALSE = 0,

    /// The function did what it was supposed to and returned true.
    DQCS_TRUE = 1,
}

impl From<bool> for dqcs_bool_return_t {
    fn from(b: bool) -> Self {
        if b {
            dqcs_bool_return_t::DQCS_TRUE
        } else {
            dqcs_bool_return_t::DQCS_FALSE
        }
    }
}
