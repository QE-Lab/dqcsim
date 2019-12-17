
/**
 * Sets the arbitrary JSON data to the given serialized JSON string
 * (builder pattern).
 *
 * \param json A string representation of a JSON dictionary object to assign.
 * \returns `&self`, to continue building.
 * \throws std::runtime_error When the string representation is invalid,
 * or when the current handle is invalid.
 */
ARB_BUILDER_SUBCLASS &with_json_string(const std::string &json) {
  set_arb_json_string(json);
  return *this;
}

/**
 * Sets the arbitrary JSON data to the given serialized CBOR string
 * (builder pattern).
 *
 * \param cbor A JSON object represented as a CBOR binary string.
 * \returns `&self`, to continue building.
 * \throws std::runtime_error When the CBOR string is invalid, or when the
 * current handle is invalid.
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
 *
 * \param json The C++ JSON object representation of the object to set.
 * \returns `&self`, to continue building.
 * \throws std::runtime_error When the current handle is invalid.
 */
template <class JSON>
ARB_BUILDER_SUBCLASS &with_json(const JSON &json) {
  set_arb_json(json);
  return *this;
}

/**
 * Pushes a (binary) string to the back of the arbitrary argument list
 * (builder pattern).
 *
 * \param data The data for the new argument, represented as a (binary)
 * string.
 * \returns `&self`, to continue building.
 * \throws std::runtime_error When the current handle is invalid.
 */
ARB_BUILDER_SUBCLASS &with_arg_string(const std::string &data) {
  push_arb_arg_string(data);
  return *this;
}

/**
 * Pushes a value of type `T` to the back of the arbitrary argument list
 * (builder pattern).
 *
 * \warning Type `T` must be a primitive value (like an `int`) or a struct
 * thereof, without pointers or any other "complicated" constructs. DQCsim
 * just copies the bytes over. It is up to you to ensure that that's what
 * you want to happen; unfortunately C++11 does not provide a way to
 * statically ensure that this is the case.
 *
 * \param data The data for the new argument, represented as some C object.
 * \returns `&self`, to continue building.
 * \throws std::runtime_error When the current handle is invalid.
 */
template <typename T>
ARB_BUILDER_SUBCLASS &with_arg(const T &data) {
  push_arb_arg(data);
  return *this;
}

#undef ARB_BUILDER_SUBCLASS
