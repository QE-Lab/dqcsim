//*****************************************************************************
// Functions that cannot be implemented in Rust
//*****************************************************************************

// Place log function in namespace if this is the C++ header.
#ifdef _DQCSIM_HDR_CPP_
namespace dqcsim {

/**
 * Namespace containing all the symbols defined by the C API.
 */
namespace raw {
#endif

// Use namespaced C stdlib functions if this is the C++ header.
#ifdef _DQCSIM_HDR_CPP_
#define _DQCSIM_STD_PREFIX_ ::std::
#else
#define _DQCSIM_STD_PREFIX_
#endif

/**
 * Sends a log message using the current logger using printf-like formatting.
 *
 * This function is identical to `dqcs_log_raw()`, except instead of a single
 * string it takes a printf-like format string and varargs to compose the
 * message.
 */
inline void dqcs_log_format(
  dqcs_loglevel_t level,
  const char *module,
  const char *file,
  uint32_t line,
  const char *fmt,
  ...
)
#if defined(__GNUC__)
__attribute__((format(printf,5,6)))
#endif
;
inline void dqcs_log_format(
  dqcs_loglevel_t level,
  const char *module,
  const char *file,
  uint32_t line,
  const char *fmt,
  ...
)
{
  // Figure out the buffer size we need.
  _DQCSIM_STD_PREFIX_ va_list ap;
  va_start(ap, fmt);
  int size = _DQCSIM_STD_PREFIX_ vsnprintf(NULL, 0, fmt, ap) + 1;
  va_end(ap);

  // Allocate the buffer.
  char *buffer = (char*)_DQCSIM_STD_PREFIX_ malloc(size);
  if (buffer == NULL) {
    _DQCSIM_STD_PREFIX_ fprintf(stderr, "Error: failed to allocate buffer for log message!\n");
    return;
  }

  // Perform the actual formatting operation.
  _DQCSIM_STD_PREFIX_ va_list ap2;
   va_start(ap2, fmt);
  _DQCSIM_STD_PREFIX_ vsnprintf(buffer, size, fmt, ap2);
  va_end(ap2);

  // Send to DQCsim.
  if (((int)dqcs_log_raw(level, module, file, line, buffer)) < 0) {
    _DQCSIM_STD_PREFIX_ fprintf(stderr, "Error while trying to log: %s\n", dqcs_error_get());
    _DQCSIM_STD_PREFIX_ fprintf(stderr, "The message was: %s\n", buffer);
    _DQCSIM_STD_PREFIX_ fprintf(stderr, "In %s, %s:%u\n", module, file, line);
  }

  // Don't leak!
  _DQCSIM_STD_PREFIX_ free(buffer);
}

#undef _DQCSIM_STD_PREFIX_

#ifdef _DQCSIM_HDR_CPP_
} // namespace raw
} // namespace dqcsim
#endif

//*****************************************************************************
// Macros
//*****************************************************************************

#ifndef _DQCSIM_LANGUAGE_
#ifdef _DQCSIM_HDR_CPP_
#define _DQCSIM_LANGUAGE_ "CPP"
#else
#define _DQCSIM_LANGUAGE_ "C"
#endif
#endif

#ifndef _DQCSIM_LOGLEVEL_PREFIX_
#ifdef _DQCSIM_HDR_CPP_
#define _DQCSIM_LOGLEVEL_PREFIX_ ::dqcsim::raw::dqcs_loglevel_t::
#else
#define _DQCSIM_LOGLEVEL_PREFIX_
#endif
#endif

#ifndef dqcs_log_trace
/**
 * Convenience macro for calling `dqcs_log_format()` with trace loglevel and
 * automatically determined function name, filename, and line number.
 */
#define dqcs_log_trace(fmt, ...)                \
  dqcs_log_format(                              \
    _DQCSIM_LOGLEVEL_PREFIX_ DQCS_LOG_TRACE,    \
    _DQCSIM_LANGUAGE_,                          \
    __FILE__,                                   \
    __LINE__,                                   \
    fmt,                                        \
    ##__VA_ARGS__                               \
  )
#endif

#ifndef dqcs_log_debug
/**
 * Convenience macro for calling `dqcs_log_format()` with debug loglevel and
 * automatically determined function name, filename, and line number.
 */
#define dqcs_log_debug(fmt, ...)                \
  dqcs_log_format(                              \
    _DQCSIM_LOGLEVEL_PREFIX_ DQCS_LOG_DEBUG,    \
    _DQCSIM_LANGUAGE_,                          \
    __FILE__,                                   \
    __LINE__,                                   \
    fmt,                                        \
    ##__VA_ARGS__                               \
  )
#endif

#ifndef dqcs_log_info
/**
 * Convenience macro for calling `dqcs_log_format()` with info loglevel and
 * automatically determined function name, filename, and line number.
 */
#define dqcs_log_info(fmt, ...)                 \
  dqcs_log_format(                              \
    _DQCSIM_LOGLEVEL_PREFIX_ DQCS_LOG_INFO,     \
    _DQCSIM_LANGUAGE_,                          \
    __FILE__,                                   \
    __LINE__,                                   \
    fmt,                                        \
    ##__VA_ARGS__                               \
  )
#endif

#ifndef dqcs_log_note
/**
 * Convenience macro for calling `dqcs_log_format()` with note loglevel and
 * automatically determined function name, filename, and line number.
 */
#define dqcs_log_note(fmt, ...)                 \
  dqcs_log_format(                              \
    _DQCSIM_LOGLEVEL_PREFIX_ DQCS_LOG_NOTE,     \
    _DQCSIM_LANGUAGE_,                          \
    __FILE__,                                   \
    __LINE__,                                   \
    fmt,                                        \
    ##__VA_ARGS__                               \
  )
#endif

#ifndef dqcs_log_warn
/**
 * Convenience macro for calling `dqcs_log_format()` with warn loglevel and
 * automatically determined function name, filename, and line number.
 */
#define dqcs_log_warn(fmt, ...)                 \
  dqcs_log_format(                              \
    _DQCSIM_LOGLEVEL_PREFIX_ DQCS_LOG_WARN,     \
    _DQCSIM_LANGUAGE_,                          \
    __FILE__,                                   \
    __LINE__,                                   \
    fmt,                                        \
    ##__VA_ARGS__                               \
  )
#endif

#ifndef dqcs_log_error
/**
 * Convenience macro for calling `dqcs_log_format()` with error loglevel and
 * automatically determined function name, filename, and line number.
 */
#define dqcs_log_error(fmt, ...)                \
  dqcs_log_format(                              \
    _DQCSIM_LOGLEVEL_PREFIX_ DQCS_LOG_ERROR,    \
    _DQCSIM_LANGUAGE_,                          \
    __FILE__,                                   \
    __LINE__,                                   \
    fmt,                                        \
    ##__VA_ARGS__                               \
  )
#endif

#ifndef dqcs_log_fatal
/**
 * Convenience macro for calling `dqcs_log_format()` with fatal loglevel and
 * automatically determined function name, filename, and line number.
 */
#define dqcs_log_fatal(fmt, ...)                \
  dqcs_log_format(                              \
    _DQCSIM_LOGLEVEL_PREFIX_ DQCS_LOG_FATAL,    \
    _DQCSIM_LANGUAGE_,                          \
    __FILE__,                                   \
    __LINE__,                                   \
    fmt,                                        \
    ##__VA_ARGS__                               \
  )
#endif

#ifdef __cplusplus
} // extern "C"
#endif

#ifdef _DQCSIM_HDR_CPP_
#undef _DQCSIM_HDR_CPP_
#endif

#endif
