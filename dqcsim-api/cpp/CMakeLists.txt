cmake_minimum_required(VERSION 3.11)

project(dqcsim_cpp_test
    VERSION 0.1.0
    DESCRIPTION "Tests the C++ bindings for the Delft Quantum & Classical Simulator"
    LANGUAGES CXX
)

#------------------------------------------------------------------------------
# CMake options
#------------------------------------------------------------------------------
option(USE_RELEASE_LIB  "Use target/release instead of target/debug lib" OFF)
option(TEST_LIB         "Test the C++ library"                           ON)
option(TEST_RAW         "Test the raw C API"                             ON)

#------------------------------------------------------------------------------
# GoogleTest inclusion
#------------------------------------------------------------------------------
include(FetchContent)

FetchContent_Declare(
    googletest
    GIT_REPOSITORY https://github.com/google/googletest.git
    GIT_TAG        release-1.8.0
)

FetchContent_GetProperties(googletest)
if(NOT googletest_POPULATED)
    FetchContent_Populate(googletest)
    add_subdirectory(${googletest_SOURCE_DIR} ${googletest_BINARY_DIR})
endif()

enable_testing()

add_custom_target(test+verbose COMMAND ${CMAKE_CTEST_COMMAND} --output-on-failure)

#------------------------------------------------------------------------------
# Search paths
#------------------------------------------------------------------------------
include_directories(gen/include)

if(USE_RELEASE_LIB)
    link_directories(../../target/release)
else()
    link_directories(../../target/debug)
endif()

if(TEST_LIB)
    add_subdirectory(test_lib)
endif()

if(TEST_RAW)
    add_subdirectory(test_raw)
endif()
