cmake_minimum_required(VERSION 3.9)

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/")

project(dqcsim_cpp_test
    VERSION 0.1.0
    DESCRIPTION "Tests the C++ bindings for the Delft Quantum & Classical Simulator"
    LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

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

#------------------------------------------------------------------------------
# Search paths
#------------------------------------------------------------------------------
if(TEST_CPP)
    add_subdirectory(test_cpp)
endif()

if(TEST_RAW_C)
    add_subdirectory(test_raw_c)
endif()
