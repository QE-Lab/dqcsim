# Comparison to the C API

As stated earlier, the C++ API is basically just a wrapper around the C API:
it makes use of C++11 features to hide some of the verbosity of the C
interface, making it more ergonomic to use. The primary advantages of the C++
interface over the C interface are:

 - DQCsim's error handling is abstracted through exceptions.
 - Handles are wrapped by classes with appropriate inheritance.
 - Handle construction and deletion is more-or-less abstracted away by RAII,
   so you never have to worry about calling `dqcs_handle_delete()`.
 - All strings are wrapped using `std::string`, so you don't need to worry
   about malloc/free when dealing with DQCsim's string functions.
 - All callbacks support C-style callbacks with a template for the user data
   argument type, class member functions, and `std::function` objects, so
   you don't have to deal with `void*` casts.
 - Many function/method overloads are provided to help you make your code
   more succinct.
 - Basic support for `nlohmann::json` for the `ArbData` JSON/CBOR object.

There shouldn't be any downsides to using the C++ interface over the C
interface in C++ programs. If one should ever occur, you can just mix in the
C API calls where needed.
