//*****************************************************************************
// Functions that cannot be implemented in Rust
//*****************************************************************************

#ifdef __cplusplus
namespace dqcsim {
#endif

/**
 * Sends a log message using the current logger using printf-like formatting.
 *
 * This function is identical to `dqcs_log()`, except instead of a single
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
  va_list ap;
  va_start(ap, fmt);
  int size = vsnprintf(NULL, 0, fmt, ap) + 1;
  va_end(ap);

  // Allocate the buffer.
  char *buffer = (char*)malloc(size);
  if (buffer == NULL) {
    fprintf(stderr, "Error: failed to allocate buffer for log message!\n");
    return;
  }

  // Perform the actual formatting operation.
  va_list ap2;
  va_start(ap2, fmt);
  vsnprintf(buffer, size, fmt, ap);
  va_end(ap2);

  // Send to DQCsim.
  if (((int)dqcs_log_raw(level, module, file, line, buffer)) < 0) {
    fprintf(stderr, "Error while trying to log: %s\n", dqcs_error_get());
    fprintf(stderr, "The message was: %s\n", buffer);
    fprintf(stderr, "In %s, %s:%u\n", module, file, line);
  }

  // Don't leak!
  free(buffer);
}

#ifdef __cplusplus
} // namespace dqcsim
#endif

//*****************************************************************************
// Macros
//*****************************************************************************

#ifdef __cplusplus
#define DQCS_LANGUAGE "CPP"
#define DQCS_LOGLEVEL_PREFIX ::dqcsim::dqcs_loglevel_t::
#else
#define DQCS_LANGUAGE "C"
#define DQCS_LOGLEVEL_PREFIX
#endif

/**
 * Convenience macro for calling `dqcs_log_format()` with trace loglevel and
 * automatically determined function name, filename, and line number.
 */
#define dqcs_log_trace(fmt, ...)          \
  dqcs_log_format(                        \
    DQCS_LOGLEVEL_PREFIX DQCS_LOG_TRACE,  \
    DQCS_LANGUAGE,                        \
    __FILE__,                             \
    __LINE__,                             \
    fmt,                                  \
    __VA_ARGS__                           \
  )

/**
 * Convenience macro for calling `dqcs_log_format()` with debug loglevel and
 * automatically determined function name, filename, and line number.
 */
#define dqcs_log_debug(fmt, ...)          \
  dqcs_log_format(                        \
    DQCS_LOGLEVEL_PREFIX DQCS_LOG_DEBUG,  \
    DQCS_LANGUAGE,                        \
    __FILE__,                             \
    __LINE__,                             \
    fmt,                                  \
    __VA_ARGS__                           \
  )

/**
 * Convenience macro for calling `dqcs_log_format()` with info loglevel and
 * automatically determined function name, filename, and line number.
 */
#define dqcs_log_info(fmt, ...)           \
  dqcs_log_format(                        \
    DQCS_LOGLEVEL_PREFIX DQCS_LOG_INFO,   \
    DQCS_LANGUAGE,                        \
    __FILE__,                             \
    __LINE__,                             \
    fmt,                                  \
    __VA_ARGS__                           \
  )

/**
 * Convenience macro for calling `dqcs_log_format()` with note loglevel and
 * automatically determined function name, filename, and line number.
 */
#define dqcs_log_note(fmt, ...)           \
  dqcs_log_format(                        \
    DQCS_LOGLEVEL_PREFIX DQCS_LOG_NOTE,   \
    DQCS_LANGUAGE,                        \
    __FILE__,                             \
    __LINE__,                             \
    fmt,                                  \
    __VA_ARGS__                           \
  )

/**
 * Convenience macro for calling `dqcs_log_format()` with warn loglevel and
 * automatically determined function name, filename, and line number.
 */
#define dqcs_log_warn(fmt, ...)           \
  dqcs_log_format(                        \
    DQCS_LOGLEVEL_PREFIX DQCS_LOG_WARN,   \
    DQCS_LANGUAGE,                        \
    __FILE__,                             \
    __LINE__,                             \
    fmt,                                  \
    __VA_ARGS__                           \
  )

/**
 * Convenience macro for calling `dqcs_log_format()` with error loglevel and
 * automatically determined function name, filename, and line number.
 */
#define dqcs_log_error(fmt, ...)          \
  dqcs_log_format(                        \
    DQCS_LOGLEVEL_PREFIX DQCS_LOG_ERROR,  \
    DQCS_LANGUAGE,                        \
    __FILE__,                             \
    __LINE__,                             \
    fmt,                                  \
    __VA_ARGS__                           \
  )

/**
 * Convenience macro for calling `dqcs_log_format()` with fatal loglevel and
 * automatically determined function name, filename, and line number.
 */
#define dqcs_log_fatal(fmt, ...)          \
  dqcs_log_format(                        \
    DQCS_LOGLEVEL_PREFIX DQCS_LOG_FATAL,  \
    DQCS_LANGUAGE,                        \
    __FILE__,                             \
    __LINE__,                             \
    fmt,                                  \
    __VA_ARGS__                           \
  )

#undef DQCS_LANGUAGE
#undef DQCS_LOGLEVEL_PREFIX
