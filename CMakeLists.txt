cmake_minimum_required(VERSION 3.14.0 FATAL_ERROR)

project(dqcsim
    VERSION 0.0.12
    DESCRIPTION "C++ bindings for the Delft Quantum & Classical Simulator"
    LANGUAGES CXX
)

if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif()
string(TOLOWER ${CMAKE_BUILD_TYPE} BUILD_DIR)
if (BUILD_DIR MATCHES  "release")
    set(CARGO_BUILD_TYPE "--release")
endif()
get_filename_component(TARGET_DIR ${CMAKE_SOURCE_DIR}/target/ ABSOLUTE)

include(ExternalProject)
ExternalProject_Add(
    dqcsim_crate
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    SOURCE_DIR "${CMAKE_SOURCE_DIR}/rust"
    BINARY_DIR "${TARGET_DIR}/include"
    BUILD_ALWAYS ON
    BUILD_COMMAND cargo build ${CARGO_BUILD_TYPE} --all-features
    INSTALL_COMMAND ""
)
add_library(dqcsim SHARED IMPORTED)
add_dependencies(dqcsim dqcsim_crate)
target_include_directories(dqcsim INTERFACE ${TARGET_DIR}/include)
set_target_properties(dqcsim PROPERTIES IMPORTED_LOCATION ${TARGET_DIR}/${BUILD_DIR}/${CMAKE_SHARED_MODULE_PREFIX}dqcsim${CMAKE_SHARED_LIBRARY_SUFFIX})

include(FetchContent)
FetchContent_Declare(cmake-modules
  GIT_REPOSITORY  https://github.com/abs-tudelft/cmake-modules.git
  GIT_TAG         master
)
FetchContent_MakeAvailable(cmake-modules)

include(CompileUnits)

add_subdirectory(cpp)

compile_units()

# NOTE: CMake-based installations are ONLY intended for testing, for instance
# when you want to link a C++ plugin to a specific (or modified) version of
# DQCsim without dealing with Python virtualenvs and the like. In ALL OTHER
# CASES, use setup.py and pip to handle installation!
#
# Some differences between the two:
#
#  - the CMake install does not pre-preprocess the C++ headers into a single
#    file, making the line numbers consistent with the sources. setup.py
#    merges them to a single header to reduce clutter.
#  - CMake actually honors the difference between lib and lib64 on multilib
#    operating system, while pip is too stupid to do so.
#  - CMake only installs the headers and shared object, not the binaries.
#
# In general, installing a pip wheel in the same prefix as the CMake install
# will lead to death and destruction. Note also that if you use plugins that
# are linked to other versions of DQCsim, they need to be protocol-compatible;
# DQCsim will currently not check for this and will probably invoke
# halt-and-catch-fire if you mix versions. You have been warned!

# Install header files.
install(
    FILES
        ${CMAKE_SOURCE_DIR}/target/include/cdqcsim
        ${CMAKE_SOURCE_DIR}/target/include/dqcsim.h
    DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
)

# Install shared object.
install(
    FILES ${CMAKE_SOURCE_DIR}/target/${BUILD_DIR}/libdqcsim${CMAKE_SHARED_LIBRARY_SUFFIX}
    DESTINATION ${CMAKE_INSTALL_LIBDIR}
)
