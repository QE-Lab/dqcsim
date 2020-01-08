#ifndef _DQCSIM_H_INCLUDED_
//! \cond Doxygen_Suppress
#define _DQCSIM_H_INCLUDED_
//! \endcond

/*!
 * \file dqcsim.h
 * \brief Provides DQCsim's raw C API functions, typedefs, and macros.
 *
 * It is safe to include this file both in C-only and C++ projects. However,
 * including the `<cdqcsim>` equivalent is preferable for C++-only projects,
 * as it does not pollute the global namespace.
 */

#include <stdio.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#ifdef _DQCSIM_HDR_CPP_
#undef _DQCSIM_HDR_CPP_
#endif

#ifdef __cplusplus
extern "C" {
#endif
