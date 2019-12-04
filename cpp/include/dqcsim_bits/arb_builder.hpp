// Include this file into a class derived from `Arb` after defining
// `ARB_BUILDER_SUBCLASS` to the subclass name to add the `ArbData` builder
// functions to it.

#ifndef ARB_BUILDER_SUBCLASS
#error
#endif

/**
 * Sets the arbitrary JSON data to the given serialized JSON string
 * (builder pattern).
 */
ARB_BUILDER_SUBCLASS &with_json_string(const std::string &json) {
  set_arb_json_string(json);
  return *this;
}

/**
 * Sets the arbitrary JSON data to the given serialized CBOR string
 * (builder pattern).
 */
ARB_BUILDER_SUBCLASS &with_cbor_string(const std::string &cbor) {
  set_arb_cbor_string(cbor);
  return *this;
}

/**
 * Sets the arbitrary JSON data to the given JSON object from
 * `nlohmann::json` (builder pattern). Since that is a header-only library
 * that isn't usually installed system-wide and be using a specific version
 * in your project already, you need to specify the `nlohmann::json` type
 * as a generic to this function.
 */
template <class JSON>
ARB_BUILDER_SUBCLASS &with_json(const JSON &json) {
  set_arb_json(json);
  return *this;
}

/**
 * Pushes a (binary) string to the back of the arbitrary argument list
 * (builder pattern).
 */
ARB_BUILDER_SUBCLASS &with_arg_string(const std::string &data) {
  push_arb_arg_string(data);
  return *this;
}

/**
 * Pushes a value of type `T` to the back of the arbitrary argument list
 * (builder pattern).
 *
 * WARNING: type `T` must be a primitive value (like an `int`) or a struct
 * thereof, without pointers or any other "complicated" constructs. DQCsim
 * just copies the bytes over. It is up to you to ensure that that's what
 * you want to happen; unfortunately C++11 does not provide a way to
 * statically ensure that this is the case.
 */
template <typename T>
ARB_BUILDER_SUBCLASS &with_arg(const T &data) {
  push_arb_arg(data);
  return *this;
}

#undef ARB_BUILDER_SUBCLASS
