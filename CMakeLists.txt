cmake_minimum_required(VERSION 3.9)

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/")

project(dqcsim_cpp_test
    VERSION 0.0.1
    DESCRIPTION "Tests the C++ bindings for the Delft Quantum & Classical Simulator"
    LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(ExternalProject)

#------------------------------------------------------------------------------
# DCQcsi build
#------------------------------------------------------------------------------
ExternalProject_Add(
    dqcsim
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    SOURCE_DIR "${CMAKE_SOURCE_DIR}/rust"
    BUILD_IN_SOURCE ON
    BUILD_ALWAYS ON
    BUILD_COMMAND cargo build --all-features
    INSTALL_COMMAND ""
)

if(APPLE)
    set(SHARED_LIB_SUFFIX ".dylib")
else()
    set(SHARED_LIB_SUFFIX ".so")
endif()
set(LIBDQCSIM "${CMAKE_SOURCE_DIR}/target/debug/libdqcsim${SHARED_LIB_SUFFIX}")
set(LIBDQCSIM_INCLUDE "${CMAKE_SOURCE_DIR}/target/include/")

#------------------------------------------------------------------------------
# CMake options
#------------------------------------------------------------------------------
option(TEST_CPP         "Test the C++ library"                           ON)
option(TEST_RAW_C       "Test the raw C API"                             ON)

#------------------------------------------------------------------------------
# GoogleTest inclusion
#------------------------------------------------------------------------------
include(FetchContent)

FetchContent_Declare(
    googletest
    GIT_REPOSITORY https://github.com/google/googletest.git
    GIT_TAG        release-1.8.0
)

FetchContent_MakeAvailable(googletest)
enable_testing()
add_custom_target(test+verbose COMMAND ${CMAKE_CTEST_COMMAND} --output-on-failure)

add_subdirectory(cpp)
