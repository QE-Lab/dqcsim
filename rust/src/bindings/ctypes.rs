use super::*;
use crate::core::common::{error::Error, gates::GateType};
use std::convert::TryFrom;

/// Type for a handle.
///
/// Handles are like pointers into DQCsim's internal structures: all API calls
/// use these to refer to objects. Besides the object, they contain type
/// information. This type can be retrieved using `dqcs_handle_type()`.
///
/// Handles are always positive integers, counting upwards from 1 upon
/// allocation, and they are not reused even after being deleted. Thus, every
/// subsequent object allocation returns a handle one greater than the
/// previous. Note however that DQCsim may allocate objects as well without
/// the user specifically requesting this, so external code should generally
/// *not* rely on this behavior unless otherwise noted. The value zero is
/// reserved for invalid references or error propagation.
///
/// Note that the scope for handles is thread-local. That is, data referenced
/// by a handle cannot be shared or moved between threads.
///
/// The value zero is reserved for invalid references or error propagation.
#[allow(non_camel_case_types)]
pub type dqcs_handle_t = c_ulonglong;

/// Type for a qubit reference.
///
/// Qubit references are exchanged between the frontend, operator, and backend
/// plugins to indicate which qubits a gate operates on. Note that this makes
/// them fundamentally different from handles, which are thread-local.
///
/// Qubit references are always positive integers, counting upwards from 1 upon
/// allocation, and they are not reused even after the qubit is deallocated.
/// Thus, every subsequent allocation returns a qubit reference one greater
/// than the previous. This is guaranteed behavior that external code can rely
/// upon. The value zero is reserved for invalid references or error
/// propagation.
#[allow(non_camel_case_types)]
pub type dqcs_qubit_t = c_ulonglong;

/// Type for a simulation cycle timestamp.
///
/// Timestamps count upward from zero. The type is signed to allow usage of -1
/// for errors, and to allow numerical differences to be represented.
#[allow(non_camel_case_types)]
pub type dqcs_cycle_t = c_longlong;

/// Type for a plugin state.
///
/// This is an opaque type that is passed along to plugin implementation
/// callback functions, which those callbacks can then use to interact with the
/// plugin instance. User code shall not create or modify values of this type,
/// and shall only use the values when calling `dqcs_plugin_*` functions.
#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct dqcs_plugin_state_t(*mut c_void);

impl<'a> From<&mut PluginState<'a>> for dqcs_plugin_state_t {
    /// Convert a plugin state reference to its FFI representation.
    fn from(pc: &mut PluginState) -> dqcs_plugin_state_t {
        dqcs_plugin_state_t(pc as *mut PluginState as *mut c_void)
    }
}

impl Into<&mut PluginState<'static>> for dqcs_plugin_state_t {
    /// Convert the FFI representation of a plugin state back to a Rust
    /// reference.
    fn into(self) -> &'static mut PluginState<'static> {
        unsafe { &mut *(self.0 as *mut PluginState) }
    }
}

impl dqcs_plugin_state_t {
    pub fn resolve(self) -> Result<&'static mut PluginState<'static>> {
        if self.0.is_null() {
            inv_arg("plugin state pointer is null")
        } else {
            Ok(self.into())
        }
    }
}

/// Enumeration of types that can be associated with a handle.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum dqcs_handle_type_t {
    /// Indicates that the given handle is invalid.
    ///
    /// This indicates one of the following:
    ///
    ///  - The handle value is invalid (zero or negative).
    ///  - The handle has not been used yet.
    ///  - The object associated with the handle was deleted.
    DQCS_HTYPE_INVALID = 0,

    /// Indicates that the given handle belongs to an `ArbData` object.
    ///
    /// This means that the handle supports the `handle` and `arb` interfaces.
    DQCS_HTYPE_ARB_DATA = 100,

    /// Indicates that the given handle belongs to an `ArbCmd` object.
    ///
    /// This means that the handle supports the `handle`, `arb`, and `cmd`
    /// interfaces.
    DQCS_HTYPE_ARB_CMD = 101,

    /// Indicates that the given handle belongs to a queue of `ArbCmd` object.
    ///
    /// This means that the handle supports the `handle`, `arb`, `cmd`, and
    /// `cq` interfaces.
    DQCS_HTYPE_ARB_CMD_QUEUE = 102,

    /// Indicates that the given handle belongs to a set of qubit references.
    ///
    /// This means that the handle supports the `handle` and `qbset`
    /// interfaces.
    DQCS_HTYPE_QUBIT_SET = 103,

    /// Indicates that the given handle belongs to a quantum gate description.
    ///
    /// This means that the handle supports the `handle`, `gate`, and `arb`
    /// interfaces.
    DQCS_HTYPE_GATE = 104,

    /// Indicates that the given handle belongs to a qubit measurement result.
    ///
    /// This means that the handle supports the `handle`, `meas`, and `arb`
    /// interfaces. It can also be used in place of a qubit measurement result
    /// set by functions that consume the object.
    DQCS_HTYPE_MEAS = 105,

    /// Indicates that the given handle belongs to a set of qubit measurement
    /// results.
    ///
    /// This means that the handle supports the `handle` and `mset` interfaces.
    DQCS_HTYPE_MEAS_SET = 106,

    /// Indicates that the given handle belongs to a matrix.
    ///>
    ///> This means that the handle supports the `handle` and `mat` interfaces.
    DQCS_HTYPE_MATRIX = 107,

    /// Indicates that the given handle belongs to a gate map.
    ///>
    ///> This means that the handle supports the `handle` and `gm` interfaces.
    DQCS_HTYPE_GATE_MAP = 108,

    /// Indicates that the given handle belongs to a frontend plugin process
    /// configuration object.
    ///
    /// This means that the handle supports the `handle`, `pcfg`, and `xcfg`
    /// interfaces.
    DQCS_HTYPE_FRONT_PROCESS_CONFIG = 200,

    /// Indicates that the given handle belongs to an operator plugin process
    /// configuration object.
    ///
    /// This means that the handle supports the `handle`, `pcfg`, and `xcfg`
    /// interfaces.
    DQCS_HTYPE_OPER_PROCESS_CONFIG = 201,

    /// Indicates that the given handle belongs to a backend plugin process
    /// configuration object.
    ///
    /// This means that the handle supports the `handle`, `pcfg`, and `xcfg`
    /// interfaces.
    DQCS_HTYPE_BACK_PROCESS_CONFIG = 203,

    /// Indicates that the given handle belongs to a frontend plugin thread
    /// configuration object.
    ///
    /// This means that the handle supports the `handle`, `tcfg`, and `xcfg`
    /// interfaces.
    DQCS_HTYPE_FRONT_THREAD_CONFIG = 204,

    /// Indicates that the given handle belongs to an operator plugin thread
    /// configuration object.
    ///
    /// This means that the handle supports the `handle`, `tcfg`, and `xcfg`
    /// interfaces.
    DQCS_HTYPE_OPER_THREAD_CONFIG = 205,

    /// Indicates that the given handle belongs to a backend plugin thread
    /// configuration object.
    ///
    /// This means that the handle supports the `handle`, `tcfg`, and `xcfg`
    /// interfaces.
    DQCS_HTYPE_BACK_THREAD_CONFIG = 206,

    /// Indicates that the given handle belongs to a simulator configuration
    /// object.
    ///
    /// This means that the handle supports the `handle` and `scfg` interfaces.
    DQCS_HTYPE_SIM_CONFIG = 207,

    /// Indicates that the given handle belongs to a simulator instance.
    ///
    /// This means that the handle supports the `handle` and `sim` interfaces.
    DQCS_HTYPE_SIM = 208,

    /// Indicates that the given handle belongs to a frontend plugin
    /// definition object.
    ///
    /// This means that the handle supports the `handle` and `pdef` interfaces.
    DQCS_HTYPE_FRONT_DEF = 300,

    /// Indicates that the given handle belongs to an operator plugin
    /// definition object.
    ///
    /// This means that the handle supports the `handle` and `pdef` interfaces.
    DQCS_HTYPE_OPER_DEF = 301,

    /// Indicates that the given handle belongs to a backend plugin
    /// definition object.
    ///
    /// This means that the handle supports the `handle` and `pdef` interfaces.
    DQCS_HTYPE_BACK_DEF = 302,

    /// Indicates that the given handle belongs to a plugin thread join handle.
    ///
    /// This means that the handle supports the `handle` and `pjoin`
    /// interfaces.
    DQCS_HTYPE_PLUGIN_JOIN = 303,
}

/// Enumeration of the three types of plugins.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum dqcs_plugin_type_t {
    /// Invalid plugin type. Used to indicate failure of an API that returns
    /// a plugin type.
    DQCS_PTYPE_INVALID = -1,

    /// Frontend plugin.
    DQCS_PTYPE_FRONT = 0,

    /// Operator plugin.
    DQCS_PTYPE_OPER = 1,

    /// Backend plugin.
    DQCS_PTYPE_BACK = 2,
}

impl From<PluginType> for dqcs_plugin_type_t {
    fn from(x: PluginType) -> Self {
        match x {
            PluginType::Frontend => dqcs_plugin_type_t::DQCS_PTYPE_FRONT,
            PluginType::Operator => dqcs_plugin_type_t::DQCS_PTYPE_OPER,
            PluginType::Backend => dqcs_plugin_type_t::DQCS_PTYPE_BACK,
        }
    }
}

impl Into<Result<PluginType>> for dqcs_plugin_type_t {
    fn into(self) -> Result<PluginType> {
        match self {
            dqcs_plugin_type_t::DQCS_PTYPE_FRONT => Ok(PluginType::Frontend),
            dqcs_plugin_type_t::DQCS_PTYPE_OPER => Ok(PluginType::Operator),
            dqcs_plugin_type_t::DQCS_PTYPE_BACK => Ok(PluginType::Backend),
            _ => inv_arg("invalid plugin type"),
        }
    }
}

/// Enumeration of loglevels and logging modes.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum dqcs_loglevel_t {
    /// Invalid loglevel. Used to indicate failure of an API that returns a
    /// loglevel.
    DQCS_LOG_INVALID = -1,

    /// Turns logging off.
    DQCS_LOG_OFF = 0,

    /// This loglevel is to be used for reporting a fatal error, resulting from
    /// the owner of the logger getting into an illegal state from which it
    /// cannot recover. Such problems are also reported to the API caller via
    /// Result::Err if applicable.
    DQCS_LOG_FATAL = 1,

    /// This loglevel is to be used for reporting or propagating a non-fatal
    /// error caused by the API caller doing something wrong. Such problems are
    /// also reported to the API caller via Result::Err if applicable.
    DQCS_LOG_ERROR = 2,

    /// This loglevel is to be used for reporting that a called API/function is
    /// telling us we did something wrong (that we weren't expecting), but we
    /// can recover. For instance, for a failed connection attempt to something
    /// that really should not be failing, we can still retry (and eventually
    /// report critical or error if a retry counter overflows). Since we're
    /// still trying to rectify things at this point, such problems are NOT
    /// reported to the API/function caller via Result::Err.
    DQCS_LOG_WARN = 3,

    /// This loglevel is to be used for reporting information specifically
    /// requested by the user/API caller, such as the result of an API function
    /// requested through the command line, or an explicitly captured
    /// stdout/stderr stream.
    DQCS_LOG_NOTE = 4,

    /// This loglevel is to be used for reporting information NOT specifically
    /// requested by the user/API caller, such as a plugin starting up or
    /// shutting down.
    DQCS_LOG_INFO = 5,

    /// This loglevel is to be used for reporting debugging information useful
    /// for debugging the user of the API provided by the logged instance.
    DQCS_LOG_DEBUG = 6,

    /// This loglevel is to be used for reporting debugging information useful
    /// for debugging the internals of the logged instance. Such messages would
    /// normally only be generated by debug builds, to prevent them from
    /// impacting performance under normal circumstances.
    DQCS_LOG_TRACE = 7,

    /// This is intended to be used when configuring the stdout/stderr capture
    /// mode for a plugin process. Selecting it will prevent the stream from
    /// being captured; it will just be the same stream as DQCsim's own
    /// stdout/stderr. When used as the loglevel for a message, the message
    /// itself is sent to stderr instead of passing into DQCsim's log system.
    /// Using this for loglevel filters leads to undefined behavior.
    DQCS_LOG_PASS = 8,
}

impl From<StreamCaptureMode> for dqcs_loglevel_t {
    fn from(x: StreamCaptureMode) -> Self {
        match x {
            StreamCaptureMode::Pass => dqcs_loglevel_t::DQCS_LOG_PASS,
            StreamCaptureMode::Null => dqcs_loglevel_t::DQCS_LOG_OFF,
            StreamCaptureMode::Capture(loglevel) => loglevel.into(),
        }
    }
}

impl Into<Result<StreamCaptureMode>> for dqcs_loglevel_t {
    fn into(self) -> Result<StreamCaptureMode> {
        match self {
            dqcs_loglevel_t::DQCS_LOG_INVALID => inv_arg("invalid level"),
            dqcs_loglevel_t::DQCS_LOG_OFF => Ok(StreamCaptureMode::Null),
            dqcs_loglevel_t::DQCS_LOG_FATAL => Ok(StreamCaptureMode::Capture(Loglevel::Fatal)),
            dqcs_loglevel_t::DQCS_LOG_ERROR => Ok(StreamCaptureMode::Capture(Loglevel::Error)),
            dqcs_loglevel_t::DQCS_LOG_WARN => Ok(StreamCaptureMode::Capture(Loglevel::Warn)),
            dqcs_loglevel_t::DQCS_LOG_NOTE => Ok(StreamCaptureMode::Capture(Loglevel::Note)),
            dqcs_loglevel_t::DQCS_LOG_INFO => Ok(StreamCaptureMode::Capture(Loglevel::Info)),
            dqcs_loglevel_t::DQCS_LOG_DEBUG => Ok(StreamCaptureMode::Capture(Loglevel::Debug)),
            dqcs_loglevel_t::DQCS_LOG_TRACE => Ok(StreamCaptureMode::Capture(Loglevel::Trace)),
            dqcs_loglevel_t::DQCS_LOG_PASS => Ok(StreamCaptureMode::Pass),
        }
    }
}

impl From<Loglevel> for dqcs_loglevel_t {
    fn from(x: Loglevel) -> Self {
        match x {
            Loglevel::Fatal => dqcs_loglevel_t::DQCS_LOG_FATAL,
            Loglevel::Error => dqcs_loglevel_t::DQCS_LOG_ERROR,
            Loglevel::Warn => dqcs_loglevel_t::DQCS_LOG_WARN,
            Loglevel::Note => dqcs_loglevel_t::DQCS_LOG_NOTE,
            Loglevel::Info => dqcs_loglevel_t::DQCS_LOG_INFO,
            Loglevel::Debug => dqcs_loglevel_t::DQCS_LOG_DEBUG,
            Loglevel::Trace => dqcs_loglevel_t::DQCS_LOG_TRACE,
        }
    }
}

impl dqcs_loglevel_t {
    pub fn into_capture_mode(self) -> Result<StreamCaptureMode> {
        self.into()
    }

    pub fn into_loglevel(self) -> Result<Loglevel> {
        match self.into_capture_mode()? {
            StreamCaptureMode::Capture(level) => Ok(level),
            _ => inv_arg(format!("invalid loglevel {:?}", self)),
        }
    }

    pub fn into_loglevel_filter(self) -> Result<LoglevelFilter> {
        match self.into_capture_mode()? {
            StreamCaptureMode::Capture(level) => Ok(level.into()),
            StreamCaptureMode::Null => Ok(LoglevelFilter::Off),
            _ => inv_arg(format!("invalid loglevel filter {:?}", self)),
        }
    }
}

impl From<LoglevelFilter> for dqcs_loglevel_t {
    fn from(x: LoglevelFilter) -> Self {
        match x {
            LoglevelFilter::Off => dqcs_loglevel_t::DQCS_LOG_OFF,
            LoglevelFilter::Fatal => dqcs_loglevel_t::DQCS_LOG_FATAL,
            LoglevelFilter::Error => dqcs_loglevel_t::DQCS_LOG_ERROR,
            LoglevelFilter::Warn => dqcs_loglevel_t::DQCS_LOG_WARN,
            LoglevelFilter::Note => dqcs_loglevel_t::DQCS_LOG_NOTE,
            LoglevelFilter::Info => dqcs_loglevel_t::DQCS_LOG_INFO,
            LoglevelFilter::Debug => dqcs_loglevel_t::DQCS_LOG_DEBUG,
            LoglevelFilter::Trace => dqcs_loglevel_t::DQCS_LOG_TRACE,
        }
    }
}

/// Default return type for functions that don't need to return anything.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
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
#[derive(Debug, Copy, Clone, PartialEq)]
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

/// Qubit measurement value.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum dqcs_measurement_t {
    /// Error value used to indicate that something went wrong.
    DQCS_MEAS_INVALID = -1,

    /// Indicates that the qubit was measured to be zero.
    DQCS_MEAS_ZERO = 0,

    /// Indicates that the qubit was measured to be one.
    DQCS_MEAS_ONE = 1,

    /// Indicates that the measurement value is unknown for whatever reason.
    DQCS_MEAS_UNDEFINED = 2,
}

impl Into<Option<QubitMeasurementValue>> for dqcs_measurement_t {
    fn into(self) -> Option<QubitMeasurementValue> {
        match self {
            dqcs_measurement_t::DQCS_MEAS_INVALID => None,
            dqcs_measurement_t::DQCS_MEAS_ZERO => Some(QubitMeasurementValue::Zero),
            dqcs_measurement_t::DQCS_MEAS_ONE => Some(QubitMeasurementValue::One),
            dqcs_measurement_t::DQCS_MEAS_UNDEFINED => Some(QubitMeasurementValue::Undefined),
        }
    }
}

impl From<QubitMeasurementValue> for dqcs_measurement_t {
    fn from(x: QubitMeasurementValue) -> dqcs_measurement_t {
        match x {
            QubitMeasurementValue::Undefined => dqcs_measurement_t::DQCS_MEAS_UNDEFINED,
            QubitMeasurementValue::Zero => dqcs_measurement_t::DQCS_MEAS_ZERO,
            QubitMeasurementValue::One => dqcs_measurement_t::DQCS_MEAS_ONE,
        }
    }
}

/// Reproduction file path style.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum dqcs_path_style_t {
    /// Error value used to indicate that something went wrong.
    DQCS_PATH_STYLE_INVALID = -1,

    /// Specifies that paths should be saved the same way they were specified
    /// on the command line.
    DQCS_PATH_STYLE_KEEP = 0,

    /// Specifies that all paths should be saved relative to DQCsim's working
    /// directory.
    DQCS_PATH_STYLE_RELATIVE = 1,

    /// Specifies that all paths should be saved canonically, i.e. relative to
    /// the root directory.
    DQCS_PATH_STYLE_ABSOLUTE = 2,
}

impl Into<Option<ReproductionPathStyle>> for dqcs_path_style_t {
    fn into(self) -> Option<ReproductionPathStyle> {
        match self {
            dqcs_path_style_t::DQCS_PATH_STYLE_INVALID => None,
            dqcs_path_style_t::DQCS_PATH_STYLE_KEEP => Some(ReproductionPathStyle::Keep),
            dqcs_path_style_t::DQCS_PATH_STYLE_RELATIVE => Some(ReproductionPathStyle::Relative),
            dqcs_path_style_t::DQCS_PATH_STYLE_ABSOLUTE => Some(ReproductionPathStyle::Absolute),
        }
    }
}

impl From<ReproductionPathStyle> for dqcs_path_style_t {
    fn from(x: ReproductionPathStyle) -> dqcs_path_style_t {
        match x {
            ReproductionPathStyle::Keep => dqcs_path_style_t::DQCS_PATH_STYLE_KEEP,
            ReproductionPathStyle::Relative => dqcs_path_style_t::DQCS_PATH_STYLE_RELATIVE,
            ReproductionPathStyle::Absolute => dqcs_path_style_t::DQCS_PATH_STYLE_ABSOLUTE,
        }
    }
}

/// Enumeration of gates defined by DQCsim.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum dqcs_internal_gate_t {
    /// Invalid gate.
    DQCS_GATE_INVALID = 0,

    /// Identity gate.
    DQCS_GATE_PAULI_I = 100,

    /// Identity gate.
    DQCS_GATE_PAULI_X = 101,

    /// Identity gate.
    DQCS_GATE_PAULI_Y = 102,

    /// Identity gate.
    DQCS_GATE_PAULI_Z = 103,

    /// Hadamard gate.
    DQCS_GATE_H = 104,

    /// S gate.
    DQCS_GATE_S = 105,

    /// S-dagger gate.
    DQCS_GATE_S_DAG = 106,

    /// T gate.
    DQCS_GATE_T = 107,

    /// T-dagger gate.
    DQCS_GATE_T_DAG = 108,

    /// RX(pi) gate.
    DQCS_GATE_RX_90 = 109,

    /// RX(-pi) gate.
    DQCS_GATE_RX_M90 = 110,

    /// RX(2pi) gate.
    DQCS_GATE_RX_180 = 111,

    /// RX(pi) gate.
    DQCS_GATE_RY_90 = 112,

    /// RX(-pi) gate.
    DQCS_GATE_RY_M90 = 113,

    /// RX(2pi) gate.
    DQCS_GATE_RY_180 = 114,

    /// RX(pi) gate.
    DQCS_GATE_RZ_90 = 115,

    /// RX(-pi) gate.
    DQCS_GATE_RZ_M90 = 116,

    /// RX(2pi) gate.
    DQCS_GATE_RZ_180 = 117,

    /// Parameterized RX gate with radian angle.
    DQCS_GATE_RX = 150,

    /// Parameterized RY gate with radian angle.
    DQCS_GATE_RY = 151,

    /// Parameterized RZ gate with radian angle.
    DQCS_GATE_RZ = 152,

    /// Parameterized phase gate with pi/2^k angle, where k is an integer.
    DQCS_GATE_PHASE_K = 153,

    /// Parameterized phase gate. Phase is equivalent to RZ, but with its
    /// global phase defined such that using it as a submatrix for a controlled
    /// gate forms a proper controlled phase gate.
    DQCS_GATE_PHASE = 154,

    /// Any single-qubit unitary gate, parameterized as a full unitary matrix.
    DQCS_GATE_U = 190,

    /// Any single-qubit unitary gate, parameterized IBM-style with Z-Y-Z
    /// angles.
    DQCS_GATE_R = 191,

    /// Swap gate.
    DQCS_GATE_SWAP = 200,

    /// Square-root of swap gate.
    DQCS_GATE_SQRT_SWAP = 201,
}

impl TryFrom<dqcs_internal_gate_t> for GateType {
    type Error = Error;
    fn try_from(gate_type: dqcs_internal_gate_t) -> Result<Self> {
        match gate_type {
            dqcs_internal_gate_t::DQCS_GATE_PAULI_I => Ok(GateType::I),
            dqcs_internal_gate_t::DQCS_GATE_PAULI_X => Ok(GateType::X),
            dqcs_internal_gate_t::DQCS_GATE_PAULI_Y => Ok(GateType::Y),
            dqcs_internal_gate_t::DQCS_GATE_PAULI_Z => Ok(GateType::Z),
            dqcs_internal_gate_t::DQCS_GATE_H => Ok(GateType::H),
            dqcs_internal_gate_t::DQCS_GATE_S => Ok(GateType::S),
            dqcs_internal_gate_t::DQCS_GATE_S_DAG => Ok(GateType::SDAG),
            dqcs_internal_gate_t::DQCS_GATE_T => Ok(GateType::T),
            dqcs_internal_gate_t::DQCS_GATE_T_DAG => Ok(GateType::TDAG),
            dqcs_internal_gate_t::DQCS_GATE_RX_90 => Ok(GateType::RX90),
            dqcs_internal_gate_t::DQCS_GATE_RX_M90 => Ok(GateType::RXM90),
            dqcs_internal_gate_t::DQCS_GATE_RX_180 => Ok(GateType::RX180),
            dqcs_internal_gate_t::DQCS_GATE_RY_90 => Ok(GateType::RY90),
            dqcs_internal_gate_t::DQCS_GATE_RY_M90 => Ok(GateType::RYM90),
            dqcs_internal_gate_t::DQCS_GATE_RY_180 => Ok(GateType::RY180),
            dqcs_internal_gate_t::DQCS_GATE_RZ_90 => Ok(GateType::RZ90),
            dqcs_internal_gate_t::DQCS_GATE_RZ_M90 => Ok(GateType::RZM90),
            dqcs_internal_gate_t::DQCS_GATE_RZ_180 => Ok(GateType::RZ180),
            dqcs_internal_gate_t::DQCS_GATE_RX => Ok(GateType::RX),
            dqcs_internal_gate_t::DQCS_GATE_RY => Ok(GateType::RY),
            dqcs_internal_gate_t::DQCS_GATE_PHASE => Ok(GateType::Phase),
            dqcs_internal_gate_t::DQCS_GATE_PHASE_K => Ok(GateType::PhaseK),
            dqcs_internal_gate_t::DQCS_GATE_RZ => Ok(GateType::RZ),
            dqcs_internal_gate_t::DQCS_GATE_U => Ok(GateType::U(1)),
            dqcs_internal_gate_t::DQCS_GATE_R => Ok(GateType::R),
            dqcs_internal_gate_t::DQCS_GATE_SWAP => Ok(GateType::SWAP),
            dqcs_internal_gate_t::DQCS_GATE_SQRT_SWAP => Ok(GateType::SQSWAP),
            dqcs_internal_gate_t::DQCS_GATE_INVALID => inv_arg("invalid gate"),
        }
    }
}

impl From<dqcs_internal_gate_t> for Option<GateType> {
    fn from(gate_type: dqcs_internal_gate_t) -> Self {
        match gate_type {
            dqcs_internal_gate_t::DQCS_GATE_INVALID => None,
            dqcs_internal_gate_t::DQCS_GATE_PAULI_I => Some(GateType::I),
            dqcs_internal_gate_t::DQCS_GATE_PAULI_X => Some(GateType::X),
            dqcs_internal_gate_t::DQCS_GATE_PAULI_Y => Some(GateType::Y),
            dqcs_internal_gate_t::DQCS_GATE_PAULI_Z => Some(GateType::Z),
            dqcs_internal_gate_t::DQCS_GATE_H => Some(GateType::H),
            dqcs_internal_gate_t::DQCS_GATE_S => Some(GateType::S),
            dqcs_internal_gate_t::DQCS_GATE_S_DAG => Some(GateType::SDAG),
            dqcs_internal_gate_t::DQCS_GATE_T => Some(GateType::T),
            dqcs_internal_gate_t::DQCS_GATE_T_DAG => Some(GateType::TDAG),
            dqcs_internal_gate_t::DQCS_GATE_RX_90 => Some(GateType::RX90),
            dqcs_internal_gate_t::DQCS_GATE_RX_M90 => Some(GateType::RXM90),
            dqcs_internal_gate_t::DQCS_GATE_RX_180 => Some(GateType::RX180),
            dqcs_internal_gate_t::DQCS_GATE_RY_90 => Some(GateType::RY90),
            dqcs_internal_gate_t::DQCS_GATE_RY_M90 => Some(GateType::RYM90),
            dqcs_internal_gate_t::DQCS_GATE_RY_180 => Some(GateType::RY180),
            dqcs_internal_gate_t::DQCS_GATE_RZ_90 => Some(GateType::RZ90),
            dqcs_internal_gate_t::DQCS_GATE_RZ_M90 => Some(GateType::RZM90),
            dqcs_internal_gate_t::DQCS_GATE_RZ_180 => Some(GateType::RZ180),
            dqcs_internal_gate_t::DQCS_GATE_RX => Some(GateType::RX),
            dqcs_internal_gate_t::DQCS_GATE_RY => Some(GateType::RY),
            dqcs_internal_gate_t::DQCS_GATE_RZ => Some(GateType::RZ),
            dqcs_internal_gate_t::DQCS_GATE_PHASE => Some(GateType::Phase),
            dqcs_internal_gate_t::DQCS_GATE_PHASE_K => Some(GateType::PhaseK),
            dqcs_internal_gate_t::DQCS_GATE_U => Some(GateType::U(1)),
            dqcs_internal_gate_t::DQCS_GATE_R => Some(GateType::R),
            dqcs_internal_gate_t::DQCS_GATE_SWAP => Some(GateType::SWAP),
            dqcs_internal_gate_t::DQCS_GATE_SQRT_SWAP => Some(GateType::SQSWAP),
        }
    }
}
