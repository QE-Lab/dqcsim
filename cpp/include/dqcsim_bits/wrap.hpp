#ifndef _DQCSIM_INCLUDED_
//! \cond Doxygen_Suppress
#define _DQCSIM_INCLUDED_
//! \endcond

/*! \mainpage
 *
 * This is the generated documentation for the C++ interface of
 * <a href="../index.html">DQCsim</a>. A more tutorial-esque description is
 * available <a href="../cpp-api/index.html">here</a>; you should probably
 * have a look at that first.
 *
 * Everything related to the C++ API is provided by
 *
 * ```
 * #include <dqcsim>
 * ```
 *
 * Click <a href="dqcsim.html">here</a> for the documentation of that file.
 */

/*!
 * \file dqcsim
 * \brief Provides DQCsim's entire C++ API.
 *
 * This is the main file you should be including in pure C++ projects:
 *
 * \code
 * #include <dqcsim>
 * \endcode
 *
 * It should be installed into `/usr/include` by installing the DQCsim Python
 * module using `pip` as root. If you're pulling DQCsim in through CMake
 * instead, the appropriate include path should be added to your project
 * automatically.
 */

#include <stdexcept>
#include <string>
#include <vector>
#include <cstring>
#include <iostream>
#include <complex>
#include <functional>
#include <memory>
#include <cmath>
#include <limits>
#include <chrono>
#include <cdqcsim>

/**
 * Main DQCsim namespace.
 */
namespace dqcsim {

/**
 * Namespace containing thin wrapper objects around the handles exposed by
 * the raw C interface.
 *
 * The symbols in this namespace fully wrap those provided by the raw C API.
 * The following things are abstracted away.
 *
 *  - The signal-error-by-return-value and `dqcs_set_error`/`dqcs_get_error`
 *    error reporting system is replaced with C++ exceptions. This is handled
 *    primarily by the `check` function; placing this around any raw C API
 *    function will turn DQCsim errors into an `std::runtime_error`.
 *
 *  - The typedefs from the raw C API are replaced with or aliased by C++
 *    equivalents, making full use of C++'s namespacing to abbreviate names,
 *    and using more C++-esque naming conventions.
 *
 *  - Handles are abstracted to classes, using RAII to ensure that there are
 *    no leaks, inheritance to represent the difference interfaces supported by
 *    different handles, and strong typing to prevent APIs from being called on
 *    incompatible handles.
 *
 *  - The `dqcs_plugin_state_t` type is encapsulated by a class which can
 *    only be constructed by this library, cannot be moved or copied, and
 *    cannot be mutated. This prevents mistakes leading to crashes in the Rust
 *    portion of DQCsim.
 */
namespace wrap {

  /**
   * C++-styled type name for `raw::dqcs_handle_t`.
   *
   * This integer type is used to represent C API handles internally. You
   * normally shouldn't encounter this type.
   *
   * \note DQCsim starts counting handles from 1. Handle 0 is reserved for
   * signaling errors in the C API.
   */
  using HandleIndex = raw::dqcs_handle_t;

  /**
   * C++-styled type name for `raw::dqcs_qubit_t`.
   *
   * This integer type is used to represent qubit indices. You can do math on
   * this type if you like, though this normally doesn't bear any significance.
   * Most of the time you'll be using `QubitRef` instead, which prevents such
   * (usually) meaningless operations.
   *
   * \note DQCsim starts counting qubits from index 1. Index 0 is reserved for
   * signaling errors in the C API.
   */
  using QubitIndex = raw::dqcs_qubit_t;

  /**
   * C++-styled type name for `raw::dqcs_cycle_t`.
   *
   * This integer type is used to represent simulation cycles.
   */
  using Cycle = raw::dqcs_cycle_t;

  /**
   * Checks a `dqcs_return_t` return value; if failure, throws a runtime error
   * with DQCsim's error message.
   *
   * \param code The raw function return code.
   * \throws std::runtime_error When the return code indicated failure.
   */
  inline void check(raw::dqcs_return_t code) {
    if (code == raw::dqcs_return_t::DQCS_FAILURE) {
      throw std::runtime_error(raw::dqcs_error_get());
    }
  }

  /**
   * Checks a `dqcs_bool_return_t` return value; if failure, throws a runtime
   * error with DQCsim's error message.
   *
   * \param code The raw function return code.
   * \returns The wrapped boolean.
   * \throws std::runtime_error When the return code indicated failure.
   */
  inline bool check(raw::dqcs_bool_return_t code) {
    if (code == raw::dqcs_bool_return_t::DQCS_BOOL_FAILURE) {
      throw std::runtime_error(raw::dqcs_error_get());
    } else if (code == raw::dqcs_bool_return_t::DQCS_TRUE) {
      return true;
    } else {
      return false;
    }
  }

  /**
   * Checks a `dqcs_handle_t` or `dqcs_qubit_t` return value; if failure,
   * throws a runtime error with DQCsim's error message.
   *
   * \param handle The raw function return code.
   * \returns The wrapped handle or qubit index.
   * \throws std::runtime_error When the return code indicated failure.
   */
  inline unsigned long long check(unsigned long long handle) {
    if (handle == 0) {
      throw std::runtime_error(raw::dqcs_error_get());
    }
    return handle;
  }

  /**
   * Checks a `dqcs_cycle_t` return value; if failure, throws a runtime error
   * with DQCsim's error message.
   *
   * \param cycle The raw function return code.
   * \returns The wrapped cycle count.
   * \throws std::runtime_error When the return code indicated failure.
   */
  inline Cycle check(Cycle cycle) {
    if (cycle == -1) {
      throw std::runtime_error(raw::dqcs_error_get());
    }
    return cycle;
  }

  /**
   * Checks a size return value; if failure, throws a runtime error
   * with DQCsim's error message.
   *
   * \param size The raw function return code.
   * \returns The wrapped size.
   * \throws std::runtime_error When the return code indicated failure.
   */
  inline size_t check(ssize_t size) {
    if (size < 0) {
      throw std::runtime_error(raw::dqcs_error_get());
    }
    return static_cast<size_t>(size);
  }

  /**
   * Checks a `double` return value from `dqcs_pcfg_*_timeout_get()`; if
   * failure, throws a runtime error with DQCsim's error message.
   *
   * \param value The raw function return code.
   * \returns The wrapped double.
   * \throws std::runtime_error When the return code indicated failure.
   */
  inline double check(double value) {
    if (value < 0.0) {
      throw std::runtime_error(raw::dqcs_error_get());
    }
    return value;
  }

  /**
   * Represents the type of a raw handle.
   *
   * This wraps `raw::handle_type_t`, not including the `invalid` option
   * (since we use exceptions to communicate failure).
   */
  enum class HandleType {

    /**
     * Indicates that a handle is an `ArbData`.
     */
    ArbData = 100,

    /**
     * Indicates that a handle is an `ArbCmd`.
     */
    ArbCmd = 101,

    /**
     * Indicates that a handle is an `ArbCmdQueue`.
     */
    ArbCmdQueue = 102,

    /**
     * Indicates that a handle is a `QubitSet`.
     */
    QubitSet = 103,

    /**
     * Indicates that a handle is a `Gate`.
     */
    Gate = 104,

    /**
     * Indicates that a handle is a `Measurement`.
     */
    Measurement = 105,

    /**
     * Indicates that a handle is a `MeasurementSet`.
     */
    MeasurementSet = 106,

    /**
     * Indicates that a handle is a `Matrix`.
     */
    Matrix = 107,

    /**
     * Indicates that a handle is a `GateMap`.
     */
    GateMap = 108,

    /**
     * Indicates that a handle is a `PluginProcessConfiguration` for a frontend
     * plugin.
     */
    FrontendProcessConfig = 200,

    /**
     * Indicates that a handle is a `PluginProcessConfiguration` for an
     * operator plugin.
     */
    OperatorProcessConfig = 201,

    /**
     * Indicates that a handle is a `PluginProcessConfiguration` for a backend
     * plugin.
     */
    BackendProcessConfig = 203,

    /**
     * Indicates that a handle is a `PluginThreadConfiguration` for a frontend
     * plugin.
     */
    FrontendThreadConfig = 204,

    /**
     * Indicates that a handle is a `PluginThreadConfiguration` for an operator
     * plugin.
     */
    OperatorThreadConfig = 205,

    /**
     * Indicates that a handle is a `PluginThreadConfiguration` for a backend
     * plugin.
     */
    BackendThreadConfig = 206,

    /**
     * Indicates that a handle is a `SimulationConfiguration`.
     */
    SimulationConfig = 207,

    /**
     * Indicates that a handle is a `Simulation`.
     */
    Simulation = 208,

    /**
     * Indicates that a handle is a frontend `Plugin`.
     */
    FrontendDefinition = 300,

    /**
     * Indicates that a handle is an operator `Plugin`.
     */
    OperatorDefinition = 301,

    /**
     * Indicates that a handle is a backend `Plugin`.
     */
    BackendDefinition = 302,

    /**
     * Indicates that a handle is a `PluginJoinHandle`.
     */
    PluginJoinHandle = 303
  };

  /**
   * Converts a `HandleType` to its raw C enum.
   *
   * \param type The C++ handle type to convert.
   * \returns The raw handle type.
   */
  inline raw::dqcs_handle_type_t to_raw(HandleType type) noexcept {
    switch (type) {
      case HandleType::ArbData:               return raw::dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA;
      case HandleType::ArbCmd:                return raw::dqcs_handle_type_t::DQCS_HTYPE_ARB_CMD;
      case HandleType::ArbCmdQueue:           return raw::dqcs_handle_type_t::DQCS_HTYPE_ARB_CMD_QUEUE;
      case HandleType::QubitSet:              return raw::dqcs_handle_type_t::DQCS_HTYPE_QUBIT_SET;
      case HandleType::Gate:                  return raw::dqcs_handle_type_t::DQCS_HTYPE_GATE;
      case HandleType::Measurement:           return raw::dqcs_handle_type_t::DQCS_HTYPE_MEAS;
      case HandleType::MeasurementSet:        return raw::dqcs_handle_type_t::DQCS_HTYPE_MEAS_SET;
      case HandleType::Matrix:                return raw::dqcs_handle_type_t::DQCS_HTYPE_MATRIX;
      case HandleType::GateMap:               return raw::dqcs_handle_type_t::DQCS_HTYPE_GATE_MAP;
      case HandleType::FrontendProcessConfig: return raw::dqcs_handle_type_t::DQCS_HTYPE_FRONT_PROCESS_CONFIG;
      case HandleType::OperatorProcessConfig: return raw::dqcs_handle_type_t::DQCS_HTYPE_OPER_PROCESS_CONFIG;
      case HandleType::BackendProcessConfig:  return raw::dqcs_handle_type_t::DQCS_HTYPE_BACK_PROCESS_CONFIG;
      case HandleType::FrontendThreadConfig:  return raw::dqcs_handle_type_t::DQCS_HTYPE_FRONT_THREAD_CONFIG;
      case HandleType::OperatorThreadConfig:  return raw::dqcs_handle_type_t::DQCS_HTYPE_OPER_THREAD_CONFIG;
      case HandleType::BackendThreadConfig:   return raw::dqcs_handle_type_t::DQCS_HTYPE_BACK_THREAD_CONFIG;
      case HandleType::SimulationConfig:      return raw::dqcs_handle_type_t::DQCS_HTYPE_SIM_CONFIG;
      case HandleType::Simulation:            return raw::dqcs_handle_type_t::DQCS_HTYPE_SIM;
      case HandleType::FrontendDefinition:    return raw::dqcs_handle_type_t::DQCS_HTYPE_FRONT_DEF;
      case HandleType::OperatorDefinition:    return raw::dqcs_handle_type_t::DQCS_HTYPE_OPER_DEF;
      case HandleType::BackendDefinition:     return raw::dqcs_handle_type_t::DQCS_HTYPE_BACK_DEF;
      case HandleType::PluginJoinHandle:      return raw::dqcs_handle_type_t::DQCS_HTYPE_PLUGIN_JOIN;
    }
    std::cerr << "unknown handle type" << std::endl;
    std::terminate();
  }

  /**
   * Checks a `dqcs_handle_type_t` return value and converts it to its C++
   * enum representation; if failure, throws a runtime error with DQCsim's
   * error message.
   *
   * \param type The raw function return code.
   * \returns The wrapped handle type.
   * \throws std::runtime_error When the return code indicated failure.
   */
  inline HandleType check(raw::dqcs_handle_type_t type) {
    switch (type) {
      case raw::dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA:             return HandleType::ArbData;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_ARB_CMD:              return HandleType::ArbCmd;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_ARB_CMD_QUEUE:        return HandleType::ArbCmdQueue;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_QUBIT_SET:            return HandleType::QubitSet;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_GATE:                 return HandleType::Gate;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_MEAS:                 return HandleType::Measurement;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_MEAS_SET:             return HandleType::MeasurementSet;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_MATRIX:               return HandleType::Matrix;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_GATE_MAP:             return HandleType::GateMap;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_FRONT_PROCESS_CONFIG: return HandleType::FrontendProcessConfig;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_OPER_PROCESS_CONFIG:  return HandleType::OperatorProcessConfig;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_BACK_PROCESS_CONFIG:  return HandleType::BackendProcessConfig;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_FRONT_THREAD_CONFIG:  return HandleType::FrontendThreadConfig;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_OPER_THREAD_CONFIG:   return HandleType::OperatorThreadConfig;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_BACK_THREAD_CONFIG:   return HandleType::BackendThreadConfig;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_SIM_CONFIG:           return HandleType::SimulationConfig;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_SIM:                  return HandleType::Simulation;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_FRONT_DEF:            return HandleType::FrontendDefinition;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_OPER_DEF:             return HandleType::OperatorDefinition;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_BACK_DEF:             return HandleType::BackendDefinition;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_PLUGIN_JOIN:          return HandleType::PluginJoinHandle;
      case raw::dqcs_handle_type_t::DQCS_HTYPE_INVALID:              throw std::runtime_error(raw::dqcs_error_get());
    }
    throw std::invalid_argument("unknown handle type");
  }

  /**
   * Represents the loglevel of a message, a loglevel filter level, or one
   * of the possible actions to take when a message is received from a plugin
   * through `stdout` or `stderr`.
   *
   * This wraps `raw::dqcs_loglevel_t`, not including the `invalid` option
   * (since we use exceptions to communicate failure).
   */
  enum class Loglevel {

    /**
     * In the context of a filter, turns logging off.
     */
    Off = 0,

    /**
     * This loglevel is to be used for reporting a fatal error, resulting from
     * the owner of the logger getting into an illegal state from which it
     * cannot recover. Such problems are also reported to the API caller if
     * applicable.
     */
    Fatal = 1,

    /**
     * This loglevel is to be used for reporting or propagating a non-fatal
     * error caused by the API caller doing something wrong. Such problems are
     * also reported to the API caller if applicable.
     */
    Error = 2,

    /**
     * This loglevel is to be used for reporting that a called API/function is
     * telling us we did something wrong (that we weren't expecting), but we
     * can recover. For instance, for a failed connection attempt to something
     * that really should not be failing, we can still retry (and eventually
     * report critical or error if a retry counter overflows). Since we're
     * still trying to rectify things at this point, such problems are NOT
     * reported to the API/function caller via Result::Err.
     */
    Warn = 3,

    /**
     * This loglevel is to be used for reporting information specifically
     * requested by the user/API caller, such as the result of an API function
     * requested through the command line, or an explicitly captured
     * stdout/stderr stream.
     */
    Note = 4,

    /**
     * This loglevel is to be used for reporting information NOT specifically
     * requested by the user/API caller, such as a plugin starting up or
     * shutting down.
     */
    Info = 5,

    /**
     * This loglevel is to be used for reporting debugging information useful
     * for debugging the user of the API provided by the logged instance.
     */
    Debug = 6,

    /**
     * This loglevel is to be used for reporting debugging information useful
     * for debugging the internals of the logged instance. Such messages would
     * normally only be generated by debug builds, to prevent them from
     * impacting performance under normal circumstances.
     */
    Trace = 7,

    /**
     * This is intended to be used when configuring the stdout/stderr capture
     * mode for a plugin process. Selecting it will prevent the stream from
     * being captured; it will just be the same stream as DQCsim's own
     * stdout/stderr. When used as the loglevel for a message, the message
     * itself is sent to stderr instead of passing into DQCsim's log system.
     * Using this for loglevel filters leads to undefined behavior.
     */
    Pass = 8

  };

  /**
   * Converts a `Loglevel` to its raw C enum.
   *
   * \param loglevel The C++ loglevel to convert.
   * \returns The raw loglevel.
   */
  inline raw::dqcs_loglevel_t to_raw(Loglevel loglevel) noexcept {
    switch (loglevel) {
      case Loglevel::Off:   return raw::dqcs_loglevel_t::DQCS_LOG_OFF;
      case Loglevel::Fatal: return raw::dqcs_loglevel_t::DQCS_LOG_FATAL;
      case Loglevel::Error: return raw::dqcs_loglevel_t::DQCS_LOG_ERROR;
      case Loglevel::Warn:  return raw::dqcs_loglevel_t::DQCS_LOG_WARN;
      case Loglevel::Note:  return raw::dqcs_loglevel_t::DQCS_LOG_NOTE;
      case Loglevel::Info:  return raw::dqcs_loglevel_t::DQCS_LOG_INFO;
      case Loglevel::Debug: return raw::dqcs_loglevel_t::DQCS_LOG_DEBUG;
      case Loglevel::Trace: return raw::dqcs_loglevel_t::DQCS_LOG_TRACE;
      case Loglevel::Pass:  return raw::dqcs_loglevel_t::DQCS_LOG_PASS;
    }
    std::cerr << "unknown loglevel" << std::endl;
    std::terminate();
  }

  /**
   * Checks a `dqcs_loglevel_t` return value and converts it to its C++
   * enum representation; if failure, throws a runtime error with DQCsim's
   * error message.
   *
   * \param loglevel The raw function return code.
   * \returns The wrapped loglevel.
   * \throws std::runtime_error When the return code indicated failure.
   */
  inline Loglevel check(raw::dqcs_loglevel_t loglevel) {
    switch (loglevel) {
      case raw::dqcs_loglevel_t::DQCS_LOG_OFF:      return Loglevel::Off;
      case raw::dqcs_loglevel_t::DQCS_LOG_FATAL:    return Loglevel::Fatal;
      case raw::dqcs_loglevel_t::DQCS_LOG_ERROR:    return Loglevel::Error;
      case raw::dqcs_loglevel_t::DQCS_LOG_WARN:     return Loglevel::Warn;
      case raw::dqcs_loglevel_t::DQCS_LOG_NOTE:     return Loglevel::Note;
      case raw::dqcs_loglevel_t::DQCS_LOG_INFO:     return Loglevel::Info;
      case raw::dqcs_loglevel_t::DQCS_LOG_DEBUG:    return Loglevel::Debug;
      case raw::dqcs_loglevel_t::DQCS_LOG_TRACE:    return Loglevel::Trace;
      case raw::dqcs_loglevel_t::DQCS_LOG_PASS:     return Loglevel::Pass;
      case raw::dqcs_loglevel_t::DQCS_LOG_INVALID:  throw std::runtime_error(raw::dqcs_error_get());
    }
    throw std::invalid_argument("unknown loglevel");
  }

  /**
   * Represents the result of a qubit measurement.
   *
   * This wraps `raw::dqcs_measurement_t`, not including the `invalid`
   * option (since we use exceptions to communicate failure).
   */
  enum class MeasurementValue {

    /**
     * Qubit measurement returned zero.
     */
    Zero = 0,

    /**
     * Qubit measurement returned one.
     */
    One = 1,

    /**
     * This value can be used by backends to indicate that the qubit state
     * was collapsed as part of the measurement process, but the value is
     * unknown for some reason.
     */
    Undefined = 2

  };

  /**
   * Converts a `MeasurementValue` to its raw C enum.
   *
   * \param measurement The C++ measurement value to convert.
   * \returns The raw measurement value.
   */
  inline raw::dqcs_measurement_t to_raw(MeasurementValue measurement) noexcept {
    switch (measurement) {
      case MeasurementValue::Zero:      return raw::dqcs_measurement_t::DQCS_MEAS_ZERO;
      case MeasurementValue::One:       return raw::dqcs_measurement_t::DQCS_MEAS_ONE;
      case MeasurementValue::Undefined: return raw::dqcs_measurement_t::DQCS_MEAS_UNDEFINED;
    }
    std::cerr << "unknown measurement value" << std::endl;
    std::terminate();
  }

  /**
   * Checks a `dqcs_measurement_t` return value and converts it to its C++
   * enum representation; if failure, throws a runtime error with DQCsim's
   * error message.
   *
   * \param measurement The raw function return code.
   * \returns The wrapped measurement value.
   * \throws std::runtime_error When the return code indicated failure.
   */
  inline MeasurementValue check(raw::dqcs_measurement_t measurement) {
    switch (measurement) {
      case raw::dqcs_measurement_t::DQCS_MEAS_ZERO:      return MeasurementValue::Zero;
      case raw::dqcs_measurement_t::DQCS_MEAS_ONE:       return MeasurementValue::One;
      case raw::dqcs_measurement_t::DQCS_MEAS_UNDEFINED: return MeasurementValue::Undefined;
      case raw::dqcs_measurement_t::DQCS_MEAS_INVALID:   throw std::runtime_error(raw::dqcs_error_get());
    }
    throw std::invalid_argument("unknown measurement value");
  }

  /**
   * Represents the possible options for dealing with paths when writing a
   * reproduction file.
   *
   * This wraps `raw::dqcs_path_style_t`, not including the `invalid` option
   * (since we use exceptions to communicate failure).
   */
  enum class PathStyle {

    /**
     * Specifies that paths should be saved the same way they were specified
     * on the command line.
     */
    Keep = 0,

    /**
     * Specifies that all paths should be saved relative to DQCsim's working
     * directory.
     */
    Relative = 1,

    /**
     * Specifies that all paths should be saved canonically, i.e. relative to
     * the root directory.
     */
    Absolute = 2

  };

  /**
   * Converts a `PathStyle` to its raw C enum.
   *
   * \param style The C++ path style to convert.
   * \returns The raw path style.
   */
  inline raw::dqcs_path_style_t to_raw(PathStyle style) noexcept {
    switch (style) {
      case PathStyle::Keep:     return raw::dqcs_path_style_t::DQCS_PATH_STYLE_KEEP;
      case PathStyle::Relative: return raw::dqcs_path_style_t::DQCS_PATH_STYLE_RELATIVE;
      case PathStyle::Absolute: return raw::dqcs_path_style_t::DQCS_PATH_STYLE_ABSOLUTE;
    }
    std::cerr << "unknown path style" << std::endl;
    std::terminate();
  }

  /**
   * Checks a `dqcs_path_style_t` return value and converts it to its C++
   * enum representation; if failure, throws a runtime error with DQCsim's
   * error message.
   *
   * \param style The raw function return code.
   * \returns The wrapped path style.
   * \throws std::runtime_error When the return code indicated failure.
   */
  inline PathStyle check(raw::dqcs_path_style_t style) {
    switch (style) {
      case raw::dqcs_path_style_t::DQCS_PATH_STYLE_KEEP:      return PathStyle::Keep;
      case raw::dqcs_path_style_t::DQCS_PATH_STYLE_RELATIVE:  return PathStyle::Relative;
      case raw::dqcs_path_style_t::DQCS_PATH_STYLE_ABSOLUTE:  return PathStyle::Absolute;
      case raw::dqcs_path_style_t::DQCS_PATH_STYLE_INVALID:   throw std::runtime_error(raw::dqcs_error_get());
    }
    throw std::invalid_argument("unknown path style");
  }

  /**
   * Enumeration of the three types of plugins.
   *
   * This wraps `raw::dqcs_plugin_type_t`, not including the `invalid` option
   * (since we use exceptions to communicate failure).
   */
  enum class PluginType {

    /**
     * Frontend plugin.
     */
    Frontend = 0,

    /**
     * Operator plugin.
     */
    Operator = 1,

    /**
     * Backend plugin.
     */
    Backend = 2

  };

  /**
   * Converts a `PluginType` to its raw C enum.
   *
   * \param type The C++ plugin type to convert.
   * \returns The raw plugin type.
   */
  inline raw::dqcs_plugin_type_t to_raw(PluginType type) noexcept {
    switch (type) {
      case PluginType::Frontend:  return raw::dqcs_plugin_type_t::DQCS_PTYPE_FRONT;
      case PluginType::Operator:  return raw::dqcs_plugin_type_t::DQCS_PTYPE_OPER;
      case PluginType::Backend:   return raw::dqcs_plugin_type_t::DQCS_PTYPE_BACK;
    }
    std::cerr << "unknown plugin type" << std::endl;
    std::terminate();
  }

  /**
   * Checks a `dqcs_plugin_type_t` return value and converts it to its C++
   * enum representation; if failure, throws a runtime error with DQCsim's
   * error message.
   *
   * \param type The raw function return code.
   * \returns The wrapped plugin type.
   * \throws std::runtime_error When the return code indicated failure.
   */
  inline PluginType check(raw::dqcs_plugin_type_t type) {
    switch (type) {
      case raw::dqcs_plugin_type_t::DQCS_PTYPE_FRONT:   return PluginType::Frontend;
      case raw::dqcs_plugin_type_t::DQCS_PTYPE_OPER:    return PluginType::Operator;
      case raw::dqcs_plugin_type_t::DQCS_PTYPE_BACK:    return PluginType::Backend;
      case raw::dqcs_plugin_type_t::DQCS_PTYPE_INVALID: throw std::runtime_error(raw::dqcs_error_get());
    }
    throw std::invalid_argument("unknown plugin type");
  }

  /**
   * Checks a pointer return value; if failure, throws a runtime error with
   * DQCsim's error message.
   *
   * \param pointer The raw function return code.
   * \returns The non-null pointer.
   * \throws std::runtime_error When the return code indicated failure.
   */
  template <typename T>
  inline T *check(T *pointer) {
    if (pointer == nullptr) {
      throw std::runtime_error(raw::dqcs_error_get());
    }
    return pointer;
  }

  /**
   * Shim around the `dqcs_log_format` C API using `std::string` for the
   * strings.
   *
   * \note The `printf` format arguments are passed directly into a C-only
   * `printf`-like function. Therefore, you must use the `c_str()` function if
   * you want to format `std::string` variables.
   *
   * If logging through DQCsim's conventional channels fails, this function
   * simply outputs to stderr directly. Therefore, it cannot fail.
   *
   * To avoid having to fill out `module`, `file`, and `line_nr` manually, you
   * can use the `DQCSIM_LOG` macro (or its loglevel-specific friends). If you
   * define `DQCSIM_SHORT_LOGGING_MACROS` before including `<dqcsim>`, you can
   * also use the `LOG` shorthand (or its loglevel-specific friends).
   *
   * \param level The severity level of the message.
   * \param module The "module" sending the message. This is unused by the C++
   * macros.
   * \param file The source file for the code sending the message (you can use
   * `__FILE__` for this).
   * \param line_nr The line number within the source file of the code sending
   * the message (you can use `__LINE__` for this).
   * \param format The `printf`-style format string for the log message.
   * \param args The arguments for the `printf`-style format string.
   */
  template<typename... Args>
  inline void log(
    wrap::Loglevel level,
    const std::string &module,
    const std::string &file,
    unsigned int line_nr,
    const std::string &format,
    Args... args
  ) noexcept {
    raw::dqcs_log_format(
      to_raw(level),
      module.c_str(),
      file.c_str(),
      line_nr,
      format.c_str(),
      args...
    );
  }

  /**
   * Shim around the `dqcs_log_raw` C API using `std::string` for the strings.
   *
   * To avoid having to fill out `module`, `file`, and `line_nr` manually, you
   * can use the `DQCSIM_LOG` macro (or its loglevel-specific friends). If you
   * define `DQCSIM_SHORT_LOGGING_MACROS` before including `<dqcsim>`, you can
   * also use the `LOG` shorthand (or its loglevel-specific friends).
   *
   * \param level The severity level of the message.
   * \param module The "module" sending the message. This is unused by the C++
   * macros.
   * \param file The source file for the code sending the message (you can use
   * `__FILE__` for this).
   * \param line_nr The line number within the source file of the code sending
   * the message (you can use `__LINE__` for this).
   * \param message The log message.
   * \returns Whether logging succeeded.
   */
  inline bool log_raw(
    wrap::Loglevel level,
    const std::string &module,
    const std::string &file,
    unsigned int line_nr,
    const std::string &message
  ) noexcept {
    return raw::dqcs_log_raw(
      to_raw(level),
      module.c_str(),
      file.c_str(),
      line_nr,
      message.c_str()
    ) == raw::dqcs_return_t::DQCS_SUCCESS;
  }

  /**
   * Convenience macro for calling `log()` with automatically determined filename
   * and line number, but a dynamic loglevel (first argument).
   *
   * \param level The severity level of the message.
   * \param fmt The `printf`-style format string for the log message.
   * \param ... The arguments for the `printf`-style format string.
   */
  #define DQCSIM_LOG(level, fmt, ...)             \
    ::dqcsim::wrap::log(                          \
      level, "C++", __FILE__, __LINE__,           \
      fmt, ##__VA_ARGS__)

  /**
   * Convenience macro for calling `log()` with trace loglevel and automatically
   * determined filename and line number.
   *
   * \param fmt The `printf`-style format string for the log message.
   * \param ... The arguments for the `printf`-style format string.
   */
  #define DQCSIM_TRACE(fmt, ...) DQCSIM_LOG(::dqcsim::wrap::Loglevel::Trace, fmt, ##__VA_ARGS__)

  /**
   * Convenience macro for calling `log()` with debug loglevel and automatically
   * determined filename and line number.
   *
   * \param fmt The `printf`-style format string for the log message.
   * \param ... The arguments for the `printf`-style format string.
   */
  #define DQCSIM_DEBUG(fmt, ...) DQCSIM_LOG(::dqcsim::wrap::Loglevel::Debug, fmt, ##__VA_ARGS__)

  /**
   * Convenience macro for calling `log()` with info loglevel and automatically
   * determined filename and line number.
   *
   * \param fmt The `printf`-style format string for the log message.
   * \param ... The arguments for the `printf`-style format string.
   */
  #define DQCSIM_INFO(fmt, ...) DQCSIM_LOG(::dqcsim::wrap::Loglevel::Info, fmt, ##__VA_ARGS__)

  /**
   * Convenience macro for calling `log()` with note loglevel and automatically
   * determined filename and line number.
   *
   * \param fmt The `printf`-style format string for the log message.
   * \param ... The arguments for the `printf`-style format string.
   */
  #define DQCSIM_NOTE(fmt, ...) DQCSIM_LOG(::dqcsim::wrap::Loglevel::Note, fmt, ##__VA_ARGS__)

  /**
   * Convenience macro for calling `log()` with warn loglevel and automatically
   * determined filename and line number.
   *
   * \param fmt The `printf`-style format string for the log message.
   * \param ... The arguments for the `printf`-style format string.
   */
  #define DQCSIM_WARN(fmt, ...) DQCSIM_LOG(::dqcsim::wrap::Loglevel::Warn, fmt, ##__VA_ARGS__)

  /**
   * Convenience macro for calling `log()` with warn loglevel and automatically
   * determined filename and line number.
   *
   * \param fmt The `printf`-style format string for the log message.
   * \param ... The arguments for the `printf`-style format string.
   */
  #define DQCSIM_WARNING(fmt, ...) DQCSIM_LOG(::dqcsim::wrap::Loglevel::Warn, fmt, ##__VA_ARGS__)

  /**
   * Convenience macro for calling `log()` with error loglevel and automatically
   * determined filename and line number.
   *
   * \param fmt The `printf`-style format string for the log message.
   * \param ... The arguments for the `printf`-style format string.
   */
  #define DQCSIM_ERROR(fmt, ...) DQCSIM_LOG(::dqcsim::wrap::Loglevel::Error, fmt, ##__VA_ARGS__)

  /**
   * Convenience macro for calling `log()` with fatal loglevel and automatically
   * determined filename and line number.
   *
   * \param fmt The `printf`-style format string for the log message.
   * \param ... The arguments for the `printf`-style format string.
   */
  #define DQCSIM_FATAL(fmt, ...) DQCSIM_LOG(::dqcsim::wrap::Loglevel::Fatal, fmt, ##__VA_ARGS__)

  /**
   * Base class for wrapping any handle.
   *
   * This class ensures that the wrapped handle is freed when it is destroyed,
   * preventing memory leaks. You can also free the handle before destruction,
   * which allows any errors reported by DQCsim to be caught. You can take back
   * ownership of the handle using `take_handle`, preventing it from being
   * freed at destruction.
   *
   * Only one `Handle` should wrap a single handle at a time. Since handles
   * cannot typically be cloned, this class is not copy constructible or
   * assignable. If you need it to be, consider wrapping the `Handle` in a
   * `std::shared_ptr`.
   *
   * \warning `Handle` objects can not and should never be passed between
   * threads. The wrapped C API handles are thread local can cannot be moved
   * between threads, therefore this wrapper can't either. Trying to do so
   * anyway is undefined behavior and will absolutely not work! If you need to
   * move data between threads, copy the data represented by the handle into
   * C++ variables, pass those over, and reconstruct the handle from them in
   * the destination thread.
   */
  class Handle {
  protected:

    /**
     * The wrapped handle.
     */
    HandleIndex handle;

  public:

    /**
     * Constructs an empty wrapper.
     *
     * \note The only way to use a wrapper constructed this way is to move a
     * handle into it using the move assignment operator.
     */
    Handle() noexcept : handle(0) {
    }

    /**
     * Wraps the given raw handle.
     *
     * \note This class will take ownership of the handle, i.e. it is in charge
     * of freeing it.
     *
     * \param handle The raw handle to wrap.
     */
    Handle(HandleIndex handle) noexcept : handle(handle) {
    }

    /**
     * Delete the handle and its wrapper.
     */
    virtual ~Handle() noexcept {
      if (handle) {
        // NOTE: no check; ignore errors (because destructors should be
        // noexcept).
        raw::dqcs_handle_delete(handle);
      }
    }

    /**
     * Explicitly delete the handle, allowing errors to be caught.
     *
     * \note The wrapper no longer owns a handle after this call. That means
     * `is_valid` will return `false` and all other methods will likely fail.
     *
     * \throws std::runtime_error When deletion of the handle fails for some
     * reason.
     */
    void free() {
      check(raw::dqcs_handle_delete(handle));
      handle = 0;
    }

    /**
     * Returns whether this wrapper (still) contains a valid handle.
     *
     * \returns Whether this wrapper (still) contains a valid handle.
     */
    bool is_valid() const noexcept {
      try {
        return raw::dqcs_handle_type(handle) != raw::dqcs_handle_type_t::DQCS_HTYPE_INVALID;
      } catch (std::runtime_error) {
        return false;
      }
    }

    /**
     * Returns the raw handle without relinquishing ownership.
     *
     * \returns The wrapped raw handle, or 0 if no handle was attached.
     */
    HandleIndex get_handle() const noexcept {
      return handle;
    }

    /**
     * Returns the raw handle and relinquishes ownership.
     *
     * \note The wrapper no longer owns a handle after this call. That means
     * `is_valid` will return `false` and all other methods will likely fail.
     *
     * \returns The wrapped raw handle, or 0 if no handle was attached.
     */
    HandleIndex take_handle() noexcept {
      HandleIndex h = handle;
      handle = 0;
      return h;
    }

    // Handles cannot usually be copied, so delete the copy constructor and
    // copy assignment operator.
    Handle(const Handle&) = delete;
    void operator=(const Handle&) = delete;

    /**
     * Move constructor; simply moves ownership of the handle from the source
     * object to the constructed object.
     *
     * \note The source wrapper no longer owns a handle after this call. That
     * means that its `is_valid` will return `false` and all other methods will
     * likely fail. Note that using an object after moving it is explicitly
     * undefined behavior in the C++ specification, so you shouldn't be using
     * it anymore anyway.
     *
     * \param src The handle wrapper to move from.
     */
    Handle(Handle &&src) : handle(src.handle) {
      src.handle = 0;
    }

    /**
     * Move constructor; simply moves ownership of the handle from the source
     * object to the assignment target.
     *
     * \note If the assignment target wrapper already contained a handle, it
     * is implicitly freed.
     *
     * \note The source wrapper no longer owns a handle after this call. That
     * means that its `is_valid` will return `false` and all other methods will
     * likely fail. Note that using an object after moving it is explicitly
     * undefined behavior in the C++ specification, so you shouldn't be using
     * it anymore anyway.
     *
     * \param src The handle wrapper to move from.
     * \returns A reference to the destination handle.
     * \throws std::runtime_error When the destination already wrapped a
     * handle, and freeing that handle failed.
     */
    Handle &operator=(Handle &&src) {
      if (handle) {
        free();
      }
      handle = src.handle;
      src.handle = 0;
      return *this;
    }

    /**
     * Returns a string containing a debug dump of the handle.
     *
     * The string uses newlines and indentation to improve readability. It does
     * not end in a newline.
     *
     * \warning Debug dumps are not guaranteed to be the same from DQCsim
     * version to version. They should *only* be used for debugging.
     *
     * \returns A debug dump of the current handle.
     * \throws std::runtime_error When the currently wrapped handle is
     * invalid.
     */
    std::string dump() const {
      char *dump_c = check(raw::dqcs_handle_dump(handle));
      std::string dump(dump_c);
      std::free(dump_c);
      return dump;
    }

    /**
     * Write the debug dump string of the handle to the given output stream.
     *
     * Newlines and indentation are used to improve readability. However, there
     * is implicit trailing `std::endl`.
     *
     * \param out The output stream to write to.
     * \param handle The handle to dump.
     * \returns The output stream object.
     * \throws std::runtime_error When the given handle is invalid.
     */
    friend std::ostream& operator<<(std::ostream &out, const Handle &handle) {
      out << handle.dump();
      return out;
    }

    /**
     * Returns the type of this handle.
     *
     * \returns The handle type for the currently wrapped handle.
     * \throws std::runtime_error When the current handle is invalid.
     */
    HandleType type() const {
      return check(raw::dqcs_handle_type(handle));
    }

  };

  /**
   * Class wrapper for handles that support the `arb` interface.
   *
   * You normally wouldn't instantiate this directly (see `ArbData`).
   */
  class Arb : public Handle {
  public:

    /**
     * Wraps the given `arb` handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    Arb(HandleIndex handle) noexcept : Handle(handle) {
    }

    /**
     * Returns the current arbitrary JSON data as a serialized JSON string.
     *
     * \returns A string representation of the current JSON object.
     * \throws std::runtime_error When the current handle is invalid.
     */
    std::string get_arb_json_string() const {
      char *json_c = check(raw::dqcs_arb_json_get(handle));
      std::string json(json_c);
      std::free(json_c);
      return json;
    }

    /**
     * Sets the arbitrary JSON data to the given serialized JSON string.
     *
     * \note DQCsim internally stores the JSON object in CBOR format.
     * Therefore, if after calling this you subsequently call
     * `get_arb_json_string()` you may get a different string representation
     * for the same JSON object.
     *
     * \param json A string representation of a JSON dictionary object.
     * \throws std::runtime_error When the string representation is invalid,
     * or when the current handle is invalid.
     */
    void set_arb_json_string(const std::string &json) {
      check(raw::dqcs_arb_json_set(handle, json.c_str()));
    }

    /**
     * Returns the current arbitrary JSON data as a serialized CBOR string.
     *
     * \note A single JSON object may be represented with CBOR in many
     * different ways, similar to how it may have different string
     * representations (spacing, dictionary order, etc.). Therefore, never
     * use a simple binary comparison on the CBOR object to check if two
     * JSON objects are equivalent!
     *
     * \returns A CBOR string representing the current JSON object.
     * \throws std::runtime_error When the current handle is invalid.
     */
    std::string get_arb_cbor_string() const {
      size_t size = check(raw::dqcs_arb_cbor_get(handle, nullptr, 0));
      std::string cbor;
      cbor.resize(size);
      check(raw::dqcs_arb_cbor_get(handle, &cbor.front(), size));
      return cbor;
    }

    /**
     * Sets the arbitrary JSON data to the given serialized CBOR string.
     *
     * \param cbor A JSON object represented as a CBOR binary string.
     * \throws std::runtime_error When the CBOR string is invalid, or when the
     * current handle is invalid.
     */
    void set_arb_cbor_string(const std::string &cbor) {
      check(raw::dqcs_arb_cbor_set(handle, cbor.data(), cbor.size()));
    }

    /**
     * Returns the current arbitrary JSON data as a JSON object from
     * `nlohmann::json`.
     *
     * Since that is a header-only library that isn't usually installed
     * system-wide and be using a specific version in your project already,
     * you need to specify the `nlohmann::json` type as a generic to this
     * function.
     *
     * \warning This function returns a *copy* of the JSON data embedded in the
     * `ArbData`. Therefore, modifying the returned JSON object does *not*
     * modify the original `ArbData`. To modify, you need to pass the modified
     * JSON object to `set_arb_json()`.
     *
     * \returns A copy of the embedded JSON object, in the form of a C++ JSON
     * object wrapper.
     * \throws std::runtime_error When the current handle is invalid.
     */
    template <class JSON>
    JSON get_arb_json() const {
      size_t size = check(raw::dqcs_arb_cbor_get(handle, nullptr, 0));
      std::vector<uint8_t> cbor;
      cbor.resize(size);
      check(raw::dqcs_arb_cbor_get(handle, &cbor.front(), size));
      return JSON::from_cbor(cbor);
    }

    /**
     * Sets the arbitrary JSON data to the given JSON object from
     * `nlohmann::json`.
     *
     * Since that is a header-only library that isn't usually installed
     * system-wide and be using a specific version in your project already,
     * you need to specify the `nlohmann::json` type as a generic to this
     * function.
     *
     * \param json The C++ JSON object representation of the object to set.
     * \throws std::runtime_error When the current handle is invalid.
     */
    template <class JSON>
    void set_arb_json(const JSON &json) {
      std::vector<uint8_t> cbor = JSON::to_cbor(json);
      check(raw::dqcs_arb_cbor_set(handle, cbor.data(), cbor.size()));
    }

    /**
     * Returns the arbitrary argument at the given index as a (binary) string.
     *
     * Negative indices are relative to the back of the list, as in Python.
     *
     * \param index The index of the argument to retrieve.
     * \returns The (binary) string representation of the argument.
     * \throws std::runtime_error When the current handle is invalid or the
     * argument index is out of range.
     */
    std::string get_arb_arg_string(ssize_t index) const {
      size_t size = check(raw::dqcs_arb_get_size(handle, index));
      std::string data;
      data.resize(size);
      check(raw::dqcs_arb_get_raw(handle, index, &data.front(), size));
      return data;
    }

    /**
     * Returns the arbitrary argument at the given index as the given type.
     * Negative indices are relative to the back of the list, as in Python.
     *
     * \warning Type `T` must be a primitive value (like an `int`) or a struct
     * thereof, without pointers or any other "complicated" constructs. DQCsim
     * just copies the bytes over. It is up to you to ensure that that's what
     * you want to happen; unfortunately C++11 does not provide a way to
     * statically ensure that this is the case.
     *
     * \param index The index of the argument to retrieve.
     * \returns The C object representation of the argument.
     * \throws std::runtime_error When the current handle is invalid, the
     * argument index is out of range, or the size of the requested object type
     * differs from the size of the stored argument.
     */
    template <typename T>
    T get_arb_arg_as(ssize_t index) const {
      size_t size = check(raw::dqcs_arb_get_size(handle, index));
      if (size != sizeof(T)) {
        throw std::runtime_error(
          "Arbitrary argument has incorrect size: "
          "found " + std::to_string(size) + " bytes, "
          "expected " + std::to_string(sizeof(T)) + " bytes");
      }
      T data;
      check(raw::dqcs_arb_get_raw(handle, index, &data, sizeof(data)));
      return data;
    }

    /**
     * Sets the arbitrary argument list to the given iterable of
     * `std::string`s.
     *
     * \param strings Some object that can be used as an iterator over
     * `const std::string&`s.
     * \throws std::runtime_error When the current handle is invalid.
     */
    template <typename T>
    void set_arb_arg_strings(const T &strings) {
      clear_arb_args();
      for (const std::string &string : strings) {
        push_arb_arg_string(string);
      }
    }

    /**
     * Sets the arbitrary argument at the given index to a (binary) string.
     * Negative indices are relative to the back of the list, as in Python.
     *
     * \param index The index of the argument to set.
     * \param data The new argument data, represented as a (binary) string.
     * \throws std::runtime_error When the current handle is invalid or the
     * index is out of range.
     */
    void set_arb_arg_string(ssize_t index, const std::string &data) {
      check(raw::dqcs_arb_set_raw(handle, index, data.data(), data.size()));
    }

    /**
     * Sets the arbitrary argument at the given index to a value of type `T`.
     * Negative indices are relative to the back of the list, as in Python.
     *
     * \warning Type `T` must be a primitive value (like an `int`) or a struct
     * thereof, without pointers or any other "complicated" constructs. DQCsim
     * just copies the bytes over. It is up to you to ensure that that's what
     * you want to happen; unfortunately C++11 does not provide a way to
     * statically ensure that this is the case.
     *
     * \param index The index of the argument to set.
     * \param data The C object representation of the argument data to set.
     * \throws std::runtime_error When the current handle is invalid or the
     * index is out of range.
     */
    template <typename T>
    void set_arb_arg(ssize_t index, const T &data) {
      check(raw::dqcs_arb_set_raw(handle, index, &data, sizeof(data)));
    }

    /**
     * Pushes a (binary) string to the back of the arbitrary argument list.
     *
     * \param data The data for the new argument, represented as a (binary)
     * string.
     * \throws std::runtime_error When the current handle is invalid.
     */
    void push_arb_arg_string(const std::string &data) {
      check(raw::dqcs_arb_push_raw(handle, data.data(), data.size()));
    }

    /**
     * Pushes a value of type `T` to the back of the arbitrary argument list.
     *
     * \warning Type `T` must be a primitive value (like an `int`) or a struct
     * thereof, without pointers or any other "complicated" constructs. DQCsim
     * just copies the bytes over. It is up to you to ensure that that's what
     * you want to happen; unfortunately C++11 does not provide a way to
     * statically ensure that this is the case.
     *
     * \param data The data for the new argument, represented as some C object.
     * \throws std::runtime_error When the current handle is invalid.
     */
    template <typename T>
    void push_arb_arg(const T &data) {
      check(raw::dqcs_arb_push_raw(handle, &data, sizeof(data)));
    }

    /**
     * Pops from the back of the arbitrary argument list as a (binary) string.
     *
     * \returns The data of the popped argument, represented as a (binary)
     * string.
     * \throws std::runtime_error When the current handle is invalid or the
     * argument list is empty.
     */
    std::string pop_arb_arg_string() {
      size_t size = check(raw::dqcs_arb_get_size(handle, -1));
      std::string data;
      data.resize(size);
      check(raw::dqcs_arb_pop_raw(handle, &data.front(), size));
      return data;
    }

    /**
     * Pops from the back of the arbitrary argument list as a value of type
     * `T`.
     *
     * \note If there is an object size mismatch, the argument is *not* popped.
     *
     * \warning Type `T` must be a primitive value (like an `int`) or a struct
     * thereof, without pointers or any other "complicated" constructs. DQCsim
     * just copies the bytes over. It is up to you to ensure that that's what
     * you want to happen; unfortunately C++11 does not provide a way to
     * statically ensure that this is the case.
     *
     * \returns The data of the popped argument, represented as some C object.
     * \throws std::runtime_error When the current handle is invalid, the
     * argument list is empty, or there is a size mismatch.
     */
    template <typename T>
    T pop_arb_arg_as() const {
      size_t size = check(raw::dqcs_arb_get_size(handle, -1));
      if (size != sizeof(T)) {
        throw std::runtime_error(
          "Arbitrary argument has incorrect size: "
          "found " + std::to_string(size) + " bytes, "
          "expected " + std::to_string(sizeof(T)) + " bytes");
      }
      T data;
      check(raw::dqcs_arb_pop_raw(handle, &data, sizeof(data)));
      return data;
    }

    /**
     * Inserts an arbitrary argument at the given index using a (binary)
     * string. Negative indices are relative to the back of the list, as in
     * Python.
     *
     * \param index The index of the argument to insert at.
     * \param data The new argument data, represented as a (binary) string.
     * \throws std::runtime_error When the current handle is invalid or the
     * index is out of range.
     */
    void insert_arb_arg_string(ssize_t index, const std::string &data) {
      check(raw::dqcs_arb_insert_raw(handle, index, data.data(), data.size()));
    }

    /**
     * Inserts an arbitrary argument at the given index using a value of type
     * `T`. Negative indices are relative to the back of the list, as in
     * Python.
     *
     * \warning Type `T` must be a primitive value (like an `int`) or a struct
     * thereof, without pointers or any other "complicated" constructs. DQCsim
     * just copies the bytes over. It is up to you to ensure that that's what
     * you want to happen; unfortunately C++11 does not provide a way to
     * statically ensure that this is the case.
     *
     * \param index The index of the argument to insert at.
     * \param data The C object representation of the argument data to set.
     * \throws std::runtime_error When the current handle is invalid or the
     * index is out of range.
     */
    template <typename T>
    void insert_arb_arg(ssize_t index, const T &data) {
      check(raw::dqcs_arb_insert_raw(handle, index, &data, sizeof(data)));
    }

    /**
     * Removes the arbitrary argument at the given index. Negative indices are
     * relative to the back of the list, as in Python.
     *
     * \param index The index of the argument to remove.
     * \throws std::runtime_error When the current handle is invalid or the
     * index is out of range.
     */
    void remove_arb_arg(ssize_t index) {
      check(raw::dqcs_arb_remove(handle, index));
    }

    /**
     * Returns the number of arbitrary arguments.
     *
     * \returns The number of binary string arguments.
     * \throws std::runtime_error When the current handle is invalid.
     */
    size_t get_arb_arg_count() const {
      return check(raw::dqcs_arb_len(handle));
    }

    /**
     * Clears the arbitrary argument list.
     *
     * \throws std::runtime_error When the current handle is invalid.
     */
    void clear_arb_args() {
      check(raw::dqcs_arb_clear(handle));
    }

    /**
     * Assigns all arb data from the given arb to this one.
     *
     * \param src The arb-like object to copy the data from.
     * \throws std::runtime_error When either handle is invalid.
     */
    void set_arb(const Arb &src) {
      check(raw::dqcs_arb_assign(handle, src.get_handle()));
    }

  };

  /**
   * Class wrapper for `ArbData` handles.
   *
   * `ArbData` objects can be used by simulations to communicate information
   * that isn't specified by DQCsim between plugins. DQCsim only defines that
   * such arbitrary data consists of a JSON-like object and zero or more
   * binary-safe strings. This means that it is up to the plugins to agree on
   * a common format!
   *
   * `ArbData` objects (as well as the attached `ArbData` in `ArbCmd`, `Gate`,
   * and `Measurement` objects) can be be constructed in-line using the builder
   * pattern. For example, an `ArbData` object with the JSON data
   * `{"hello": "world"}`, an integer argument, and a string argument, can be
   * created as follows:
   *
   * \code
   * ArbData().with_json_string("{\"hello\": \"world\"}")
   *          .with_arg<int>(33)
   *          .with_arg_string("I'm a string!");
   * \endcode
   *
   * When you receive an `ArbData`, you can use the various getters to see
   * what's inside. You can also use the various setters to modify them.
   */
  class ArbData : public Arb {
  public:

    /**
     * Wraps the given `ArbData` handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    ArbData(HandleIndex handle) noexcept : Arb(handle) {
    }

    /**
     * Constructs an empty `ArbData` object.
     *
     * \throws std::runtime_error When DQCsim fails to construct the handle for
     * some reason.
     */
    ArbData() : Arb(check(raw::dqcs_arb_new())) {
    }

    /**
     * Copy-constructs an `ArbData` object from any object supporting the `Arb`
     * interface.
     *
     * \param src The arb-like object to copy from.
     * \throws std::runtime_error When the source handle is invalid, or DQCsim
     * fails to construct the new handle for some reason.
     */
    ArbData(const Arb &src) : Arb(check(raw::dqcs_arb_new())) {
      set_arb(src);
    }

    /**
     * Copy-constructs an `ArbData` object from another `ArbData` object.
     *
     * \param src The arb-like object to copy from.
     * \throws std::runtime_error When the source handle is invalid, or DQCsim
     * fails to construct the new handle for some reason.
     */
    ArbData(const ArbData &src) : Arb(check(raw::dqcs_arb_new())) {
      set_arb(src);
    }

    /**
     * Copy assignment operator for `ArbData` objects.
     *
     * \param src The arb-like object to assign from.
     * \throws std::runtime_error When either handle is invalid.
     */
    void operator=(const ArbData &src) {
      set_arb(src);
    }

    /**
     * Default move constructor.
     */
    ArbData(ArbData&&) = default;

    /**
     * Default move assignment.
     */
    ArbData &operator=(ArbData&&) = default;

    // Include builder pattern functions.
    /**
     * Helper macro to prevent code repetition; not visible outside of the header.
     */
    #define ARB_BUILDER_SUBCLASS ArbData
    #include "arb_builder.hpp"

  };

  /**
   * Class wrapper for handles that support the `cmd` interface.
   *
   * You normally wouldn't instantiate this directly (see `ArbCmd`).
   */
  class Cmd : public Arb {
  public:

    /**
     * Wraps the given `cmd` handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    Cmd(HandleIndex handle) noexcept : Arb(handle) {
    }

    /**
     * Returns the interface identifier of this command.
     *
     * \returns The interface identifier of this command.
     * \throws std::runtime_error When either handle is invalid.
     */
    std::string get_iface() const {
      char *iface_c = check(raw::dqcs_cmd_iface_get(handle));
      std::string iface(iface_c);
      std::free(iface_c);
      return iface;
    }

    /**
     * Returns whether this command has the given interface identifier.
     *
     * \param iface The interface to match against.
     * \returns Whether there was a match.
     * \throws std::runtime_error When either handle is invalid.
     */
    bool is_iface(const std::string &iface) const {
      return check(raw::dqcs_cmd_iface_cmp(handle, iface.c_str()));
    }

    /**
     * Returns the operation identifier of this command.
     *
     * \returns The operation identifier of this command.
     * \throws std::runtime_error When either handle is invalid.
     */
    std::string get_oper() const {
      char *oper_c = check(raw::dqcs_cmd_oper_get(handle));
      std::string oper(oper_c);
      std::free(oper_c);
      return oper;
    }

    /**
     * Returns whether this command has the given operation identifier.
     *
     * \param oper The operation to match against.
     * \returns Whether there was a match.
     * \throws std::runtime_error When either handle is invalid.
     */
    bool is_oper(const std::string &oper) const {
      return check(raw::dqcs_cmd_oper_cmp(handle, oper.c_str()));
    }

  };

  /**
   * Class wrapper for `ArbCmd` handles.
   *
   * `ArbCmd`s, like `ArbData`s, represent user-defined information that can be
   * transferred between plugins. However, where `ArbData` represents only
   * data, `ArbCmd`s represent intent. Usually, where `ArbCmd`s come into play,
   * DQCsim will accept a queue of them rather than just one (`ArbCmdQueue`),
   * where each command represents some custom action to be taken in some
   * context. For instance, when allocating qubits, you can specify one or more
   * `ArbCmd`s to modify the behavior of the qubits, such as requesting that a
   * certain error model be used, if available.
   *
   * `ArbCmd`s are essentially just `ArbData` objects with two required
   * identifier strings attached to them in addition: the *interface* and
   * *operation* identifiers. Their significance is as follows:
   *
   *  - When a plugin receives a command with unknown interface identifier, the
   *    command can be ignored.
   *  - When a plugin receives a command with a known interface identifier but
   *    unknown operation identifier, it must reject the command with an error
   *    of some kind.
   *  - When both identifiers are known, the plugin should take the respective
   *    action.
   *
   * The identifiers must be matched case-sensitively.
   *
   * The identifiers are specified when constructing an `ArbCmd` and are then
   * immutable. The attached `ArbData` can be constructed, manipulated, and
   * queried in the same way as `ArbData` objects.
   */
  class ArbCmd : public Cmd {
  public:

    /**
     * Wraps the given `ArbCmd` handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    ArbCmd(HandleIndex handle) noexcept : Cmd(handle) {
    }

    /**
     * Constructs an `ArbCmd` object.
     *
     * \param iface The interface identifier for the command.
     * \param oper The operation identifier for the command.
     * \throws std::runtime_error When constructing the handle fails for some
     * reason.
     */
    ArbCmd(const std::string &iface, const std::string &oper) : Cmd(check(raw::dqcs_cmd_new(
      iface.c_str(), oper.c_str()
    ))) {
    }

    /**
     * Copy-constructs an `ArbCmd` object from any object supporting the `Cmd`
     * interface.
     *
     * \param src The cmd-like object to copy from.
     * \throws std::runtime_error When the source handle is invalid or
     * constructing the new handle fails for some reason.
     */
    ArbCmd(const Cmd &src) : Cmd(check(raw::dqcs_cmd_new(
      src.get_iface().c_str(), src.get_oper().c_str()
    ))) {
      set_arb(src);
    }

    /**
     * Copy-constructs an `ArbCmd` object from another `ArbCmd` object.
     *
     * \param src The cmd-like object to copy from.
     * \throws std::runtime_error When the source handle is invalid or
     * constructing the new handle fails for some reason.
     */
    ArbCmd(const ArbCmd &src) : Cmd(check(raw::dqcs_cmd_new(
      src.get_iface().c_str(), src.get_oper().c_str()
    ))) {
      set_arb(src);
    }

    /**
     * Copy assignment operator for `ArbCmd` objects.
     *
     * \param src The cmd-like object to copy from.
     * \throws std::runtime_error When freeing the old handle fails, the source
     * handle is invalid, or constructing the new handle fails for some reason.
     */
    void operator=(const ArbCmd &src) {
      // The C API doesn't allow `ArbCmd`s to be copied natively, so we need to
      // make a new one and drop the old handle. We make the copy before
      // exchanging the handles to avoid changing our state if the copy
      // operation throws an error for some reason.
      ArbCmd copy(src);
      free();
      handle = copy.take_handle();
    }

    /**
     * Default move constructor.
     */
    ArbCmd(ArbCmd&&) = default;

    /**
     * Default move assignment.
     */
    ArbCmd &operator=(ArbCmd&&) = default;

    // Include builder pattern functions.
    /**
     * Helper macro to prevent code repetition; not visible outside of the header.
     */
    #define ARB_BUILDER_SUBCLASS ArbCmd
    #include "arb_builder.hpp"

  };

  /**
   * Class wrapper for queues (lists) of `ArbCmd`s.
   *
   * To construct an `ArbCmd` queue iteratively, create a new queue using the
   * default constructor and push `ArbCmd`s into it using `push()`. For
   * instance:
   *
   * \code
   * ArbCmdQueue()
   *   .push(ArbCmd("dummy", "foo").with_arg_string("bar"))
   *   .push(ArbCmd("dummy", "baz").with_arg<int>(42));
   * \endcode
   *
   * To iterate over an existing `ArbCmd` queue (destructively!) in the most
   * efficient way, you can use the following code:
   *
   * ```
   * for (; queue.size() > 0; queue.next()) {
   *   // queue can be used as the current cmd/arb without any copies now
   * }
   * ```
   *
   * You can also drain it into a `std::vector` of `ArbCmd`s
   * (`drain_into_vector`), or, if you must, copy it into one
   * (`copy_into_vector`).
   */
  class ArbCmdQueue : public Cmd {
  public:

    /**
     * Wraps the given `ArbCmdQueue` handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    ArbCmdQueue(HandleIndex handle) noexcept : Cmd(handle) {
    }

    /**
     * Constructs an empty `ArbCmd` queue object.
     *
     * \throws std::runtime_error When constructing the handle fails for some
     * reason.
     */
    ArbCmdQueue() : Cmd(check(raw::dqcs_cq_new())) {
    }

    /**
     * Pushes an `ArbCmd` into the queue by moving.
     *
     * \param cmd The `ArbCmd` to push. Consumed by this function.
     * \throws std::runtime_error When either handle is invalid.
     */
    void push(ArbCmd &&cmd) {
      check(raw::dqcs_cq_push(handle, cmd.get_handle()));
    }

    /**
     * Pushes an `ArbCmd` into the queue by copying.
     *
     * \param cmd The `ArbCmd` to push.
     * \throws std::runtime_error When either handle is invalid, or copying
     * `cmd` fails.
     */
    void push(const Cmd &cmd) {
      push(ArbCmd(cmd));
    }

    /**
     * Constructs an `ArbCmd` queue object from an iterable of `ArbCmd`s by
     * moving.
     *
     * \param cmds The iterable of `ArbCmd&`s to push. Consumed by this
     * function.
     * \returns The constructed `ArbCmdQueue`.
     * \throws std::runtime_error When any of the handles are invalid, or
     * construction of the queue handle fails.
     */
    template <class T>
    static ArbCmdQueue from_iter(T &&cmds) {
      ArbCmdQueue result;
      for (Cmd &cmd : cmds) {
        result.push(std::move(cmd));
      }
      return result;
    }

    /**
     * Constructs an `ArbCmd` queue object from an iterable of `ArbCmd`s by
     * copying.
     *
     * \param cmds The iterable of `const ArbCmd&`s to push.
     * \returns The constructed `ArbCmdQueue`.
     * \throws std::runtime_error When any of the handles are invalid, copying
     * those handles fails, or construction of the queue handle fails.
     */
    template <class T>
    static ArbCmdQueue from_iter(const T &cmds) {
      ArbCmdQueue result;
      for (const Cmd &cmd : cmds) {
        result.push(cmd);
      }
      return result;
    }

    /**
     * Pushes an `ArbCmd` into the queue by moving (builder pattern).
     *
     * \param cmd The `ArbCmd` to push. Consumed by this function.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When either handle is invalid.
     */
    ArbCmdQueue &&with(ArbCmd &&cmd) {
      push(std::move(cmd));
      return std::move(*this);
    }

    /**
     * Pushes an `ArbCmd` into the queue by copying (builder pattern).
     *
     * \param cmd The `ArbCmd` to push.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When either handle is invalid or the copy
     * fails.
     */
    ArbCmdQueue &&with(const Cmd &cmd) {
      push(cmd);
      return std::move(*this);
    }

    /**
     * Pops the first `ArbCmd` from the queue, allowing the next one to be
     * accessed.
     *
     * \throws std::runtime_error When the handle is invalid, or the queue is
     * empty.
     */
    void next() {
      check(raw::dqcs_cq_next(handle));
    }

    /**
     * Returns the number of `ArbCmd`s in the queue.
     *
     * \returns The number of `ArbCmd`s in the queue.
     * \throws std::runtime_error When the handle is invalid.
     */
    size_t size() const {
      return check(raw::dqcs_cq_len(handle));
    }

    /**
     * Drains the queue into a vector of `ArbCmd`s. This is less performant
     * than iterating over the queue manually, because it requires copies.
     *
     * \returns A `std::vector<ArbCmd>` representation of the queue.
     * \throws std::runtime_error When the handle is invalid.
     */
    std::vector<ArbCmd> drain_into_vector() {
      std::vector<ArbCmd> cmds;
      for (; size() > 0; next()) {
        cmds.emplace_back(*this);
      }
      return cmds;
    }

    /**
     * Copies the queue into a vector of `ArbCmd`s. This is less performant
     * than iterating over the queue manually or using `drain_into_vector()`,
     * because it requires (additional) copies.
     *
     * \note This function is not `const`, because exceptions during the copy
     * operation can change its value, and the underlying handle is changed.
     * However, under normal conditions, the contents appear to be unchanged.
     *
     * \returns A `std::vector<ArbCmd>` representation of the queue.
     * \throws std::runtime_error When the handle is invalid.
     */
    std::vector<ArbCmd> copy_into_vector() {
      std::vector<ArbCmd> cmds = drain_into_vector();
      free();
      handle = ArbCmdQueue::from_iter(cmds).take_handle();
      return cmds;
    }

    /**
     * Copy-constructs a queue of `ArbCmd`s.
     *
     * \note This requires destructive iteration of the source object, so it
     * isn't not const; if an exception occurs, the state of the source object
     * may be changed.
     *
     * \param src The queue to copy from.
     * \throws std::runtime_error When the source handle is invalid or
     * construction of the new object fails.
     */
    ArbCmdQueue(ArbCmdQueue &src) : Cmd(0) {
      handle = ArbCmdQueue::from_iter(src.copy_into_vector()).take_handle();
    }

    /**
     * Copy-assigns a queue of `ArbCmd`s.
     *
     * \note This requires destructive iteration of the source object, so it
     * isn't not const; if an exception occurs, the state of the source object
     * may be changed.
     *
     * \param src The queue to copy from.
     * \throws std::runtime_error When the source handle is invalid, freeing
     * any existing handle in the destination fails, or construction of the
     * new object fails.
     */
    ArbCmdQueue &operator=(ArbCmdQueue &src) {
      free();
      handle = ArbCmdQueue::from_iter(src.copy_into_vector()).take_handle();
      return *this;
    }

    /**
     * Default move constructor.
     */
    ArbCmdQueue(ArbCmdQueue&&) = default;

    /**
     * Default move assignment.
     */
    ArbCmdQueue &operator=(ArbCmdQueue&&) = default;

  };

  /**
   * Represents a qubit.
   *
   * This is a wrapper around the `QubitIndex` type, which prevents mutation
   * and mathematical operations that don't make sense.
   *
   * DQCsim's C++ API expects and gives out qubit references in this form. You
   * can convert between it and `QubitIndex`es using the constructor and the
   * `get_index` function as you please. Note that there is no performance
   * difference between one or the other as they both represent a 64-bit
   * integer in memory; this class purely adds some type-safety.
   *
   * If you have the `dqcsim::wrap` namespace in scope, a custom literal is
   * available for qubit references: instead of `QubitRef(33)` you can type
   * `33_q` as well.
   */
  class QubitRef {
  private:

    /**
     * The raw qubit index wrapped by this reference.
     */
    QubitIndex index;

  public:

    /**
     * Wraps a raw reference.
     *
     * \param index The index of the qubit to wrap.
     * \throws std::runtime_error When the qubit index is invalid (zero).
     */
    QubitRef(QubitIndex index) : index(index) {
      if (index == 0) {
        throw std::runtime_error("Qubit indices cannot be zero in DQCsim");
      }
    }

    /**
     * Default copy constructor.
     */
    QubitRef(const QubitRef&) = default;

    /**
     * Default copy assignment.
     */
    QubitRef &operator=(const QubitRef&) = default;

    /**
     * Default move constructor.
     */
    QubitRef(QubitRef&&) = default;

    /**
     * Default move assignment.
     */
    QubitRef &operator=(QubitRef&&) = default;

    /**
     * Qubit reference equality operator.
     *
     * \param other The qubit reference to compare with.
     * \returns Whether the references refer to the same qubit.
     */
    bool operator==(const QubitRef &other) const noexcept {
      return index == other.index;
    }

    /**
     * Qubit reference inequality operator.
     *
     * \param other The qubit reference to compare with.
     * \returns Whether the references refer to different qubits.
     */
    bool operator!=(const QubitRef &other) const noexcept {
      return index != other.index;
    }

    /**
     * Allow qubit references to be printed.
     *
     * \param out The output stream to write to.
     * \param qubit The qubit reference to dump.
     * \returns The output stream object.
     */
    friend std::ostream& operator<<(std::ostream &out, const QubitRef &qubit) {
      out << 'q' << qubit.index;
      return out;
    }

    /**
     * Returns the raw qubit index.
     *
     * \returns The raw qubit index.
     */
    QubitIndex get_index() const noexcept {
      return index;
    }

  };

  /**
   * Literal operator for qubits, so for instance `15_q` returns qubit 15.
   */
  inline QubitRef operator "" _q(unsigned long long int qubit) {
    return QubitRef(qubit);
  }

  /**
   * Represents an ordered set of qubit references.
   *
   * DQCsim's API primarily uses these objects to represent the gate operand
   * lists for multi-qubit gates. They can also be used to represent
   * multi-qubit registers, but typically a `std::vector<QubitRef>` will be
   * more suitable for that, as it supports fast and convenient indexation.
   * You can convert between a `QubitSet` and a vector easily using
   * `from_iter`, `drain_into_vector`, and `copy_into_vector`.
   *
   * You can use the builder pattern to construct qubit sets in a single line:
   *
   * \code
   * QubitSet().with(1_q).with(2_q).with(3_q);
   * \endcode
   */
  class QubitSet : public Handle {
  public:

    /**
     * Wraps the given qubit set handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    QubitSet(HandleIndex handle) noexcept : Handle(handle) {
    }

    /**
     * Constructs an empty qubit set.
     *
     * \throws std::runtime_error When construction of the new handle fails for
     * some reason.
     */
    QubitSet() : Handle(check(raw::dqcs_qbset_new())) {
    }

    /**
     * Constructs a qubit set object from an iterable of qubit references.
     *
     * \param qubits An iterable of `const QubitRef&` to take the qubits from.
     * \throws std::runtime_error When construction of the new handle fails for
     * some reason.
     */
    template <class T>
    static QubitSet from_iter(const T &qubits) {
      QubitSet result;
      for (const QubitRef &qubit : qubits) {
        result.push(qubit);
      }
      return result;
    }

    /**
     * Copy-constructs a qubit set.
     *
     * \param src The object to copy from.
     * \throws std::runtime_error When the source handle is invalid or
     * construction of the new handle failed for some reason.
     */
    QubitSet(const QubitSet &src) : Handle(check(raw::dqcs_qbset_copy(src.handle))) {
    }

    /**
     * Copy assignment operator for qubit sets.
     *
     * \param src The object to copy from.
     * \throws std::runtime_error When the source handle is invalid, any
     * previously wrapped handle in the destination object could not be freed,
     * or construction of the new handle failed for some reason.
     */
    void operator=(const QubitSet &src) {
      QubitSet copy(src);
      free();
      handle = copy.take_handle();
    }

    /**
     * Default move constructor.
     */
    QubitSet(QubitSet&&) = default;

    /**
     * Default move assignment.
     */
    QubitSet &operator=(QubitSet&&) = default;

    /**
     * Pushes a qubit into the set. Note that qubit sets are ordered. An
     * exception is thrown if the qubit is already in the set.
     *
     * \param qubit The qubit reference to push.
     * \throws std::runtime_error When the handle is invalid.
     */
    void push(const QubitRef &qubit) {
      check(raw::dqcs_qbset_push(handle, qubit.get_index()));
    }

    /**
     * Pushes a qubit into the set (builder pattern). Note that qubit sets are
     * ordered. An exception is thrown if the qubit is already in the set.
     *
     * \param qubit The qubit reference to push.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the handle is invalid.
     */
    QubitSet &&with(const QubitRef &qubit) {
      push(qubit);
      return std::move(*this);
    }

    /**
     * Pops a qubit from the set. Qubits are popped in the same order in which
     * they are pushed (like a FIFO).
     *
     * \returns The popped qubit reference.
     * \throws std::runtime_error When the handle is invalid.
     */
    QubitRef pop() {
      return QubitRef(check(raw::dqcs_qbset_pop(handle)));
    }

    /**
     * Returns the number of qubits in the set.
     *
     * \returns The number of qubits in the set.
     * \throws std::runtime_error When the handle is invalid.
     */
    size_t size() const {
      return check(raw::dqcs_qbset_len(handle));
    }

    /**
     * Returns whether the given qubit is contained in the set.
     *
     * \param qubit The qubit reference that containment should be checked for.
     * \returns Whether the set contains the given qubit reference.
     * \throws std::runtime_error When the handle is invalid.
     */
    bool contains(const QubitRef &qubit) const {
      return check(raw::dqcs_qbset_contains(handle, qubit.get_index()));
    }

    /**
     * Drains the qubit set into a vector.
     *
     * \note This requires destructive iteration of the source object, so it
     * isn't not const; if an exception occurs, the state of the source object
     * may be changed.
     *
     * \returns A `std::vector<QubitRef>` containing the qubits that were in
     * the set, in insertion order.
     * \throws std::runtime_error When the handle is invalid.
     */
    std::vector<QubitRef> drain_into_vector() {
      std::vector<QubitRef> qubits;
      while (size()) {
        qubits.emplace_back(pop());
      }
      return qubits;
    }

    /**
     * Copies the qubit set into a vector.
     *
     * \note This requires destructive iteration of the source object, so it
     * isn't not const; if an exception occurs, the state of the source object
     * may be changed.
     *
     * \returns A `std::vector<QubitRef>` containing the qubits that are in
     * the set, in insertion order.
     * \throws std::runtime_error When the handle is invalid.
     */
    std::vector<QubitRef> copy_into_vector() const {
      QubitSet copy(*this);
      return copy.drain_into_vector();
    }

  };

  /**
   * Typedef for the complex numbers used within the gate matrices.
   */
  using complex = std::complex<double>;

  /**
   * Represents a square matrix used for describing N-qubit gates.
   *
   * \note DQCsim is not a math library: this matrix class is solely intended
   * as an interface between DQCsim's internal matrix representation and
   * whatever math library you want to use.
   */
  class Matrix : public Handle {
  public:

    /**
     * Wraps the given matrix handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    Matrix(HandleIndex handle) noexcept : Handle(handle) {
    }

    /**
     * Constructs a matrix from a row-major flattened array of `4**num_qubits`
     * `complex`s.
     *
     * \param num_qubits The number of qubits that the matrix is intended for.
     * Must be 1 or more.
     * \param matrix Pointer to an array of complex numbers representing the
     * desired matrix in row-major form. The matrix has `4**num_qubits` complex
     * entries.
     * \returns A new matrix containing the desired data.
     * \throws std::runtime_error When constructing the matrix failed.
     */
    Matrix(size_t num_qubits, const complex *matrix)
      : Handle(check(raw::dqcs_mat_new(num_qubits, (const double *)matrix))) {
    }

    /**
     * Constructs a matrix from a row-major flattened array of
     * `2 * 4**num_qubits` `complex`s.
     *
     * \param num_qubits The number of qubits that the matrix is intended for.
     * Must be 1 or more.
     * \param matrix Pointer to an array of complex numbers representing the
     * desired matrix in row-major form using (real, imag) pairs. The matrix
     * contains `2*4**num_qubits` doubles.
     * \returns A new matrix containing the desired data.
     * \throws std::runtime_error When constructing the matrix failed.
     */
    Matrix(size_t num_qubits, const double *matrix)
      : Handle(check(raw::dqcs_mat_new(num_qubits, matrix))) {
    }

    /**
     * Copy-constructs a matrix.
     *
     * \param src The object to copy from.
     * \throws std::runtime_error When the source handle is invalid or
     * construction of the new handle failed for some reason.
     */
    Matrix(const Matrix &src)
      : Handle(check(raw::dqcs_mat_add_controls(src.get_handle(), 0))) {
    }

    /**
     * Copy assignment operator for matrices.
     *
     * \param src The object to copy from.
     * \throws std::runtime_error When the source handle is invalid, any
     * previously wrapped handle in the destination object could not be freed,
     * or construction of the new handle failed for some reason.
     */
    void operator=(const Matrix &src) {
      Matrix copy(src);
      free();
      handle = copy.take_handle();
    }

    /**
     * Default move constructor.
     */
    Matrix(Matrix&&) = default;

    /**
     * Default move assignment.
     */
    Matrix &operator=(Matrix&&) = default;

    /**
     * Returns the number of elements in the matrix.
     *
     * \returns The number of elements in the matrix.
     * \throws std::runtime_error When the matrix handle is invalid.
     */
    size_t size() const {
        return check(raw::dqcs_mat_len(handle));
    }

    /**
     * Returns the number of rows/columns in the matrix.
     *
     * \returns The number of rows/columns in the matrix.
     * \throws std::runtime_error When the matrix handle is invalid.
     */
    size_t dimension() const {
        return check(raw::dqcs_mat_dimension(handle));
    }

    /**
     * Returns the number of qubits associated with the matrix.
     *
     * \returns The number of qubits associated with the matrix.
     * \throws std::runtime_error When the matrix handle is invalid.
     */
    size_t num_qubits() const {
        return check(raw::dqcs_mat_num_qubits(handle));
    }

    /**
     * Returns the data contained by the matrix in row-major form as complex
     * numbers.
     *
     * \returns The data contained by the matrix.
     * \throws std::runtime_error When the matrix handle is invalid.
     */
    std::vector<complex> get() const {
      size_t s = size();
      std::vector<complex> vec(s);
      double *data = check(raw::dqcs_mat_get(handle));
      std::memcpy(vec.data(), data, s * sizeof(complex));
      return vec;
    }

    /**
     * Returns the data contained by the matrix in row-major form as pairs
     * of real/imag doubles.
     *
     * \returns The data contained by the matrix.
     * \throws std::runtime_error When the matrix handle is invalid.
     */
    std::vector<double> get_as_doubles() const {
      size_t s = size();
      std::vector<double> vec(s * 2);
      double *data = check(raw::dqcs_mat_get(handle));
      std::memcpy(vec.data(), data, 2 * s * sizeof(double));
      return vec;
    }

    /**
     * Matrix fuzzy equality operator.
     *
     * \param other The matrix to compare to.
     * \param epsilon The maximum tolerated RMS variation of the elements.
     * \param ignore_global_phase Whether global phase differences should be
     * ignored in the comparison.
     * \returns Whether the matrices are approximately equal.
     * \throws std::runtime_error When either handle is invalid.
     */
    bool fuzzy_equal(
      const Matrix &other,
      double epsilon = 0.000001,
      bool ignore_global_phase = true
    ) const {
      return check(raw::dqcs_mat_approx_eq(handle, other.get_handle(), epsilon, ignore_global_phase));
    }

    /**
     * Constructs a controlled matrix from the given matrix.
     *
     * \param number_of_controls The number of control qubits to add.
     * \returns The new matrix.
     * \throws std::runtime_error When the handle is invalid or construction
     * of the new matrix failed.
     */
    Matrix add_controls(size_t number_of_controls) const {
      return Matrix(check(raw::dqcs_mat_add_controls(handle, number_of_controls)));
    }

    /**
     * Splits a controlled matrix into its non-controlled submatrix and the
     * indices of the control qubits.
     *
     * \param epsilon The maximum magitude of the difference between the column
     * vectors of the input matrix and the identity matrix (after dephasing if
     * `ignore_phase` is set) for the column vector to be considered to not
     * affect the respective entry in the quantum state vector. Note that if
     * this is greater than zero, the resulting gate may not be exactly
     * equivalent.
     * \param ignore_global_phase If this is set, any global phase in the
     * matrix is ignored, but note that if control qubits are stripped the
     * "global" phase of the resulting submatrix is always significant.
     * \returns A pair consisting of a sorted vector of the qubit indices that
     * were removed and the newly constructed submatrix.
     * \throws std::runtime_error When the handle is invalid or construction
     * of the new matrix failed.
     */
    std::pair<std::vector<size_t>, Matrix> strip_control(double epsilon, bool ignore_global_phase) const {
      ssize_t *indices = nullptr;
      std::pair<std::vector<size_t>, Matrix> result(
        std::vector<size_t>(),
        check(raw::dqcs_mat_strip_control(handle, epsilon, ignore_global_phase, &indices))
      );
      while (*indices != -1) {
        result.first.push_back(*indices++);
      }
      return result;
    }

  };

  /**
   * Contains shorthand methods for a variety of commonly used gate matrices.
   *
   * \note This is a class an not a namespace because it has global constants,
   * and this is a header-only library.
   */
  class GateMatrix {
  private:

    /**
     * Dummy constructor. There is no point in ever constructing this class,
     * all members are static.
     */
    GateMatrix() {};

  public:

    /**
     * The Pauli I matrix.
     *
     * The matrix is as follows:
     *
     * \f[
     * I = \sigma_0 = \begin{bmatrix}
     * 1 & 0 \\
     * 0 & 1
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for the Pauli I gate.
     */
    static const Matrix &I() noexcept {
      const double values[8] = {
        1.0,  0.0,    0.0,  0.0,
        0.0,  0.0,    1.0,  0.0,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * The Pauli X matrix.
     *
     * The matrix is as follows:
     *
     * \f[
     * X = \sigma_1 = \begin{bmatrix}
     * 0 & 1 \\
     * 1 & 0
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for the Pauli X gate.
     */
    static const Matrix &X() noexcept {
      const double values[8] = {
        0.0,  0.0,    1.0,  0.0,
        1.0,  0.0,    0.0,  0.0,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * The Pauli Y matrix.
     *
     * The matrix is as follows:
     *
     * \f[
     * Y = \sigma_2 = \begin{bmatrix}
     * 0 & -i \\
     * i & 0
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for the Pauli Y gate.
     */
    static const Matrix &Y() noexcept {
      const double values[8] = {
        0.0,  0.0,    0.0,  -1.0,
        0.0,  1.0,    0.0,  0.0,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * The Pauli Z matrix.
     *
     * The matrix is as follows:
     *
     * \f[
     * Z = \sigma_3 = \begin{bmatrix}
     * 1 & 0 \\
     * 0 & -1
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for the Pauli Z gate.
     */
    static const Matrix &Z() noexcept {
      const double values[8] = {
        1.0,  0.0,    0.0,  0.0,
        0.0,  0.0,    -1.0, 0.0,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * Hadamard gate.
     *
     * This represents a 180-degree Y rotation, followed by a 90-degree X
     * rotation. The matrix is as follows:
     *
     * \f[
     * H = \frac{1}{\sqrt{2}} \begin{bmatrix}
     * 1 & 1 \\
     * 1 & -1
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for the Hadamard gate.
     */
    static const Matrix &H() noexcept {
      const double IR2 = M_SQRT1_2;
      const double values[8] = {
        IR2,  0.0,    IR2,  0.0,
        IR2,  0.0,    -IR2, 0.0,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * The S matrix.
     *
     * This represents a 90 degree Z rotation. The matrix is as follows:
     *
     * \f[
     * S = \begin{bmatrix}
     * 1 & 0 \\
     * 0 & i
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for the S gate.
     */
    static const Matrix &S() noexcept {
      const double values[8] = {
        1.0,  0.0,    0.0,  0.0,
        0.0,  0.0,    0.0,  1.0,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * The S-dagger matrix.
     *
     * This represents a negative 90 degree Z rotation. The matrix is as
     * follows:
     *
     * \f[
     * S^\dagger = \begin{bmatrix}
     * 1 & 0 \\
     * 0 & -i
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for the S-dagger gate.
     */
    static const Matrix &SDAG() noexcept {
      const double values[8] = {
        1.0,  0.0,    0.0,  0.0,
        0.0,  0.0,    0.0,  -1.0,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * The T matrix.
     *
     * This represents a 45 degree Z rotation. The matrix is as follows:
     *
     * \f[
     * T = \begin{bmatrix}
     * 1 & 0 \\
     * 0 & e^{i\frac{\pi}{4}}
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for the T gate.
     */
    static const Matrix &T() noexcept {
      const double IR2 = M_SQRT1_2;
      const double values[8] = {
        1.0,  0.0,    0.0,  0.0,
        0.0,  0.0,    IR2,  IR2,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * The T-dagger matrix.
     *
     * This represents a negative 45 degree Z rotation. The matrix is as follows:
     *
     * \f[
     * T^\dagger = \begin{bmatrix}
     * 1 & 0 \\
     * 0 & e^{-i\frac{\pi}{4}}
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for the T-dagger gate.
     */
    static const Matrix &TDAG() noexcept {
      const double IR2 = M_SQRT1_2;
      const double values[8] = {
        1.0,  0.0,    0.0,  0.0,
        0.0,  0.0,    IR2,  -IR2,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * Computes the matrix for an arbitrary X rotation.
     *
     * The matrix is as follows:
     *
     * \f[
     * R_x(\theta) = \begin{bmatrix}
     * \cos{\frac{\theta}{2}} & -i\sin{\frac{\theta}{2}} \\
     * -i\sin{\frac{\theta}{2}} & \cos{\frac{\theta}{2}}
     * \end{bmatrix}
     * \f]
     *
     * \param theta The rotation angle in radians.
     * \returns The matrix for an X rotation gate with angle theta.
     */
    static Matrix RX(double theta) noexcept {
      double co = std::cos(0.5 * theta);
      double si = std::sin(0.5 * theta);
      double values[8] = {
        co,   0.0,    0.0,  -si,
        0.0,  -si,    co,   0.0,
      };
      return Matrix(4, values);
    }

    /**
     * Precomputed 90-degree X rotation gate.
     *
     * The matrix is as follows:
     *
     * \f[
     * R_x\left(\frac{\pi}{2}\right) = \frac{1}{\sqrt{2}} \begin{bmatrix}
     * 1 & -i \\
     * -i & 1
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for a postive 90-degree X rotation.
     */
    static const Matrix &RX90() noexcept {
      const double IR2 = M_SQRT1_2;
      const double values[8] = {
        IR2,  0.0,    0.0,  -IR2,
        0.0,  -IR2,   IR2,  0.0,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * Precomputed negative 90-degree X rotation gate.
     *
     * The matrix is as follows:
     *
     * \f[
     * R_x\left(-\frac{\pi}{2}\right) = \frac{1}{\sqrt{2}} \begin{bmatrix}
     * 1 & i \\
     * i & 1
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for a negative 90-degree X rotation.
     */
    static const Matrix &RXM90() noexcept {
      const double IR2 = M_SQRT1_2;
      const double values[8] = {
        IR2,  0.0,    0.0,  IR2,
        0.0,  IR2,    IR2,  0.0,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * Precomputed 180-degree RX gate.
     *
     * The matrix is as follows:
     *
     * \f[
     * R_x(\pi) = \begin{bmatrix}
     * 0 & -i \\
     * -i & 0
     * \end{bmatrix}
     * \f]
     *
     * This matrix is equivalent to the Pauli X gate, but differs in global
     * phase.
     *
     * \returns The matrix for a positive 180-degree X rotation.
     */
    static const Matrix &RX180() noexcept {
      const double values[8] = {
        0.0,  0.0,    0.0,  -1.0,
        0.0,  -1.0,   0.0,  0.0,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * Computes the matrix for an arbitrary Y rotation.
     *
     * The matrix is as follows:
     *
     * \f[
     * R_y(\theta) = \begin{bmatrix}
     * \cos{\frac{\theta}{2}} & -\sin{\frac{\theta}{2}} \\
     * \sin{\frac{\theta}{2}} & \cos{\frac{\theta}{2}}
     * \end{bmatrix}
     * \f]
     *
     * \param theta The rotation angle in radians.
     * \returns The matrix for an Y rotation gate with angle theta.
     */
    static Matrix RY(double theta) noexcept {
      double co = std::cos(0.5 * theta);
      double si = std::sin(0.5 * theta);
      double values[8] = {
        co,   0.0,    -si,  0.0,
        si,   0.0,    co,   0.0,
      };
      return Matrix(4, values);
    }

    /**
     * Precomputed 90-degree Y rotation gate.
     *
     * The matrix is as follows:
     *
     * \f[
     * R_y\left(\frac{\pi}{2}\right) = \frac{1}{\sqrt{2}} \begin{bmatrix}
     * 1 & -1 \\
     * 1 & 1
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for a positive 90-degree Y rotation.
     */
    static const Matrix &RY90() noexcept {
      const double IR2 = M_SQRT1_2;
      const double values[8] = {
        IR2,  0.0,    -IR2, 0.0,
        IR2,  0.0,    IR2,  0.0,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * Precomputed negative 90-degree RY gate.
     *
     * The matrix is as follows:
     *
     * \f[
     * R_y\left(\frac{\pi}{2}\right) = \frac{1}{\sqrt{2}} \begin{bmatrix}
     * 1 & 1 \\
     * -1 & 1
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for a negative 90-degree Y rotation.
     */
    static const Matrix &RYM90() noexcept {
      const double IR2 = M_SQRT1_2;
      const double values[8] = {
        IR2,  0.0,    IR2,  0.0,
        -IR2, 0.0,    IR2,  0.0,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * Precomputed 180-degree RY gate.
     *
     * The matrix is as follows:
     *
     * \f[
     * R_y(\pi) = \begin{bmatrix}
     * 0 & -1 \\
     * 1 & 0
     * \end{bmatrix}
     * \f]
     *
     * This matrix is equivalent to the Pauli Y gate, but differs in global
     * phase.
     *
     * \returns The matrix for a positive 180-degree Y rotation.
     */
    static const Matrix &RY180() noexcept {
      const double values[8] = {
        0.0,  0.0,    -1.0, 0.0,
        1.0,  0.0,    0.0,  0.0,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * Computes the matrix for an arbitrary Z rotation.
     *
     * The matrix is as follows:
     *
     * \f[
     * R_z(\theta) = \begin{bmatrix}
     * e^{-i\frac{\theta}{2}} & 0 \\
     * 0 & e^{i\frac{\theta}{2}}
     * \end{bmatrix}
     * \f]
     *
     * \param theta The rotation angle in radians.
     * \returns The matrix for a Z rotation gate with angle theta.
     */
    static Matrix RZ(double theta) noexcept {
      double co = std::cos(0.5 * theta);
      double si = std::sin(0.5 * theta);
      double values[8] = {
        co,   -si,    0.0,  0.0,
        0.0,  0.0,    co,   si,
      };
      return Matrix(4, values);
    }

    /**
     * Precomputed 90-degree RZ gate.
     *
     * The matrix is as follows:
     *
     * \f[
     * R_z\left(\frac{\pi}{2}\right) = \frac{1}{\sqrt{2}} \begin{bmatrix}
     * 1-i & 0 \\
     * 0 & 1+i
     * \end{bmatrix}
     * \f]
     *
     * This matrix is equivalent to the S gate, but differs in global phase.
     *
     * \returns The matrix for a positive 90-degree Z rotation.
     */
    static const Matrix &RZ90() noexcept {
      const double IR2 = M_SQRT1_2;
      const double values[8] = {
        IR2,  -IR2,   0.0,  0.0,
        0.0,  0.0,    IR2,  IR2,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * Precomputed negative 90-degree RZ gate.
     *
     * The matrix is as follows:
     *
     * \f[
     * R_z\left(-\frac{\pi}{2}\right) = \frac{1}{\sqrt{2}} \begin{bmatrix}
     * 1+i & 0 \\
     * 0 & 1-i
     * \end{bmatrix}
     * \f]
     *
     * This matrix is equivalent to the S-dagger gate, but differs in global
     * phase.
     *
     * \returns The matrix for a negative 90-degree Z rotation.
     */
    static const Matrix &RZM90() noexcept {
      const double IR2 = M_SQRT1_2;
      const double values[8] = {
        IR2,  IR2,    0.0,  0.0,
        0.0,  0.0,    IR2,  -IR2,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * Precomputed 180-degree RZ gate.
     *
     * The matrix is as follows:
     *
     * \f[
     * R_z(\pi) = \begin{bmatrix}
     * -i & 0 \\
     * 0 & i
     * \end{bmatrix}
     * \f]
     *
     * This matrix is equivalent to the Pauli Z gate, but differs in global
     * phase.
     *
     * \returns The matrix for a positive 180-degree Z rotation.
     */
    static const Matrix &RZ180() noexcept {
      const double values[8] = {
        0.0,  -1.0,   0.0,  0.0,
        0.0,  0.0,    0.0,  1.0,
      };
      static const Matrix matrix(1, values);
      return matrix;
    }

    /**
     * Computes the matrix for an arbitrary rotation.
     *
     * The matrix is as follows:
     *
     * \f[
     * U(\theta, \phi, \lambda) = \begin{bmatrix}
     * e^{i\frac{-\phi - \lambda}{2}} \cos{\frac{\theta}{2}} & e^{i\frac{-\phi + \lambda}{2}} \sin{\frac{\theta}{2}} \\
     * e^{i\frac{ \phi - \lambda}{2}} \sin{\frac{\theta}{2}} & e^{i\frac{ \phi + \lambda}{2}} \cos{\frac{\theta}{2}}
     * \end{bmatrix}
     * \f]
     *
     * This is equivalent to the following:
     *
     * \f[
     * U(\theta, \phi, \lambda) = R_z(\phi) \cdot R_y(\theta) \cdot R_z(\lambda)
     * \f]
     *
     * The rotation order is taken from Qiskit's U3 gate, but the global phase
     * is defined differently.
     *
     * The rotation angles are specified in radians.
     *
     * \param theta The rotation angle in radians for the Y rotation.
     * \param phi The rotation angle in radians for the pre-Y Z rotation.
     * \param lambda The rotation angle in radians for the post-Y Z rotation.
     * \returns The matrix for a Z rotation gate with angle theta.
     */
    static Matrix R(double theta, double phi, double lambda) noexcept {
      double co = std::cos(0.5 * theta);
      double si = std::sin(0.5 * theta);
      complex values[4] = {
        +co * std::exp(std::complex<double>(0.0, 0.5 * (- lambda - phi))),
        -si * std::exp(std::complex<double>(0.0, 0.5 * (+ lambda - phi))),
        +si * std::exp(std::complex<double>(0.0, 0.5 * (- lambda + phi))),
        +co * std::exp(std::complex<double>(0.0, 0.5 * (+ lambda + phi))),
      };
      return Matrix(4, values);
    }

    /**
     * The matrix for a swap gate.
     *
     * The matrix is as follows:
     *
     * \f[
     * \textit{SWAP} = \begin{bmatrix}
     * 1 & 0 & 0 & 0 \\
     * 0 & 0 & 1 & 0 \\
     * 0 & 1 & 0 & 0 \\
     * 0 & 0 & 0 & 1
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for a swap gate.
     */
    static const Matrix &SWAP() noexcept {
      const double values[32] = {
        1.0,  0.0,    0.0,  0.0,    0.0,  0.0,    0.0,  0.0,
        0.0,  0.0,    0.0,  0.0,    1.0,  0.0,    0.0,  0.0,
        0.0,  0.0,    1.0,  0.0,    0.0,  0.0,    0.0,  0.0,
        0.0,  0.0,    0.0,  0.0,    0.0,  0.0,    1.0,  0.0,
      };
      static const Matrix matrix(2, values);
      return matrix;
    }

    /**
     * The square-root of a swap gate matrix.
     *
     * The matrix is as follows:
     *
     * \f[
     * \sqrt{\textit{SWAP}} = \begin{bmatrix}
     * 1 & 0 & 0 & 0 \\
     * 0 & \frac{i+1}{2} & \frac{i-1}{2} & 0 \\
     * 0 & \frac{i-1}{2} & \frac{i+1}{2} & 0 \\
     * 0 & 0 & 0 & 1
     * \end{bmatrix}
     * \f]
     *
     * \returns The matrix for a square-root-of-swap gate.
     */
    static const Matrix &SQSWAP() noexcept {
      const double values[32] = {
        1.0,  0.0,    0.0,  0.0,    0.0,  0.0,    0.0,  0.0,
        0.0,  0.0,    0.5,  0.5,    0.5,  -0.5,   0.0,  0.0,
        0.0,  0.0,    0.5,  -0.5,   0.5,  0.5,    0.0,  0.0,
        0.0,  0.0,    0.0,  0.0,    0.0,  0.0,    1.0,  0.0,
      };
      static const Matrix matrix(2, values);
      return matrix;
    }

  };

  /**
   * Represents any kind of gate with qubits bound to it.
   *
   * DQCsim currently knows three kinds of gates: %unitary gates, Z-axis
   * %measurement gates, and %custom gates. These are constructed with the
   * `unitary`, `measurement`, and `custom` constructors respectively.
   * Briefly put:
   *
   *  - %unitary gates represent any kind of pure-quantum gate, represented
   *    using a unitary matrix operating on one or more qubits, and zero or
   *    more condition qubits that are implicitly added to the gate matrix by
   *    the backend.
   *
   *  - %measurement gates simply measure one or more qubits along the Z axis.
   *    Measurements along different axes can be emulated using a sequence of
   *    gates or (if needed) through a %custom gate.
   *
   *  - %custom gates allow any of the above and anything in addition. They
   *    are case-sensitively identified by their name. Use them sparingly;
   *    limiting your plugin to using regular %unitary and %measurement gates
   *    greatly increases the chance of it playing together nicely with other
   *    plugins out of the box.
   *
   * Refer to the constructor overloads for more information.
   *
   * Gates may have arbitrary data attached to them. This is particularly
   * useful for %custom gates, but non-%custom gates can also have such data to
   * augment their behavior. The difference in that case is that %custom gates
   * must be rejected by downstream plugins that don't support them, while the
   * arbitrary dats for augmented %unitary and %measurement gates may be
   * ignored. To construct a gate with arbitrary data, use one of the
   * constructor overloads followed by the `ArbData` builder pattern functions,
   * for instance:
   *
   * \code
   * Gate::custom(
   *   "foobar", // name
   *   QubitSet().with(1_q) // target
   * ).with_json_string("{\"hello\": \"world\"}")
   *  .with_arg<int>(33)
   *  .with_arg_string("I'm a string!");
   * \endcode
   */
  class Gate : public Arb {
  public:

    /**
     * Wraps the given `Gate` handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    Gate(HandleIndex handle) noexcept : Arb(handle) {
    }

    // Delete copy construct/assign.
    Gate(const Gate&) = delete;
    void operator=(const Gate&) = delete;

    /**
     * Default move constructor.
     */
    Gate(Gate&&) = default;

    /**
     * Default move assignment.
     */
    Gate &operator=(Gate&&) = default;

    /**
     * Constructs a new unitary gate.
     *
     * \param targets A qubit reference set with the target qubits.
     * \param matrix The matrix to be applied to the target qubits. It must be
     * appropriately sized for the number of target qubits (2^n by 2^n).
     * \returns The requested unitary gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate unitary(QubitSet &&targets, const Matrix &matrix) {
      return Gate(check(raw::dqcs_gate_new_unitary(
        targets.get_handle(),
        0,
        matrix.get_handle()
      )));
    }

    /**
     * Constructs a new unitary gate.
     *
     * \param targets A qubit reference set with the target qubits, passed by
     * copy.
     * \param matrix The matrix to be applied to the target qubits. It must be
     * appropriately sized for the number of target qubits (2^n by 2^n).
     * \returns The requested unitary gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate unitary(const QubitSet &targets, const Matrix &matrix) {
      return unitary(QubitSet(targets), matrix);
    }

    /**
     * Constructs a new unitary gate with control qubits.
     *
     * \param targets A qubit reference set with the target qubits.
     * \param controls A qubit reference set with the target qubits. The
     * control qubits are not represented in the matrix; the backend will
     * supplement it as needed. The `targets` and `controls` qubit sets must be
     * disjoint.
     * \param matrix The matrix to be applied to the target qubits. It must be
     * appropriately sized for the number of target qubits (2^n by 2^n).
     * \returns The requested unitary gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate unitary(QubitSet &&targets, QubitSet &&controls, const Matrix &matrix) {
      return Gate(check(raw::dqcs_gate_new_unitary(
        targets.get_handle(),
        controls.get_handle(),
        matrix.get_handle()
      )));
    }

    /**
     * Constructs a new unitary gate with control qubits.
     *
     * \param targets A qubit reference set with the target qubits, passed by
     * copy.
     * \param controls A qubit reference set with the target qubits, passed by
     * copy. The control qubits are not represented in the matrix; the backend
     * will supplement it as needed. The `targets` and `controls` qubit sets
     * must be disjoint.
     * \param matrix The matrix to be applied to the target qubits. It must be
     * appropriately sized for the number of target qubits (2^n by 2^n).
     * \returns The requested unitary gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate unitary(const QubitSet &targets, const QubitSet &controls, const Matrix &matrix) {
      return unitary(QubitSet(targets), QubitSet(controls), matrix);
    }

    /**
     * Constructs a new Z-axis measurement gate.
     *
     * \param measures A qubit reference set with the to-be-measured qubits.
     * The measurement results can be queried from `PluginState` after the gate
     * is executed. Any previous measurement results for those qubits will be
     * overridden.
     * \returns The requested measurement gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate measure(QubitSet &&measures) {
      return Gate(check(raw::dqcs_gate_new_measurement(measures.get_handle())));
    }

    /**
     * Constructs a new Z-axis measurement gate.
     *
     * \param measures A qubit reference set with the to-be-measured qubits,
     * passed by copy. The measurement results can be queried from
     * `PluginState` after the gate is executed. Any previous measurement
     * results for those qubits will be overridden.
     * \returns The requested measurement gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate measure(const QubitSet &measures) {
      return measure(QubitSet(measures));
    }

    /**
     * Constructs a new custom gate with target qubits, control qubits,
     * measured qubits, and a matrix.
     *
     * \param name A name identifying the custom gate. Which gates are
     * available is determined by the backend.
     * \param targets A qubit reference set with the target qubits.
     * \param controls A qubit reference set with the target qubits. The
     * control qubits are not represented in the matrix; the backend will
     * supplement it as needed. The `targets` and `controls` qubit sets must be
     * disjoint.
     * \param measures A qubit reference set with to-be-measured qubits. The
     * measurement results can be queried from `PluginState` after the gate
     * is executed. Any previous measurement results for those qubits will be
     * overridden.
     * \param matrix The matrix to be applied to the target qubits. It must be
     * appropriately sized for the number of target qubits (2^n by 2^n).
     * \returns The requested custom gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate custom(
      const std::string &name,
      QubitSet &&targets,
      QubitSet &&controls,
      QubitSet &&measures,
      const Matrix &matrix
    ) {
      return Gate(check(raw::dqcs_gate_new_custom(
        name.c_str(),
        targets.get_handle(),
        controls.get_handle(),
        measures.get_handle(),
        matrix.get_handle()
      )));
    }

    /**
     * Constructs a new custom gate with target qubits, control qubits,
     * measured qubits, and a matrix.
     *
     * \param name A name identifying the custom gate. Which gates are
     * available is determined by the backend.
     * \param targets A qubit reference set with the target qubits, passed by
     * copy.
     * \param controls A qubit reference set with the target qubits, passed by
     * copy. The control qubits are not represented in the matrix; the backend
     * will supplement it as needed. The `targets` and `controls` qubit sets
     * must be disjoint.
     * \param measures A qubit reference set with to-be-measured qubits, passed
     * by copy. The measurement results can be queried from `PluginState` after
     * the gate is executed. Any previous measurement results for those qubits
     * will be overridden.
     * \param matrix The matrix to be applied to the target qubits. It must be
     * appropriately sized for the number of target qubits (2^n by 2^n).
     * \returns The requested custom gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate custom(
      const std::string &name,
      const QubitSet &targets,
      const QubitSet &controls,
      const QubitSet &measures,
      const Matrix &matrix
    ) {
      return custom(
        name,
        QubitSet(targets),
        QubitSet(controls),
        QubitSet(measures),
        matrix
      );
    }

    /**
     * Constructs a new custom gate with target qubits, control qubits, and
     * measured qubits.
     *
     * \param name A name identifying the custom gate. Which gates are
     * available is determined by the backend.
     * \param targets A qubit reference set with the target qubits.
     * \param controls A qubit reference set with the target qubits. The
     * `targets` and `controls` qubit sets must be disjoint.
     * \param measures A qubit reference set with to-be-measured qubits. The
     * measurement results can be queried from `PluginState` after the gate
     * is executed. Any previous measurement results for those qubits will be
     * overridden.
     * \returns The requested custom gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate custom(
      const std::string &name,
      QubitSet &&targets,
      QubitSet &&controls,
      QubitSet &&measures
    ) {
      return Gate(check(raw::dqcs_gate_new_custom(
        name.c_str(),
        targets.get_handle(),
        controls.get_handle(),
        measures.get_handle(),
        0
      )));
    }

    /**
     * Constructs a new custom gate with target qubits, control qubits, and
     * measured qubits.
     *
     * \param name A name identifying the custom gate. Which gates are
     * available is determined by the backend.
     * \param targets A qubit reference set with the target qubits, passed by
     * copy.
     * \param controls A qubit reference set with the target qubits, passed by
     * copy. The `targets` and `controls` qubit sets must be disjoint.
     * \param measures A qubit reference set with to-be-measured qubits, passed
     * by copy. The measurement results can be queried from `PluginState` after
     * the gate is executed. Any previous measurement results for those qubits
     * will be overridden.
     * \returns The requested custom gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate custom(
      const std::string &name,
      const QubitSet &targets,
      const QubitSet &controls,
      const QubitSet &measures
    ) {
      return custom(
        name,
        QubitSet(targets),
        QubitSet(controls),
        QubitSet(measures)
      );
    }

    /**
     * Constructs a new custom gate with target qubits, control qubits,
     * and a matrix.
     *
     * \param name A name identifying the custom gate. Which gates are
     * available is determined by the backend.
     * \param targets A qubit reference set with the target qubits.
     * \param controls A qubit reference set with the target qubits. The
     * control qubits are not represented in the matrix; the backend will
     * supplement it as needed. The `targets` and `controls` qubit sets must be
     * disjoint.
     * \param matrix The matrix to be applied to the target qubits. It must be
     * appropriately sized for the number of target qubits (2^n by 2^n).
     * \returns The requested custom gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate custom(
      const std::string &name,
      QubitSet &&targets,
      QubitSet &&controls,
      const Matrix &matrix
    ) {
      return Gate(check(raw::dqcs_gate_new_custom(
        name.c_str(),
        targets.get_handle(),
        controls.get_handle(),
        0,
        matrix.get_handle()
      )));
    }

    /**
     * Constructs a new custom gate with target qubits, control qubits,
     * and a matrix.
     *
     * \param name A name identifying the custom gate. Which gates are
     * available is determined by the backend.
     * \param targets A qubit reference set with the target qubits, passed by
     * copy.
     * \param controls A qubit reference set with the target qubits, passed by
     * copy. The control qubits are not represented in the matrix; the backend
     * will supplement it as needed. The `targets` and `controls` qubit sets
     * must be disjoint.
     * \param matrix The matrix to be applied to the target qubits. It must be
     * appropriately sized for the number of target qubits (2^n by 2^n).
     * \returns The requested custom gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate custom(
      const std::string &name,
      const QubitSet &targets,
      const QubitSet &controls,
      const Matrix &matrix
    ) {
      return custom(
        name,
        QubitSet(targets),
        QubitSet(controls),
        matrix
      );
    }

    /**
     * Constructs a new custom gate with target qubits and control qubits.
     *
     * \param name A name identifying the custom gate. Which gates are
     * available is determined by the backend.
     * \param targets A qubit reference set with the target qubits.
     * \param controls A qubit reference set with the target qubits. The
     * `targets` and `controls` qubit sets must be disjoint.
     * \returns The requested custom gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate custom(
      const std::string &name,
      QubitSet &&targets,
      QubitSet &&controls
    ) {
      return Gate(check(raw::dqcs_gate_new_custom(
        name.c_str(),
        targets.get_handle(),
        controls.get_handle(),
        0,
        0
      )));
    }

    /**
     * Constructs a new custom gate with target qubits and control qubits.
     *
     * \param name A name identifying the custom gate. Which gates are
     * available is determined by the backend.
     * \param targets A qubit reference set with the target qubits, passed by
     * copy.
     * \param controls A qubit reference set with the target qubits, passed by
     * copy. The `targets` and `controls` qubit sets must be disjoint.
     * \returns The requested custom gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate custom(
      const std::string &name,
      const QubitSet &targets,
      const QubitSet &controls
    ) {
      return custom(
        name,
        QubitSet(targets),
        QubitSet(controls)
      );
    }

    /**
     * Constructs a new custom gate with target qubits and a matrix.
     *
     * \param name A name identifying the custom gate. Which gates are
     * available is determined by the backend.
     * \param targets A qubit reference set with the target qubits.
     * \param matrix The matrix to be applied to the target qubits. It must be
     * appropriately sized for the number of target qubits (2^n by 2^n).
     * \returns The requested custom gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate custom(
      const std::string &name,
      QubitSet &&targets,
      const Matrix &matrix
    ) {
      return Gate(check(raw::dqcs_gate_new_custom(
        name.c_str(),
        targets.get_handle(),
        0,
        0,
        matrix.get_handle()
      )));
    }

    /**
     * Constructs a new custom gate with target qubits and a matrix.
     *
     * \param name A name identifying the custom gate. Which gates are
     * available is determined by the backend.
     * \param targets A qubit reference set with the target qubits, passed by
     * copy.
     * \param matrix The matrix to be applied to the target qubits. It must be
     * appropriately sized for the number of target qubits (2^n by 2^n).
     * \returns The requested custom gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate custom(
      const std::string &name,
      const QubitSet &targets,
      const Matrix &matrix
    ) {
      return custom(
        name,
        QubitSet(targets),
        matrix
      );
    }

    /**
     * Constructs a new custom gate with only target qubits.
     *
     * \param name A name identifying the custom gate. Which gates are
     * available is determined by the backend.
     * \param targets A qubit reference set with the target qubits.
     * \returns The requested custom gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate custom(
      const std::string &name,
      QubitSet &&targets
    ) {
      return Gate(check(raw::dqcs_gate_new_custom(
        name.c_str(),
        targets.get_handle(),
        0,
        0,
        0
      )));
    }

    /**
     * Constructs a new custom gate with only target qubits.
     *
     * \param name A name identifying the custom gate. Which gates are
     * available is determined by the backend.
     * \param targets A qubit reference set with the target qubits, passed by
     * copy.
     * \returns The requested custom gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate custom(
      const std::string &name,
      const QubitSet &targets
    ) {
      return custom(
        name,
        QubitSet(targets)
      );
    }

    /**
     * Constructs a new custom gate without qubit operands.
     *
     * \param name A name identifying the custom gate. Which gates are
     * available is determined by the backend.
     * \returns The requested custom gate.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    static Gate custom(
      const std::string &name
    ) {
      return Gate(check(raw::dqcs_gate_new_custom(
        name.c_str(),
        0,
        0,
        0,
        0
      )));
    }

    /**
     * Returns a new qubit reference set with the target qubits for this gate.
     *
     * \returns A new qubit reference set with the target qubits for this gate.
     * \throws std::runtime_error When construction of the new qubit set (handle)
     * failed for some reason or the current handle is invalid.
     */
    QubitSet get_targets() const {
      return QubitSet(check(raw::dqcs_gate_targets(handle)));
    }

    /**
     * Returns whether this gate has target qubits.
     *
     * \returns Whether this gate has target qubits.
     * \throws std::runtime_error When the current handle is invalid.
     */
    bool has_targets() const {
      return check(raw::dqcs_gate_has_targets(handle));
    }

    /**
     * Returns a new qubit reference set with the control qubits for this gate.
     *
     * \returns A new qubit reference set with the control qubits for this gate.
     * \throws std::runtime_error When construction of the new qubit set (handle)
     * failed for some reason or the current handle is invalid.
     */
    QubitSet get_controls() const {
      return QubitSet(check(raw::dqcs_gate_controls(handle)));
    }

    /**
     * Returns whether this gate has control qubits.
     *
     * \returns Whether this gate has control qubits.
     * \throws std::runtime_error When the current handle is invalid.
     */
    bool has_controls() const {
      return check(raw::dqcs_gate_has_controls(handle));
    }

    /**
     * Returns a new qubit reference set with the measurement qubits for this
     * gate.
     *
     * \returns A new qubit reference set with the measurement qubits for this
     * gate.
     * \throws std::runtime_error When construction of the new qubit set (handle)
     * failed for some reason or the current handle is invalid.
     */
    QubitSet get_measures() const {
      return QubitSet(check(raw::dqcs_gate_measures(handle)));
    }

    /**
     * Returns whether this gate has measurement qubits.
     *
     * \returns Whether this gate has measurement qubits.
     * \throws std::runtime_error When the current handle is invalid.
     */
    bool has_measures() const {
      return check(raw::dqcs_gate_has_measures(handle));
    }

    /**
     * Returns the matrix that belongs to this gate.
     *
     * \returns The matrix that belongs to this gate.
     * \throws std::runtime_error When the current handle is invalid or the
     * gate doesn't have a matrix.
     */
    Matrix get_matrix() const {
      return Matrix(check(raw::dqcs_gate_matrix(handle)));
    }

    /**
     * Returns whether this gate has a matrix.
     *
     * \returns Whether this gate has a matrix.
     * \throws std::runtime_error When the current handle is invalid.
     */
    bool has_matrix() const {
      return check(raw::dqcs_gate_has_matrix(handle));
    }

    /**
     * Returns the name of a custom gate.
     *
     * \returns The name of a custom gate.
     * \throws std::runtime_error When the current handle is invalid.
     */
    std::string get_name() const {
      char *data = check(raw::dqcs_gate_name(handle));
      std::string name(data);
      std::free(data);
      return name;
    }

    /**
     * Returns whether this gate is a custom gate.
     *
     * \returns Whether this gate is a custom gate.
     * \throws std::runtime_error When the current handle is invalid.
     */
    bool is_custom() const {
      return check(raw::dqcs_gate_is_custom(handle));
    }

    // Include `ArbData` builder pattern functions.
    /**
     * Helper macro to prevent code repetition; not visible outside of the header.
     */
    #define ARB_BUILDER_SUBCLASS Gate
    #include "arb_builder.hpp"

  };

  /**
   * Class representation of the measurement result for a single qubit.
   *
   * Measurement objects carry the following information:
   *
   *  - which qubit was measured;
   *  - what the measured state was (zero, one, or undefined);
   *  - optional arbitrary data.
   *
   * You can construct the arbitrary data part of a measurement object with the
   * builder pattern, for instance:
   *
   * \code
   * Measurement(1_q, MeasurementValue::One)
   *    .with_json_string("{\"hello\": \"world\"}")
   *    .with_arg<int>(33)
   *    .with_arg_string("I'm a string!");
   * \endcode
   */
  class Measurement : public Arb {
  public:

    /**
     * Wraps the given measurement handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    Measurement(HandleIndex handle) noexcept : Arb(handle) {
    }

    /**
     * Constructs a measurement object.
     *
     * \param qubit The qubit reference that this measurement belongs to.
     * \param value The measurement value.
     * \throws std::runtime_error When construction of the new handle failed
     * for some reason.
     */
    Measurement(const QubitRef &qubit, MeasurementValue value) : Arb(check(
      raw::dqcs_meas_new(qubit.get_index(), to_raw(value))
    )) {
    }

    /**
     * Copy-constructs a `Measurement` object.
     *
     * \param src The measurement object to copy from.
     * \throws std::runtime_error When the source handle is invalid or
     * construction of the new handle failed for some reason.
     */
    Measurement(const Measurement &src) : Arb(check(
      raw::dqcs_meas_new(src.get_qubit().get_index(), to_raw(src.get_value()))
    )) {
      set_arb(src);
    }

    /**
     * Copy assignment operator for `Measurement` objects.
     *
     * \param src The measurement object to copy from.
     * \throws std::runtime_error When the source handle is invalid or
     * construction of the new handle failed for some reason.
     */
    void operator=(const Measurement &src) {
      set_qubit(src.get_qubit());
      set_value(src.get_value());
      set_arb(src);
    }

    /**
     * Default move constructor.
     */
    Measurement(Measurement &&handle) = default;

    /**
     * Default move assignment.
     */
    Measurement &operator=(Measurement&&) = default;

    /**
     * Returns the measurement value.
     *
     * \returns The measurement value.
     * \throws std::runtime_error When the current handle is invalid.
     */
    MeasurementValue get_value() const {
      return check(raw::dqcs_meas_value_get(handle));
    }

    /**
     * Sets the measurement value.
     *
     * \param value The new measurement value.
     * \throws std::runtime_error When the current handle is invalid.
     */
    void set_value(MeasurementValue value) {
      check(raw::dqcs_meas_value_set(handle, to_raw(value)));
    }

    /**
     * Returns the qubit reference associated with this measurement.
     *
     * \returns The qubit reference associated with this measurement.
     * \throws std::runtime_error When the current handle is invalid.
     */
    QubitRef get_qubit() const {
      return QubitRef(check(raw::dqcs_meas_qubit_get(handle)));
    }

    /**
     * Sets the qubit reference associated with this measurement.
     *
     * \param qubit The new qubit reference.
     * \throws std::runtime_error When the current handle is invalid.
     */
    void set_qubit(QubitRef qubit) {
      check(raw::dqcs_meas_qubit_set(handle, qubit.get_index()));
    }

    // Include `ArbData` builder pattern functions.
    /**
     * Helper macro to prevent code repetition; not visible outside of the header.
     */
    #define ARB_BUILDER_SUBCLASS Measurement
    #include "arb_builder.hpp"

  };

  /**
   * Represents a set of measurements.
   *
   * Measurement sets contain `Measurement` data for zero or more qubits.
   */
  class MeasurementSet : public Handle {
  public:

    /**
     * Wraps the given measurement set handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    MeasurementSet(HandleIndex handle) noexcept : Handle(handle) {
    }

    /**
     * Constructs an empty measurement set.
     */
    MeasurementSet() : Handle(check(raw::dqcs_mset_new())) {
    }

    /**
     * Constructs a measurement set object from an iterable of measurements.
     *
     * \param measurements An object that can be iterated over, capable of
     * yielding `const Measurement&`s.
     * \returns The new measurement set.
     * \throws std::runtime_error When construction of the measurement set
     * (handle) failed for some reason or any measurement handle within the
     * `measurements` iterable is invalid.
     */
    template <class T>
    static MeasurementSet from_iter(const T &measurements) {
      MeasurementSet result;
      for (const Measurement &measurement : measurements) {
        result.set(measurement);
      }
      return result;
    }

    /**
     * Moves the given measurement object into the set. If the set already
     * contained measurement data for the qubit associated with the measurement
     * object, the previous measurement data is overwritten.
     *
     * \param measurement The measurement object to move into the set.
     * \throws std::runtime_error When the measurement handle or the current
     * handle is invalid.
     */
    void set(Measurement &&measurement) {
      check(raw::dqcs_mset_set(handle, measurement.get_handle()));
    }

    /**
     * Copies the given measurement object into the set. If the set already
     * contained measurement data for the qubit associated with the measurement
     * object, the previous measurement data is overwritten.
     *
     * \param measurement The measurement object to copy into the set.
     * \throws std::runtime_error When the measurement handle or the current
     * handle is invalid.
     */
    void set(const Measurement &measurement) {
      set(Measurement(measurement));
    }

    /**
     * Moves the given measurement object into the set (builder pattern). If
     * the set already contained measurement data for the qubit associated with
     * the measurement object, the previous measurement data is overwritten.
     *
     * \param measurement The measurement object to move into the set.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the measurement handle or the current
     * handle is invalid.
     */
    MeasurementSet &&with(Measurement &&measurement) {
      set(std::move(measurement));
      return std::move(*this);
    }

    /**
     * Copies the given measurement object into the set (builder pattern). If
     * the set already contained measurement data for the qubit associated with
     * the measurement object, the previous measurement data is overwritten.
     *
     * \param measurement The measurement object to copy into the set.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the measurement handle or the current
     * handle is invalid.
     */
    MeasurementSet &&with(const Measurement &measurement) {
      set(measurement);
      return std::move(*this);
    }

    /**
     * Returns a copy of the measurement object for the given qubit.
     *
     * \param qubit A reference to the qubit to query the measurement result
     * for.
     * \returns The measurement result.
     * \throws std::runtime_error When the current handle is invalid,
     * construction of the new measurement handle failed, or no measurement
     * data is available for the requested qubit.
     */
    Measurement get(const QubitRef &qubit) const {
      return Measurement(check(raw::dqcs_mset_get(handle, qubit.get_index())));
    }

    /**
     * Moves the measurement object for the given qubit out of the set. An
     * exception is thrown if no data is available for this qubit.
     *
     * \param qubit A reference to the qubit to query the measurement result
     * for.
     * \returns The measurement result.
     * \throws std::runtime_error When the current handle is invalid,
     * construction of the new measurement handle failed, or no measurement
     * data is available for the requested qubit.
     */
    Measurement take(const QubitRef &qubit) {
      return Measurement(check(raw::dqcs_mset_take(handle, qubit.get_index())));
    }

    /**
     * Moves any measurement object out of the set. An exception is thrown if
     * the set is empty.
     *
     * \returns The measurement result for any qubit in the measurement result
     * set.
     * \throws std::runtime_error When the current handle is invalid, it is
     * empty, or construction of the new measurement handle failed.
     */
    Measurement take_any() {
      return Measurement(check(raw::dqcs_mset_take_any(handle)));
    }

    /**
     * Removes the measurement object for the given qubit from the set.
     *
     * \param qubit A reference to the qubit to query the measurement result
     * for.
     * \throws std::runtime_error When the current handle is invalid, or no
     * measurement data was available for the requested qubit.
     */
    void remove(const QubitRef &qubit) {
      check(raw::dqcs_mset_remove(handle, qubit.get_index()));
    }

    /**
     * Returns the number of measurements in the set.
     *
     * \returns The number of measurements in the set.
     * \throws std::runtime_error When the current handle is invalid.
     */
    size_t size() const {
      return check(raw::dqcs_mset_len(handle));
    }

    /**
     * Returns whether the set contains measurement data for the given qubit.
     *
     * \param qubit A reference to the qubit to query the measurement result
     * for.
     * \returns Whether the set contains measurement data for the given qubit.
     * \throws std::runtime_error When the current handle is invalid.
     */
    bool contains(const QubitRef &qubit) const {
      return check(raw::dqcs_mset_contains(handle, qubit.get_index()));
    }

    /**
     * Drains the measurement set into a vector.
     *
     * That is, the measurement set object remains valid, but is emptied after
     * this call.
     *
     * \returns A `std::vector` containing the individual measurement objects
     * in arbitrary order.
     * \throws std::runtime_error When the current handle is invalid or
     * construction of any of the individual measurement objects fails for some
     * reason.
     */
    std::vector<Measurement> drain_into_vector() {
      std::vector<Measurement> measurements;
      while (size()) {
        measurements.emplace_back(take_any());
      }
      return measurements;
    }

    /**
     * Copies the qubit set into a vector.
     *
     * \note This requires destructive iteration, so the function is not const;
     * if an exception occurs, the state of the measurement set may be changed.
     *
     * \returns A `std::vector` containing the individual measurement objects
     * in arbitrary order.
     * \throws std::runtime_error When the current handle is invalid or
     * construction of any of the individual measurement objects fails for some
     * reason.
     */
    std::vector<Measurement> copy_into_vector() {
      std::vector<Measurement> vector = drain_into_vector();
      MeasurementSet copy = MeasurementSet::from_iter(vector);
      free();
      handle = copy.take_handle();
      return vector;
    }

    /**
     * Copy-constructs a measurement set object.
     *
     * \note This requires destructive iteration, so the function is not const;
     * if an exception occurs, the state of the measurement set may be changed.
     *
     * \param src The object to be copied.
     * \throws std::runtime_error When the `src` handle is invalid or
     * construction of the new measurement set object fails for some reason.
     *
     */
    MeasurementSet(MeasurementSet &src) : Handle(0) {
      handle = MeasurementSet::from_iter(src.copy_into_vector()).take_handle();
    }

    /**
     * Copy-assigns a measurement set object.
     *
     * \note This requires destructive iteration, so the function is not const;
     * if an exception occurs, the state of the measurement set may be changed.
     *
     * \param src The object to be copied.
     * \throws std::runtime_error When the `src` handle or the current handle
     * is invalid.
     */
    MeasurementSet &operator=(MeasurementSet &src) {
      free();
      handle = MeasurementSet::from_iter(src.copy_into_vector()).take_handle();
      return *this;
    }

    /**
     * Default move constructor.
     */
    MeasurementSet(MeasurementSet &&handle) = default;

    /**
     * Default move assignment.
     */
    MeasurementSet &operator=(MeasurementSet&&) = default;

  };

  /**
   * Wrapper for DQCsim's internal plugin state within the context of
   * upstream-synchronous plugin callbacks (that is, the `modify_measurement`
   * callback).
   *
   * Cannot be moved or copied, as it must stay in scope of the plugin
   * callbacks. Can also not be constructed except for by the callback wrapper
   * classes.
   */
  class UpstreamPluginState {
  protected:

    /**
     * The wrapped plugin state. This is actually a void* pointing to a
     * Rust-managed object.
     */
    const raw::dqcs_plugin_state_t state;

    /**
     * Hidden constructor, only to be used by the callback wrappers.
     */
    UpstreamPluginState(raw::dqcs_plugin_state_t state) : state(state) {
    }

    // Allow the C-style callbacks to construct the plugin state wrapper.
    friend class CallbackEntryPoints;

  public:

    // Delete the copy and move constructors and assignments.
    UpstreamPluginState(const UpstreamPluginState&) = delete;
    void operator=(const UpstreamPluginState&) = delete;
    UpstreamPluginState(UpstreamPluginState&&) = delete;
    UpstreamPluginState &operator=(UpstreamPluginState&&) = delete;

    /**
     * Generates a random floating point number using the simulator random
     * seed.
     *
     * \returns A uniformly distributed floating-point number between 0
     * (inclusive) and 1 (exclusive).
     */
    double random_f64() noexcept {
      return raw::dqcs_plugin_random_f64(state);
    }

    /**
     * Generates a random integer using the simulator random seed.
     *
     * \returns A uniformly distributed unsigned 64-bit integer.
     */
    uint64_t random_u64() noexcept {
      return raw::dqcs_plugin_random_u64(state);
    }

    /**
     * Generates a random value using the simulator random seed.
     *
     * \returns A randomized value of the given type. All bits are randomized
     * with a 50/50 probability.
     *
     * \warning This function is not sensible for every kind of type, but will
     * probably "work" regardless; use with care.
     */
    template <typename T>
    T random() noexcept {
      uint64_t data[(sizeof(T) + 7) / 8];
      for (size_t i = 0; i < (sizeof(T) + 7) / 8; i++) {
        data = random_u64();
      }
      T retval;
      std::memcpy(&retval, data, sizeof(T));
      return retval;
    }

  };

  /**
   * Wrapper for DQCsim's internal plugin state within the context of
   * downstream-synchronous plugin callbacks.
   *
   * Cannot be moved or copied, as it must stay in scope of the plugin
   * callbacks. Can also not be constructed except for by the callback wrapper
   * classes.
   */
  class PluginState : public UpstreamPluginState {
  protected:

    /**
     * Hidden constructor, only to be used by the callback wrappers.
     */
    PluginState(raw::dqcs_plugin_state_t state) : UpstreamPluginState(state) {
    }

    // Allow the C-style callbacks to construct the plugin state wrapper.
    friend class CallbackEntryPoints;

  public:

    // Delete the copy and move constructors and assignments.
    PluginState(const PluginState&) = delete;
    void operator=(const PluginState&) = delete;
    PluginState(PluginState&&) = delete;
    PluginState &operator=(PluginState&&) = delete;

    /**
     * Allocates a number of downstream qubits, copying in the given command
     * queue as arbitrary additional data for the qubits.
     *
     * \param num_qubits The number of qubits to allocate.
     * \param cmds A command queue with zero or more commands to apply to the
     * qubits during their initialization. The significance of these commands
     * is dependent on the downstream plugin.
     * \returns A qubit set with references to the newly allocated qubits.
     * \throws std::runtime_error When the command queue is invalid,
     * construction of the qubit set fails for some reason, an asynchronous
     * exception is received, or this is called by a backend plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    QubitSet allocate(size_t num_qubits, ArbCmdQueue &&cmds) {
      return QubitSet(check(raw::dqcs_plugin_allocate(state, num_qubits, cmds.get_handle())));
    }

    /**
     * Allocates a number of downstream qubits, moving in the given command
     * queue as arbitrary additional data for the qubits.
     *
     * \param num_qubits The number of qubits to allocate.
     * \param cmds A command queue with zero or more commands to apply to the
     * qubits during their initialization. The significance of these commands
     * is dependent on the downstream plugin.
     * \returns A qubit set with references to the newly allocated qubits.
     * \throws std::runtime_error When the command queue is invalid,
     * construction of the qubit set fails for some reason, an asynchronous
     * exception is received, or this is called by a backend plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    QubitSet allocate(size_t num_qubits, ArbCmdQueue &cmds) {
      return allocate(num_qubits, ArbCmdQueue(cmds));
    }

    /**
     * Allocates a number of default downstream qubits.
     *
     * \param num_qubits The number of qubits to allocate.
     * \returns A qubit set with references to the newly allocated qubits.
     * \throws std::runtime_error When construction of the qubit set fails for
     * some reason, an asynchronous exception is received, or this is called by
     * a backend plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    QubitSet allocate(size_t num_qubits) {
      return QubitSet(check(raw::dqcs_plugin_allocate(state, num_qubits, 0)));
    }

    /**
     * Allocates a single downstream qubit, copying in the given command queue
     * as arbitrary additional data for the qubits.
     *
     * \param cmds A command queue with zero or more commands to apply to the
     * qubit during its initialization. The significance of these commands
     * is dependent on the downstream plugin.
     * \returns A reference to the newly allocated qubit.
     * \throws std::runtime_error When the command queue is invalid, an
     * asynchronous exception is received, or this is called by a backend
     * plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    QubitRef allocate(ArbCmdQueue &&cmds) {
      return allocate(1, std::move(cmds)).pop();
    }

    /**
     * Allocates a single downstream qubit, moving in the given command queue
     * as arbitrary additional data for the qubits.
     *
     * \param cmds A command queue with zero or more commands to apply to the
     * qubit during its initialization. The significance of these commands
     * is dependent on the downstream plugin.
     * \returns A reference to the newly allocated qubit.
     * \throws std::runtime_error When the command queue is invalid, an
     * asynchronous exception is received, or this is called by a backend
     * plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    QubitRef allocate(ArbCmdQueue &cmds) {
      return allocate(ArbCmdQueue(cmds));
    }

    /**
     * Allocates a single downstream qubit.
     *
     * \returns A reference to the newly allocated qubit.
     * \throws std::runtime_error When an asynchronous exception is received
     * or this is called by a backend plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    QubitRef allocate() {
      return allocate(1).pop();
    }

    /**
     * Frees the given downstream qubits.
     *
     * \param qubits The list of qubits to free, passed by move.
     * \throws std::runtime_error When the qubit set handle is invalid, an
     * asynchronous exception is received, or this is called by a backend
     * plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    void free(QubitSet &&qubits) {
      check(raw::dqcs_plugin_free(state, qubits.get_handle()));
    }

    /**
     * Frees the given downstream qubits.
     *
     * \param qubits The list of qubits to free, passed by copy.
     * \throws std::runtime_error When the qubit set handle is invalid, an
     * asynchronous exception is received, or this is called by a backend
     * plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    void free(const QubitSet &qubits) {
      free(QubitSet(qubits));
    }

    /**
     * Frees the given downstream qubit.
     *
     * \param qubit The qubit to free.
     * \throws std::runtime_error When an asynchronous exception is received
     * or this is called by a backend plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    void free(const QubitRef &qubit) {
      free(QubitSet().with(qubit));
    }

    /**
     * Sends a gate to the downstream plugin.
     *
     * \param gate The gate to send.
     * \throws std::runtime_error When the gate handle is invalid, an
     * asynchronous exception is received, or this is called by a backend
     * plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    void gate(Gate &&gate) {
      check(raw::dqcs_plugin_gate(state, gate.get_handle()));
    }

    /**
     * Shorthand for sending a single-qubit gate to the downstream plugin.
     *
     * \param matrix The gate matrix. Must be 2x2 in size.
     * \param q The qubit to operate on.
     * \throws std::runtime_error When an asynchronous exception is received
     * or this is called by a backend plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    void gate(const Matrix &matrix, const QubitRef &q) {
      if (matrix.size() == 2) {
        gate(Gate::unitary(QubitSet().with(q), matrix));
      } else {
        throw std::invalid_argument("matrix has incorrect size");
      }
    }

    /**
     * Shorthand for sending a two-qubit gate to the downstream plugin.
     *
     * \param matrix The gate matrix. It may be 2x2 (one-qubit gate) or 4x4
     * (two-qubit gate) in size. If it is 2x2, `qa` is used as an implicit
     * control qubit while `qb` is the target; for instance, `gate(X, a, b)`
     * represents a CNOT with `a` as control and `b` as target.
     * \param qa The first qubit argument.
     * \param qb The second qubit argument.
     * \throws std::runtime_error When an asynchronous exception is received
     * or this is called by a backend plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    void gate(const Matrix &matrix, const QubitRef &qa, const QubitRef &qb) {
      if (matrix.size() == 2) {
        gate(Gate::unitary(QubitSet().with(qb), QubitSet().with(qa), matrix));
      } else if (matrix.size() == 4) {
        gate(Gate::unitary(QubitSet().with(qa).with(qb), matrix));
      } else {
        throw std::invalid_argument("matrix has incorrect size");
      }
    }

    /**
     * Shorthand for sending a three-qubit gate to the downstream plugin.
     *
     * The matrix may be 2x2 (one-qubit gate), 4x4 (two-qubit gate), or 8x8
     * (three-qubit gate) in size. If it is 2x2, `qa` and `qb` are used as
     * control qubits and `qc` is the target. If it is 4x4, `qa` is used as
     * a control qubit and `qb` and `qc` are the targets.
     *
     * \param matrix The gate matrix. It may be 2x2 (one-qubit gate), 4x4
     * (two-qubit gate), or 8x8 (three-qubit gate) in size. If it is 2x2,
     * `qa` and `qb` are used as control qubits and `qc` is the target. If it
     * is 4x4, `qa` is used as a control qubit and `qb` and `qc` are the
     * targets. For instance, `gate(X, a, b, c)` represents a Toffoli gate with
     * `a` and `b` as controls and `c` as target, and `gate(SWAP, a, b, c)`
     * represents a Fredkin gate with `a` as control and `b` and `c` as
     * targets.
     * \param qa The first qubit argument.
     * \param qb The second qubit argument.
     * \param qc The third qubit argument.
     * \throws std::runtime_error When an asynchronous exception is received
     * or this is called by a backend plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    void gate(const Matrix &matrix, const QubitRef &qa, const QubitRef &qb, const QubitRef &qc) {
      if (matrix.size() == 2) {
        gate(Gate::unitary(QubitSet().with(qc), QubitSet().with(qa).with(qb), matrix));
      } else if (matrix.size() == 4) {
        gate(Gate::unitary(QubitSet().with(qb).with(qc), QubitSet().with(qa), matrix));
      } else if (matrix.size() == 8) {
        gate(Gate::unitary(QubitSet().with(qa).with(qb).with(qc), matrix));
      } else {
        throw std::invalid_argument("matrix has incorrect size");
      }
    }

    /**
     * Shorthand for sending a single-qubit X-axis measurement to the
     * downstream plugin.
     *
     * This actually sends the following gates to the downstream plugin for
     * each qubit:
     *
     * ```C++
     * gate(GateMatrix::H(), q);
     * measure_z(q);
     * gate(GateMatrix::H(), q);
     * ```
     *
     * \param q The qubit to measure.
     * \throws std::runtime_error When an asynchronous exception is received
     * or this is called by a backend plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    void measure_x(const QubitRef &q) {
      gate(GateMatrix::H(), q);
      measure_z(q);
      gate(GateMatrix::H(), q);
    }

    /**
     * Shorthand for sending a multi-qubit X-axis measurement to the downstream
     * plugin.
     *
     * This actually sends the following gates to the downstream plugin for
     * each qubit:
     *
     * ```C++
     * for (q : qs) gate(GateMatrix::H(), q);
     * measure_z(qs);
     * for (q : qs) gate(GateMatrix::H(), q);
     * ```
     *
     * \param qs The qubits to measure.
     * \throws std::runtime_error When the qubit set handle is invalid, an
     * asynchronous exception is received, or this is called by a backend
     * plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    void measure_x(const QubitSet &qs) {
      auto qs_vec = qs.copy_into_vector();
      for (auto &q : qs_vec) gate(GateMatrix::H(), q);
      measure_z(qs);
      for (auto &q : qs_vec) gate(GateMatrix::H(), q);
    }

    /**
     * Shorthand for sending a single-qubit Y-axis measurement to the
     * downstream plugin.
     *
     * This actually sends the following gates to the downstream plugin for
     * each qubit:
     *
     * ```C++
     * gate(GateMatrix::S(), q);
     * gate(GateMatrix::Z(), q);
     * measure_z(q);
     * gate(GateMatrix::S(), q);
     * ```
     *
     * \param q The qubit to measure.
     * \throws std::runtime_error When an asynchronous exception is received
     * or this is called by a backend plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    void measure_y(const QubitRef &q) {
      gate(GateMatrix::S(), q);
      gate(GateMatrix::Z(), q);
      measure_z(q);
      gate(GateMatrix::S(), q);
    }

    /**
     * Shorthand for sending a multi-qubit Y-axis measurement to the downstream
     * plugin.
     *
     * This actually sends the following gates to the downstream plugin for
     * each qubit:
     *
     * ```C++
     * for (q : qs) gate(GateMatrix::S(), q);
     * for (q : qs) gate(GateMatrix::Z(), q);
     * measure_z(qs);
     * for (q : qs) gate(GateMatrix::S(), q);
     * ```
     *
     * \param qs The qubits to measure.
     * \throws std::runtime_error When the qubit set handle is invalid, an
     * asynchronous exception is received, or this is called by a backend
     * plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    void measure_y(const QubitSet &qs) {
      auto qs_vec = qs.copy_into_vector();
      for (auto &q : qs_vec) gate(GateMatrix::S(), q);
      for (auto &q : qs_vec) gate(GateMatrix::Z(), q);
      measure_z(qs);
      for (auto &q : qs_vec) gate(GateMatrix::S(), q);
    }

    /**
     * Shorthand for sending a single-qubit Z-axis measurement to the
     * downstream plugin.
     *
     * \param q The qubit to measure.
     * \throws std::runtime_error When an asynchronous exception is received,
     * or this is called by a backend plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    void measure_z(const QubitRef &q) {
      measure_z(std::move(QubitSet().with(q)));
    }

    /**
     * Shorthand for sending a multi-qubit Z-axis measurement to the downstream
     * plugin.
     *
     * \param qs The qubits to measure.
     * \throws std::runtime_error When the qubit set handle is invalid, an
     * asynchronous exception is received, or this is called by a backend
     * plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    void measure_z(QubitSet &&qs) {
      gate(Gate::measure(std::move(qs)));
    }

    /**
     * Shorthand for sending a multi-qubit Z-axis measurement to the downstream
     * plugin.
     *
     * \param qs The qubits to measure.
     * \throws std::runtime_error When the qubit set handle is invalid, an
     * asynchronous exception is received, or this is called by a backend
     * plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    void measure_z(const QubitSet &qs) {
      measure_z(QubitSet(qs));
    }

    /**
     * Tells the downstream plugin to run for the specified number of cycles.
     *
     * \param cycles The number of cycles to advance by. Must be positive.
     * \returns The return value is the new cycle counter.
     * \throws std::runtime_error When number of cycles is negative, an
     * asynchronous exception is received, or this is called by a backend plugin.
     *
     * \note This function is implemented asynchronously for multiprocessing
     * performance reasons. Therefore, any exception thrown by the downstream
     * plugin will not be (immediately) visible.
     */
    Cycle advance(Cycle cycles) {
      return check(raw::dqcs_plugin_advance(state, cycles));
    }

    /**
     * Sends an arbitrary command downstream.
     *
     * \param cmd The command to send to the downstream plugin.
     * \returns The `ArbData` object returned by the downstream plugin.
     * \throws std::runtime_error When the command fails, an asynchronous
     * exception is received, or this is called by a backend plugin.
     */
    ArbData arb(ArbCmd &&cmd) {
      return ArbData(check(raw::dqcs_plugin_arb(state, cmd.get_handle())));
    }

    /**
     * Sends an arbitrary command downstream.
     *
     * \param cmd The command to send to the downstream plugin.
     * \returns The `ArbData` object returned by the downstream plugin.
     * \throws std::runtime_error When the command fails, an asynchronous
     * exception is received, or this is called by a backend plugin.
     */
    ArbData arb(const ArbCmd &cmd) {
      return arb(ArbCmd(cmd));
    }

    /**
     * Returns the latest measurement of the given downstream qubit.
     *
     * \param qubit The qubit to return the latest measurement for.
     * \returns The latest measurement result.
     * \throws std::runtime_error When no data is known for the given qubit,
     * measurement object construction fails, or this is called by a backend
     * plugin.
     */
    Measurement get_measurement(const QubitRef &qubit) {
      return Measurement(check(raw::dqcs_plugin_get_measurement(state, qubit.get_index())));
    }

    /**
     * Returns the number of downstream cycles since the latest measurement of
     * the given downstream qubit.
     *
     * \param qubit The qubit to return the cycle count for.
     * \returns The number of downstream cycles since the latest measurement.
     * \throws std::runtime_error When no data is known for the given qubit or
     * this is called by a backend plugin.
     */
    Cycle get_cycles_since_measure(const QubitRef &qubit) {
      return check(raw::dqcs_plugin_get_cycles_since_measure(state, qubit.get_index()));
    }

    /**
     * Returns the number of downstream cycles between the last two
     * measurements of the given downstream qubit.
     *
     * \param qubit The qubit to return the cycle count for.
     * \returns The number of downstream cycles between the previous
     * measurement and the one before.
     * \throws std::runtime_error When no data is known for the given qubit or
     * this is called by a backend plugin.
     */
    Cycle get_cycles_between_measures(const QubitRef &qubit) {
      return check(raw::dqcs_plugin_get_cycles_between_measures(state, qubit.get_index()));
    }

    /**
     * Returns the current value of the downstream cycle counter.
     *
     * \returns The number downstream simulation cycle counter value.
     * \throws std::runtime_error When this is called by a backend plugin.
     */
    Cycle get_cycle() {
      return check(raw::dqcs_plugin_get_cycle(state));
    }

  };

  /**
   * Wrapper for DQCsim's internal plugin state within the context of the `run`
   * callback in a frontend.
   *
   * Cannot be moved or copied, as it must stay in scope of the plugin
   * callbacks. Can also not be constructed except for by the callback wrapper
   * classes.
   */
  class RunningPluginState : public PluginState {
  protected:

    /**
     * Hidden constructor, only to be used by the callback wrappers.
     */
    RunningPluginState(raw::dqcs_plugin_state_t state) : PluginState(state) {
    }

    // Allow the C-style callbacks to construct the plugin state wrapper.
    friend class CallbackEntryPoints;

  public:

    // Delete the copy and move constructors and assignments.
    RunningPluginState(const RunningPluginState&) = delete;
    void operator=(const RunningPluginState&) = delete;
    RunningPluginState(RunningPluginState&&) = delete;
    RunningPluginState &operator=(RunningPluginState&&) = delete;

    /**
     * Sends a message to the host.
     *
     * The host must do an accompanying `recv()` call, which returns the
     * data sent here. Failure to do so will result in a deadlock error to
     * the host.
     *
     * \param message The message to send.
     * \throws std::runtime_error When the message handle is invalid or
     * delivery fails for some reason.
     */
    void send(ArbData &&message) {
      check(raw::dqcs_plugin_send(state, message.get_handle()));
    }

    /**
     * Sends a message to the host.
     *
     * The host must do an accompanying `recv()` call, which returns the
     * data sent here. Failure to do so will result in a deadlock error to
     * the host.
     *
     * \param message The message to send.
     * \throws std::runtime_error When the message handle is invalid or
     * delivery fails for some reason.
     */
    void send(const ArbData &message) {
      send(ArbData(message));
    }

    /**
     * Receives a message from the host.
     *
     * The host must do an accompanying `send()` call. The data passed to
     * this call is returned by this function. Failure to do so will result
     * in a deadlock error to the host.
     *
     * \returns The message sent by the host.
     * \throws std::runtime_error When no handle could be constructed for the
     * message or reception fails for some reason.
     */
    ArbData recv() {
      return ArbData(check(raw::dqcs_plugin_recv(state)));
    }

  };

  /**
   * Class template shared between all callback functions.
   *
   * This class is specialized for all the callbacks supported by DQCsim in the
   * `callback` namespace.
   */
  template <class R, class... Args>
  class Callback {
  private:

    /**
     * `std::bind` helper function for the `Callback` template class; one
     * argument, C-style user data.
     */
    template <class T, class S, class A>
    static std::function<S(A)> bind_first(S (*cb)(T, A), T user) {
      using namespace std::placeholders;
      return std::bind(cb, user, _1);
    }

    /**
     * `std::bind` helper function for the `Callback` template class; two
     * arguments, C-style user data.
     */
    template <class T, class S, class A, class B>
    static std::function<S(A, B)> bind_first(S (*cb)(T, A, B), T user) {
      using namespace std::placeholders;
      return std::bind(cb, user, _1, _2);
    }

    /**
     * `std::bind` helper function for the `Callback` template class; three
     * arguments, C-style user data.
     */
    template <class T, class S, class A, class B, class C>
    static std::function<S(A, B, C)> bind_first(S (*cb)(T, A, B, C), T user) {
      using namespace std::placeholders;
      return std::bind(cb, user, _1, _2, _3);
    }

    /**
     * `std::bind` helper function for the `Callback` template class; one
     * argument, member function.
     */
    template <class T, class S, class A>
    static std::function<S(A)> bind_instance(T *instance, S (T::*cb)(A)) {
      using namespace std::placeholders;
      return std::bind(cb, instance, _1);
    }

    /**
     * `std::bind` helper function for the `Callback` template class; two
     * arguments, member function.
     */
    template <class T, class S, class A, class B>
    static std::function<S(A, B)> bind_instance(T *instance, S (T::*cb)(A, B)) {
      using namespace std::placeholders;
      return std::bind(cb, instance, _1, _2);
    }

    /**
     * `std::bind` helper function for the `Callback` template class; three
     * arguments, member function.
     */
    template <class T, class S, class A, class B, class C>
    static std::function<S(A, B, C)> bind_instance(T *instance, S (T::*cb)(A, B, C)) {
      using namespace std::placeholders;
      return std::bind(cb, instance, _1, _2, _3);
    }

    /**
     * The stored callback.
     */
    std::shared_ptr<std::function<R(Args...)>> cb;

    // Allow the C-style callbacks access to this class.
    friend class CallbackEntryPoints;

  public:

    /**
     * Constructs the callback wrapper from a regular C-style function.
     *
     * \param cb The C function pointer to wrap.
     */
    Callback(R (*cb)(Args...)) noexcept
      : cb(std::make_shared<std::function<R(Args...)>>(cb))
    {}

    /**
     * Constructs the callback wrapper from a regular C-style function with a
     * user argument bound to it. Usually this would be a pointer to some
     * shared data structure containing the user's plugin state.
     *
     * \param cb The C function pointer to wrap. When it is called, the first
     * argument is always set to whatever is passed to `user` here.
     * \param user The data to pass to the first argument of `cb`.
     */
    template <class T>
    Callback(R (*cb)(T, Args...), T user) noexcept
      : cb(std::make_shared<std::function<R(Args...)>>(bind_first<T, R, Args...>(cb, user)))
    {}

    /**
     * Constructs the callback wrapper from a member function.
     *
     * \param instance The class instance whose member function is to be
     * wrapped.
     * \param cb The pointer to the member function that is to be wrapped.
     */
    template <class T>
    Callback(T *instance, R (T::*cb)(Args...)) noexcept
      : cb(std::make_shared<std::function<R(Args...)>>(bind_instance<T, R, Args...>(instance, cb)))
    {}

    /**
     * Constructs the callback wrapper by moving a `std::function`.
     *
     * \param cb The C++11 `std::function` to wrap.
     */
    Callback(std::function<R(Args...)> &&cb) noexcept
      : cb(std::make_shared<std::function<R(Args...)>>(std::move(cb)))
    {}

    /**
     * Constructs the callback wrapper by copying a `std::function`.
     *
     * \param cb The C++11 `std::function` to wrap.
     */
    Callback(const std::function<R(Args...)> &cb) noexcept
      : cb(std::make_shared<std::function<R(Args...)>>(cb))
    {}

    /**
     * Constructs the callback wrapper by means of moving a `shared_ptr`
     * to a `std::function`.
     *
     * \param cb A C++11 `std::shared_ptr` to the `std::function` to wrap.
     */
    Callback(std::shared_ptr<std::function<R(Args...)>> &&cb) noexcept : cb(cb) {
    }

    /**
     * Constructs the callback wrapper by means of a copying a `shared_ptr`
     * to a `std::function`.
     *
     * \param cb A C++11 `std::shared_ptr` to the `std::function` to wrap.
     */
    Callback(const std::shared_ptr<std::function<R(Args...)>> &cb) noexcept : cb(cb) {
    }

  };

  /**
   * Namespace containing all necessarily specializations of `Callback` as
   * typedefs.
   */
  namespace callback {

    /**
     * Callback wrapper specialized for the `initialize` callback.
     */
    typedef Callback<void, PluginState&, ArbCmdQueue&&> Initialize;

    /**
     * Callback wrapper specialized for the `drop` callback.
     */
    typedef Callback<void, PluginState&> Drop;

    /**
     * Callback wrapper specialized for the `run` callback.
     */
    typedef Callback<ArbData, RunningPluginState&, ArbData&&> Run;

    /**
     * Callback wrapper specialized for the `allocate` callback.
     */
    typedef Callback<void, PluginState&, QubitSet&&, ArbCmdQueue&&> Allocate;

    /**
     * Callback wrapper specialized for the `allocate` callback.
     */
    typedef Callback<void, PluginState&, QubitSet&&> Free;

    /**
     * Callback wrapper specialized for the `gate` callback.
     */
    typedef Callback<MeasurementSet, PluginState&, Gate&&> Gate;

    /**
     * Callback wrapper specialized for the `modify_measurement` callback.
     */
    typedef Callback<MeasurementSet, UpstreamPluginState&, Measurement&&> ModifyMeasurement;

    /**
     * Callback wrapper specialized for the `advance` callback.
     */
    typedef Callback<void, PluginState&, Cycle> Advance;

    /**
     * Callback wrapper specialized for the `*_arb` callbacks.
     */
    typedef Callback<ArbData, PluginState&, ArbCmd> Arb;

    /**
     * Callback wrapper specialized for the manual plugin spawning callback.
     */
    typedef Callback<void, std::string&&> SpawnPlugin;

    /**
     * Callback wrapper specialized for the simulation logging callback.
     *
     * This callback takes the following arguments:
     *
     *  - `std::string&&`: log message string, excluding metadata.
     *  - `std::string&&`: name assigned to the logger that was used to produce
     *     the message (= "dqcsim" or a plugin name).
     *  - `Loglevel`: the severity of the log message.
     *  - `std::string&&`: string representing the source of the log message, or
     *    empty when no source is known.
     *  - `std::string&&`: string containing the filename of the source that
     *     generated the message, or empty when no source is known.
     *  - `uint32_t`: line number within the aforementioned file, or 0 if not
     *    known.
     *  - `std::chrono::system_clock::time_point&&`: timestamp for the message.
     *  - `uint32_t`: PID of the generating process.
     *  - `uint64_t`: TID of the generating thread.
     *
     * If an internal log record is particularly malformed and cannot be coerced
     * into the C equivalents of the above (nul bytes in the strings, invalid
     * timestamp, whatever) the message is silently ignored.
     */
    typedef Callback<
      void,
      std::string&&,                            // message
      std::string&&,                            // logger
      Loglevel,                                 // severity
      std::string&&,                            // module
      std::string&&,                            // file
      uint32_t,                                 // line number
      std::chrono::system_clock::time_point&&,  // timestamp
      uint32_t,                                 // process ID
      uint64_t                                  // thread ID
    > Log;

  } // namespace callback

  //! \cond Doxygen_Suppress
  /**
   * Class containing the static C-style entry points for all callbacks, that
   * simply defer to the generic, user-definable C++ callbacks and manage their
   * memory.
   *
   * The library user should never do anything with these functions directly,
   * therefore they are hidden.
   */
  class CallbackEntryPoints {
  private:
    friend class Plugin;
    friend class SimulationConfiguration;
    friend class PluginConfigurationBuilder;

    /**
     * Entry point for the `initialize` callback.
     */
    static raw::dqcs_return_t initialize(
      void *user_data,
      raw::dqcs_plugin_state_t state,
      raw::dqcs_handle_t init_cmds
    ) noexcept {

      // Wrap inputs.
      callback::Initialize *cb_wrapper = reinterpret_cast<callback::Initialize*>(user_data);
      PluginState state_wrapper(state);
      ArbCmdQueue init_cmds_wrapper(init_cmds);

      // Catch exceptions thrown in the user function to convert them to
      // DQCsim's error reporting protocol.
      try {
        (*(cb_wrapper->cb))(state_wrapper, std::move(init_cmds_wrapper));
        return raw::dqcs_return_t::DQCS_SUCCESS;
      } catch (const std::exception &e) {
        raw::dqcs_error_set(e.what());
      }
      return raw::dqcs_return_t::DQCS_FAILURE;
    }

    /**
     * Entry point for the `drop` callback.
     */
    static raw::dqcs_return_t drop(
      void *user_data,
      raw::dqcs_plugin_state_t state
    ) noexcept {

      // Wrap inputs.
      callback::Drop *cb_wrapper = reinterpret_cast<callback::Drop*>(user_data);
      PluginState state_wrapper(state);

      // Catch exceptions thrown in the user function to convert them to
      // DQCsim's error reporting protocol.
      try {
        (*(cb_wrapper->cb))(state_wrapper);
        return raw::dqcs_return_t::DQCS_SUCCESS;
      } catch (const std::exception &e) {
        raw::dqcs_error_set(e.what());
      }
      return raw::dqcs_return_t::DQCS_FAILURE;
    }

    /**
     * Entry point for the `run` callback.
     */
    static raw::dqcs_handle_t run(
      void *user_data,
      raw::dqcs_plugin_state_t state,
      raw::dqcs_handle_t args
    ) noexcept {

      // Wrap inputs.
      callback::Run *cb_wrapper = reinterpret_cast<callback::Run*>(user_data);
      RunningPluginState state_wrapper(state);
      ArbData args_wrapper(args);

      // Catch exceptions thrown in the user function to convert them to
      // DQCsim's error reporting protocol.
      try {
        return (*(cb_wrapper->cb))(state_wrapper, std::move(args_wrapper)).take_handle();
      } catch (const std::exception &e) {
        raw::dqcs_error_set(e.what());
      }
      return 0;
    }

    /**
     * Entry point for the `allocate` callback.
     */
    static raw::dqcs_return_t allocate(
      void *user_data,
      raw::dqcs_plugin_state_t state,
      raw::dqcs_handle_t qubits,
      raw::dqcs_handle_t alloc_cmds
    ) noexcept {

      // Wrap inputs.
      callback::Allocate *cb_wrapper = reinterpret_cast<callback::Allocate*>(user_data);
      PluginState state_wrapper(state);
      QubitSet qubits_wrapper(qubits);
      ArbCmdQueue alloc_cmds_wrapper(alloc_cmds);

      // Catch exceptions thrown in the user function to convert them to
      // DQCsim's error reporting protocol.
      try {
        (*(cb_wrapper->cb))(state_wrapper, std::move(qubits_wrapper), std::move(alloc_cmds_wrapper));
        return raw::dqcs_return_t::DQCS_SUCCESS;
      } catch (const std::exception &e) {
        raw::dqcs_error_set(e.what());
      }
      return raw::dqcs_return_t::DQCS_FAILURE;
    }

    /**
     * Entry point for the `free` callback.
     */
    static raw::dqcs_return_t free(
      void *user_data,
      raw::dqcs_plugin_state_t state,
      raw::dqcs_handle_t qubits
    ) noexcept {

      // Wrap inputs.
      callback::Free *cb_wrapper = reinterpret_cast<callback::Free*>(user_data);
      PluginState state_wrapper(state);
      QubitSet qubits_wrapper(qubits);

      // Catch exceptions thrown in the user function to convert them to
      // DQCsim's error reporting protocol.
      try {
        (*(cb_wrapper->cb))(state_wrapper, std::move(qubits_wrapper));
        return raw::dqcs_return_t::DQCS_SUCCESS;
      } catch (const std::exception &e) {
        raw::dqcs_error_set(e.what());
      }
      return raw::dqcs_return_t::DQCS_FAILURE;
    }

    /**
     * Entry point for the `gate` callback.
     */
    static raw::dqcs_handle_t gate(
      void *user_data,
      raw::dqcs_plugin_state_t state,
      raw::dqcs_handle_t gate
    ) noexcept {

      // Wrap inputs.
      callback::Gate *cb_wrapper = reinterpret_cast<callback::Gate*>(user_data);
      PluginState state_wrapper(state);
      Gate gate_wrapper(gate);

      // Catch exceptions thrown in the user function to convert them to
      // DQCsim's error reporting protocol.
      try {
        return (*(cb_wrapper->cb))(state_wrapper, std::move(gate_wrapper)).take_handle();
      } catch (const std::exception &e) {
        raw::dqcs_error_set(e.what());
      }
      return 0;
    }

    /**
     * Entry point for the `modify_measurement` callback.
     */
    static raw::dqcs_handle_t modify_measurement(
      void *user_data,
      raw::dqcs_plugin_state_t state,
      raw::dqcs_handle_t meas
    ) noexcept {

      // Wrap inputs.
      callback::ModifyMeasurement *cb_wrapper = reinterpret_cast<callback::ModifyMeasurement*>(user_data);
      UpstreamPluginState state_wrapper(state);
      Measurement meas_wrapper(meas);

      // Catch exceptions thrown in the user function to convert them to
      // DQCsim's error reporting protocol.
      try {
        return (*(cb_wrapper->cb))(state_wrapper, std::move(meas_wrapper)).take_handle();
      } catch (const std::exception &e) {
        raw::dqcs_error_set(e.what());
      }
      return 0;
    }

    /**
     * Entry point for the `advance` callback.
     */
    static raw::dqcs_return_t advance(
      void *user_data,
      raw::dqcs_plugin_state_t state,
      Cycle cycles
    ) noexcept {

      // Wrap inputs.
      callback::Advance *cb_wrapper = reinterpret_cast<callback::Advance*>(user_data);
      PluginState state_wrapper(state);

      // Catch exceptions thrown in the user function to convert them to
      // DQCsim's error reporting protocol.
      try {
        (*(cb_wrapper->cb))(state_wrapper, cycles);
        return raw::dqcs_return_t::DQCS_SUCCESS;
      } catch (const std::exception &e) {
        raw::dqcs_error_set(e.what());
      }
      return raw::dqcs_return_t::DQCS_FAILURE;
    }

    /**
     * Entry point for the `upstream_arb` callback.
     */
    static raw::dqcs_handle_t upstream_arb(
      void *user_data,
      raw::dqcs_plugin_state_t state,
      raw::dqcs_handle_t cmd
    ) noexcept {

      // Wrap inputs.
      callback::Arb *cb_wrapper = reinterpret_cast<callback::Arb*>(user_data);
      PluginState state_wrapper(state);
      ArbCmd cmd_wrapper(cmd);

      // Catch exceptions thrown in the user function to convert them to
      // DQCsim's error reporting protocol.
      try {
        return (*(cb_wrapper->cb))(state_wrapper, std::move(cmd_wrapper)).take_handle();
      } catch (const std::exception &e) {
        raw::dqcs_error_set(e.what());
      }
      return 0;
    }

    /**
     * Entry point for the `host_arb` callback.
     */
    static raw::dqcs_handle_t host_arb(
      void *user_data,
      raw::dqcs_plugin_state_t state,
      raw::dqcs_handle_t cmd
    ) noexcept {

      // Wrap inputs.
      callback::Arb *cb_wrapper = reinterpret_cast<callback::Arb*>(user_data);
      PluginState state_wrapper(state);
      ArbCmd cmd_wrapper(cmd);

      // Catch exceptions thrown in the user function to convert them to
      // DQCsim's error reporting protocol.
      try {
        return (*(cb_wrapper->cb))(state_wrapper, std::move(cmd_wrapper)).take_handle();
      } catch (const std::exception &e) {
        raw::dqcs_error_set(e.what());
      }
      return 0;
    }

    /**
     * Entry point for the manual plugin spawning callback.
     */
    static void spawn_plugin(
      void *user_data,
      const char *simulator
    ) noexcept {

      // Wrap inputs.
      callback::SpawnPlugin *cb_wrapper = reinterpret_cast<callback::SpawnPlugin*>(user_data);
      std::string simulator_wrapper(simulator);

      // Catch exceptions thrown in the user function to convert them to
      // DQCsim's error reporting protocol.
      try {
        (*(cb_wrapper->cb))(std::move(simulator_wrapper));
      } catch (const std::exception &e) {
        DQCSIM_FATAL("DQCsim caught std::exception in plugin thread: %s", e.what());
      }
    }

    /**
     * Entry point for the simulation logging callback.
     */
    static void log(
      void *user_data,
      const char *message,
      const char *logger,
      raw::dqcs_loglevel_t level,
      const char *module,
      const char *file,
      uint32_t line,
      uint64_t time_s,
      uint32_t time_ns,
      uint32_t pid,
      uint64_t tid
    ) noexcept {

      // Wrap inputs.
      callback::Log *cb_wrapper = reinterpret_cast<callback::Log*>(user_data);

      // Catch exceptions thrown in the user function to convert them to
      // DQCsim's error reporting protocol.
      try {
        (*(cb_wrapper->cb))(
          std::string(message ? message : ""),
          std::string(logger ? logger : ""),
          check(level),
          std::string(module ? module : ""),
          std::string(file ? file : ""),
          line,
          std::chrono::system_clock::time_point(
            std::chrono::duration_cast<std::chrono::microseconds>(
              std::chrono::seconds(time_s)
              + std::chrono::nanoseconds(time_ns)
            )
          ),
          pid,
          tid
        );
      } catch (const std::exception &e) {
        std::cerr << "DQCsim caught std::exception in log callback: " << e.what() << std::endl;
      }
    }

    /**
     * Entry point for freeing callback data structures.
     */
    template <class T>
    static void user_free(void *user_data) noexcept {
      T *cb_wrapper = reinterpret_cast<T*>(user_data);
      delete cb_wrapper;
    }

  };
  //! \endcond

  /**
   * Class wrapper for plugin join handles.
   *
   * Join handles are used only by the `Plugin::start` function, which starts
   * the plugin process in a different, DQCsim-controlled thread. You can then
   * use this object to wait for the thread to terminate, or let the
   * destructor handle it if you want, RAII-style.
   */
  class PluginJoinHandle : public Handle {
  public:

    /**
     * Wraps the given plugin join handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    PluginJoinHandle(HandleIndex handle) noexcept : Handle(handle) {
    }

    // Delete copy construct/assign.
    PluginJoinHandle(const PluginJoinHandle&) = delete;
    void operator=(const PluginJoinHandle&) = delete;

    /**
     * Default move constructor.
     */
    PluginJoinHandle(PluginJoinHandle&&) = default;

    /**
     * Default move assignment.
     */
    PluginJoinHandle &operator=(PluginJoinHandle&&) = default;

    /**
     * Waits for the plugin to terminate.
     *
     * \throws std::runtime_error When plugin execution failed.
     */
    void wait() {
      if (handle) {
        check(raw::dqcs_plugin_wait(handle));
        take_handle();
      }
    }

  };

  /**
   * Plugin definition class.
   *
   * To make a DQCsim plugin in C++, you must construct one of these objects,
   * assign callback functions that define your plugin's functionality, and
   * then call `run` or `start`. Having done so, you can point the `dqcsim`
   * command line or a host program to your plugin's executable.
   *
   * DQCsim supports three kinds of plugins: frontends, operators, and
   * backends:
   *
   *  - Frontends deal with the microarchitecture-specific classical part of
   *    the simulation. They usually ingest an algorithm described in some other
   *    language (cQASM, OpenQASM, etc.), but the frontend can also be the
   *    algorithm itself. Ultimately, frontends produce a stream of
   *    quantum-only gates.
   *
   *  - Operators can modify the gatestream produced by the frontend or an
   *    earlier operator, before it is passed on to the backend or the next
   *    operator. This is useful for all kinds of things; monitoring for
   *    statistics, error injection, runtime map and route algorithms, and so
   *    on.
   *
   *  - Backends handle the mathematics of the quantum simulation itself,
   *    usually in a microarchitecture-agnostic way.
   *
   * Each plugin type supports a different set of callback functions to define
   * the plugin's functionality, but many of the callback functions are shared.
   * The following matrix shows which functions are required (x), optional (o),
   * and not applicable (-):
   *
   * | Callback             | Frontend  | Operator  |  Backend  |
   * |----------------------|:---------:|:---------:|:---------:|
   * | `initialize`         |     o     |     o     |     o     |
   * | `drop`               |     o     |     o     |     o     |
   * | `run`                |     x     |     -     |     -     |
   * | `allocate`           |     -     |     o     |     o     |
   * | `free`               |     -     |     o     |     o     |
   * | `gate`               |     -     |     o     |     x     |
   * | `modify_measurement` |     -     |     o     |     -     |
   * | `advance`            |     -     |     o     |     o     |
   * | `upstream_arb`       |     -     |     o     |     o     |
   * | `host_arb`           |     o     |     o     |     o     |
   *
   * These callbacks perform the following functions.
   *
   * # Initialize
   *
   * The initialize callback is to be used by plugins to initialize their
   * internal state, if any. All plugin types support it. It is always called
   * before any of the other callbacks are run, but after the downstream
   * plugins have been initialized. That means it's legal to execute downstream
   * commands here already, such as pre-allocating a number of qubits,
   * preparing their state, querying the capabilities of downstream plugins
   * through `ArbCmd`s, etc.
   *
   * Besides the common arguments, the callback receives a queue of `ArbCmd`s
   * containing user-defined initialization commands. These can be thought of
   * as parameters or command-line arguments affecting the behavior of the
   * plugin. Note that plugins should not use the actual command line arguments
   * for their own purposes, as they cannot be set with DQCsim. Rather, the
   * command-line arguments of the plugin are used by DQCsim to pass a
   * connection endpoint string used to let the plugin connect to DQCsim's
   * main process. There is one exception to this baked into DQCsim intended
   * to make running quantum algorithm files more intuitive: if such a script
   * is passed to DQCsim, DQCsim will search for a plugin based on the file
   * extension of the script, and pass the filename of the script to the plugin
   * using the second command-line argument. The search algorithm is described
   * in DQCsim's command-line help, among other places.
   *
   * If this callback is not supplied, the default behavior is no-op.
   *
   * # Drop
   *
   * The drop callback can be used to do the inverse operation of the
   * initialize callback. It is called when a plugin is gracefully terminated,
   * and is supported by all plugin types. Note that graceful termination does
   * not mean there was no error; it just means that things didn't crash hard
   * enough for DQCsim to lose the ability to send the drop command.
   *
   * It is not recommended to execute any downstream instructions at this time,
   * as any errors caused at this time will crash DQCsim. However, it will work
   * in case this is really necessary.
   *
   * If this callback is not supplied, the default behavior is no-op.
   *
   * # Run
   *
   * This is the primary callback for frontend plugins, in which all the magic
   * should happen. It must therefore be defined. The other plugin types don't
   * support it.
   *
   * It is called in response to a `start()` host API call. It can therefore be
   * called multiple times, though for most simulations it's only called once.
   * The `start()` function takes an `ArbData` object, which is passed to the
   * callback as an argument. The return value is also an `ArbData`, which is
   * returned to the host through its `wait()` API call.
   *
   * # Allocate
   *
   * The allocate callback is called when the upstream plugin requests that a
   * number of qubits be allocated. DQCsim will allocate the qubit indices
   * automatically, which are passed to the function in the form of a
   * `QubitSet`, but of course it is up to the plugin to allocate its own data
   * structures to track the state of the qubits.
   *
   * The upstream plugin can also specify an `ArbCmdQueue` to supply additional
   * information for the qubit, for instance to request that a specific error
   * model be used. Plugins are free to ignore this, as per the semantics of
   * the `ArbCmd` interface and operation identifiers.
   *
   * The default behavior for operator plugins is to pass the allocation
   * command on to the backend without modification. For backends it is no-op.
   * Frontends do not support this callback.
   *
   * \note If an operator changes the allocation scheme, for instance to
   * allocate 9 downstream qubits for each upstream qubit with the intent of
   * applying surface-9 error correction, the upstream and downstream qubit
   * indices will diverge. It is up to the operator to track the mapping
   * between these qubits, and virtual the free, gate, and modify-measurement
   * callbacks accordingly to translate between them.
   *
   * # Free
   *
   * The free callback is called when the upstream plugin requests that a
   * number of qubits be freed. After this call, DQCsim will ensure that the
   * qubit indices are never used again. Plugins may use this to collapse any
   * remaining superposition state in a random way, free up memory, or reuse
   * the memory already allocated for these qubits for new qubits allocated
   * later.
   *
   * The default behavior for operator plugins is to pass the command on to
   * the backend without modification. For backends it is no-op. Frontends do
   * not support this callback.
   *
   * # Gate
   *
   * The gate callback is called when the upstream plugin requests that a gate
   * be executed. The semantics of this are documented in in the `Gate` class.
   *
   * Backend plugins must return a measurement result set containing exactly
   * those qubits specified in the measurement set. For operators, however, the
   * story is more complicated. Let's say we want to make a silly operator that
   * inverts all measurements. The trivial way to do this would be to forward
   * the gate, query all the measurement results using
   * `PluginState::get_measurement()`, invert them, stick them in a measurement
   * result set, and return that result set. However, this approach would not
   * be very efficient, because `PluginState::get_measurement()` has to wait
   * for all downstream plugins to finish executing the gate, forcing the OS
   * to switch threads, etc. Instead, operators are allowed to return only a
   * subset (or none) of the measured qubits, as long as they return exactly
   * the remaining measurements as they arrive through the modify-measurement
   * callback.
   *
   * The default implementation for this callback for operators is to pass the
   * gate through to the downstream plugin and return an empty set of
   * measurements. Combined with the default implementation of
   * modify-measurement, this behavior is correct. Backends must virtual this
   * callback, and frontends do not support it.
   *
   * Note that for our silly example operator, the default behavior for this
   * function is actually sufficient; you'd only have to virtual the
   * modify-measurement callback in that case.
   *
   * # Modify-measurement
   *
   * This callback is called for every measurement result received from the
   * downstream plugin, and returns the measurements that should be reported to
   * the upstream plugin.
   *
   * Note that while this function is called for only a single measurement at a
   * time, it is allowed to produce a set of measurements. This allows you to
   * cancel propagation of the measurement by returning an empty vector, to
   * just modify the measurement data itself, or to generate additional
   * measurements from a single measurement. However, if you need to modify the
   * qubit references for operators that remap qubits, take care to only send
   * measurement data upstream when these were explicitly requested through the
   * associated upstream gate function's measured list. It is up to you to keep
   * the stream of measurements returned upstream consistent with the gates it
   * sent to your plugin.
   *
   * \note The results from our plugin's `PluginState::get_measurement()` and
   * friends are consistent with the results received from downstream. That is,
   * they are not affected by this function. Only the measurement retrieval
   * functions in the upstream plugin are.
   *
   * \note This callback is somewhat special in that it is not allowed to call
   * any plugin command other than logging and the pseudorandom number
   * generator functions. This is because this function is called
   * asynchronously with respect to the downstream functions, making the timing
   * of these calls non-deterministic based on operating system scheduling.
   *
   * # Advance
   *
   * This callback is called when the upstream plugin calls `advance` to
   * advance the simulation time by a number of cycles. Operators and backends
   * can use this to update their error models. The default behavior for an
   * operator is to pass the request on to the downstream plugin, while the
   * default for backends is to ignore it. Frontends don't support it.
   *
   * # Upstream-arb
   *
   * This callback is called when the upstream plugin calls `arb`. It receives
   * the `ArbCmd` as an argument, and must return an `ArbData`, which is in
   * turn returned through the upstream plugin's `arb` function.
   *
   * The default behavior for operators is to forward the command to the
   * downstream plugin. Even if you virtual this callback, you should maintain
   * this behavior for any interface identifiers unknown to you. The default
   * for backends is to ignore it. Frontends don't support it.
   *
   * # Host-arb
   *
   * This callback is called when the host sends an `ArbCmd` to the plugin. It
   * receives the `ArbCmd` as an argument, and must return an `ArbData`, which
   * is in turn returned to the host.
   *
   * All plugins support this callback. The default behavior is no-op.
   */
  class Plugin : public Handle {
  public:

    /**
     * Wraps the given plugin definition handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    Plugin(HandleIndex handle) noexcept : Handle(handle) {
    }

    /**
     * Constructs a new plugin definition object.
     *
     * \param type The type of the plugin being defined.
     * \param name Name with which the plugin class can be identified, not to
     * be confused with the instance name later.
     * \param author Name of the plugin author.
     * \param version Version information for the plugin.
     * \throws std::runtime_error When plugin definition handle construction
     * fails.
     */
    Plugin(
      PluginType type,
      const std::string &name,
      const std::string &author,
      const std::string &version
    ) : Handle(raw::dqcs_pdef_new(
      to_raw(type), name.c_str(), author.c_str(), version.c_str()
    )) {
    }

    /**
     * Shorthand for constructing a new frontend plugin.
     *
     * \param name Name with which the plugin class can be identified, not to
     * be confused with the instance name later.
     * \param author Name of the plugin author.
     * \param version Version information for the plugin.
     * \returns The constructed plugin definition object.
     * \throws std::runtime_error When plugin definition handle construction
     * fails.
     */
    static Plugin Frontend(
      const std::string &name,
      const std::string &author,
      const std::string &version
    ) {
      return Plugin(PluginType::Frontend, name, author, version);
    }

    /**
     * Shorthand for constructing a new operator plugin.
     *
     * \param name Name with which the plugin class can be identified, not to
     * be confused with the instance name later.
     * \param author Name of the plugin author.
     * \param version Version information for the plugin.
     * \returns The constructed plugin definition object.
     * \throws std::runtime_error When plugin definition handle construction
     * fails.
     */
    static Plugin Operator(
      const std::string &name,
      const std::string &author,
      const std::string &version
    ) {
      return Plugin(PluginType::Operator, name, author, version);
    }

    /**
     * Shorthand for constructing a new backend plugin.
     *
     * \param name Name with which the plugin class can be identified, not to
     * be confused with the instance name later.
     * \param author Name of the plugin author.
     * \param version Version information for the plugin.
     * \returns The constructed plugin definition object.
     * \throws std::runtime_error When plugin definition handle construction
     * fails.
     */
    static Plugin Backend(
      const std::string &name,
      const std::string &author,
      const std::string &version
    ) {
      return Plugin(PluginType::Backend, name, author, version);
    }

    // Delete copy construct/assign.
    Plugin(const Plugin&) = delete;
    void operator=(const Plugin&) = delete;

    /**
     * Default move constructor.
     */
    Plugin(Plugin&&) = default;

    /**
     * Default move assignment.
     */
    Plugin &operator=(Plugin&&) = default;

    /**
     * Returns the plugin type described by this object.
     *
     * \returns The plugin type described by this object.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    PluginType get_type() const {
      return check(raw::dqcs_pdef_type(handle));
    }

    /**
     * Returns the name of the described plugin.
     *
     * \returns The name of the described plugin class.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    std::string get_name() const {
      char *cstr = check(raw::dqcs_pdef_name(handle));
      std::string str(cstr);
      std::free(cstr);
      return str;
    }

    /**
     * Returns the author of the described plugin.
     *
     * \returns The author of the described plugin.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    std::string get_author() const {
      char *cstr = check(raw::dqcs_pdef_author(handle));
      std::string str(cstr);
      std::free(cstr);
      return str;
    }

    /**
     * Returns the version of the described plugin.
     *
     * \returns The version of the described plugin.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    std::string get_version() const {
      char *cstr = check(raw::dqcs_pdef_version(handle));
      std::string str(cstr);
      std::free(cstr);
      return str;
    }

    // Code below is generated using the following Python script:
    // print('    // Code below is generated using the following Python script:')
    // with open(__file__, 'r') as f:
    //     print(''.join(map(lambda x: '    // ' + x, f.readlines())), end='')
    //
    // template = """
    //   private:
    //
    //     /**
    //      * Assigns the {0[0]} callback function from a `new`-initialized
    //      * raw pointer to a `callback::{0[2]}` object. Callee will ensure that
    //      * `delete` is called.
    //      */
    //     void set_{0[1]}(callback::{0[2]} *cb) {{
    //       try {{
    //         check(raw::dqcs_pdef_set_{0[1]}_cb(
    //           handle,
    //           CallbackEntryPoints::{0[1]},
    //           CallbackEntryPoints::user_free<callback::{0[2]}>,
    //           cb));
    //       }} catch (...) {{
    //         delete cb;
    //         throw;
    //       }}
    //     }}
    //
    //   public:
    //
    //     /**
    //      * Assigns the {0[0]} callback function from a pre-existing
    //      * `callback::{0[2]}` object by copy.
    //      *
    //      * \\param cb The callback object.
    //      * \\returns `&self`, to continue building.
    //      * \\throws std::runtime_error When the current handle is invalid or of an
    //      * unsupported plugin type, or when the callback object is invalid.
    //      */
    //     Plugin &&with_{0[1]}(const callback::{0[2]} &cb) {{
    //       set_{0[1]}(new callback::{0[2]}(cb));
    //       return std::move(*this);
    //     }}
    //
    //     /**
    //      * Assigns the {0[0]} callback function from a pre-existing
    //      * `callback::{0[2]}` object by move.
    //      *
    //      * \\param cb The callback object.
    //      * \\returns `&self`, to continue building.
    //      * \\throws std::runtime_error When the current handle is invalid or of an
    //      * unsupported plugin type, or when the callback object is invalid.
    //      */
    //     Plugin &&with_{0[1]}(callback::{0[2]} &&cb) {{
    //       set_{0[1]}(new callback::{0[2]}(std::move(cb)));
    //       return std::move(*this);
    //     }}
    //
    //     /**
    //      * Assigns the {0[0]} callback function by constructing the
    //      * callback object implicitly.
    //      *
    //      * \\returns `&self`, to continue building.
    //      * \\throws std::runtime_error When the current handle is invalid or of an
    //      * unsupported plugin type, or when the callback object is invalid.
    //      */
    //     template<typename... Args>
    //     Plugin &&with_{0[1]}(Args... args) {{
    //       set_{0[1]}(new callback::{0[2]}(args...));
    //       return std::move(*this);
    //     }}
    // """
    //
    // print(''.join(map(template.format, [
    //     ('initialize',          'initialize',           'Initialize'),
    //     ('drop',                'drop',                 'Drop'),
    //     ('run',                 'run',                  'Run'),
    //     ('allocate',            'allocate',             'Allocate'),
    //     ('free',                'free',                 'Free'),
    //     ('gate',                'gate',                 'Gate'),
    //     ('modify-measurement',  'modify_measurement',   'ModifyMeasurement'),
    //     ('advance',             'advance',              'Advance'),
    //     ('upstream-arb',        'upstream_arb',         'Arb'),
    //     ('host-arb',            'host_arb',             'Arb'),
    // ])))
    //
    // print('    // End of generated code.')

  private:

    /**
     * Assigns the initialize callback function from a `new`-initialized
     * raw pointer to a `callback::Initialize` object. Callee will ensure that
     * `delete` is called.
     */
    void set_initialize(callback::Initialize *cb) {
      try {
        check(raw::dqcs_pdef_set_initialize_cb(
          handle,
          CallbackEntryPoints::initialize,
          CallbackEntryPoints::user_free<callback::Initialize>,
          cb));
      } catch (...) {
        delete cb;
        throw;
      }
    }

  public:

    /**
     * Assigns the initialize callback function from a pre-existing
     * `callback::Initialize` object by copy.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_initialize(const callback::Initialize &cb) {
      set_initialize(new callback::Initialize(cb));
      return std::move(*this);
    }

    /**
     * Assigns the initialize callback function from a pre-existing
     * `callback::Initialize` object by move.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_initialize(callback::Initialize &&cb) {
      set_initialize(new callback::Initialize(std::move(cb)));
      return std::move(*this);
    }

    /**
     * Assigns the initialize callback function by constructing the
     * callback object implicitly.
     *
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    template<typename... Args>
    Plugin &&with_initialize(Args... args) {
      set_initialize(new callback::Initialize(args...));
      return std::move(*this);
    }

  private:

    /**
     * Assigns the drop callback function from a `new`-initialized
     * raw pointer to a `callback::Drop` object. Callee will ensure that
     * `delete` is called.
     */
    void set_drop(callback::Drop *cb) {
      try {
        check(raw::dqcs_pdef_set_drop_cb(
          handle,
          CallbackEntryPoints::drop,
          CallbackEntryPoints::user_free<callback::Drop>,
          cb));
      } catch (...) {
        delete cb;
        throw;
      }
    }

  public:

    /**
     * Assigns the drop callback function from a pre-existing
     * `callback::Drop` object by copy.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_drop(const callback::Drop &cb) {
      set_drop(new callback::Drop(cb));
      return std::move(*this);
    }

    /**
     * Assigns the drop callback function from a pre-existing
     * `callback::Drop` object by move.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_drop(callback::Drop &&cb) {
      set_drop(new callback::Drop(std::move(cb)));
      return std::move(*this);
    }

    /**
     * Assigns the drop callback function by constructing the
     * callback object implicitly.
     *
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    template<typename... Args>
    Plugin &&with_drop(Args... args) {
      set_drop(new callback::Drop(args...));
      return std::move(*this);
    }

  private:

    /**
     * Assigns the run callback function from a `new`-initialized
     * raw pointer to a `callback::Run` object. Callee will ensure that
     * `delete` is called.
     */
    void set_run(callback::Run *cb) {
      try {
        check(raw::dqcs_pdef_set_run_cb(
          handle,
          CallbackEntryPoints::run,
          CallbackEntryPoints::user_free<callback::Run>,
          cb));
      } catch (...) {
        delete cb;
        throw;
      }
    }

  public:

    /**
     * Assigns the run callback function from a pre-existing
     * `callback::Run` object by copy.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_run(const callback::Run &cb) {
      set_run(new callback::Run(cb));
      return std::move(*this);
    }

    /**
     * Assigns the run callback function from a pre-existing
     * `callback::Run` object by move.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_run(callback::Run &&cb) {
      set_run(new callback::Run(std::move(cb)));
      return std::move(*this);
    }

    /**
     * Assigns the run callback function by constructing the
     * callback object implicitly.
     *
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    template<typename... Args>
    Plugin &&with_run(Args... args) {
      set_run(new callback::Run(args...));
      return std::move(*this);
    }

  private:

    /**
     * Assigns the allocate callback function from a `new`-initialized
     * raw pointer to a `callback::Allocate` object. Callee will ensure that
     * `delete` is called.
     */
    void set_allocate(callback::Allocate *cb) {
      try {
        check(raw::dqcs_pdef_set_allocate_cb(
          handle,
          CallbackEntryPoints::allocate,
          CallbackEntryPoints::user_free<callback::Allocate>,
          cb));
      } catch (...) {
        delete cb;
        throw;
      }
    }

  public:

    /**
     * Assigns the allocate callback function from a pre-existing
     * `callback::Allocate` object by copy.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_allocate(const callback::Allocate &cb) {
      set_allocate(new callback::Allocate(cb));
      return std::move(*this);
    }

    /**
     * Assigns the allocate callback function from a pre-existing
     * `callback::Allocate` object by move.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_allocate(callback::Allocate &&cb) {
      set_allocate(new callback::Allocate(std::move(cb)));
      return std::move(*this);
    }

    /**
     * Assigns the allocate callback function by constructing the
     * callback object implicitly.
     *
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    template<typename... Args>
    Plugin &&with_allocate(Args... args) {
      set_allocate(new callback::Allocate(args...));
      return std::move(*this);
    }

  private:

    /**
     * Assigns the free callback function from a `new`-initialized
     * raw pointer to a `callback::Free` object. Callee will ensure that
     * `delete` is called.
     */
    void set_free(callback::Free *cb) {
      try {
        check(raw::dqcs_pdef_set_free_cb(
          handle,
          CallbackEntryPoints::free,
          CallbackEntryPoints::user_free<callback::Free>,
          cb));
      } catch (...) {
        delete cb;
        throw;
      }
    }

  public:

    /**
     * Assigns the free callback function from a pre-existing
     * `callback::Free` object by copy.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_free(const callback::Free &cb) {
      set_free(new callback::Free(cb));
      return std::move(*this);
    }

    /**
     * Assigns the free callback function from a pre-existing
     * `callback::Free` object by move.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_free(callback::Free &&cb) {
      set_free(new callback::Free(std::move(cb)));
      return std::move(*this);
    }

    /**
     * Assigns the free callback function by constructing the
     * callback object implicitly.
     *
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    template<typename... Args>
    Plugin &&with_free(Args... args) {
      set_free(new callback::Free(args...));
      return std::move(*this);
    }

  private:

    /**
     * Assigns the gate callback function from a `new`-initialized
     * raw pointer to a `callback::Gate` object. Callee will ensure that
     * `delete` is called.
     */
    void set_gate(callback::Gate *cb) {
      try {
        check(raw::dqcs_pdef_set_gate_cb(
          handle,
          CallbackEntryPoints::gate,
          CallbackEntryPoints::user_free<callback::Gate>,
          cb));
      } catch (...) {
        delete cb;
        throw;
      }
    }

  public:

    /**
     * Assigns the gate callback function from a pre-existing
     * `callback::Gate` object by copy.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_gate(const callback::Gate &cb) {
      set_gate(new callback::Gate(cb));
      return std::move(*this);
    }

    /**
     * Assigns the gate callback function from a pre-existing
     * `callback::Gate` object by move.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_gate(callback::Gate &&cb) {
      set_gate(new callback::Gate(std::move(cb)));
      return std::move(*this);
    }

    /**
     * Assigns the gate callback function by constructing the
     * callback object implicitly.
     *
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    template<typename... Args>
    Plugin &&with_gate(Args... args) {
      set_gate(new callback::Gate(args...));
      return std::move(*this);
    }

  private:

    /**
     * Assigns the modify-measurement callback function from a `new`-initialized
     * raw pointer to a `callback::ModifyMeasurement` object. Callee will ensure that
     * `delete` is called.
     */
    void set_modify_measurement(callback::ModifyMeasurement *cb) {
      try {
        check(raw::dqcs_pdef_set_modify_measurement_cb(
          handle,
          CallbackEntryPoints::modify_measurement,
          CallbackEntryPoints::user_free<callback::ModifyMeasurement>,
          cb));
      } catch (...) {
        delete cb;
        throw;
      }
    }

  public:

    /**
     * Assigns the modify-measurement callback function from a pre-existing
     * `callback::ModifyMeasurement` object by copy.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_modify_measurement(const callback::ModifyMeasurement &cb) {
      set_modify_measurement(new callback::ModifyMeasurement(cb));
      return std::move(*this);
    }

    /**
     * Assigns the modify-measurement callback function from a pre-existing
     * `callback::ModifyMeasurement` object by move.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_modify_measurement(callback::ModifyMeasurement &&cb) {
      set_modify_measurement(new callback::ModifyMeasurement(std::move(cb)));
      return std::move(*this);
    }

    /**
     * Assigns the modify-measurement callback function by constructing the
     * callback object implicitly.
     *
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    template<typename... Args>
    Plugin &&with_modify_measurement(Args... args) {
      set_modify_measurement(new callback::ModifyMeasurement(args...));
      return std::move(*this);
    }

  private:

    /**
     * Assigns the advance callback function from a `new`-initialized
     * raw pointer to a `callback::Advance` object. Callee will ensure that
     * `delete` is called.
     */
    void set_advance(callback::Advance *cb) {
      try {
        check(raw::dqcs_pdef_set_advance_cb(
          handle,
          CallbackEntryPoints::advance,
          CallbackEntryPoints::user_free<callback::Advance>,
          cb));
      } catch (...) {
        delete cb;
        throw;
      }
    }

  public:

    /**
     * Assigns the advance callback function from a pre-existing
     * `callback::Advance` object by copy.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_advance(const callback::Advance &cb) {
      set_advance(new callback::Advance(cb));
      return std::move(*this);
    }

    /**
     * Assigns the advance callback function from a pre-existing
     * `callback::Advance` object by move.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_advance(callback::Advance &&cb) {
      set_advance(new callback::Advance(std::move(cb)));
      return std::move(*this);
    }

    /**
     * Assigns the advance callback function by constructing the
     * callback object implicitly.
     *
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    template<typename... Args>
    Plugin &&with_advance(Args... args) {
      set_advance(new callback::Advance(args...));
      return std::move(*this);
    }

  private:

    /**
     * Assigns the upstream-arb callback function from a `new`-initialized
     * raw pointer to a `callback::Arb` object. Callee will ensure that
     * `delete` is called.
     */
    void set_upstream_arb(callback::Arb *cb) {
      try {
        check(raw::dqcs_pdef_set_upstream_arb_cb(
          handle,
          CallbackEntryPoints::upstream_arb,
          CallbackEntryPoints::user_free<callback::Arb>,
          cb));
      } catch (...) {
        delete cb;
        throw;
      }
    }

  public:

    /**
     * Assigns the upstream-arb callback function from a pre-existing
     * `callback::Arb` object by copy.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_upstream_arb(const callback::Arb &cb) {
      set_upstream_arb(new callback::Arb(cb));
      return std::move(*this);
    }

    /**
     * Assigns the upstream-arb callback function from a pre-existing
     * `callback::Arb` object by move.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_upstream_arb(callback::Arb &&cb) {
      set_upstream_arb(new callback::Arb(std::move(cb)));
      return std::move(*this);
    }

    /**
     * Assigns the upstream-arb callback function by constructing the
     * callback object implicitly.
     *
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    template<typename... Args>
    Plugin &&with_upstream_arb(Args... args) {
      set_upstream_arb(new callback::Arb(args...));
      return std::move(*this);
    }

  private:

    /**
     * Assigns the host-arb callback function from a `new`-initialized
     * raw pointer to a `callback::Arb` object. Callee will ensure that
     * `delete` is called.
     */
    void set_host_arb(callback::Arb *cb) {
      try {
        check(raw::dqcs_pdef_set_host_arb_cb(
          handle,
          CallbackEntryPoints::host_arb,
          CallbackEntryPoints::user_free<callback::Arb>,
          cb));
      } catch (...) {
        delete cb;
        throw;
      }
    }

  public:

    /**
     * Assigns the host-arb callback function from a pre-existing
     * `callback::Arb` object by copy.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_host_arb(const callback::Arb &cb) {
      set_host_arb(new callback::Arb(cb));
      return std::move(*this);
    }

    /**
     * Assigns the host-arb callback function from a pre-existing
     * `callback::Arb` object by move.
     *
     * \param cb The callback object.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    Plugin &&with_host_arb(callback::Arb &&cb) {
      set_host_arb(new callback::Arb(std::move(cb)));
      return std::move(*this);
    }

    /**
     * Assigns the host-arb callback function by constructing the
     * callback object implicitly.
     *
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the current handle is invalid or of an
     * unsupported plugin type, or when the callback object is invalid.
     */
    template<typename... Args>
    Plugin &&with_host_arb(Args... args) {
      set_host_arb(new callback::Arb(args...));
      return std::move(*this);
    }

    // End of generated code.

    /**
     * Runs the defined plugin in the current thread with the given simulator
     * connection descriptor string.
     *
     * \param simulator Simulator connection descriptor. This should come from
     * the first (for normal plugins) or second (for script-interpreting
     * plugins) command-line argument of the plugin executable (in the latter
     * case, the first argument is the script filename).
     * \throws std::runtime_error When plugin execution fails.
     */
    void run(const char *simulator) {
      check(raw::dqcs_plugin_run(handle, simulator));
      take_handle();
    }

    /**
     * Runs the defined plugin in the current thread with the given command
     * line.
     *
     * This is normally the end of the only statement in your plugin
     * executable's `main()`.
     *
     * \param argc The `argc` parameter from your plugin executable's `main()`.
     * \param argv The `argv` parameter from your plugin executable's `main()`.
     * \returns The proper exit code for the plugin.
     */
    int run(int argc, char *argv[]) noexcept {
      if (argc != 2) {
        DQCSIM_FATAL("Expecting exactly one command-line argument, but got %d", argc);
        return 1;
      }
      try {
        run(argv[1]);
      } catch (const std::exception &e) {
        DQCSIM_FATAL("Returning failure because of the following: %s", e.what());
        return 1;
      } catch (...) {
        DQCSIM_FATAL("Returning failure because of an unknown exception");
        return 1;
      }
      return 0;
    }

    /**
     * Starts the defined plugin in the current thread.
     *
     * \param simulator Simulator connection descriptor. This should come from
     * the first (for normal plugins) or second (for script-interpreting
     * plugins) command-line argument of the plugin executable (in the latter
     * case, the first argument is the script filename).
     * \returns A `PluginJoinHandle` object that allows the plugin to be waited
     * on.
     * \throws std::runtime_error When the plugin could not be started.
     */
    PluginJoinHandle start(const char *simulator) {
      PluginJoinHandle join_handle(check(raw::dqcs_plugin_start(handle, simulator)));
      take_handle();
      return join_handle;
    }

  };

  /**
   * Generic class for plugin configurations.
   */
  class PluginConfiguration : public Handle {
  public:

    /**
     * Wraps the given plugin process or thread configuration handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    PluginConfiguration(HandleIndex handle) noexcept : Handle(handle) {
    }

    // Delete copy construct/assign.
    PluginConfiguration(const PluginConfiguration&) = delete;
    void operator=(const PluginConfiguration&) = delete;

    /**
     * Default move constructor.
     */
    PluginConfiguration(PluginConfiguration&&) = default;

    /**
     * Default move assignment.
     */
    PluginConfiguration &operator=(PluginConfiguration&&) = default;

    /**
     * Returns the plugin type.
     *
     * \returns The plugin type.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    virtual PluginType get_plugin_type() const = 0;

    /**
     * Returns the name given to the plugin.
     *
     * \note This returns the instance name, not the class name. The latter can
     * only be queried once the plugin thread or process has been started.
     *
     * \returns the name given to the plugin instance.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    virtual std::string get_name() const = 0;

    /**
     * Attaches an arbitrary initialization command to the plugin.
     *
     * \param cmd The initialization command to attach.
     * \throws std::runtime_error When the plugin definition or command handle
     * is invalid.
     */
    virtual void add_init_cmd(ArbCmd &&cmd) = 0;

    /**
     * Attaches an arbitrary initialization command to the plugin.
     *
     * \param cmd The initialization command to attach.
     * \throws std::runtime_error When the plugin definition or command handle
     * is invalid.
     */
    void add_init_cmd(const ArbCmd &cmd) {
      add_init_cmd(ArbCmd(cmd));
    }

    /**
     * Sets the logging verbosity level of the plugin.
     *
     * \param level The desired logging verbosity for the plugin instance.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    virtual void set_verbosity(Loglevel level) = 0;

    /**
     * Returns the current logging verbosity level of the plugin.
     *
     * \returns The current logging verbosity level of the plugin.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    virtual Loglevel get_verbosity() const = 0;

    /**
     * Configures a plugin thread to also output its log messages to a file.
     *
     * \param verbosity Configures the verbosity level for the tee'd output
     * file only.
     * \param filename The path to the file to tee log messages to.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    virtual void log_tee(Loglevel verbosity, const std::string &filename) = 0;

  };

  /**
   * Wrapper class for plugin process configurations.
   */
  class PluginProcessConfiguration : public PluginConfiguration {
  public:

    /**
     * Wraps the given plugin process configuration handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    PluginProcessConfiguration(HandleIndex handle) noexcept : PluginConfiguration(handle) {
    }

    // Delete copy construct/assign.
    PluginProcessConfiguration(const PluginProcessConfiguration&) = delete;
    void operator=(const PluginProcessConfiguration&) = delete;

    /**
     * Default move constructor.
     */
    PluginProcessConfiguration(PluginProcessConfiguration&&) = default;

    /**
     * Default move assignment.
     */
    PluginProcessConfiguration &operator=(PluginProcessConfiguration&&) = default;

    /**
     * Returns the plugin type.
     *
     * \returns The plugin type.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    PluginType get_plugin_type() const override {
      return check(raw::dqcs_pcfg_type(handle));
    }

    /**
     * Returns the name given to the plugin.
     *
     * \note This returns the instance name, not the class name. The latter can
     * only be queried once the plugin thread or process has been started.
     *
     * \returns the name given to the plugin instance.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    std::string get_name() const override {
      char *ptr = check(raw::dqcs_pcfg_name(handle));
      std::string retval(ptr);
      std::free(ptr);
      return retval;
    }

    /**
     * Returns the configured executable path for the plugin.
     *
     * \returns The configured executable path for the plugin.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    std::string get_executable() const {
      char *ptr = check(raw::dqcs_pcfg_executable(handle));
      std::string retval(ptr);
      std::free(ptr);
      return retval;
    }

    /**
     * Returns the configured script path for the plugin.
     *
     * \returns The configured script path for the plugin.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    std::string get_script() const {
      char *ptr = check(raw::dqcs_pcfg_script(handle));
      std::string retval(ptr);
      std::free(ptr);
      return retval;
    }

    /**
     * Attaches an arbitrary initialization command to the plugin.
     *
     * \param cmd The initialization command to attach.
     * \throws std::runtime_error When the plugin definition or command handle
     * is invalid.
     */
    void add_init_cmd(ArbCmd &&cmd) override {
      check(raw::dqcs_pcfg_init_cmd(handle, cmd.get_handle()));
    }

    /**
     * Attaches an arbitrary initialization command to the plugin (builder
     * pattern).
     *
     * \param cmd The initialization command to attach.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition or command handle
     * is invalid.
     */
    PluginProcessConfiguration &&with_init_cmd(ArbCmd &&cmd) {
      add_init_cmd(std::move(cmd));
      return std::move(*this);
    }

    /**
     * Overrides an environment variable for the plugin process.
     *
     * The environment variable `key` is set to `value` regardless of whether
     * it exists in the parent environment variable scope.
     *
     * \param key The environment variable to set.
     * \param value The value to set it to.
     * \throws std::runtime_error When the plugin definition or command handle
     * is invalid.
     */
    void set_env_var(const std::string &key, const std::string &value) {
      check(raw::dqcs_pcfg_env_set(handle, key.c_str(), value.c_str()));
    }

    /**
     * Overrides an environment variable for the plugin process (builder
     * pattern).
     *
     * The environment variable `key` is set to `value` regardless of whether
     * it exists in the parent environment variable scope.
     *
     * \param key The environment variable to set.
     * \param value The value to set it to.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition or command handle
     * is invalid.
     */
    PluginProcessConfiguration &&with_env_var(const std::string &key, const std::string &value) {
      set_env_var(key, value);
      return std::move(*this);
    }

    /**
     * Removes/unsets an environment variable for the plugin process.
     *
     * The environment variable key is unset regardless of whether it exists
     * in the parent environment variable scope.
     *
     * \param key The environment variable to unset.
     * \throws std::runtime_error When the plugin definition or command handle
     * is invalid.
     */
    void unset_env_var(const std::string &key) {
      check(raw::dqcs_pcfg_env_unset(handle, key.c_str()));
    }

    /**
     * Removes/unsets an environment variable for the plugin process (builder
     * pattern).
     *
     * The environment variable key is unset regardless of whether it exists
     * in the parent environment variable scope.
     *
     * \param key The environment variable to unset.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition or command handle
     * is invalid.
     */
    PluginProcessConfiguration &&without_env_var(const std::string &key) {
      unset_env_var(key);
      return std::move(*this);
    }

    /**
     * Overrides the working directory for the plugin process.
     *
     * \param dir The working directory for the plugin process.
     * \throws std::runtime_error When the plugin definition or command handle
     * is invalid.
     */
    void set_work_dir(const std::string &dir) {
      check(raw::dqcs_pcfg_work_set(handle, dir.c_str()));
    }

    /**
     * Overrides the working directory for the plugin process (builder
     * pattern).
     *
     * \param dir The working directory for the plugin process.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition or command handle
     * is invalid.
     */
    PluginProcessConfiguration &&with_work_dir(const std::string &dir) {
      set_work_dir(dir);
      return std::move(*this);
    }

    /**
     * Returns the configured working directory for the given plugin process.
     *
     * \returns Tthe configured working directory for the given plugin process.
     * \throws std::runtime_error When the plugin definition or command handle
     * is invalid.
     */
    std::string get_work_dir() const {
      char *ptr = check(raw::dqcs_pcfg_work_get(handle));
      std::string retval(ptr);
      std::free(ptr);
      return retval;
    }

    /**
     * Sets the logging verbosity level of the plugin.
     *
     * \param level The desired logging verbosity for the plugin instance.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    void set_verbosity(Loglevel level) override {
      check(raw::dqcs_pcfg_verbosity_set(handle, to_raw(level)));
    }

    /**
     * Sets the logging verbosity level of the plugin (builder pattern).
     *
     * \param level The desired logging verbosity for the plugin instance.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    PluginProcessConfiguration &&with_verbosity(Loglevel level) {
      set_verbosity(level);
      return std::move(*this);
    }

    /**
     * Returns the current logging verbosity level of the plugin.
     *
     * \returns The current logging verbosity level of the plugin.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    Loglevel get_verbosity() const override {
      return check(raw::dqcs_pcfg_verbosity_get(handle));
    }

    /**
     * Configures a plugin thread to also output its log messages to a file.
     *
     * \param verbosity Configures the verbosity level for the tee'd output
     * file only.
     * \param filename The path to the file to tee log messages to.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    void log_tee(Loglevel verbosity, const std::string &filename) override {
      return check(raw::dqcs_pcfg_tee(handle, to_raw(verbosity), filename.c_str()));
    }

    /**
     * Configures a plugin thread to also output its log messages to a file
     * (builder pattern).
     *
     * \param verbosity Configures the verbosity level for the tee'd output
     * file only.
     * \param filename The path to the file to tee log messages to.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    PluginProcessConfiguration &&with_log_tee(Loglevel verbosity, const std::string &filename) {
      log_tee(verbosity, filename);
      return std::move(*this);
    }

    /**
     * Configures the capture mode for the stdout stream of the specified
     * plugin process.
     *
     * \param level The loglevel with which stdout is captured.
     * `Loglevel::Pass` instructs the logging thread to not capture stdout at
     * all.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    void set_stdout_loglevel(Loglevel level) {
      check(raw::dqcs_pcfg_stdout_mode_set(handle, to_raw(level)));
    }

    /**
     * Configures the capture mode for the stdout stream of the specified
     * plugin process (builder pattern).
     *
     * \param level The loglevel with which stdout is captured.
     * `Loglevel::Pass` instructs the logging thread to not capture stdout at
     * all.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    PluginProcessConfiguration &&with_stdout_loglevel(Loglevel level) {
      set_stdout_loglevel(level);
      return std::move(*this);
    }

    /**
     * Returns the configured stdout capture mode for the specified plugin
     * process.
     *
     * \returns The configured stdout capture mode.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    Loglevel get_stdout_loglevel() const {
      return check(raw::dqcs_pcfg_stdout_mode_get(handle));
    }

    /**
     * Configures the capture mode for the stderr stream of the specified
     * plugin process.
     *
     * \param level The loglevel with which stderr is captured.
     * `Loglevel::Pass` instructs the logging thread to not capture stderr at
     * all.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    void set_stderr_loglevel(Loglevel level) {
      check(raw::dqcs_pcfg_stderr_mode_set(handle, to_raw(level)));
    }

    /**
     * Configures the capture mode for the stderr stream of the specified
     * plugin process (builder pattern).
     *
     * \param level The loglevel with which stderr is captured.
     * `Loglevel::Pass` instructs the logging thread to not capture stderr at
     * all.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    PluginProcessConfiguration &&with_stderr_loglevel(Loglevel level) {
      set_stderr_loglevel(level);
      return std::move(*this);
    }

    /**
     * Returns the configured stderr capture mode for the specified plugin
     * process.
     *
     * \returns The configured stderr capture mode.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    Loglevel get_stderr_loglevel() const {
      return check(raw::dqcs_pcfg_stderr_mode_get(handle));
    }

    /**
     * Configures the timeout for the plugin process to connect to DQCsim.
     *
     * The default is 5 seconds, so you should normally be able to leave this
     * alone.
     *
     * \param timeout The timeout in seconds. You can use IEEE positive
     * infinity from `<limits>` to specify an infinite timeout.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    void set_accept_timeout(double timeout) {
      check(raw::dqcs_pcfg_accept_timeout_set(handle, timeout));
    }

    /**
     * Configures the timeout for the plugin process to connect to DQCsim
     * (builder pattern).
     *
     * The default is 5 seconds, so you should normally be able to leave this
     * alone.
     *
     * \param timeout The timeout in seconds. You can use IEEE positive
     * infinity from `<limits>` to specify an infinite timeout.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    PluginProcessConfiguration &&with_accept_timeout(double timeout) {
      set_accept_timeout(timeout);
      return std::move(*this);
    }

    /**
     * Disables the timeout for the plugin process to connect to DQCsim
     * (builder pattern).
     *
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    PluginProcessConfiguration &&without_accept_timeout() {
      set_accept_timeout(std::numeric_limits<double>::infinity());
      return std::move(*this);
    }

    /**
     * Returns the configured timeout for the plugin process to connect to
     * DQCsim.
     *
     * \returns The configured timeout in seconds.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    double get_accept_timeout() const {
      return check(raw::dqcs_pcfg_accept_timeout_get(handle));
    }

    /**
     * Configures the timeout for the plugin process to shut down gracefully.
     *
     * The default is 5 seconds, so you should normally be able to leave this
     * alone.
     *
     * \param timeout The timeout in seconds. You can use IEEE positive
     * infinity from `<limits>` to specify an infinite timeout.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    void set_shutdown_timeout(double timeout) {
      check(raw::dqcs_pcfg_shutdown_timeout_set(handle, timeout));
    }

    /**
     * Configures the timeout for the plugin process to shut down gracefully
     * (builder pattern).
     *
     * The default is 5 seconds, so you should normally be able to leave this
     * alone.
     *
     * \param timeout The timeout in seconds. You can use IEEE positive
     * infinity from `<limits>` to specify an infinite timeout.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    PluginProcessConfiguration &&with_shutdown_timeout(double timeout) {
      set_shutdown_timeout(timeout);
      return std::move(*this);
    }

    /**
     * Disables the timeout for the plugin process to shut down gracefully
     * (builder pattern).
     *
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    PluginProcessConfiguration &&without_shutdown_timeout() {
      set_shutdown_timeout(std::numeric_limits<double>::infinity());
      return std::move(*this);
    }

    /**
     * Returns the configured timeout for the plugin process to shut down
     * gracefully.
     *
     * \returns The configured timeout in seconds.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    double get_shutdown_timeout() const {
      return check(raw::dqcs_pcfg_shutdown_timeout_get(handle));
    }

  };

  /**
   * Wrapper class for local plugin thread configurations.
   */
  class PluginThreadConfiguration : public PluginConfiguration {
  public:

    /**
     * Wraps the given plugin thread configuration handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    PluginThreadConfiguration(HandleIndex handle) noexcept : PluginConfiguration(handle) {
    }

    // Delete copy construct/assign.
    PluginThreadConfiguration(const PluginThreadConfiguration&) = delete;
    void operator=(const PluginThreadConfiguration&) = delete;

    /**
     * Default move constructor.
     */
    PluginThreadConfiguration(PluginThreadConfiguration&&) = default;

    /**
     * Default move assignment.
     */
    PluginThreadConfiguration &operator=(PluginThreadConfiguration&&) = default;

    /**
     * Returns the plugin type.
     *
     * \returns The plugin type.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    PluginType get_plugin_type() const override {
      return check(raw::dqcs_tcfg_type(handle));
    }

    /**
     * Returns the name given to the plugin.
     *
     * \note This returns the instance name, not the class name. The latter can
     * only be queried once the plugin thread or process has been started.
     *
     * \returns the name given to the plugin instance.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    std::string get_name() const override {
      char *ptr = check(raw::dqcs_tcfg_name(handle));
      std::string retval(ptr);
      std::free(ptr);
      return retval;
    }

    /**
     * Attaches an arbitrary initialization command to the plugin.
     *
     * \param cmd The initialization command to attach.
     * \throws std::runtime_error When the plugin definition or command handle
     * is invalid.
     */
    void add_init_cmd(ArbCmd &&cmd) override {
      check(raw::dqcs_tcfg_init_cmd(handle, cmd.get_handle()));
    }

    /**
     * Attaches an arbitrary initialization command to the plugin (builder
     * pattern).
     *
     * \param cmd The initialization command to attach.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition or command handle
     * is invalid.
     */
    PluginThreadConfiguration &&with_init_cmd(ArbCmd &&cmd) {
      add_init_cmd(std::move(cmd));
      return std::move(*this);
    }

    /**
     * Sets the logging verbosity level of the plugin.
     *
     * \param level The desired logging verbosity for the plugin instance.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    void set_verbosity(Loglevel level) override {
      check(raw::dqcs_tcfg_verbosity_set(handle, to_raw(level)));
    }

    /**
     * Sets the logging verbosity level of the plugin (builder pattern).
     *
     * \param level The desired logging verbosity for the plugin instance.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    PluginThreadConfiguration &&with_verbosity(Loglevel level) {
      set_verbosity(level);
      return std::move(*this);
    }

    /**
     * Returns the current logging verbosity level of the plugin.
     *
     * \returns The current logging verbosity level of the plugin.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    Loglevel get_verbosity() const override {
      return check(raw::dqcs_tcfg_verbosity_get(handle));
    }

    /**
     * Configures a plugin thread to also output its log messages to a file.
     *
     * \param verbosity Configures the verbosity level for the tee'd output
     * file only.
     * \param filename The path to the file to tee log messages to.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    void log_tee(Loglevel verbosity, const std::string &filename) override {
      return check(raw::dqcs_tcfg_tee(handle, to_raw(verbosity), filename.c_str()));
    }

    /**
     * Configures a plugin thread to also output its log messages to a file
     * (builder pattern).
     *
     * \param verbosity Configures the verbosity level for the tee'd output
     * file only.
     * \param filename The path to the file to tee log messages to.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the plugin definition handle is invalid.
     */
    PluginThreadConfiguration &&with_log_tee(Loglevel verbosity, const std::string &filename) {
      log_tee(verbosity, filename);
      return std::move(*this);
    }

  };

  /**
   * Builder class used to construct plugin configurations.
   */
  class PluginConfigurationBuilder {
  private:
    PluginType type;
    std::string name;
  public:

    /**
     * Constructs a plugin configuration builder for the given plugin type.
     *
     * You can use the `dqcsim::wrap::Frontend()`, `dqcsim::wrap::Operator()`,
     * and `dqcsim::wrap::Backend()` functions as shorthands for this
     * constructor.
     *
     * \param type The type of the to-be-configured plugin.
     */
    PluginConfigurationBuilder(PluginType type) noexcept : type(type), name() {}

    /**
     * Builder function for naming the plugin instance.
     *
     * The name must be unique within the simulation. It is used, among other
     * things, by the logging system.
     *
     * If this is not called or is called with an empty string, auto-naming
     * will be performed: "front" for the frontend, "oper[i]" for the operators
     * (indices starting at 1 from frontend to backend), and "back" for the
     * backend.
     *
     * \param name The name for the plugin.
     * \returns `&self`, to continue building.
     */
    PluginConfigurationBuilder &&with_name(const std::string &name) noexcept {
      this->name = name;
      return std::move(*this);
    }

    /**
     * Builds a plugin process configuration object from a "sugared" plugin
     * specification string, using the same syntax that the `dqcsim` command
     * line interface uses.
     *
     * \param spec The command-line interface specification string for the
     * desired plugin.
     * \returns A `PluginProcessConfiguration` object to continue building.
     * \throws std::runtime_error When the specified plugin could not be found,
     * or construction of the configuration handle fails for some reason.
     */
    PluginProcessConfiguration with_spec(const std::string &spec) {
      return PluginProcessConfiguration(check(raw::dqcs_pcfg_new(
        to_raw(type), name.c_str(), spec.c_str()
      )));
    }

    /**
     * Builds a plugin process configuration object from a path to a plugin
     * executable and an optional path to a script for it to run.
     *
     * Note that not all plugins will use the optional `script` parameter.
     *
     * \param executable The path to the plugin executable to load.
     * \param script The optional first command-line argument passed to the
     * plugin executable.
     * \returns A `PluginProcessConfiguration` object to continue building.
     * \throws std::runtime_error When the specified plugin could not be found,
     * or construction of the configuration handle fails for some reason.
     */
    PluginProcessConfiguration with_executable(const std::string &executable, const std::string &script = "") {
      return PluginProcessConfiguration(check(raw::dqcs_pcfg_new_raw(
        to_raw(type), name.c_str(), executable.c_str(), script.c_str()
      )));
    }

    /**
     * Builds a plugin thread configuration object from a plugin definition
     * object, containing a bunch of callback functions.
     *
     * \param plugin The plugin definition object to wrap. Note that this must
     * be `std::move()`d in if it is not constructed in-place.
     * \returns A `PluginThreadConfiguration` object to continue building.
     * \throws std::runtime_error When the specified plugin could not be found,
     * or construction of the configuration handle fails for some reason.
     */
    PluginThreadConfiguration with_callbacks(Plugin &&plugin) {
      if (plugin.get_type() != type) {
        throw std::invalid_argument("plugin type does not match callback object type");
      }
      return PluginThreadConfiguration(check(raw::dqcs_tcfg_new(
        plugin.get_handle(), name.c_str()
      )));
    }

  private:

    /**
     * Builds a plugin thread configuration object using a single callback that
     * spawns the entire plugin.
     *
     * This version takes a pointer to a callback object, pre-allocated using
     * new. The callback is called by DQCsim from a dedicated thread when
     * DQCsim wants to start the plugin. The callback must then in some way
     * spawn a plugin process that connects to the provided simulator string.
     * The callback should return only when the process terminates.
     *
     * \param data The wrapper object for the plugin spawning callback.
     * \returns A `PluginThreadConfiguration` object to continue building.
     * \throws std::runtime_error When construction of the configuration handle
     * fails for some reason.
     */
    PluginThreadConfiguration with_spawner_ptr(callback::SpawnPlugin *data) {
      return PluginThreadConfiguration(check(raw::dqcs_tcfg_new_raw(
        to_raw(type),
        name.c_str(),
        CallbackEntryPoints::spawn_plugin,
        CallbackEntryPoints::user_free<callback::SpawnPlugin>,
        data
      )));
    }

  public:

    /**
     * Builds a plugin thread configuration object using a single callback that
     * spawns the entire plugin.
     *
     * This version takes a pre-existing callback object by copy. The callback
     * is called by DQCsim from a dedicated thread when DQCsim wants to start
     * the plugin. The callback must then in some way spawn a plugin process
     * that connects to the provided simulator string. The callback should
     * return only when the process terminates.
     *
     * \param data The wrapper object for the plugin spawning callback.
     * \returns A `PluginThreadConfiguration` object to continue building.
     * \throws std::runtime_error When construction of the configuration handle
     * fails for some reason.
     */
    PluginThreadConfiguration with_spawner(const callback::SpawnPlugin &data) {
      return with_spawner_ptr(new callback::SpawnPlugin(data));
    }

    /**
     * Builds a plugin thread configuration object using a single callback that
     * spawns the entire plugin.
     *
     * This version takes a pre-existing callback object by move. The callback
     * is called by DQCsim from a dedicated thread when DQCsim wants to start
     * the plugin. The callback must then in some way spawn a plugin process
     * that connects to the provided simulator string. The callback should
     * return only when the process terminates.
     *
     * \param data The wrapper object for the plugin spawning callback.
     * \returns A `PluginThreadConfiguration` object to continue building.
     * \throws std::runtime_error When construction of the configuration handle
     * fails for some reason.
     */
    PluginThreadConfiguration with_spawner(callback::SpawnPlugin &&data) {
      return with_spawner_ptr(new callback::SpawnPlugin(std::move(data)));
    }

    /**
     * Builds a plugin thread configuration object using a single callback that
     * spawns the entire plugin.
     *
     * This version allows the callback object to be (copy-)constructed
     * in-place. The callback is called by DQCsim from a dedicated thread when
     * DQCsim wants to start the plugin. The callback must then in some way
     * spawn a plugin process that connects to the provided simulator string.
     * The callback should return only when the process terminates.
     *
     * \param args Any legal set of arguments for one of
     * `callback::SpawnPlugin`'s constructors.
     * \returns A `PluginThreadConfiguration` object to continue building.
     * \throws std::runtime_error When construction of the configuration handle
     * fails for some reason.
     */
    template<typename... Args>
    PluginThreadConfiguration with_spawner(Args... args) {
      return with_spawner_ptr(new callback::SpawnPlugin(args...));
    }

  };

  /**
   * Shorthand for constructing a frontend plugin configuration builder.
   *
   * \param name An optional name for the plugin. See
   * `PluginConfigurationBuilder::with_name()`.
   * \returns A `PluginConfigurationBuilder` for a frontend plugin with the
   * given name.
   */
  inline PluginConfigurationBuilder Frontend(const std::string &name = "") noexcept {
    return PluginConfigurationBuilder(PluginType::Frontend).with_name(name);
  }

  /**
   * Shorthand for constructing an operator plugin configuration builder.
   *
   * \param name An optional name for the plugin. See
   * `PluginConfigurationBuilder::with_name()`.
   * \returns A `PluginConfigurationBuilder` for an operator plugin with the
   * given name.
   */
  inline PluginConfigurationBuilder Operator(const std::string &name = "") noexcept {
    return PluginConfigurationBuilder(PluginType::Operator).with_name(name);
  }

  /**
   * Shorthand for constructing a backend plugin configuration builder.
   *
   * \param name An optional name for the plugin. See
   * `PluginConfigurationBuilder::with_name()`.
   * \returns A `PluginConfigurationBuilder` for a backend plugin with the
   * given name.
   */
  inline PluginConfigurationBuilder Backend(const std::string &name = "") noexcept {
    return PluginConfigurationBuilder(PluginType::Backend).with_name(name);
  }

  /**
   * Wrapper class for a running simulation.
   */
  class Simulation : public Handle {
  public:

    /**
     * Wraps the given simulation handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    Simulation(HandleIndex handle) noexcept : Handle(handle) {
    }

    // Delete copy construct/assign.
    Simulation(const Simulation&) = delete;
    void operator=(const Simulation&) = delete;

    /**
     * Default move constructor.
     */
    Simulation(Simulation&&) = default;

    /**
     * Default move assignment.
     */
    Simulation &operator=(Simulation&&) = default;

    /**
     * Starts a program on the simulated accelerator without an argument.
     *
     * What constitutes "running a program" depends on the frontend plugin.
     *
     * \note This is an asynchronous call: nothing happens until `yield()`,
     * `recv()`, `wait()`, or `run()` is called.
     *
     * \throws std::runtime_error When the simulation is in an invalid state.
     */
    void start() {
      check(raw::dqcs_sim_start(handle, 0));
    }

    /**
     * Starts a program on the simulated accelerator using the given `ArbData`
     * object as an argument (passed by move).
     *
     * What constitutes "running a program" depends on the frontend plugin.
     *
     * \note This is an asynchronous call: nothing happens until `yield()`,
     * `recv()`, `wait()`, or `run()` is called.
     *
     * \param data An `ArbData` object to pass to the plugin's `run` callback.
     * \throws std::runtime_error When the simulation is in an invalid state.
     */
    void start(ArbData &&data) {
      check(raw::dqcs_sim_start(handle, data.get_handle()));
    }

    /**
     * Starts a program on the simulated accelerator using the given `ArbData`
     * object as an argument (passed by copy).
     *
     * What constitutes "running a program" depends on the frontend plugin.
     *
     * \note This is an asynchronous call: nothing happens until `yield()`,
     * `recv()`, `wait()`, or `run()` is called.
     *
     * \param data An `ArbData` object to pass to the plugin's `run` callback.
     * \throws std::runtime_error When the simulation is in an invalid state.
     */
    void start(const ArbData &data) {
      start(ArbData(data));
    }

    /**
     * Waits for the simulated accelerator to finish its current program.
     *
     * \returns The `ArbData` object that was returned by the frontend plugin's
     * implementation of the `run` callback.
     * \throws std::runtime_error When the simulation is in an invalid state or
     * a deadlock occurs because the frontend is waiting for a call to
     * `send()`.
     */
    ArbData wait() {
      return ArbData(check(raw::dqcs_sim_wait(handle)));
    }

    /**
     * Runs a program on the simulated accelerator without an argument.
     *
     * This is just a shorthand for `start()` followed by `wait()`.
     *
     * \returns The `ArbData` object that was returned by the frontend plugin's
     * implementation of the `run` callback.
     * \throws std::runtime_error When the simulation is in an invalid state or
     * a deadlock occurs because the frontend is waiting for a call to
     * `send()`.
     */
    ArbData run() {
      start();
      return wait();
    }

    /**
     * Runs a program on the simulated accelerator using the given `ArbData`
     * object as an argument (passed by move).
     *
     * This is just a shorthand for `start()` followed by `wait()`.
     *
     * \param data An `ArbData` object to pass to the plugin's `run` callback.
     * \returns The `ArbData` object that was returned by the frontend plugin's
     * implementation of the `run` callback.
     * \throws std::runtime_error When the simulation is in an invalid state or
     * a deadlock occurs because the frontend is waiting for a call to
     * `send()`.
     */
    ArbData run(ArbData &&data) {
      start(std::move(data));
      return wait();
    }

    /**
     * Runs a program on the simulated accelerator using the given `ArbData`
     * object as an argument (passed by copy).
     *
     * This is just a shorthand for `start()` followed by `wait()`.
     *
     * \param data An `ArbData` object to pass to the plugin's `run` callback.
     * \returns The `ArbData` object that was returned by the frontend plugin's
     * implementation of the `run` callback.
     * \throws std::runtime_error When the simulation is in an invalid state or
     * a deadlock occurs because the frontend is waiting for a call to
     * `send()`.
     */
    ArbData run(const ArbData &data) {
      start(data);
      return wait();
    }

    /**
     * Sends an empty message to the simulated accelerator.
     *
     * \note This is an asynchronous call: nothing happens until `yield()`,
     * `recv()`, `wait()`, or `run()` is called.
     *
     * \throws std::runtime_error When the simulation is in an invalid state.
     */
    void send() {
      check(raw::dqcs_sim_send(handle, 0));
    }

    /**
     * Sends the given `ArbData` message to the simulated accelerator (passed
     * by move).
     *
     * \note This is an asynchronous call: nothing happens until `yield()`,
     * `recv()`, `wait()`, or `run()` is called.
     *
     * \param data The `ArbData` object to send to the plugin.
     * \throws std::runtime_error When the simulation is in an invalid state.
     */
    void send(ArbData &&data) {
      check(raw::dqcs_sim_send(handle, data.get_handle()));
    }

    /**
     * Sends the given `ArbData` message to the simulated accelerator (passed
     * by copy).
     *
     * \note This is an asynchronous call: nothing happens until `yield()`,
     * `recv()`, `wait()`, or `run()` is called.
     *
     * \param data The `ArbData` object to send to the plugin.
     * \throws std::runtime_error When the simulation is in an invalid state.
     */
    void send(const ArbData &data) {
      send(ArbData(data));
    }

    /**
     * Waits for the simulated accelerator to send a message to us.
     *
     * \returns The `ArbData` sent to us.
     * \throws std::runtime_error When the simulation is in an invalid state
     * or when the plugin's `run` callback returned before sending (more) data.
     */
    ArbData recv() {
      return ArbData(check(raw::dqcs_sim_recv(handle)));
    }

    /**
     * Yields to the simulator.
     *
     * The simulation runs until it blocks again. This is useful if you want an
     * immediate response to an otherwise asynchronous call through the logging
     * system or some communication channel outside of DQCsim's control.
     *
     * This function silently returns immediately if no asynchronous data was
     * pending or if the simulator is waiting for something that has not been
     * sent yet.
     *
     * \throws std::runtime_error When the simulation is in an invalid state.
     */
    void yield() {
      check(raw::dqcs_sim_yield(handle));
    }

    /**
     * Sends an `ArbCmd` (passed by move) to the given plugin (referenced by
     * instance name).
     *
     * `ArbCmd`s are executed immediately after yielding to the simulator, so
     * all pending asynchronous calls are flushed and executed *before* the
     * `ArbCmd`.
     *
     * \param name The instance name of the plugin to send the command to.
     * \param cmd The command to send.
     * \returns The `ArbData` object returned by the command.
     * \throws std::runtime_error When the given name does not identify a
     * plugin, the command failed, or the simulation is in an invalid state.
     */
    ArbData arb(const std::string &name, ArbCmd &&cmd) {
      return ArbData(check(raw::dqcs_sim_arb(handle, name.c_str(), cmd.get_handle())));
    }

    /**
     * Sends an `ArbCmd` (passed by copy) to the given plugin (referenced by
     * instance name).
     *
     * `ArbCmd`s are executed immediately after yielding to the simulator, so
     * all pending asynchronous calls are flushed and executed *before* the
     * `ArbCmd`.
     *
     * \param name The instance name of the plugin to send the command to.
     * \param cmd The command to send.
     * \returns The `ArbData` object returned by the command.
     * \throws std::runtime_error When the given name does not identify a
     * plugin, the command failed, or the simulation is in an invalid state.
     */
    ArbData arb(const std::string &name, const ArbCmd &cmd) {
      return arb(name, ArbCmd(cmd));
    }

    /**
     * Sends an `ArbCmd` (passed by move) to the given plugin (referenced by
     * index).
     *
     * `ArbCmd`s are executed immediately after yielding to the simulator, so
     * all pending asynchronous calls are flushed and executed *before* the
     * `ArbCmd`.
     *
     * \param index The index of the plugin to send the command to. The frontend
     * always has index 0. 1 through N are used for the operators in front to
     * back order (where N is the number of operators). The backend is at index
     * N+1. Python-style negative indices are also supported. That is, -1 can be
     * used to refer to the backend, -2 to the last operator, and so on.
     * \param cmd The command to send.
     * \returns The `ArbData` object returned by the command.
     * \throws std::runtime_error When the given index is out of range, the
     * command failed, or the simulation is in an invalid state.
     */
    ArbData arb(ssize_t index, ArbCmd &&cmd) {
      return ArbData(check(raw::dqcs_sim_arb_idx(handle, index, cmd.get_handle())));
    }

    /**
     * Sends an `ArbCmd` (passed by copy) to the given plugin (referenced by
     * index).
     *
     * `ArbCmd`s are executed immediately after yielding to the simulator, so
     * all pending asynchronous calls are flushed and executed *before* the
     * `ArbCmd`.
     *
     * \param index The index of the plugin to send the command to. The frontend
     * always has index 0. 1 through N are used for the operators in front to
     * back order (where N is the number of operators). The backend is at index
     * N+1. Python-style negative indices are also supported. That is, -1 can be
     * used to refer to the backend, -2 to the last operator, and so on.
     * \param cmd The command to send.
     * \returns The `ArbData` object returned by the command.
     * \throws std::runtime_error When the given index is out of range, the
     * command failed, or the simulation is in an invalid state.
     */
    ArbData arb(ssize_t index, const ArbCmd &cmd) {
      return arb(index, ArbCmd(cmd));
    }

    /**
     * Queries the class name of a plugin, referenced by instance name.
     *
     * \param name The instance name of the plugin to query.
     * \returns The class name of the plugin.
     * \throws std::runtime_error When the given name does not identify a
     * plugin, or when the simulation is in an invalid state.
     */
    std::string get_name(const std::string &name) {
      char *ptr = check(raw::dqcs_sim_get_name(handle, name.c_str()));
      std::string str(ptr);
      std::free(ptr);
      return str;
    }

    /**
     * Queries the class name of a plugin, referenced by index.
     *
     * \param index The index of the plugin to query. The frontend always has
     * index 0. 1 through N are used for the operators in front to back order
     * (where N is the number of operators). The backend is at index N+1.
     * Python-style negative indices are also supported. That is, -1 can be
     * used to refer to the backend, -2 to the last operator, and so on.
     * \returns The class name of the plugin.
     * \throws std::runtime_error When the given index is out of range, or when
     * the simulation is in an invalid state.
     */
    std::string get_name(ssize_t index) {
      char *ptr = check(raw::dqcs_sim_get_name_idx(handle, index));
      std::string str(ptr);
      std::free(ptr);
      return str;
    }

    /**
     * Queries the author of a plugin, referenced by instance name.
     *
     * \param name The instance name of the plugin to query.
     * \returns The plugin author.
     * \throws std::runtime_error When the given name does not identify a
     * plugin, or when the simulation is in an invalid state.
     */
    std::string get_author(const std::string &name) {
      char *ptr = check(raw::dqcs_sim_get_author(handle, name.c_str()));
      std::string str(ptr);
      std::free(ptr);
      return str;
    }

    /**
     * Queries the author of a plugin, referenced by index.
     *
     * \param index The index of the plugin to query. The frontend always has
     * index 0. 1 through N are used for the operators in front to back order
     * (where N is the number of operators). The backend is at index N+1.
     * Python-style negative indices are also supported. That is, -1 can be
     * used to refer to the backend, -2 to the last operator, and so on.
     * \returns The plugin author.
     * \throws std::runtime_error When the given index is out of range, or when
     * the simulation is in an invalid state.
     */
    std::string get_author(ssize_t index) {
      char *ptr = check(raw::dqcs_sim_get_author_idx(handle, index));
      std::string str(ptr);
      std::free(ptr);
      return str;
    }

    /**
     * Queries the version of a plugin, referenced by instance name.
     *
     * \param name The instance name of the plugin to query.
     * \returns The plugin version.
     * \throws std::runtime_error When the given name does not identify a
     * plugin, or when the simulation is in an invalid state.
     */
    std::string get_version(const std::string &name) {
      char *ptr = check(raw::dqcs_sim_get_version(handle, name.c_str()));
      std::string str(ptr);
      std::free(ptr);
      return str;
    }

    /**
     * Queries the version of a plugin, referenced by index.
     *
     * \param index The index of the plugin to query. The frontend always has
     * index 0. 1 through N are used for the operators in front to back order
     * (where N is the number of operators). The backend is at index N+1.
     * Python-style negative indices are also supported. That is, -1 can be
     * used to refer to the backend, -2 to the last operator, and so on.
     * \returns The plugin version.
     * \throws std::runtime_error When the given index is out of range, or when
     * the simulation is in an invalid state.
     */
    std::string get_version(ssize_t index) {
      char *ptr = check(raw::dqcs_sim_get_version_idx(handle, index));
      std::string str(ptr);
      std::free(ptr);
      return str;
    }

    /**
     * Writes a reproduction file for the simulation so far.
     *
     * \param filename The file to write to.
     * \throws std::runtime_error When the simulation cannot be reproduced or
     * when it's in an invalid state.
     */
    void write_reproduction_file(const std::string &filename) {
      check(raw::dqcs_sim_write_reproduction_file(handle, filename.c_str()));
    }

  };

  /**
   * Wrapper class for configuring a simulation.
   */
  class SimulationConfiguration : public Handle {
  public:

    /**
     * Wraps the given simulation configuration handle.
     *
     * \note This constructor does not verify that the handle is actually
     * valid.
     *
     * \param handle The raw handle to wrap.
     */
    SimulationConfiguration(HandleIndex handle) noexcept : Handle(handle) {
    }

    /**
     * Creates a new simulation configuration.
     */
    SimulationConfiguration() : Handle(check(raw::dqcs_scfg_new())) {
    }

    // Delete copy construct/assign.
    SimulationConfiguration(const SimulationConfiguration&) = delete;
    void operator=(const SimulationConfiguration&) = delete;

    /**
     * Default move constructor.
     */
    SimulationConfiguration(SimulationConfiguration&&) = default;

    /**
     * Default move assignment.
     */
    SimulationConfiguration &operator=(SimulationConfiguration&&) = default;

    /**
     * Appends a plugin to a simulation configuration.
     *
     * Frontend and backend plugins will automatically be inserted at the front
     * and back of the pipeline when the simulation is created. Operators are
     * inserted in front to back order. This function does not provide
     * safeguards against multiple frontends/backends; such errors will only be
     * reported when the simulation is started.
     *
     * \note It is not possible to observe or mutate a plugin configuration
     * once it has been added to a simulator configuration handle. If you want
     * to do this for some reason, you should maintain your own data
     * structures, and only build the DQCsim structures from them when you're
     * done.
     *
     * \param plugin The plugin configuration object. If not constructed
     * in-place, this must be `std::move()`d into this method.
     * \throws std::runtime_error When the simulation or plugin configuration
     * handle is invalid for some reason.
     */
    void add_plugin(PluginConfiguration &&plugin) {
      check(raw::dqcs_scfg_push_plugin(handle, plugin.get_handle()));
    }

    /**
     * Appends a plugin to a simulation configuration (builder pattern).
     *
     * @see add_plugin()
     *
     * \param plugin The plugin configuration object. If not constructed
     * in-place, this must be `std::move()`d into this method.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the simulation or plugin configuration
     * handle is invalid for some reason.
     */
    SimulationConfiguration &&with_plugin(PluginConfiguration &&plugin) {
      add_plugin(std::move(plugin));
      return std::move(*this);
    }

    /**
     * Configures the random seed that the simulation should use.
     *
     * Note that the seed is randomized by default.
     *
     * \param seed The random seed to use.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    void set_seed(uint64_t seed) {
      check(raw::dqcs_scfg_seed_set(handle, seed));
    }

    /**
     * Configures the random seed that the simulation should use (builder
     * pattern).
     *
     * Note that the seed is randomized by default.
     *
     * \param seed The random seed to use.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    SimulationConfiguration &&with_seed(uint64_t seed) {
      set_seed(seed);
      return std::move(*this);
    }

    /**
     * Returns the configured random seed.
     *
     * \returns The configured random seed.
     *
     * \note Due to the C interface layer underneath, this function cannot
     * distinguish between seed 0 and an exception. Therefore, it is
     * `noexcept`.
     */
    uint64_t get_seed() const noexcept {
      return raw::dqcs_scfg_seed_get(handle);
    }

    /**
     * Sets the path style used when writing reproduction files.
     *
     * By default, the generated reproduction file will specify the plugin
     * executable and script paths as they were generated or specified.
     * However, depending on how you intend to reproduce the simulation later,
     * you may want purely relative or purely absolute paths instead. This
     * function sets the style used.
     *
     * \param style The path style to use.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    void set_reproduction_style(PathStyle style) {
      check(raw::dqcs_scfg_repro_path_style_set(handle, to_raw(style)));
    }

    /**
     * Sets the path style used when writing reproduction files (builder
     * pattern).
     *
     * By default, the generated reproduction file will specify the plugin
     * executable and script paths as they were generated or specified.
     * However, depending on how you intend to reproduce the simulation later,
     * you may want purely relative or purely absolute paths instead. This
     * function sets the style used.
     *
     * \param style The path style to use.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    SimulationConfiguration &&with_reproduction_style(PathStyle style) {
      set_reproduction_style(style);
      return std::move(*this);
    }

    /**
     * Returns the path style used when writing reproduction files.
     *
     * \returns The path style used when writing reproduction files.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    PathStyle get_reproduction_style() const {
      return check(raw::dqcs_scfg_repro_path_style_get(handle));
    }

    /**
     * Disables the reproduction logging system.
     *
     * Calling this will disable the warnings printed when a simulation that
     * cannot be reproduced is constructed.
     *
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    void disable_reproduction() {
      check(raw::dqcs_scfg_repro_disable(handle));
    }

    /**
     * Disables the reproduction logging system (builder pattern).
     *
     * Calling this will disable the warnings printed when a simulation that
     * cannot be reproduced is constructed.
     *
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    SimulationConfiguration &&without_reproduction() {
      check(raw::dqcs_scfg_repro_disable(handle));
      return std::move(*this);
    }

    /**
     * Configures the logging verbosity for DQCsim's own messages.
     *
     * \param level The verbosity level for DQCsim's own messages.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    void set_dqcsim_verbosity(Loglevel level) {
      check(raw::dqcs_scfg_dqcsim_verbosity_set(handle, to_raw(level)));
    }

    /**
     * Configures the logging verbosity for DQCsim's own messages (builder
     * pattern).
     *
     * \param level The verbosity level for DQCsim's own messages.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    SimulationConfiguration &&with_dqcsim_verbosity(Loglevel level) {
      set_dqcsim_verbosity(level);
      return std::move(*this);
    }

    /**
     * Returns the configured verbosity for DQCsim's own messages.
     *
     * \returns The configured verbosity for DQCsim's own messages.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    Loglevel get_dqcsim_verbosity() const {
      return check(raw::dqcs_scfg_dqcsim_verbosity_get(handle));
    }

    /**
     * Configures the stderr sink verbosity for a simulation.
     *
     * That is, the minimum loglevel that a messages needs to have for it to
     * be printed to stderr.
     *
     * \param level The verbosity level to set.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    void set_stderr_verbosity(Loglevel level) {
      check(raw::dqcs_scfg_stderr_verbosity_set(handle, to_raw(level)));
    }

    /**
     * Configures the stderr sink verbosity for a simulation (builder pattern).
     *
     * That is, the minimum loglevel that a messages needs to have for it to
     * be printed to stderr.
     *
     * \param level The verbosity level to set.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    SimulationConfiguration &&with_stderr_verbosity(Loglevel level) {
      set_stderr_verbosity(level);
      return std::move(*this);
    }

    /**
     * Returns the configured stderr sink verbosity for a simulation.
     *
     * That is, the minimum loglevel that a messages needs to have for it to
     * be printed to stderr.
     *
     * \returns The configured stderr sink verbosity for a simulation.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    Loglevel get_stderr_verbosity() const {
      return check(raw::dqcs_scfg_stderr_verbosity_get(handle));
    }

    /**
     * Configures DQCsim to also output its log messages to a file.
     *
     * \param verbosity The logging verbosity for the tee file.
     * \param filename The name of the file to log to.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    void log_tee(Loglevel verbosity, const std::string &filename) {
      return check(raw::dqcs_scfg_tee(handle, to_raw(verbosity), filename.c_str()));
    }

    /**
     * Configures DQCsim to also output its log messages to a file (builder
     * pattern).
     *
     * \param verbosity The logging verbosity for the tee file.
     * \param filename The name of the file to log to.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    SimulationConfiguration &&with_log_tee(Loglevel verbosity, const std::string &filename) {
      log_tee(verbosity, filename);
      return std::move(*this);
    }

  private:

    /**
     * Configures DQCsim to also output its log messages to callback function.
     *
     * `verbosity` specifies the minimum importance of a message required for
     * the callback to be called. `data` takes a pointer to the callback
     * information object, which must have been previously allocated using
     * `new`. Refer to `callback::Log` for more information.
     *
     * The primary use of this callback is to pipe DQCsim's messages to an
     * external logging framework. When you do this, you probably also want to
     * call `set_stderr_verbosity_set(Loglevel::Off)` to prevent DQCsim from
     * writing the messages to stderr itself.
     *
     * \note This callback may be called from a thread spawned by the
     * simulator. Calling any API calls from the callback is therefore
     * undefined behavior!
     *
     * \param verbosity The minimum loglevel needed for a log message for
     * the callback to be called.
     * \param data The wrapper object for the log callback.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    void set_log_callback_ptr(Loglevel verbosity, callback::Log *data) {
      check(raw::dqcs_scfg_log_callback(
        handle,
        to_raw(verbosity),
        CallbackEntryPoints::log,
        CallbackEntryPoints::user_free<callback::Log>,
        data
      ));
    }

  public:

    /**
     * Configures DQCsim to also output its log messages to callback function.
     *
     * `verbosity` specifies the minimum importance of a message required for
     * the callback to be called. `data` is the callback information object,
     * taken by copy by this function. Refer to `callback::Log` for more
     * information.
     *
     * \note This callback may be called from a thread spawned by the
     * simulator. Calling any API calls from the callback is therefore
     * undefined behavior!
     *
     * \param verbosity The minimum loglevel needed for a log message for
     * the callback to be called.
     * \param data The wrapper object for the log callback.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    void set_log_callback(Loglevel verbosity, const callback::Log &data) {
      set_log_callback_ptr(verbosity, new callback::Log(data));
    }

    /**
     * Configures DQCsim to also output its log messages to callback function.
     *
     * `verbosity` specifies the minimum importance of a message required for
     * the callback to be called. `data` is the callback information object,
     * taken by move by this function. Refer to `callback::Log` for more
     * information.
     *
     * \note This callback may be called from a thread spawned by the
     * simulator. Calling any API calls from the callback is therefore
     * undefined behavior!
     *
     * \param verbosity The minimum loglevel needed for a log message for
     * the callback to be called.
     * \param data The wrapper object for the log callback.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    void set_log_callback(Loglevel verbosity, callback::Log &&data) {
      set_log_callback_ptr(verbosity, new callback::Log(std::move(data)));
    }

    /**
     * Configures DQCsim to also output its log messages to callback function.
     *
     * `verbosity` specifies the minimum importance of a message required for
     * the callback to be called. The callback information object is
     * constructed in place from the remaining arguments. Refer to
     * `callback::Log` for more information.
     *
     * \note This callback may be called from a thread spawned by the
     * simulator. Calling any API calls from the callback is therefore
     * undefined behavior!
     *
     * \param verbosity The minimum loglevel needed for a log message for
     * the callback to be called.
     * \param args Any valid argument list for constructing `callback::Log`.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    template<typename... Args>
    void set_log_callback(Loglevel verbosity, Args... args) {
      set_log_callback_ptr(verbosity, new callback::Log(args...));
    }

    /**
     * Configures DQCsim to also output its log messages to callback function
     * (builder pattern).
     *
     * `verbosity` specifies the minimum importance of a message required for
     * the callback to be called. `data` is the callback information object,
     * taken by copy by this function. Refer to `callback::Log` for more
     * information.
     *
     * \note This callback may be called from a thread spawned by the
     * simulator. Calling any API calls from the callback is therefore
     * undefined behavior!
     *
     * \param verbosity The minimum loglevel needed for a log message for
     * the callback to be called.
     * \param data The wrapper object for the log callback.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    SimulationConfiguration &&with_log_callback(Loglevel verbosity, const callback::Log &data) {
      set_log_callback_ptr(verbosity, new callback::Log(data));
      return std::move(*this);
    }

    /**
     * Configures DQCsim to also output its log messages to callback function
     * (builder pattern).
     *
     * `verbosity` specifies the minimum importance of a message required for
     * the callback to be called. `data` is the callback information object,
     * taken by move by this function. Refer to `callback::Log` for more
     * information.
     *
     * \note This callback may be called from a thread spawned by the
     * simulator. Calling any API calls from the callback is therefore
     * undefined behavior!
     *
     * \param verbosity The minimum loglevel needed for a log message for
     * the callback to be called.
     * \param data The wrapper object for the log callback.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    SimulationConfiguration &&with_log_callback(Loglevel verbosity, callback::Log &&data) {
      set_log_callback_ptr(verbosity, new callback::Log(std::move(data)));
      return std::move(*this);
    }

    /**
     * Configures DQCsim to also output its log messages to callback function
     * (builder pattern).
     *
     * `verbosity` specifies the minimum importance of a message required for
     * the callback to be called. The callback information object is
     * constructed in place from the remaining arguments. Refer to
     * `callback::Log` for more information.
     *
     * \note This callback may be called from a thread spawned by the
     * simulator. Calling any API calls from the callback is therefore
     * undefined behavior!
     *
     * \param verbosity The minimum loglevel needed for a log message for
     * the callback to be called.
     * \param args Any valid argument list for constructing `callback::Log`.
     * \returns `&self`, to continue building.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason.
     */
    template<typename... Args>
    SimulationConfiguration &&with_log_callback(Loglevel verbosity, Args... args) {
      set_log_callback_ptr(verbosity, new callback::Log(args...));
      return std::move(*this);
    }

    /**
     * Constructs the DQCsim simulation from this configuration object.
     *
     * \note The builder object can only be used once. After calling
     * `build()`, the behavior of every other member function is undefined.
     *
     * \note It is currently not possible to have more than one simulation
     * handle within a single thread at the same time. This has to do with
     * DQCsim's log system, which uses thread-local storage to determine where
     * log messages should go. If you want to run multiple simulations in
     * parallel, you'll have to run them from different threads.
     *
     * \returns The constructed simulation object.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason, or initializing the simulation fails.
     */
    Simulation build() {
      return Simulation(check(raw::dqcs_sim_new(handle)));
    }

    /**
     * Constructs the DQCsim simulation from this configuration object, and
     * runs a program on the simulated accelerator without an argument.
     *
     * This is simply a shorthand for `build()` followed by
     * `Simulation::run()`. The accelerator return value is discarded in favor
     * of returning the simulation object, which you can then call
     * `write_reproduction_file()` on.
     *
     * \returns The constructed simulation object.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason, or running the simulation fails.
     */
    Simulation run() {
      Simulation sim = build();
      sim.run();
      return sim;
    }

    /**
     * Constructs the DQCsim simulation from this configuration object, and
     * runs a program on the simulated accelerator with the given `ArbData`
     * argument (passed by move).
     *
     * This is simply a shorthand for `build()` followed by
     * `Simulation::run()`. The accelerator return value is discarded in favor
     * of returning the simulation object, which you can then call
     * `write_reproduction_file()` on.
     *
     * \param data The `ArbData` object to pass to the frontend's `run`
     * callback.
     * \returns The constructed simulation object.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason, or running the simulation fails.
     */
    Simulation run(ArbData &&data) {
      Simulation sim = build();
      sim.run(std::move(data));
      return sim;
    }

    /**
     * Constructs the DQCsim simulation from this configuration object, and
     * runs a program on the simulated accelerator with the given `ArbData`
     * argument (passed by copy).
     *
     * This is simply a shorthand for `build()` followed by
     * `Simulation::run()`. The accelerator return value is discarded in favor
     * of returning the simulation object, which you can then call
     * `write_reproduction_file()` on.
     *
     * \param data The `ArbData` object to pass to the frontend's `run`
     * callback.
     * \returns The constructed simulation object.
     * \throws std::runtime_error When the simulation configuration handle is
     * invalid for some reason, or running the simulation fails.
     */
    Simulation run(const ArbData &data) {
      Simulation sim = build();
      sim.run(data);
      return sim;
    }

  };

} // namespace wrap

} // namespace dqcsim

#endif
