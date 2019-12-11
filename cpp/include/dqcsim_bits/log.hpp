
//! \cond Doxygen_Suppress
#ifdef DQCSIM_SHORT_LOGGING_MACROS
//! \endcond

#ifndef _DQCSIM_LOGGING_INCLUDED_
//! \cond Doxygen_Suppress
#define _DQCSIM_LOGGING_INCLUDED_
//! \endcond

/**
 * Convenience macro for calling `log()` with trace loglevel and automatically
 * determined filename and line number.
 *
 * \note Defined only when `DQCSIM_SHORT_LOGGING_MACROS` is defined before the
 * `<dqcsim>` header is included.
 */
#define TRACE DQCSIM_TRACE

/**
 * Convenience macro for calling `log()` with debug loglevel and automatically
 * determined filename and line number.
 *
 * \note Defined only when `DQCSIM_SHORT_LOGGING_MACROS` is defined before the
 * `<dqcsim>` header is included.
 */
#define DEBUG DQCSIM_DEBUG

/**
 * Convenience macro for calling `log()` with info loglevel and automatically
 * determined filename and line number.
 *
 * \note Defined only when `DQCSIM_SHORT_LOGGING_MACROS` is defined before the
 * `<dqcsim>` header is included.
 */
#define INFO DQCSIM_INFO

/**
 * Convenience macro for calling `log()` with note loglevel and automatically
 * determined filename and line number.
 *
 * \note Defined only when `DQCSIM_SHORT_LOGGING_MACROS` is defined before the
 * `<dqcsim>` header is included.
 */
#define NOTE DQCSIM_NOTE

/**
 * Convenience macro for calling `log()` with warn loglevel and automatically
 * determined filename and line number.
 *
 * \note Defined only when `DQCSIM_SHORT_LOGGING_MACROS` is defined before the
 * `<dqcsim>` header is included.
 */
#define WARN DQCSIM_WARN

/**
 * Convenience macro for calling `log()` with warn loglevel and automatically
 * determined filename and line number.
 *
 * \note Defined only when `DQCSIM_SHORT_LOGGING_MACROS` is defined before the
 * `<dqcsim>` header is included.
 */
#define WARNING DQCSIM_WARNING

/**
 * Convenience macro for calling `log()` with error loglevel and automatically
 * determined filename and line number.
 *
 * \note Defined only when `DQCSIM_SHORT_LOGGING_MACROS` is defined before the
 * `<dqcsim>` header is included.
 */
#define ERROR DQCSIM_ERROR

/**
 * Convenience macro for calling `log()` with fatal loglevel and automatically
 * determined filename and line number.
 *
 * \note Defined only when `DQCSIM_SHORT_LOGGING_MACROS` is defined before the
 * `<dqcsim>` header is included.
 */
#define FATAL DQCSIM_FATAL

/**
 * Convenience macro for calling `log()` with automatically determined filename
 * and line number, but a dynamic loglevel (first argument).
 *
 * \note Defined only when `DQCSIM_SHORT_LOGGING_MACROS` is defined before the
 * `<dqcsim>` header is included.
 */
#define LOG DQCSIM_LOG

#endif // _DQCSIM_LOGGING_INCLUDED_

//! \cond Doxygen_Suppress
#endif // DQCSIM_SHORT_LOGGING_MACROS
//! \endcond
