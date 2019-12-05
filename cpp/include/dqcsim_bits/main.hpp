#ifndef _DQCSIM_INCLUDED_
#define _DQCSIM_INCLUDED_

#include <stdexcept>
#include <string>
#include <vector>
#include <cstring>
#include <iostream>
#include <complex>
#include <functional>
#include <memory>
#include <cdqcsim>

namespace dqcsim {

/**
 * Namespace containing thin wrapper objects around the handles exposed by
 * the raw C interface.
 */
namespace wrap {

  /**
   * C++-styled type name for `raw::dqcs_handle_t`.
   */
  using HandleIndex = raw::dqcs_handle_t;

  /**
   * C++-styled type name for `raw::dqcs_qubit_t`.
   */
  using QubitIndex = raw::dqcs_qubit_t;

  /**
   * C++-styled type name for `raw::dqcs_cycle_t`.
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
   * More C++-like wrapper for `raw::handle_type_t`, not including the
   * `invalid` option (since we use exceptions to communicate failure).
   */
  enum class HandleType {
    ArbData = 100,
    ArbCmd = 101,
    ArbCmdQueue = 102,
    QubitSet = 103,
    Gate = 104,
    Measurement = 105,
    MeasurementSet = 106,
    FrontendProcessConfig = 200,
    OperatorProcessConfig = 201,
    BackendProcessConfig = 203,
    FrontendThreadConfig = 204,
    OperatorThreadConfig = 205,
    BackendThreadConfig = 206,
    SimulationConfig = 207,
    Simulation = 208,
    FrontendDefinition = 300,
    OperatorDefinition = 301,
    BackendDefinition = 302,
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
   * More C++-like wrapper for `raw::dqcs_loglevel_t`, not including the
   * `invalid` option (since we use exceptions to communicate failure).
   */
  enum class Loglevel {
    Off = 0,
    Fatal = 1,
    Error = 2,
    Warn = 3,
    Note = 4,
    Info = 5,
    Debug = 6,
    Trace = 7,
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
   * More C++-like wrapper for `raw::dqcs_measurement_t`, not including the
   * `invalid` option (since we use exceptions to communicate failure).
   */
  enum class MeasurementValue {
    Zero = 0,
    One = 1,
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
   * More C++-like wrapper for `raw::dqcs_path_style_t`, not including the
   * `invalid` option (since we use exceptions to communicate failure).
   */
  enum class PathStyle {
    Keep = 0,
    Relative = 1,
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
   * More C++-like wrapper for `raw::dqcs_plugin_type_t`, not including the
   * `invalid` option (since we use exceptions to communicate failure).
   */
  enum class PluginType {
    Frontend = 0,
    Operator = 1,
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
   */
  class Handle {
  protected:

    /**
     * The wrapped handle.
     */
    HandleIndex handle;

  public:

    /**
     * Wrap the given raw handle.
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
     * Unwrap the raw handle; that is, without deleting it. By moving the
     * wrapper into the static function (in conjunction with the lack of a copy
     * constructor) the compiler can statically check that the wrapper object
     * is not reused.
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
     * Move constructor; simply move ownership of the handle.
     */
    Handle(Handle &&src) : handle(src.handle) {
      src.handle = 0;
    }

    /**
     * Move assignment; simply move ownership of the handle.
     */
    Handle &operator=(Handle &&src) {
      handle = src.handle;
      src.handle = 0;
      return *this;
    }

    /**
     * Returns a string containing a debug dump of the handle.
     */
    std::string dump() const {
      char *dump_c = check(raw::dqcs_handle_dump(handle));
      std::string dump(dump_c);
      std::free(dump_c);
      return dump;
    }

    /**
     * Write the debug dump string of the handle to the given output stream.
     */
    friend std::ostream& operator<<(std::ostream &out, const Handle &handle) {
      out << handle.dump();
      return out;
    }

    /**
     * Return the raw handle type for the given handle.
     */
    HandleType type() const {
      return check(raw::dqcs_handle_type(handle));
    }

  };

  /**
   * Class wrapper for handles that support the `arb` interface.
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
     * WARNING: this function returns a *copy* of the JSON data embedded in the
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
     * WARNING: type `T` must be a primitive value (like an `int`) or a struct
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
     * WARNING: type `T` must be a primitive value (like an `int`) or a struct
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
     * WARNING: type `T` must be a primitive value (like an `int`) or a struct
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
     * WARNING: type `T` must be a primitive value (like an `int`) or a struct
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
     * WARNING: type `T` must be a primitive value (like an `int`) or a struct
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

    // Default move construct/assign.
    ArbData(ArbData&&) = default;
    ArbData &operator=(ArbData&&) = default;

    // Include builder pattern functions.
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
   */
  class ArbCmd : public Cmd {
  public:

    /**
     * Wrap the given `ArbCmd` handle.
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

    // Default move construct/assign.
    ArbCmd(ArbCmd&&) = default;
    ArbCmd &operator=(ArbCmd&&) = default;

    // Include builder pattern functions.
    #define ARB_BUILDER_SUBCLASS ArbCmd
    #include "arb_builder.hpp"

  };

  /**
   * Class wrapper for `ArbCmd` queue handles.
   *
   * To construct an `ArbCmd` queue iteratively, create a new queue using the
   * default constructor and push `ArbCmd`s into it using `push()`. Note that
   * there is an rvalue reference `push()` operation that entirely avoids
   * copying the `ArbCmd`. You can also construct the queue from an iterable of
   * `ArbCmd`s directly; again, including a zero-copy function using an rvalue
   * reference.
   *
   * To iterate over an existing `ArbCmd` queue (destructively!) in the most
   * efficient way, use the following code:
   *
   * ```
   * for (; queue.size() > 0; queue.next()) {
   *   // queue can be used as the current cmd/arb without any copies now
   * }
   * ```
   *
   * You can also drain it into a `std::vector` of `ArbCmd`s, or (if you must)
   * copy it into one.
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
     * NOTE: this function is not `const`, because exceptions during the copy
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

    // Default move construct/assign.
    ArbCmdQueue(ArbCmdQueue&&) = default;
    ArbCmdQueue &operator=(ArbCmdQueue&&) = default;

  };

  /**
   * Wrapper around the qubit reference typedef in the raw C bindings. This
   * prevents mutation and mathematical operations that don't make sense.
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

    // The default assignment, copy, and move operators are fine and need not
    // be restricted.
    QubitRef(const QubitRef&) = default;
    QubitRef &operator=(const QubitRef&) = default;
    QubitRef(QubitRef &&handle) = default;
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
   * Wrapper around qubit set handles.
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

    // Default move construct/assign.
    QubitSet(QubitSet&&) = default;
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

    // Default copy/move construct/assign.
    Matrix(const Matrix&) = default;
    Matrix &operator=(const Matrix&) = default;
    Matrix(Matrix&&) = default;
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
   * Class wrapper for `Gate` handles.
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

    // Default move construct/assign.
    Gate(Gate&&) = default;
    Gate &operator=(Gate&&) = default;

    /**
     * Constructs a new unitary gate with no control qubits.
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
     * Constructs a new unitary gate with no control qubits.
     */
    static Gate unitary(const QubitSet &targets, const Matrix &matrix) {
      return unitary(std::move(QubitSet(targets)), matrix);
    }

    /**
     * Constructs a new unitary gate with control qubits.
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
     */
    static Gate unitary(const QubitSet &targets, const QubitSet &controls, const Matrix &matrix) {
      return unitary(std::move(QubitSet(targets)), std::move(QubitSet(controls)), matrix);
    }

    /**
     * Constructs a new Z-axis measurement gate.
     */
    static Gate measure(QubitSet &&measures) {
      return Gate(check(raw::dqcs_gate_new_measurement(measures.get_handle())));
    }

    /**
     * Constructs a new Z-axis measurement gate.
     */
    static Gate measure(const QubitSet &measures) {
      return measure(std::move(QubitSet(measures)));
    }

    /**
     * Constructs a new custom gate with a matrix.
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
     * Constructs a new custom gate with a matrix.
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
     * Constructs a new custom gate without a matrix.
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
     * Constructs a new custom gate without a matrix.
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
    #define ARB_BUILDER_SUBCLASS Gate
    #include "arb_builder.hpp"

  };

  /**
   * Class wrapper for measurement handles.
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

    // Defaults for move construct/assign.
    Measurement(Measurement &&handle) = default;
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
    #define ARB_BUILDER_SUBCLASS Measurement
    #include "arb_builder.hpp"

  };

  /**
   * Wrapper around measurement set handles.
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

    // Defaults for move construct/assign.
    MeasurementSet(MeasurementSet &&handle) = default;
    MeasurementSet &operator=(MeasurementSet&&) = default;

  };

} // namespace wrap

/**
 * Namespace for the plugin callback function wrappers.
 */
namespace callback {

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
   * Wrapper for the `dqcs_plugin_state_t` type for upstream-synchronous
   * callbacks. Cannot be moved or copied, as it must stay in scope of the
   * plugin callbacks. Can also not be constructed except for by the callback
   * wrapper classes.
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
   * Wrapper for the `dqcs_plugin_state_t` type for downstream-synchronous
   * callbacks. Cannot be moved or copied, as it must stay in scope of the
   * plugin callbacks. Can also not be constructed except for by the callback
   * wrapper classes.
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
   * Wrapper for the `dqcs_plugin_state_t` type for downstream-synchronous
   * callbacks. Cannot be moved or copied, as it must stay in scope of the
   * plugin callbacks. Can also not be constructed except for by the callback
   * wrapper classes.
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
   * Wrapper for the initialization callback.
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

    // Default move construct/assign.
    PluginJoinHandle(PluginJoinHandle&&) = default;
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
   * Class wrapper for plugin definition handles.
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

    // Default move construct/assign.
    Plugin(Plugin&&) = default;
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
