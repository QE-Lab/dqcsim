#ifndef _DQCSIM_INCLUDED_
//! \cond Doxygen_Suppress
#define _DQCSIM_INCLUDED_
//! \endcond

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
   */
  inline void check(raw::dqcs_return_t code) {
    if (code == raw::dqcs_return_t::DQCS_FAILURE) {
      throw std::runtime_error(raw::dqcs_error_get());
    }
  }

  /**
   * Checks a `dqcs_bool_return_t` return value; if failure, throws a runtime
   * error with DQCsim's error message.
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
   */
  inline size_t check(ssize_t size) {
    if (size < 0) {
      throw std::runtime_error(raw::dqcs_error_get());
    }
    return static_cast<size_t>(size);
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

    // TODO document these entries
    FrontendProcessConfig = 200,
    OperatorProcessConfig = 201,
    BackendProcessConfig = 203,
    FrontendThreadConfig = 204,
    OperatorThreadConfig = 205,
    BackendThreadConfig = 206,
    SimulationConfig = 207,
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
   */
  inline raw::dqcs_handle_type_t to_raw(HandleType type) {
    switch (type) {
      case HandleType::ArbData:               return raw::dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA;
      case HandleType::ArbCmd:                return raw::dqcs_handle_type_t::DQCS_HTYPE_ARB_CMD;
      case HandleType::ArbCmdQueue:           return raw::dqcs_handle_type_t::DQCS_HTYPE_ARB_CMD_QUEUE;
      case HandleType::QubitSet:              return raw::dqcs_handle_type_t::DQCS_HTYPE_QUBIT_SET;
      case HandleType::Gate:                  return raw::dqcs_handle_type_t::DQCS_HTYPE_GATE;
      case HandleType::Measurement:           return raw::dqcs_handle_type_t::DQCS_HTYPE_MEAS;
      case HandleType::MeasurementSet:        return raw::dqcs_handle_type_t::DQCS_HTYPE_MEAS_SET;
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
  }

  /**
   * Checks a `dqcs_handle_type_t` return value and converts it to its C++
   * enum representation; if failure, throws a runtime error with DQCsim's
   * error message.
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
   */
  inline raw::dqcs_loglevel_t to_raw(Loglevel loglevel) {
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
  }

  /**
   * Checks a `dqcs_loglevel_t` return value and converts it to its C++
   * enum representation; if failure, throws a runtime error with DQCsim's
   * error message.
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
   */
  inline raw::dqcs_measurement_t to_raw(MeasurementValue measurement) {
    switch (measurement) {
      case MeasurementValue::Zero:      return raw::dqcs_measurement_t::DQCS_MEAS_ZERO;
      case MeasurementValue::One:       return raw::dqcs_measurement_t::DQCS_MEAS_ONE;
      case MeasurementValue::Undefined: return raw::dqcs_measurement_t::DQCS_MEAS_UNDEFINED;
    }
  }

  /**
   * Checks a `dqcs_measurement_t` return value and converts it to its C++
   * enum representation; if failure, throws a runtime error with DQCsim's
   * error message.
   */
  inline MeasurementValue check(raw::dqcs_measurement_t measurement) {
    switch (measurement) {
      case raw::dqcs_measurement_t::DQCS_MEAS_ZERO:      return MeasurementValue::Zero;
      case raw::dqcs_measurement_t::DQCS_MEAS_ONE:       return MeasurementValue::One;
      case raw::dqcs_measurement_t::DQCS_MEAS_UNDEFINED: return MeasurementValue::Undefined;
      case raw::dqcs_measurement_t::DQCS_MEAS_INVALID:   throw std::runtime_error(raw::dqcs_error_get());
    }
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
   */
  inline raw::dqcs_path_style_t to_raw(PathStyle style) {
    switch (style) {
      case PathStyle::Keep:     return raw::dqcs_path_style_t::DQCS_PATH_STYLE_KEEP;
      case PathStyle::Relative: return raw::dqcs_path_style_t::DQCS_PATH_STYLE_RELATIVE;
      case PathStyle::Absolute: return raw::dqcs_path_style_t::DQCS_PATH_STYLE_ABSOLUTE;
    }
  }

  /**
   * Checks a `dqcs_path_style_t` return value and converts it to its C++
   * enum representation; if failure, throws a runtime error with DQCsim's
   * error message.
   */
  inline PathStyle check(raw::dqcs_path_style_t style) {
    switch (style) {
      case raw::dqcs_path_style_t::DQCS_PATH_STYLE_KEEP:      return PathStyle::Keep;
      case raw::dqcs_path_style_t::DQCS_PATH_STYLE_RELATIVE:  return PathStyle::Relative;
      case raw::dqcs_path_style_t::DQCS_PATH_STYLE_ABSOLUTE:  return PathStyle::Absolute;
      case raw::dqcs_path_style_t::DQCS_PATH_STYLE_INVALID:   throw std::runtime_error(raw::dqcs_error_get());
    }
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
   */
  inline raw::dqcs_plugin_type_t to_raw(PluginType type) {
    switch (type) {
      case PluginType::Frontend:  return raw::dqcs_plugin_type_t::DQCS_PTYPE_FRONT;
      case PluginType::Operator:  return raw::dqcs_plugin_type_t::DQCS_PTYPE_OPER;
      case PluginType::Backend:   return raw::dqcs_plugin_type_t::DQCS_PTYPE_BACK;
    }
  }

  /**
   * Checks a `dqcs_plugin_type_t` return value and converts it to its C++
   * enum representation; if failure, throws a runtime error with DQCsim's
   * error message.
   */
  inline PluginType check(raw::dqcs_plugin_type_t type) {
    switch (type) {
      case raw::dqcs_plugin_type_t::DQCS_PTYPE_FRONT:   return PluginType::Frontend;
      case raw::dqcs_plugin_type_t::DQCS_PTYPE_OPER:    return PluginType::Operator;
      case raw::dqcs_plugin_type_t::DQCS_PTYPE_BACK:    return PluginType::Backend;
      case raw::dqcs_plugin_type_t::DQCS_PTYPE_INVALID: throw std::runtime_error(raw::dqcs_error_get());
    }
  }

  /**
   * Checks a pointer return value; if failure, throws a runtime error with
   * DQCsim's error message.
   */
  template <typename T>
  inline T *check(T *pointer) {
    if (pointer == nullptr) {
      throw std::runtime_error(raw::dqcs_error_get());
    }
    return pointer;
  }

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
    Handle() : handle(0) {
    }

    /**
     * Wrap the given raw handle.
     *
     * \note This class will take ownership of the handle, i.e. it is in charge
     * of freeing it.
     */
    Handle(HandleIndex handle) : handle(handle) {
    }

    /**
     * Delete the handle and its wrapper.
     */
    virtual ~Handle() noexcept {
      if (handle) {
        raw::dqcs_handle_delete(handle);
      }
    }

    /**
     * Explicitly delete the handle, allowing errors to be caught.
     *
     * \note The wrapper no longer owns a handle after this call. That means
     * `is_valid` will return `false` and all other methods will likely fail.
     */
    void free() {
      check(raw::dqcs_handle_delete(handle));
      handle = 0;
    }

    /**
     * Returns whether this wrapper (still) contains a valid handle.
     */
    bool is_valid() const noexcept {
      return raw::dqcs_handle_type(handle) != raw::dqcs_handle_type_t::DQCS_HTYPE_INVALID;
    }

    /**
     * Returns the raw handle without relinquishing ownership.
     */
    HandleIndex get_handle() const noexcept {
      return handle;
    }

    /**
     * Returns the raw handle and relinquishes ownership.
     *
     * \note The wrapper no longer owns a handle after this call. That means
     * `is_valid` will return `false` and all other methods will likely fail.
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
     */
    Handle &operator=(Handle &&src) {
      if (handle) {
        free(handle);
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
     */
    friend std::ostream& operator<<(std::ostream &out, const Handle &handle) {
      out << handle.dump();
      return out;
    }

    /**
     * Returns the type of this handle.
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
     * Wrap the given `arb` handle.
     */
    Arb(HandleIndex handle) : Handle(handle) {
    }

    /**
     * Returns the current arbitrary JSON data as a serialized JSON string.
     */
    std::string get_arb_json_string() const {
      char *json_c = check(raw::dqcs_arb_json_get(handle));
      std::string json(json_c);
      std::free(json_c);
      return json;
    }

    /**
     * Sets the arbitrary JSON data to the given serialized JSON string.
     */
    void set_arb_json_string(const std::string &json) {
      check(raw::dqcs_arb_json_set(handle, json.c_str()));
    }

    /**
     * Returns the current arbitrary JSON data as a serialized CBOR string.
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
     */
    void set_arb_cbor_string(const std::string &cbor) {
      check(raw::dqcs_arb_cbor_set(handle, cbor.data(), cbor.size()));
    }

    /**
     * Returns the current arbitrary JSON data as a JSON object from
     * `nlohmann::json`. Since that is a header-only library that isn't usually
     * installed system-wide and be using a specific version in your project
     * already, you need to specify the `nlohmann::json` type as a generic to
     * this function.
     *
     * \warning This function returns a *copy* of the JSON data embedded in the
     * `ArbData`. Therefore, modifying the returned JSON object does *not*
     * modify the original `ArbData`. To modify, you need to pass the modified
     * JSON object to `set_arb_json()`.
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
     * `nlohmann::json`. Since that is a header-only library that isn't usually
     * installed system-wide and be using a specific version in your project
     * already, you need to specify the `nlohmann::json` type as a generic to
     * this function.
     */
    template <class JSON>
    void set_arb_json(const JSON &json) {
      std::vector<uint8_t> cbor = JSON::to_cbor(json);
      check(raw::dqcs_arb_cbor_set(handle, cbor.data(), cbor.size()));
    }

    /**
     * Returns the arbitrary argument at the given index as a (binary) string.
     * Negative indices are relative to the back of the list, as in Python.
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
     */
    template <typename T>
    void set_arb_arg(ssize_t index, const T &data) {
      check(raw::dqcs_arb_set_raw(handle, index, &data, sizeof(data)));
    }

    /**
     * Pushes a (binary) string to the back of the arbitrary argument list.
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
     */
    template <typename T>
    void push_arb_arg(const T &data) {
      check(raw::dqcs_arb_push_raw(handle, &data, sizeof(data)));
    }

    /**
     * Pops from the back of the arbitrary argument list as a (binary) string.
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
     * \warning Type `T` must be a primitive value (like an `int`) or a struct
     * thereof, without pointers or any other "complicated" constructs. DQCsim
     * just copies the bytes over. It is up to you to ensure that that's what
     * you want to happen; unfortunately C++11 does not provide a way to
     * statically ensure that this is the case.
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
     */
    template <typename T>
    void insert_arb_arg(ssize_t index, const T &data) {
      check(raw::dqcs_arb_insert_raw(handle, index, &data, sizeof(data)));
    }

    /**
     * Removes the arbitrary argument at the given index. Negative indices are
     * relative to the back of the list, as in Python.
     */
    void remove_arb_arg(ssize_t index) {
      check(raw::dqcs_arb_remove(handle, index));
    }

    /**
     * Returns the number of arbitrary arguments.
     */
    size_t get_arb_arg_count() const {
      return check(raw::dqcs_arb_len(handle));
    }

    /**
     * Clears the arbitrary argument list.
     */
    void clear_arb_args() {
      check(raw::dqcs_arb_clear(handle));
    }

    /**
     * Assigns all arb data from the given arb to this one.
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
     * Wrap the given `ArbData` handle.
     */
    ArbData(HandleIndex handle) : Arb(handle) {
    }

    /**
     * Constructs an empty `ArbData` object.
     */
    ArbData() : Arb(check(raw::dqcs_arb_new())) {
    }

    /**
     * Copy-constructs an `ArbData` object from any object supporting the `Arb`
     * interface.
     */
    ArbData(const Arb &src) : Arb(check(raw::dqcs_arb_new())) {
      set_arb(src);
    }

    /**
     * Copy-constructs an `ArbData` object from another `ArbData` object.
     */
    ArbData(const ArbData &src) : Arb(check(raw::dqcs_arb_new())) {
      set_arb(src);
    }

    /**
     * Copy assignment operator for `ArbData` objects.
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
   */
  class Cmd : public Arb {
  public:

    /**
     * Wrap the given `cmd` handle.
     */
    Cmd(HandleIndex handle) : Arb(handle) {
    }

    /**
     * Returns the interface identifier of this command.
     */
    std::string get_iface() const {
      char *iface_c = check(raw::dqcs_cmd_iface_get(handle));
      std::string iface(iface_c);
      std::free(iface_c);
      return iface;
    }

    /**
     * Returns whether this command has the given interface identifier.
     */
    bool is_iface(const std::string &iface) const {
      return check(raw::dqcs_cmd_iface_cmp(handle, iface.c_str()));
    }

    /**
     * Returns the operation identifier of this command.
     */
    std::string get_oper() const {
      char *oper_c = check(raw::dqcs_cmd_oper_get(handle));
      std::string oper(oper_c);
      std::free(oper_c);
      return oper;
    }

    /**
     * Returns whether this command has the given operation identifier.
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
     */
    ArbCmd(HandleIndex handle) : Cmd(handle) {
    }

    /**
     * Constructs an `ArbCmd` object.
     */
    ArbCmd(const std::string &iface, const std::string &oper) : Cmd(check(raw::dqcs_cmd_new(
      iface.c_str(), oper.c_str()
    ))) {
    }

    /**
     * Copy-constructs an `ArbCmd` object from any object supporting the `Cmd`
     * interface.
     */
    ArbCmd(const Cmd &src) : Cmd(check(raw::dqcs_cmd_new(
      src.get_iface().c_str(), src.get_oper().c_str()
    ))) {
      set_arb(src);
    }

    /**
     * Copy-constructs an `ArbCmd` object from another `ArbCmd` object.
     */
    ArbCmd(const ArbCmd &src) : Cmd(check(raw::dqcs_cmd_new(
      src.get_iface().c_str(), src.get_oper().c_str()
    ))) {
      set_arb(src);
    }

    /**
     * Copy assignment operator for `ArbCmd` objects.
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
     * Wrap the given `ArbCmd` handle.
     */
    ArbCmdQueue(HandleIndex handle) : Cmd(handle) {
    }

    /**
     * Constructs an empty `ArbCmd` queue object.
     */
    ArbCmdQueue() : Cmd(check(raw::dqcs_cq_new())) {
    }

    /**
     * Constructs an `ArbCmd` queue object from an iterable of `ArbCmd`s by
     * copying.
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
     * Constructs an `ArbCmd` queue object from an iterable of `ArbCmd`s by
     * moving.
     */
    template <class T>
    static ArbCmdQueue from_iter(T &&cmds) {
      ArbCmdQueue result;
      for (const Cmd &cmd : cmds) {
        result.push(std::move(cmd));
      }
      return result;
    }

    /**
     * Pushes an `ArbCmd` into the queue by copying.
     */
    void push(const Cmd &cmd) {
      push(std::move(ArbCmd(cmd)));
    }

    /**
     * Pushes an `ArbCmd` into the queue by moving.
     */
    void push(ArbCmd &&cmd) {
      check(raw::dqcs_cq_push(handle, cmd.get_handle()));
    }

    /**
     * Pushes an `ArbCmd` into the queue by copying (builder pattern).
     */
    ArbCmdQueue &with(const Cmd &cmd) {
      push(cmd);
      return *this;
    }

    /**
     * Pushes an `ArbCmd` into the queue by moving (builder pattern).
     */
    ArbCmdQueue &with(ArbCmd &&cmd) {
      push(std::move(cmd));
      return *this;
    }

    /**
     * Pops the first `ArbCmd` from the queue, allowing the next one to be
     * accessed.
     */
    void next() {
      check(raw::dqcs_cq_next(handle));
    }

    /**
     * Returns the number of `ArbCmd`s in the queue.
     */
    size_t size() const {
      return check(raw::dqcs_cq_len(handle));
    }

    /**
     * Drains the queue into a vector of `ArbCmd`s. This is less performant
     * than iterating over the queue manually, because it requires copies.
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
     */
    std::vector<ArbCmd> copy_into_vector() {
      std::vector<ArbCmd> cmds = drain_into_vector();
      free();
      handle = ArbCmdQueue::from_iter(cmds).take_handle();
      return cmds;
    }

    /**
     * Copy-constructs a queue of `ArbCmd`s. This requires destructive
     * iteration of the source object, so it isn't not const; if an exception
     * occurs, the state of the source object may be changed.
     */
    ArbCmdQueue(ArbCmdQueue &src) : Cmd(0) {
      handle = ArbCmdQueue::from_iter(src.copy_into_vector()).take_handle();
    }

    /**
     * Copy-assigns a queue of `ArbCmd`s. This requires destructive
     * iteration of the source object, so it isn't not const; if an exception
     * occurs, the state of the source object may be changed.
     */
    ArbCmdQueue &operator=(ArbCmdQueue &src) {
      free();
      handle = ArbCmdQueue::from_iter(src.copy_into_vector()).take_handle();
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
    QubitRef(QubitRef &&handle) = default;

    /**
     * Default move assignment.
     */
    QubitRef &operator=(QubitRef&&) = default;

    /**
     * Qubit reference equality operator.
     */
    bool operator==(const QubitRef &other) const {
      return index == other.index;
    }

    /**
     * Qubit reference inequality operator.
     */
    bool operator!=(const QubitRef &other) const {
      return index != other.index;
    }

    /**
     * Allow qubit references to be printed.
     */
    friend std::ostream& operator<<(std::ostream &out, const QubitRef &qubit) {
      out << 'q' << qubit.index;
      return out;
    }

    /**
     * Returns the raw qubit index.
     */
    QubitIndex get_index() const {
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
     * Wrap the given qubit set handle.
     */
    QubitSet(HandleIndex handle) : Handle(handle) {
    }

    /**
     * Constructs an empty qubit set.
     */
    QubitSet() : Handle(check(raw::dqcs_qbset_new())) {
    }

    /**
     * Constructs a qubit set object from an iterable of qubit references.
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
     */
    QubitSet(const QubitSet &src) : Handle(check(raw::dqcs_qbset_copy(src.handle))) {
    }

    /**
     * Copy assignment operator for qubit sets.
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
     */
    void push(const QubitRef &qubit) {
      check(raw::dqcs_qbset_push(handle, qubit.get_index()));
    }

    /**
     * Pushes a qubit into the set (builder pattern). Note that qubit sets are
     * ordered. An exception is thrown if the qubit is already in the set.
     */
    QubitSet &with(const QubitRef &qubit) {
      push(qubit);
      return *this;
    }

    /**
     * Pops a qubit from the set. Qubits are popped in the same order in which
     * they are pushed (like a FIFO).
     */
    QubitRef pop() {
      return QubitRef(check(raw::dqcs_qbset_pop(handle)));
    }

    /**
     * Returns the number of qubits in the set.
     */
    size_t size() const {
      return check(raw::dqcs_qbset_len(handle));
    }

    /**
     * Returns whether the given qubit is contained in the set.
     */
    bool contains(const QubitRef &qubit) const {
      return check(raw::dqcs_qbset_contains(handle, qubit.get_index()));
    }

    /**
     * Drains the qubit set into a vector.
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
     */
    std::vector<QubitRef> copy_into_vector() const {
      QubitSet copy(*this);
      return copy.drain_into_vector();
    }

  };

  /**
   * Convenience class for the square complex matrices used to express the
   * qubit gates.
   */
  class Matrix {
  private:

    /**
     * Row-major data storage.
     */
    std::vector<std::complex<double>> d;

    /**
     * Number of rows == number of columns. So we don't have to compute the
     * sqrt of the vector size all the time.
     */
    const size_t n;

  public:

    /**
     * Delete the default constructor, as it's nonsensical with no size
     * parameter.
     */
    Matrix() = delete;

    /**
     * Constructs an identity matrix of the given size.
     */
    Matrix(size_t size) : d(size * size, std::complex<double>(0.0, 0.0)), n(size) {
      for (size_t i = 0; i < n; i++) {
        (*this)(i, i) = std::complex<double>(1.0, 0.0);
      }
    }

    /**
     * Constructs a matrix from a row-major flattened array of `size` x `size`
     * `std::complex<double>`s.
     */
    Matrix(size_t size, const std::complex<double> *data) : d(size*size), n(size) {
      std::memcpy(d.data(), data, d.size() * sizeof(std::complex<double>));
    }

    /**
     * Constructs a matrix from a row-major, real-first flattened array of
     * 2 x `size` x `size` `double`s.
     */
    Matrix(size_t size, const double *data) : d(size*size), n(size) {
      std::memcpy(d.data(), data, d.size() * sizeof(std::complex<double>));
    }

    /**
     * Default copy constructor.
     */
    Matrix(const Matrix&) = default;

    /**
     * Default copy assignment.
     */
    Matrix &operator=(const Matrix&) = default;

    /**
     * Default move constructor.
     */
    Matrix(Matrix&&) = default;

    /**
     * Default move assignment.
     */
    Matrix &operator=(Matrix&&) = default;

    /**
     * Mutable matrix element accessor.
     */
    std::complex<double>& operator()(size_t row, size_t column) {
      if (row >= n || column >= n) {
        throw std::invalid_argument("matrix subscript out of bounds");
      }
      return d[n*row + column];
    }

    /**
     * Const matrix element accessor.
     */
    const std::complex<double>& operator()(size_t row, size_t column) const {
      if (row >= n || column >= n) {
        throw std::invalid_argument("matrix subscript out of bounds");
      }
      return d[n*row + column];
    }

    /**
     * Mutable matrix flattened data accessor.
     */
    std::complex<double> *data() {
      return d.data();
    }

    /**
     * Const matrix flattened data accessor.
     */
    const std::complex<double> *data() const {
      return d.data();
    }

    /**
     * Returns the size of the matrix (number of rows = number of columns).
     */
    size_t size() const {
      return n;
    }

    /**
     * Allow matrices to be printed.
     */
    friend std::ostream& operator<<(std::ostream &out, const Matrix &matrix) {
      out << '{';
      for (size_t row = 0; row < matrix.size(); row++) {
        if (row) out << ", ";
        out << '[';
        for (size_t col = 0; col < matrix.size(); col++) {
          if (col) out << ", ";
          auto e = matrix(row, col);
          if (e.real() != 0.0) {
            out << e.real();
            if (e.imag() < 0.0) {
              out << '-' << -e.imag() << 'i';
            } else if (e.imag() > 0.0) {
              out << '+' << e.imag() << 'i';
            }
          } else if (e.imag()) {
            out << e.imag() << 'i';
          } else {
            out << '0';
          }
        }
        out << ']';
      }
      out << '}';
      return out;
    }

    /**
     * Matrix equality operator.
     */
    bool operator==(const Matrix &other) const {
      return d == other.d;
    }

    /**
     * Matrix inequality operator.
     */
    bool operator!=(const Matrix &other) const {
      return d != other.d;
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
  private:

    /**
     * Integer square root.
     */
    template <typename T>
    static T isqrt(T n) {
      T c = (T)1 << (sizeof(T) * 4 - 1);
      if (c < 0) {
        c = (T)1 << (sizeof(T) * 4 - 2);
      }
      T g = c;
      while (true) {
        if (g*g > n) {
          g ^= c;
        }
        c >>= 1;
        if (c == 0) {
          return g;
        }
        g |= c;
      }
    }

  public:

    /**
     * Wrap the given `Gate` handle.
     */
    Gate(HandleIndex handle) : Arb(handle) {
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
     * The matrix must be appropriately sized for the number of qubits in the
     * `targets` qubit set (2^n by 2^n).
     */
    static Gate unitary(QubitSet &&targets, const Matrix &matrix) {
      return Gate(check(raw::dqcs_gate_new_unitary(
        targets.get_handle(),
        0,
        reinterpret_cast<const double*>(matrix.data()),
        matrix.size() * matrix.size()
      )));
    }

    /**
     * Constructs a new unitary gate.
     *
     * The matrix must be appropriately sized for the number of qubits in the
     * `targets` qubit set (2^n by 2^n).
     */
    static Gate unitary(const QubitSet &targets, const Matrix &matrix) {
      return unitary(std::move(QubitSet(targets)), matrix);
    }

    /**
     * Constructs a new unitary gate with control qubits.
     *
     * The matrix must be appropriately sized for the number of qubits in the
     * `targets` qubit set (2^n by 2^n). The control qubits do not count toward
     * n; the backend will supplement the gate matrix as needed. The `targets`
     * and `controls` qubit sets must be disjoint.
     */
    static Gate unitary(QubitSet &&targets, QubitSet &&controls, const Matrix &matrix) {
      return Gate(check(raw::dqcs_gate_new_unitary(
        targets.get_handle(),
        controls.get_handle(),
        reinterpret_cast<const double*>(matrix.data()),
        matrix.size() * matrix.size()
      )));
    }

    /**
     * Constructs a new unitary gate with control qubits.
     *
     * The matrix must be appropriately sized for the number of qubits in the
     * `targets` qubit set (2^n by 2^n). The control qubits do not count toward
     * n; the backend will supplement the gate matrix as needed. The `targets`
     * and `controls` qubit sets must be disjoint.
     */
    static Gate unitary(const QubitSet &targets, const QubitSet &controls, const Matrix &matrix) {
      return unitary(std::move(QubitSet(targets)), std::move(QubitSet(controls)), matrix);
    }

    /**
     * Constructs a new Z-axis measurement gate.
     *
     * Exactly those qubits in the `measures` set must be measured. The results
     * can be queried from `PluginState` after the gate is executed. Any
     * previous measurement results for those qubits will be overridden.
     */
    static Gate measure(QubitSet &&measures) {
      return Gate(check(raw::dqcs_gate_new_measurement(measures.get_handle())));
    }

    /**
     * Constructs a new Z-axis measurement gate.
     *
     * Exactly those qubits in the `measures` set must be measured. The results
     * can be queried from `PluginState` after the gate is executed. Any
     * previous measurement results for those qubits will be overridden.
     */
    static Gate measure(const QubitSet &measures) {
      return measure(std::move(QubitSet(measures)));
    }

    /**
     * Constructs a new custom gate with target qubits, control qubits,
     * measured qubits, and a matrix.
     *
     * The `targets` and `controls` qubit sets must be disjoint.
     *
     * Exactly those qubits in the `measures` set must be measured. The results
     * can be queried from `PluginState` after the gate is executed. Any
     * previous measurement results for those qubits will be overridden.
     *
     * The matrix must be appropriately sized for the number of qubits in the
     * `targets` qubit set (2^n by 2^n). The control qubits do not count toward
     * n.
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
        reinterpret_cast<const double*>(matrix.data()),
        matrix.size() * matrix.size()
      )));
    }

    /**
     * Constructs a new custom gate with target qubits, control qubits,
     * measured qubits, and a matrix.
     *
     * The `targets` and `controls` qubit sets must be disjoint.
     *
     * Exactly those qubits in the `measures` set must be measured. The results
     * can be queried from `PluginState` after the gate is executed. Any
     * previous measurement results for those qubits will be overridden.
     *
     * The matrix must be appropriately sized for the number of qubits in the
     * `targets` qubit set (2^n by 2^n). The control qubits do not count toward
     * n.
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
        std::move(QubitSet(targets)),
        std::move(QubitSet(controls)),
        std::move(QubitSet(measures)),
        matrix
      );
    }

    /**
     * Constructs a new custom gate with target qubits, control qubits, and
     * measured qubits.
     *
     * The `targets` and `controls` qubit sets must be disjoint.
     *
     * Exactly those qubits in the `measures` set must be measured. The results
     * can be queried from `PluginState` after the gate is executed. Any
     * previous measurement results for those qubits will be overridden.
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
        nullptr,
        0
      )));
    }

    /**
     * Constructs a new custom gate with target qubits, control qubits, and
     * measured qubits.
     *
     * The `targets` and `controls` qubit sets must be disjoint.
     *
     * Exactly those qubits in the `measures` set must be measured. The results
     * can be queried from `PluginState` after the gate is executed. Any
     * previous measurement results for those qubits will be overridden.
     */
    static Gate custom(
      const std::string &name,
      const QubitSet &targets,
      const QubitSet &controls,
      const QubitSet &measures
    ) {
      return custom(
        name,
        std::move(QubitSet(targets)),
        std::move(QubitSet(controls)),
        std::move(QubitSet(measures))
      );
    }

    /**
     * Constructs a new custom gate with target qubits, control qubits,
     * and a matrix.
     *
     * The `targets` and `controls` qubit sets must be disjoint.
     *
     * The matrix must be appropriately sized for the number of qubits in the
     * `targets` qubit set (2^n by 2^n). The control qubits do not count toward
     * n.
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
        reinterpret_cast<const double*>(matrix.data()),
        matrix.size() * matrix.size()
      )));
    }

    /**
     * Constructs a new custom gate with target qubits, control qubits,
     * and a matrix.
     *
     * The `targets` and `controls` qubit sets must be disjoint.
     *
     * The matrix must be appropriately sized for the number of qubits in the
     * `targets` qubit set (2^n by 2^n). The control qubits do not count toward
     * n.
     */
    static Gate custom(
      const std::string &name,
      const QubitSet &targets,
      const QubitSet &controls,
      const Matrix &matrix
    ) {
      return custom(
        name,
        std::move(QubitSet(targets)),
        std::move(QubitSet(controls)),
        matrix
      );
    }

    /**
     * Constructs a new custom gate with target qubits and control qubits.
     *
     * The `targets` and `controls` qubit sets must be disjoint.
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
        nullptr,
        0
      )));
    }

    /**
     * Constructs a new custom gate with target qubits and control qubits.
     *
     * The `targets` and `controls` qubit sets must be disjoint.
     */
    static Gate custom(
      const std::string &name,
      const QubitSet &targets,
      const QubitSet &controls
    ) {
      return custom(
        name,
        std::move(QubitSet(targets)),
        std::move(QubitSet(controls))
      );
    }

    /**
     * Constructs a new custom gate with target qubits and a matrix.
     *
     * The matrix must be appropriately sized for the number of qubits in the
     * `targets` qubit set (2^n by 2^n).
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
        reinterpret_cast<const double*>(matrix.data()),
        matrix.size() * matrix.size()
      )));
    }

    /**
     * Constructs a new custom gate with target qubits and a matrix.
     *
     * The matrix must be appropriately sized for the number of qubits in the
     * `targets` qubit set (2^n by 2^n).
     */
    static Gate custom(
      const std::string &name,
      const QubitSet &targets,
      const Matrix &matrix
    ) {
      return custom(
        name,
        std::move(QubitSet(targets)),
        matrix
      );
    }

    /**
     * Constructs a new custom gate with only target qubits.
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
        nullptr,
        0
      )));
    }

    /**
     * Constructs a new custom gate with only target qubits.
     */
    static Gate custom(
      const std::string &name,
      const QubitSet &targets
    ) {
      return custom(
        name,
        std::move(QubitSet(targets))
      );
    }

    /**
     * Constructs a new custom gate without qubit operands.
     */
    static Gate custom(
      const std::string &name
    ) {
      return Gate(check(raw::dqcs_gate_new_custom(
        name.c_str(),
        0,
        0,
        0,
        nullptr,
        0
      )));
    }

    /**
     * Returns a new qubit reference set with the target qubits for this gate.
     */
    QubitSet get_targets() const {
      return QubitSet(check(raw::dqcs_gate_targets(handle)));
    }

    /**
     * Returns whether this gate has target qubits.
     */
    bool has_targets() const {
      return check(raw::dqcs_gate_has_targets(handle));
    }

    /**
     * Returns a new qubit reference set with the control qubits for this gate.
     */
    QubitSet get_controls() const {
      return QubitSet(check(raw::dqcs_gate_controls(handle)));
    }

    /**
     * Returns whether this gate has control qubits.
     */
    bool has_controls() const {
      return check(raw::dqcs_gate_has_controls(handle));
    }

    /**
     * Returns a new qubit reference set with the measurement qubits for this
     * gate.
     */
    QubitSet get_measures() const {
      return QubitSet(check(raw::dqcs_gate_measures(handle)));
    }

    /**
     * Returns whether this gate has measurement qubits.
     */
    bool has_measures() const {
      return check(raw::dqcs_gate_has_measures(handle));
    }

    /**
     * Returns the matrix that belongs to this gate.
     */
    Matrix get_matrix() const {
      double *data = check(raw::dqcs_gate_matrix(handle));
      Matrix matrix(isqrt(check(raw::dqcs_gate_matrix_len(handle))), data);
      std::free(data);
      return matrix;
    }

    /**
     * Returns whether this gate has a matrix.
     */
    bool has_matrix() const {
      return check(raw::dqcs_gate_has_matrix(handle));
    }

    /**
     * Returns the name of a custom gate.
     */
    std::string get_name() const {
      char *data = check(raw::dqcs_gate_name(handle));
      std::string name(data);
      std::free(data);
      return name;
    }

    /**
     * Returns whether this gate is a custom gate.
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
     * Wrap the given measurement handle.
     */
    Measurement(HandleIndex handle) : Arb(handle) {
    }

    /**
     * Constructs a measurement object.
     */
    Measurement(const QubitRef &qubit, MeasurementValue value) : Arb(check(
      raw::dqcs_meas_new(qubit.get_index(), to_raw(value))
    )) {
    }

    /**
     * Copy-constructs a `Measurement` object.
     */
    Measurement(const Measurement &src) : Arb(check(
      raw::dqcs_meas_new(src.get_qubit().get_index(), to_raw(src.get_value()))
    )) {
      set_arb(src);
    }

    /**
     * Copy assignment operator for `Measurement` objects.
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
     */
    MeasurementValue get_value() const {
      return check(raw::dqcs_meas_value_get(handle));
    }

    /**
     * Sets the measurement value.
     */
    void set_value(MeasurementValue value) {
      check(raw::dqcs_meas_value_set(handle, to_raw(value)));
    }

    /**
     * Returns the qubit reference associated with this measurement.
     */
    QubitRef get_qubit() const {
      return QubitRef(check(raw::dqcs_meas_qubit_get(handle)));
    }

    /**
     * Sets the qubit reference associated with this measurement.
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
     * Wrap the given measurement set handle.
     */
    MeasurementSet(HandleIndex handle) : Handle(handle) {
    }

    /**
     * Constructs an empty measurement set.
     */
    MeasurementSet() : Handle(check(raw::dqcs_mset_new())) {
    }

    /**
     * Constructs a measurement set object from an iterable of measurements.
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
     * Copies the given measurement object into the set. If the set already
     * contained measurement data for the qubit associated with the measurement
     * object, the previous measurement data is overwritten.
     */
    void set(const Measurement &measurement) {
      Measurement copy = measurement;
      check(raw::dqcs_mset_set(handle, copy.get_handle()));
    }

    /**
     * Moves the given measurement object into the set. If the set already
     * contained measurement data for the qubit associated with the measurement
     * object, the previous measurement data is overwritten.
     */
    void set(Measurement &&measurement) {
      check(raw::dqcs_mset_set(handle, measurement.get_handle()));
    }

    /**
     * Copies the given measurement object into the set (builder pattern). If
     * the set already contained measurement data for the qubit associated with
     * the measurement object, the previous measurement data is overwritten.
     */
    MeasurementSet &with(const Measurement &measurement) {
      set(measurement);
    }

    /**
     * Moves the given measurement object into the set (builder pattern). If
     * the set already contained measurement data for the qubit associated with
     * the measurement object, the previous measurement data is overwritten.
     */
    MeasurementSet &with(Measurement &&measurement) {
      set(std::move(measurement));
    }

    /**
     * Returns a copy of the measurement object for the given qubit. An
     * exception is thrown if no data is available for this qubit.
     */
    Measurement get(const QubitRef &qubit) const {
      return Measurement(check(raw::dqcs_mset_get(handle, qubit.get_index())));
    }

    /**
     * Moves the measurement object for the given qubit out of the set. An
     * exception is thrown if no data is available for this qubit.
     */
    Measurement take(const QubitRef &qubit) {
      return Measurement(check(raw::dqcs_mset_take(handle, qubit.get_index())));
    }

    /**
     * Moves any measurement object out of the set. An exception is thrown if
     * the set is empty.
     */
    Measurement take_any() {
      return Measurement(check(raw::dqcs_mset_take_any(handle)));
    }

    /**
     * Removes the measurement object for the given qubit from the set.
     */
    void remove(const QubitRef &qubit) {
      check(raw::dqcs_mset_remove(handle, qubit.get_index()));
    }

    /**
     * Returns the number of measurements in the set.
     */
    size_t size() const {
      return check(raw::dqcs_mset_len(handle));
    }

    /**
     * Returns whether the set contains measurement data for the given qubit.
     */
    bool contains(const QubitRef &qubit) const {
      return check(raw::dqcs_mset_contains(handle, qubit.get_index()));
    }

    /**
     * Drains the measurement set into a vector.
     */
    std::vector<Measurement> drain_into_vector() {
      std::vector<Measurement> measurements;
      while (size()) {
        measurements.emplace_back(take_any());
      }
      return measurements;
    }

    /**
     * Copies the qubit set into a vector. This requires destructive iteration,
     * so the function is not const; if an exception occurs, the state of the
     * measurement set may be changed.
     */
    std::vector<Measurement> copy_into_vector() {
      std::vector<Measurement> vector = drain_into_vector();
      MeasurementSet copy = MeasurementSet::from_iter(vector);
      free();
      handle = copy.take_handle();
      return vector;
    }

    /**
     * Copy-constructs a measurement set object. This requires destructive
     * iteration of the source object, so it isn't not const; if an exception
     * occurs, the state of the source object may be changed.
     */
    MeasurementSet(MeasurementSet &src) : Handle(0) {
      handle = MeasurementSet::from_iter(src.copy_into_vector()).take_handle();
    }

    /**
     * Copy-assigns a measurement set object. This requires destructive
     * iteration of the source object, so it isn't not const; if an exception
     * occurs, the state of the source object may be changed.
     */
    MeasurementSet &operator=(MeasurementSet &src) {
      free();
      handle = MeasurementSet::from_iter(src.copy_into_vector()).take_handle();
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

} // namespace wrap

/**
 * Namespace for the plugin callback function wrappers.
 */
namespace callback {

  /**
   * Helper macro to prevent code repetition; not visible outside of the header.
   */
  #define DQCSIM_CALLBACK_FRIENDS                             \
    friend raw::dqcs_return_t cb_entry_initialize(            \
      void *user_data,                                        \
      raw::dqcs_plugin_state_t state,                         \
      raw::dqcs_handle_t init_cmds                            \
    );                                                        \
    friend raw::dqcs_return_t cb_entry_drop(                  \
      void *user_data,                                        \
      raw::dqcs_plugin_state_t state                          \
    );                                                        \
    friend raw::dqcs_handle_t cb_entry_run(                   \
      void *user_data,                                        \
      raw::dqcs_plugin_state_t state,                         \
      raw::dqcs_handle_t args                                 \
    );                                                        \
    friend raw::dqcs_return_t cb_entry_allocate(              \
      void *user_data,                                        \
      raw::dqcs_plugin_state_t state,                         \
      raw::dqcs_handle_t qubits,                              \
      raw::dqcs_handle_t alloc_cmds                           \
    );                                                        \
    friend raw::dqcs_return_t cb_entry_free(                  \
      void *user_data,                                        \
      raw::dqcs_plugin_state_t state,                         \
      raw::dqcs_handle_t qubits                               \
    );                                                        \
    friend raw::dqcs_handle_t cb_entry_gate(                  \
      void *user_data,                                        \
      raw::dqcs_plugin_state_t state,                         \
      raw::dqcs_handle_t gate                                 \
    );                                                        \
    friend raw::dqcs_handle_t cb_entry_modify_measurement(    \
      void *user_data,                                        \
      raw::dqcs_plugin_state_t state,                         \
      raw::dqcs_handle_t meas                                 \
    );                                                        \
    friend raw::dqcs_return_t cb_entry_advance(               \
      void *user_data,                                        \
      raw::dqcs_plugin_state_t state,                         \
      wrap::Cycle cycles                                      \
    );                                                        \
    friend raw::dqcs_handle_t cb_entry_upstream_arb(          \
      void *user_data,                                        \
      raw::dqcs_plugin_state_t state,                         \
      raw::dqcs_handle_t cmd                                  \
    );                                                        \
    friend raw::dqcs_handle_t cb_entry_host_arb(              \
      void *user_data,                                        \
      raw::dqcs_plugin_state_t state,                         \
      raw::dqcs_handle_t cmd                                  \
    );

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
    DQCSIM_CALLBACK_FRIENDS

  public:

    // Delete the copy and move constructors and assignments.
    UpstreamPluginState(const UpstreamPluginState&) = delete;
    void operator=(const UpstreamPluginState&) = delete;
    UpstreamPluginState(UpstreamPluginState&&) = delete;
    UpstreamPluginState &operator=(UpstreamPluginState&&) = delete;

    /**
     * Generates a random floating point number using the simulator random
     * seed. The generated numbers are uniformly distributed between 0
     * (inclusive) and 1 (exclusive).
     */
    double random_f64() {
      return raw::dqcs_plugin_random_f64(state);
    }

    /**
     * Generates a random integer using the simulator random seed.
     */
    unsigned long long random_u64() {
      return raw::dqcs_plugin_random_u64(state);
    }

    /**
     * Generates a random value using the simulator random seed. All bits are
     * randomized with a 50/50 probability.
     */
    template <typename T>
    T random() {
      unsigned long long data[(sizeof(T) + 7) / 8];
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
    DQCSIM_CALLBACK_FRIENDS

  public:

    // Delete the copy and move constructors and assignments.
    PluginState(const PluginState&) = delete;
    void operator=(const PluginState&) = delete;
    PluginState(PluginState&&) = delete;
    PluginState &operator=(PluginState&&) = delete;

    /**
     * Allocates a number of downstream qubits, moving in the given command
     * queue as arbitrary additional data for the qubits.
     *
     * Backend plugins are not allowed to call this. Doing so will result in an
     * error.
     */
    wrap::QubitSet allocate(size_t num_qubits, wrap::ArbCmdQueue &queue) {
      return allocate(num_qubits, std::move(wrap::ArbCmdQueue(queue)));
    }

    /**
     * Allocates a number of downstream qubits, copying in the given command
     * queue as arbitrary additional data for the qubits.
     *
     * Backend plugins are not allowed to call this. Doing so will result in an
     * error.
     */
    wrap::QubitSet allocate(size_t num_qubits, wrap::ArbCmdQueue &&queue) {
      return wrap::QubitSet(wrap::check(raw::dqcs_plugin_allocate(state, num_qubits, queue.get_handle())));
    }

    /**
     * Allocates a number of default downstream qubits.
     *
     * Backend plugins are not allowed to call this. Doing so will result in an
     * error.
     */
    wrap::QubitSet allocate(size_t num_qubits) {
      return wrap::QubitSet(wrap::check(raw::dqcs_plugin_allocate(state, num_qubits, 0)));
    }

    /**
     * Allocates a single downstream qubit, moving in the given command queue
     * as arbitrary additional data for the qubits.
     *
     * Backend plugins are not allowed to call this. Doing so will result in an
     * error.
     */
    wrap::QubitRef allocate(wrap::ArbCmdQueue &queue) {
      return allocate(std::move(wrap::ArbCmdQueue(queue)));
    }

    /**
     * Allocates a single downstream qubit, copying in the given command queue
     * as arbitrary additional data for the qubits.
     *
     * Backend plugins are not allowed to call this. Doing so will result in an
     * error.
     */
    wrap::QubitRef allocate(wrap::ArbCmdQueue &&queue) {
      return allocate(1, std::move(queue)).pop();
    }

    /**
     * Allocates a single downstream qubit.
     *
     * Backend plugins are not allowed to call this. Doing so will result in an
     * error.
     */
    wrap::QubitRef allocate() {
      return allocate(1).pop();
    }

    /**
     * Frees the given downstream qubits.
     *
     * Backend plugins are not allowed to call this. Doing so will result in an
     * error.
     */
    void free(const wrap::QubitSet &qubits) {
      free(std::move(wrap::QubitSet(qubits)));
    }

    /**
     * Frees the given downstream qubits.
     *
     * Backend plugins are not allowed to call this. Doing so will result in an
     * error.
     */
    void free(wrap::QubitSet &&qubits) {
      wrap::check(raw::dqcs_plugin_free(state, qubits.get_handle()));
    }

    /**
     * Frees the given downstream qubit.
     *
     * Backend plugins are not allowed to call this. Doing so will result in an
     * error.
     */
    void free(const wrap::QubitRef &qubit) {
      free(std::move(wrap::QubitRef(qubit)));
    }

    /**
     * Frees the given downstream qubit.
     *
     * Backend plugins are not allowed to call this. Doing so will result in an
     * error.
     */
    void free(wrap::QubitRef &&qubit) {
      free(std::move(wrap::QubitSet().with(std::move(qubit))));
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
    DQCSIM_CALLBACK_FRIENDS

  public:

    // Delete the copy and move constructors and assignments.
    RunningPluginState(const RunningPluginState&) = delete;
    void operator=(const RunningPluginState&) = delete;
    RunningPluginState(RunningPluginState&&) = delete;
    RunningPluginState &operator=(RunningPluginState&&) = delete;

    /**
     * Sends a message to the host.
     */
    void send(const wrap::ArbData &message) {
      send(std::move(wrap::ArbData(message)));
    }

    /**
     * Sends a message to the host.
     */
    void send(wrap::ArbData &&message) {
      wrap::check(raw::dqcs_plugin_send(state, message.get_handle()));
    }

    /**
     * Receives a message from the host.
     */
    wrap::ArbData receive() {
      return wrap::ArbData(wrap::check(raw::dqcs_plugin_recv(state)));
    }

  };

  /**
   * `std::bind` helper function for the `Callback` template class; one
   * argument, C-style user data.
   */
  template <class T, class R, class A>
  std::function<R(A)> bind_first(R (*cb)(T, A), T user) {
    using namespace std::placeholders;
    return std::bind(cb, user, _1);
  }

  /**
   * `std::bind` helper function for the `Callback` template class; two
   * arguments, C-style user data.
   */
  template <class T, class R, class A, class B>
  std::function<R(A, B)> bind_first(R (*cb)(T, A, B), T user) {
    using namespace std::placeholders;
    return std::bind(cb, user, _1, _2);
  }

  /**
   * `std::bind` helper function for the `Callback` template class; three
   * arguments, C-style user data.
   */
  template <class T, class R, class A, class B, class C>
  std::function<R(A, B, C)> bind_first(R (*cb)(T, A, B, C), T user) {
    using namespace std::placeholders;
    return std::bind(cb, user, _1, _2, _3);
  }

  /**
   * `std::bind` helper function for the `Callback` template class; one
   * argument, member function.
   */
  template <class T, class R, class A>
  std::function<R(A)> bind_instance(T *instance, R (T::*cb)(A)) {
    using namespace std::placeholders;
    return std::bind(cb, instance, _1);
  }

  /**
   * `std::bind` helper function for the `Callback` template class; two
   * arguments, member function.
   */
  template <class T, class R, class A, class B>
  std::function<R(A, B)> bind_instance(T *instance, R (T::*cb)(A, B)) {
    using namespace std::placeholders;
    return std::bind(cb, instance, _1, _2);
  }

  /**
   * `std::bind` helper function for the `Callback` template class; three
   * arguments, member function.
   */
  template <class T, class R, class A, class B, class C>
  std::function<R(A, B, C)> bind_instance(T *instance, R (T::*cb)(A, B, C)) {
    using namespace std::placeholders;
    return std::bind(cb, instance, _1, _2, _3);
  }

  /**
   * Class template shared between all callback functions.
   */
  template <class R, class... Args>
  class Callback {
  private:

    /**
     * The stored callback.
     */
    std::shared_ptr<std::function<R(Args...)>> cb;

    // Allow the C-style callbacks access to this class.
    DQCSIM_CALLBACK_FRIENDS

  public:

    /**
     * Constructs the callback wrapper from a regular C-style function.
     */
    Callback(R (*cb)(Args...))
      : cb(std::make_shared<std::function<R(Args...)>>(cb))
    {}

    /**
     * Constructs the callback wrapper from a regular C-style function with a
     * user argument bound to it. Usually this would be a pointer to some
     * shared data structure containing the user's plugin state.
     */
    template <class T>
    Callback(R (*cb)(T, Args...), T user)
      : cb(std::make_shared<std::function<R(Args...)>>(bind_first<T, R, Args...>(cb, user)))
    {}

    /**
     * Constructs the callback wrapper from a member function.
     */
    template <class T>
    Callback(T *instance, R (T::*cb)(Args...))
      : cb(std::make_shared<std::function<R(Args...)>>(bind_instance<T, R, Args...>(instance, cb)))
    {}

    /**
     * Constructs the callback wrapper by copying a `std::function`.
     */
    Callback(const std::function<R(Args...)> &cb)
      : cb(std::make_shared<std::function<R(Args...)>>(cb))
    {}

    /**
     * Constructs the callback wrapper by moving a `std::function`.
     */
    Callback(std::function<R(Args...)> &&cb)
      : cb(std::make_shared<std::function<R(Args...)>>(std::move(cb)))
    {}

    /**
     * Constructs the callback wrapper by means of a copying a `shared_ptr`
     * to a `std::function`.
     */
    Callback(const std::shared_ptr<std::function<R(Args...)>> &cb) : cb(cb) {
    }

    /**
     * Constructs the callback wrapper by means of moving a `shared_ptr`
     * to a `std::function`.
     */
    Callback(std::shared_ptr<std::function<R(Args...)>> &&cb) : cb(cb) {
    }

  };

  #undef DQCSIM_CALLBACK_FRIENDS

  /**
   * Callback wrapper specialized for the `initialize` callback.
   */
  typedef Callback<void, PluginState&, wrap::ArbCmdQueue&&> Initialize;

  /**
   * Entry point for the `initialize` callback.
   */
  raw::dqcs_return_t cb_entry_initialize(
    void *user_data,
    raw::dqcs_plugin_state_t state,
    raw::dqcs_handle_t init_cmds
  ) {

    // Wrap inputs.
    Initialize *cb_wrapper = reinterpret_cast<Initialize*>(user_data);
    PluginState state_wrapper(state);
    wrap::ArbCmdQueue init_cmds_wrapper(init_cmds);

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
   * Callback wrapper specialized for the `drop` callback.
   */
  typedef Callback<void, PluginState&> Drop;

  /**
   * Entry point for the `drop` callback.
   */
  raw::dqcs_return_t cb_entry_drop(
    void *user_data,
    raw::dqcs_plugin_state_t state
  ) {

    // Wrap inputs.
    Drop *cb_wrapper = reinterpret_cast<Drop*>(user_data);
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
   * Callback wrapper specialized for the `run` callback.
   */
  typedef Callback<wrap::ArbData, RunningPluginState&, wrap::ArbData&&> Run;

  /**
   * Entry point for the `run` callback.
   */
  raw::dqcs_handle_t cb_entry_run(
    void *user_data,
    raw::dqcs_plugin_state_t state,
    raw::dqcs_handle_t args
  ) {

    // Wrap inputs.
    Run *cb_wrapper = reinterpret_cast<Run*>(user_data);
    RunningPluginState state_wrapper(state);
    wrap::ArbData args_wrapper(args);

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
   * Callback wrapper specialized for the `allocate` callback.
   */
  typedef Callback<void, PluginState&, wrap::QubitSet&&, wrap::ArbCmdQueue&&> Allocate;

  /**
   * Entry point for the `allocate` callback.
   */
  raw::dqcs_return_t cb_entry_allocate(
    void *user_data,
    raw::dqcs_plugin_state_t state,
    raw::dqcs_handle_t qubits,
    raw::dqcs_handle_t alloc_cmds
  ) {

    // Wrap inputs.
    Allocate *cb_wrapper = reinterpret_cast<Allocate*>(user_data);
    PluginState state_wrapper(state);
    wrap::QubitSet qubits_wrapper(qubits);
    wrap::ArbCmdQueue alloc_cmds_wrapper(alloc_cmds);

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
   * Callback wrapper specialized for the `allocate` callback.
   */
  typedef Callback<void, PluginState&, wrap::QubitSet&&> Free;

  /**
   * Entry point for the `free` callback.
   */
  raw::dqcs_return_t cb_entry_free(
    void *user_data,
    raw::dqcs_plugin_state_t state,
    raw::dqcs_handle_t qubits
  ) {

    // Wrap inputs.
    Free *cb_wrapper = reinterpret_cast<Free*>(user_data);
    PluginState state_wrapper(state);
    wrap::QubitSet qubits_wrapper(qubits);

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
   * Callback wrapper specialized for the `gate` callback.
   */
  typedef Callback<wrap::MeasurementSet, PluginState&, wrap::Gate&&> Gate;

  /**
   * Entry point for the `gate` callback.
   */
  raw::dqcs_handle_t cb_entry_gate(
    void *user_data,
    raw::dqcs_plugin_state_t state,
    raw::dqcs_handle_t gate
  ) {

    // Wrap inputs.
    Gate *cb_wrapper = reinterpret_cast<Gate*>(user_data);
    PluginState state_wrapper(state);
    wrap::Gate gate_wrapper(gate);

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
   * Callback wrapper specialized for the `modify_measurement` callback.
   */
  typedef Callback<wrap::MeasurementSet, UpstreamPluginState&, wrap::Measurement&&> ModifyMeasurement;

  /**
   * Entry point for the `modify_measurement` callback.
   */
  raw::dqcs_handle_t cb_entry_modify_measurement(
    void *user_data,
    raw::dqcs_plugin_state_t state,
    raw::dqcs_handle_t meas
  ) {

    // Wrap inputs.
    ModifyMeasurement *cb_wrapper = reinterpret_cast<ModifyMeasurement*>(user_data);
    UpstreamPluginState state_wrapper(state);
    wrap::Measurement meas_wrapper(meas);

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
   * Callback wrapper specialized for the `advance` callback.
   */
  typedef Callback<void, PluginState&, wrap::Cycle> Advance;

  /**
   * Entry point for the `advance` callback.
   */
  raw::dqcs_return_t cb_entry_advance(
    void *user_data,
    raw::dqcs_plugin_state_t state,
    wrap::Cycle cycles
  ) {

    // Wrap inputs.
    Advance *cb_wrapper = reinterpret_cast<Advance*>(user_data);
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
   * Callback wrapper specialized for the `*_arb` callbacks.
   */
  typedef Callback<wrap::ArbData, PluginState&, wrap::ArbCmd> Arb;

  /**
   * Entry point for the `upstream_arb` callback.
   */
  raw::dqcs_handle_t cb_entry_upstream_arb(
    void *user_data,
    raw::dqcs_plugin_state_t state,
    raw::dqcs_handle_t cmd
  ) {

    // Wrap inputs.
    Arb *cb_wrapper = reinterpret_cast<Arb*>(user_data);
    PluginState state_wrapper(state);
    wrap::ArbCmd cmd_wrapper(cmd);

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
  raw::dqcs_handle_t cb_entry_host_arb(
    void *user_data,
    raw::dqcs_plugin_state_t state,
    raw::dqcs_handle_t cmd
  ) {

    // Wrap inputs.
    Arb *cb_wrapper = reinterpret_cast<Arb*>(user_data);
    PluginState state_wrapper(state);
    wrap::ArbCmd cmd_wrapper(cmd);

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
   * Entry point for freeing callback data structures.
   */
  template <class T>
  void cb_entry_user_free(void *user_data) {
    T *cb_wrapper = reinterpret_cast<T*>(user_data);
    delete cb_wrapper;
  }

} // namespace callback

namespace wrap {

  /**
   * Expose `PluginState` outside of the `callback` namespace. The only reason
   * it's in there is because otherwise I can't get the callback entry point
   * `friend` semantics to work right.
   */
  using PluginState = callback::PluginState;

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
     * Wrap the given plugin definition handle.
     */
    PluginJoinHandle(HandleIndex handle) : Handle(handle) {
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
   * between these qubits, and override the free, gate, and modify-measurement
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
   * modify-measurement, this behavior is correct. Backends must override this
   * callback, and frontends do not support it.
   *
   * Note that for our silly example operator, the default behavior for this
   * function is actually sufficient; you'd only have to override the
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
   * downstream plugin. Even if you override this callback, you should maintain
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
     * Wrap the given plugin definition handle.
     */
    Plugin(HandleIndex handle) : Handle(handle) {
    }

    /**
     * Constructs a new plugin object.
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
     */
    PluginType get_type() const {
      return check(raw::dqcs_pdef_type(handle));
    }

    /**
     * Returns the name of the described plugin.
     */
    std::string get_name() const {
      return std::string(check(raw::dqcs_pdef_name(handle)));
    }

    /**
     * Returns the author of the described plugin.
     */
    std::string get_author() const {
      return std::string(check(raw::dqcs_pdef_author(handle)));
    }

    /**
     * Returns the version of the described plugin.
     */
    std::string get_version() const {
      return std::string(check(raw::dqcs_pdef_version(handle)));
    }

    /**
     * Constructs a new frontend.
     */
    static Plugin front(
      const std::string &name,
      const std::string &author,
      const std::string &version
    ) {
      return Plugin(PluginType::Frontend, name, author, version);
    }

    /**
     * Constructs a new operator.
     */
    static Plugin oper(
      const std::string &name,
      const std::string &author,
      const std::string &version
    ) {
      return Plugin(PluginType::Operator, name, author, version);
    }

    /**
     * Constructs a new backend.
     */
    static Plugin back(
      const std::string &name,
      const std::string &author,
      const std::string &version
    ) {
      return Plugin(PluginType::Backend, name, author, version);
    }

    // Code below is generated using the following Python script:
    //
    // print('    // Code below is generated using the following Python script:')
    // with open(__file__, 'r') as f:
    //     print(''.join(map(lambda x: '    // ' + x, f.readlines())), end='')
    //
    // template = """
    //   private:
    //
    //     /**
    //      * Assigns the {0[0]} callback function from a `new`-initialized
    //      * raw pointer to a `{0[2]}` object. Callee will ensure that
    //      * `delete` is called.
    //      */
    //     void set_{0[1]}({0[2]} *data) {{
    //       try {{
    //         check(raw::dqcs_pdef_set_{0[1]}_cb(
    //           handle,
    //           callback::cb_entry_{0[1]},
    //           callback::cb_entry_user_free<{0[2]}>,
    //           data));
    //       }} catch (...) {{
    //         delete data;
    //         throw;
    //       }}
    //     }}
    //
    //   public:
    //
    //     /**
    //      * Assigns the {0[0]} callback function from a pre-existing
    //      * `{0[2]}` object by copy.
    //      */
    //     Plugin &with_{0[1]}(const {0[2]} &data) {{
    //       set_{0[1]}(new {0[2]}(data));
    //       return *this;
    //     }}
    //
    //     /**
    //      * Assigns the {0[0]} callback function from a pre-existing
    //      * `{0[2]}` object by move.
    //      */
    //     Plugin &with_{0[1]}({0[2]} &&data) {{
    //       set_{0[1]}(new {0[2]}(std::move(data)));
    //       return *this;
    //     }}
    //
    //     /**
    //      * Assigns the {0[0]} callback function by constructing the
    //      * callback object implicitly.
    //      */
    //     template<typename... Args>
    //     Plugin &with_{0[1]}(Args... args) {{
    //       set_{0[1]}(new {0[2]}(args...));
    //       return *this;
    //     }}
    // """
    //
    // print(''.join(map(template.format, [
    //     ('initialize',          'initialize',           'callback::Initialize'),
    //     ('drop',                'drop',                 'callback::Drop'),
    //     ('run',                 'run',                  'callback::Run'),
    //     ('allocate',            'allocate',             'callback::Allocate'),
    //     ('free',                'free',                 'callback::Free'),
    //     ('gate',                'gate',                 'callback::Gate'),
    //     ('modify-measurement',  'modify_measurement',   'callback::ModifyMeasurement'),
    //     ('advance',             'advance',              'callback::Advance'),
    //     ('upstream-arb',        'upstream_arb',         'callback::Arb'),
    //     ('host-arb',            'host_arb',             'callback::Arb'),
    // ])))
    //
    // print('    // End of generated code.')

  private:

    /**
     * Assigns the initialize callback function from a `new`-initialized
     * raw pointer to a `callback::Initialize` object. Callee will ensure that
     * `delete` is called.
     */
    void set_initialize(callback::Initialize *data) {
      try {
        check(raw::dqcs_pdef_set_initialize_cb(
          handle,
          callback::cb_entry_initialize,
          callback::cb_entry_user_free<callback::Initialize>,
          data));
      } catch (...) {
        delete data;
        throw;
      }
    }

  public:

    /**
     * Assigns the initialize callback function from a pre-existing
     * `callback::Initialize` object by copy.
     */
    Plugin &with_initialize(const callback::Initialize &data) {
      set_initialize(new callback::Initialize(data));
      return *this;
    }

    /**
     * Assigns the initialize callback function from a pre-existing
     * `callback::Initialize` object by move.
     */
    Plugin &with_initialize(callback::Initialize &&data) {
      set_initialize(new callback::Initialize(std::move(data)));
      return *this;
    }

    /**
     * Assigns the initialize callback function by constructing the
     * callback object implicitly.
     */
    template<typename... Args>
    Plugin &with_initialize(Args... args) {
      set_initialize(new callback::Initialize(args...));
      return *this;
    }

  private:

    /**
     * Assigns the drop callback function from a `new`-initialized
     * raw pointer to a `callback::Drop` object. Callee will ensure that
     * `delete` is called.
     */
    void set_drop(callback::Drop *data) {
      try {
        check(raw::dqcs_pdef_set_drop_cb(
          handle,
          callback::cb_entry_drop,
          callback::cb_entry_user_free<callback::Drop>,
          data));
      } catch (...) {
        delete data;
        throw;
      }
    }

  public:

    /**
     * Assigns the drop callback function from a pre-existing
     * `callback::Drop` object by copy.
     */
    Plugin &with_drop(const callback::Drop &data) {
      set_drop(new callback::Drop(data));
      return *this;
    }

    /**
     * Assigns the drop callback function from a pre-existing
     * `callback::Drop` object by move.
     */
    Plugin &with_drop(callback::Drop &&data) {
      set_drop(new callback::Drop(std::move(data)));
      return *this;
    }

    /**
     * Assigns the drop callback function by constructing the
     * callback object implicitly.
     */
    template<typename... Args>
    Plugin &with_drop(Args... args) {
      set_drop(new callback::Drop(args...));
      return *this;
    }

  private:

    /**
     * Assigns the run callback function from a `new`-initialized
     * raw pointer to a `callback::Run` object. Callee will ensure that
     * `delete` is called.
     */
    void set_run(callback::Run *data) {
      try {
        check(raw::dqcs_pdef_set_run_cb(
          handle,
          callback::cb_entry_run,
          callback::cb_entry_user_free<callback::Run>,
          data));
      } catch (...) {
        delete data;
        throw;
      }
    }

  public:

    /**
     * Assigns the run callback function from a pre-existing
     * `callback::Run` object by copy.
     */
    Plugin &with_run(const callback::Run &data) {
      set_run(new callback::Run(data));
      return *this;
    }

    /**
     * Assigns the run callback function from a pre-existing
     * `callback::Run` object by move.
     */
    Plugin &with_run(callback::Run &&data) {
      set_run(new callback::Run(std::move(data)));
      return *this;
    }

    /**
     * Assigns the run callback function by constructing the
     * callback object implicitly.
     */
    template<typename... Args>
    Plugin &with_run(Args... args) {
      set_run(new callback::Run(args...));
      return *this;
    }

  private:

    /**
     * Assigns the allocate callback function from a `new`-initialized
     * raw pointer to a `callback::Allocate` object. Callee will ensure that
     * `delete` is called.
     */
    void set_allocate(callback::Allocate *data) {
      try {
        check(raw::dqcs_pdef_set_allocate_cb(
          handle,
          callback::cb_entry_allocate,
          callback::cb_entry_user_free<callback::Allocate>,
          data));
      } catch (...) {
        delete data;
        throw;
      }
    }

  public:

    /**
     * Assigns the allocate callback function from a pre-existing
     * `callback::Allocate` object by copy.
     */
    Plugin &with_allocate(const callback::Allocate &data) {
      set_allocate(new callback::Allocate(data));
      return *this;
    }

    /**
     * Assigns the allocate callback function from a pre-existing
     * `callback::Allocate` object by move.
     */
    Plugin &with_allocate(callback::Allocate &&data) {
      set_allocate(new callback::Allocate(std::move(data)));
      return *this;
    }

    /**
     * Assigns the allocate callback function by constructing the
     * callback object implicitly.
     */
    template<typename... Args>
    Plugin &with_allocate(Args... args) {
      set_allocate(new callback::Allocate(args...));
      return *this;
    }

  private:

    /**
     * Assigns the free callback function from a `new`-initialized
     * raw pointer to a `callback::Free` object. Callee will ensure that
     * `delete` is called.
     */
    void set_free(callback::Free *data) {
      try {
        check(raw::dqcs_pdef_set_free_cb(
          handle,
          callback::cb_entry_free,
          callback::cb_entry_user_free<callback::Free>,
          data));
      } catch (...) {
        delete data;
        throw;
      }
    }

  public:

    /**
     * Assigns the free callback function from a pre-existing
     * `callback::Free` object by copy.
     */
    Plugin &with_free(const callback::Free &data) {
      set_free(new callback::Free(data));
      return *this;
    }

    /**
     * Assigns the free callback function from a pre-existing
     * `callback::Free` object by move.
     */
    Plugin &with_free(callback::Free &&data) {
      set_free(new callback::Free(std::move(data)));
      return *this;
    }

    /**
     * Assigns the free callback function by constructing the
     * callback object implicitly.
     */
    template<typename... Args>
    Plugin &with_free(Args... args) {
      set_free(new callback::Free(args...));
      return *this;
    }

  private:

    /**
     * Assigns the gate callback function from a `new`-initialized
     * raw pointer to a `callback::Gate` object. Callee will ensure that
     * `delete` is called.
     */
    void set_gate(callback::Gate *data) {
      try {
        check(raw::dqcs_pdef_set_gate_cb(
          handle,
          callback::cb_entry_gate,
          callback::cb_entry_user_free<callback::Gate>,
          data));
      } catch (...) {
        delete data;
        throw;
      }
    }

  public:

    /**
     * Assigns the gate callback function from a pre-existing
     * `callback::Gate` object by copy.
     */
    Plugin &with_gate(const callback::Gate &data) {
      set_gate(new callback::Gate(data));
      return *this;
    }

    /**
     * Assigns the gate callback function from a pre-existing
     * `callback::Gate` object by move.
     */
    Plugin &with_gate(callback::Gate &&data) {
      set_gate(new callback::Gate(std::move(data)));
      return *this;
    }

    /**
     * Assigns the gate callback function by constructing the
     * callback object implicitly.
     */
    template<typename... Args>
    Plugin &with_gate(Args... args) {
      set_gate(new callback::Gate(args...));
      return *this;
    }

  private:

    /**
     * Assigns the modify-measurement callback function from a `new`-initialized
     * raw pointer to a `callback::ModifyMeasurement` object. Callee will ensure that
     * `delete` is called.
     */
    void set_modify_measurement(callback::ModifyMeasurement *data) {
      try {
        check(raw::dqcs_pdef_set_modify_measurement_cb(
          handle,
          callback::cb_entry_modify_measurement,
          callback::cb_entry_user_free<callback::ModifyMeasurement>,
          data));
      } catch (...) {
        delete data;
        throw;
      }
    }

  public:

    /**
     * Assigns the modify-measurement callback function from a pre-existing
     * `callback::ModifyMeasurement` object by copy.
     */
    Plugin &with_modify_measurement(const callback::ModifyMeasurement &data) {
      set_modify_measurement(new callback::ModifyMeasurement(data));
      return *this;
    }

    /**
     * Assigns the modify-measurement callback function from a pre-existing
     * `callback::ModifyMeasurement` object by move.
     */
    Plugin &with_modify_measurement(callback::ModifyMeasurement &&data) {
      set_modify_measurement(new callback::ModifyMeasurement(std::move(data)));
      return *this;
    }

    /**
     * Assigns the modify-measurement callback function by constructing the
     * callback object implicitly.
     */
    template<typename... Args>
    Plugin &with_modify_measurement(Args... args) {
      set_modify_measurement(new callback::ModifyMeasurement(args...));
      return *this;
    }

  private:

    /**
     * Assigns the advance callback function from a `new`-initialized
     * raw pointer to a `callback::Advance` object. Callee will ensure that
     * `delete` is called.
     */
    void set_advance(callback::Advance *data) {
      try {
        check(raw::dqcs_pdef_set_advance_cb(
          handle,
          callback::cb_entry_advance,
          callback::cb_entry_user_free<callback::Advance>,
          data));
      } catch (...) {
        delete data;
        throw;
      }
    }

  public:

    /**
     * Assigns the advance callback function from a pre-existing
     * `callback::Advance` object by copy.
     */
    Plugin &with_advance(const callback::Advance &data) {
      set_advance(new callback::Advance(data));
      return *this;
    }

    /**
     * Assigns the advance callback function from a pre-existing
     * `callback::Advance` object by move.
     */
    Plugin &with_advance(callback::Advance &&data) {
      set_advance(new callback::Advance(std::move(data)));
      return *this;
    }

    /**
     * Assigns the advance callback function by constructing the
     * callback object implicitly.
     */
    template<typename... Args>
    Plugin &with_advance(Args... args) {
      set_advance(new callback::Advance(args...));
      return *this;
    }

  private:

    /**
     * Assigns the upstream-arb callback function from a `new`-initialized
     * raw pointer to a `callback::Arb` object. Callee will ensure that
     * `delete` is called.
     */
    void set_upstream_arb(callback::Arb *data) {
      try {
        check(raw::dqcs_pdef_set_upstream_arb_cb(
          handle,
          callback::cb_entry_upstream_arb,
          callback::cb_entry_user_free<callback::Arb>,
          data));
      } catch (...) {
        delete data;
        throw;
      }
    }

  public:

    /**
     * Assigns the upstream-arb callback function from a pre-existing
     * `callback::Arb` object by copy.
     */
    Plugin &with_upstream_arb(const callback::Arb &data) {
      set_upstream_arb(new callback::Arb(data));
      return *this;
    }

    /**
     * Assigns the upstream-arb callback function from a pre-existing
     * `callback::Arb` object by move.
     */
    Plugin &with_upstream_arb(callback::Arb &&data) {
      set_upstream_arb(new callback::Arb(std::move(data)));
      return *this;
    }

    /**
     * Assigns the upstream-arb callback function by constructing the
     * callback object implicitly.
     */
    template<typename... Args>
    Plugin &with_upstream_arb(Args... args) {
      set_upstream_arb(new callback::Arb(args...));
      return *this;
    }

  private:

    /**
     * Assigns the host-arb callback function from a `new`-initialized
     * raw pointer to a `callback::Arb` object. Callee will ensure that
     * `delete` is called.
     */
    void set_host_arb(callback::Arb *data) {
      try {
        check(raw::dqcs_pdef_set_host_arb_cb(
          handle,
          callback::cb_entry_host_arb,
          callback::cb_entry_user_free<callback::Arb>,
          data));
      } catch (...) {
        delete data;
        throw;
      }
    }

  public:

    /**
     * Assigns the host-arb callback function from a pre-existing
     * `callback::Arb` object by copy.
     */
    Plugin &with_host_arb(const callback::Arb &data) {
      set_host_arb(new callback::Arb(data));
      return *this;
    }

    /**
     * Assigns the host-arb callback function from a pre-existing
     * `callback::Arb` object by move.
     */
    Plugin &with_host_arb(callback::Arb &&data) {
      set_host_arb(new callback::Arb(std::move(data)));
      return *this;
    }

    /**
     * Assigns the host-arb callback function by constructing the
     * callback object implicitly.
     */
    template<typename... Args>
    Plugin &with_host_arb(Args... args) {
      set_host_arb(new callback::Arb(args...));
      return *this;
    }

    // End of generated code.

    /**
     * Runs the defined plugin in the current thread. Throws an exception on
     * failure. This is normally the last statement executed in a plugin
     * executable.
     *
     * The `simulator` argument should come from the first (for normal plugins)
     * or second (for script-interpreting plugins) command line argument. In
     * the latter case, the first argument is the script filename.
     */
    void run(const char *simulator) {
      check(raw::dqcs_plugin_run(handle, simulator));
      take_handle();
    }

    /**
     * Starts the defined plugin in the current thread. Throws an exception on
     * failure. Returns a `PluginJoinHandle` object that allows the plugin to
     * be waited on.
     *
     * The `simulator` argument should come from the first (for normal plugins)
     * or second (for script-interpreting plugins) command line argument. In
     * the latter case, the first argument is the script filename.
     */
    PluginJoinHandle start(const char *simulator) {
      PluginJoinHandle join_handle(check(raw::dqcs_plugin_start(handle, simulator)));
      take_handle();
      return join_handle;
    }

  };

} // namespace wrap

} // namespace dqcsim

#endif
