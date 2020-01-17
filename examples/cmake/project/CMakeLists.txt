cmake_minimum_required(VERSION 3.14.0 FATAL_ERROR)

project(dqcsim-cmake-example
    VERSION 0.0.1
    DESCRIPTION "Example for linking to DQCsim with CMake"
    LANGUAGES CXX
)

#vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
#
# Boilerplate code for using DQCsim. By copypasting this into your project you
# can:
#
#  - reference <dqcsim.h> and friends in C/C++ sources;
#  - link your targets against DQCsim using
#      target_link_libraries(<your-target> ${DQCSIM_LIB})
#    or lazily link everything using
#      link_libraries(${DQCSIM_LIB})
#    after this snippit;
#  - build DQCsim from source and link againt it instead by adding
#      -DDQCSIM_FROM_SOURCE=<directory>
#    to your cmake command line; this is useful if you want to test with a
#    specific version or want to add debug statements in DQCsim to help debug
#    your application or plugin. If you specify a non-existing directory the
#    value will default to `dqcsim` in your source directory, which is a useful
#    shorthand for when you include DQCsim as a git submodule that points to
#    some known-working version.
#
#------------------------------------------------------------------------------
include(ExternalProject) # <-- No need to repeat if you already do it elsewhere
if (DQCSIM_FROM_SOURCE)
    if (IS_DIRECTORY ${DQCSIM_FROM_SOURCE})
        set(DQCSIM_SRCDIR ${DQCSIM_FROM_SOURCE})
    else()
        set(DQCSIM_SRCDIR ${CMAKE_SOURCE_DIR}/dqcsim)
    endif()
    if(NOT EXISTS ${DQCSIM_SRCDIR}/CMakeLists.txt)
        message(
            FATAL_ERROR
            "
            Could not find DQCsim source tree!"
        )
    endif()
    ExternalProject_Add(
        dqcsim
        SOURCE_DIR "${DQCSIM_SRCDIR}"
        CMAKE_ARGS
            "-DCMAKE_INSTALL_PREFIX=${CMAKE_CURRENT_BINARY_DIR}/dqcsim"
            "-DCMAKE_BUILD_TYPE=debug"
    )
    get_property(LIB64 GLOBAL PROPERTY FIND_LIBRARY_USE_LIB64_PATHS)
    if ("${LIB64}" STREQUAL "TRUE")
        set(DQCSIM "${CMAKE_CURRENT_BINARY_DIR}/dqcsim/lib64/libdqcsim.so")
    else()
        set(DQCSIM "${CMAKE_CURRENT_BINARY_DIR}/dqcsim/lib/libdqcsim.so")
    endif()
    set(DQCSIM_INC "${CMAKE_CURRENT_BINARY_DIR}/dqcsim/include")
else()
    find_library(DQCSIM dqcsim)
    find_path(DQCSIM_INC dqcsim.h)
    if(NOT DQCSIM OR NOT DQCSIM_INC)
        message(
            FATAL_ERROR
            "
            DQCsim could not be found! Try installing it with
            `sudo pip3 install dqcsim`, or opt to build and install
            from source using -DDQCSIM_FROM_SOURCE=<src-dir>."
        )
    endif()
endif()
include_directories(${DQCSIM_INC} BEFORE)
# End of DQCsim inclusion boilerplate code.
#
#^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

# Add a simple example executable.
add_executable(main main.cpp)

# Link the example executable against DQCsim.
target_link_libraries(main ${DQCSIM})
