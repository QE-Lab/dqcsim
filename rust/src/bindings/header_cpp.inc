// <cdqcsim> -*- C++ -*-

#ifndef _CDQCSIM_INCLUDED_
//! \cond Doxygen_Suppress
#define _CDQCSIM_INCLUDED_
//! \endcond

/*!
 * \file cdqcsim
 * \brief Provides DQCsim's raw C API symbols in the `dqcsim::raw` namespace.
 *
 * This is file is functionally identical to `dqcsim.h`, but doesn't pollute
 * the global namespace.
 */

#include <cstdio>
#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <new>

//! \cond Doxygen_Suppress
#ifndef _DQCSIM_HDR_CPP_
#define _DQCSIM_HDR_CPP_
#endif
//! \endcond

#ifdef __cplusplus
extern "C" {
#endif

// For Doxygen:
namespace dqcsim {
/**
 * Namespace containing all the symbols defined by the C API.
 *
 * When including `<cdqcsim>` instead of `<dqcsim.h>`, the C API symbols are
 * placed in this namespace. This behavior mimics the C++ standard library,
 * like how for instance including `<cstring>` gives you `std::memcpy()`
 * instead of `memcpy()`.
 *
 * Note that the `dqcs_`/`DQCS_` namespacing prefixes of the C API remain,
 * so the full name of the symbols becomes `dqcsim::raw::dqcs_*` etc. The
 * `dqcsim::wrap` namespace populated in addition by including `<dqcsim>`
 * provides more C++-like bindings, similar to `<cstring>` versus `<string>`
 * in the standard library.
 */
namespace raw {
} // namespace raw
} // namespace dqcsim
