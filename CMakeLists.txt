
# If we already have a project name, we're most likely included using
# fetchcontent.
if (PROJECT_NAME)

    # Don't try building if we're included; this will scatter directories all
    # over the place and then fail.
    cmake_minimum_required(VERSION 3.11.0 FATAL_ERROR)

    include(ExternalProject)
    if (DQCSIM_FROM_SOURCE)
        if (${DQCSIM_FROM_SOURCE} STREQUAL "yes")
            set(DQCSIM_SRCDIR ${CMAKE_CURRENT_SOURCE_DIR})
        else()
            set(DQCSIM_SRCDIR ${DQCSIM_FROM_SOURCE})
        endif()
        if (NOT DQCSIM_OPTIONS)
            set(DQCSIM_OPTIONS "-DCMAKE_BUILD_TYPE=debug")
        endif()
        set(DQCSIM_INSTALL_PREFIX "${CMAKE_CURRENT_BINARY_DIR}/_dqcsim_install_prefix_")
        get_filename_component(DQCSIM_INSTALL_PREFIX ${DQCSIM_INSTALL_PREFIX} REALPATH)
        if(NOT EXISTS ${DQCSIM_SRCDIR}/CMakeLists.txt)
            message(
                FATAL_ERROR
                "
                Could not find DQCsim source tree!"
            )
        endif()
        ExternalProject_Add(
            dqcsim_build
            SOURCE_DIR "${DQCSIM_SRCDIR}"
            CMAKE_ARGS
                "-DCMAKE_INSTALL_PREFIX=${DQCSIM_INSTALL_PREFIX}"
                "${DQCSIM_OPTIONS}"
        )
        add_dependencies(dqcsim dqcsim_build)
        get_property(LIB64 GLOBAL PROPERTY FIND_LIBRARY_USE_LIB64_PATHS)
        if ("${LIB64}" STREQUAL "TRUE")
            set(DQCSIM_LIB "${DQCSIM_INSTALL_PREFIX}/lib64/libdqcsim${CMAKE_SHARED_LIBRARY_SUFFIX}")
        else()
            set(DQCSIM_LIB "${DQCSIM_INSTALL_PREFIX}/lib/libdqcsim${CMAKE_SHARED_LIBRARY_SUFFIX}")
        endif()
        set(DQCSIM_INC "${DQCSIM_INSTALL_PREFIX}/include")
        file(MAKE_DIRECTORY ${DQCSIM_INC})
        add_library(dqcsim SHARED IMPORTED GLOBAL)
        set_target_properties(dqcsim PROPERTIES
            IMPORTED_LOCATION ${DQCSIM_LIB}
            INTERFACE_INCLUDE_DIRECTORIES ${DQCSIM_INC}
            INTERFACE_SYSTEM_INCLUDE_DIRECTORIES ${DQCSIM_INC})
    else()
        if(NOT DEFINED DQCSIM_LIB)
            if(DEFINED ENV{DQCSIM_LIB})
                set(DQCSIM_LIB $ENV{DQCSIM_LIB})
            elseif(DEFINED ENV{DQCSIM_PREFIX})
                set(DQCSIM_LIB $ENV{DQCSIM_PREFIX}/lib/libdqcsim${CMAKE_SHARED_LIBRARY_SUFFIX})
            else()
                find_library(DQCSIM_LIB dqcsim)
            endif()
        endif()
        if(NOT DEFINED DQCSIM_INC)
            if(DEFINED ENV{DQCSIM_INC})
                set(DQCSIM_INC $ENV{DQCSIM_INC})
            elseif(DEFINED ENV{DQCSIM_PREFIX})
                set(DQCSIM_INC $ENV{DQCSIM_PREFIX}/include)
            else()
                find_path(DQCSIM_INC dqcsim.h)
            endif()
        endif()
        if(NOT DQCSIM_LIB OR NOT DQCSIM_INC)
            message(
                FATAL_ERROR
                "
                DQCsim could not be found! Try installing it with
                `sudo pip3 install dqcsim`, or opt to build and install
                from source using -DDQCSIM_FROM_SOURCE=<src-dir>."
            )
        endif()
        get_filename_component(DQCSIM_LIB_PATH ${DQCSIM_LIB} DIRECTORY)
        get_filename_component(DQCSIM_LIB_PATH ${DQCSIM_LIB_PATH} REALPATH)
        add_library(dqcsim SHARED IMPORTED GLOBAL)
        set_target_properties(dqcsim PROPERTIES
            IMPORTED_LOCATION ${DQCSIM_LIB}
            IMPORTED_NO_SONAME YES
            INTERFACE_INCLUDE_DIRECTORIES ${DQCSIM_INC}
            INTERFACE_SYSTEM_INCLUDE_DIRECTORIES ${DQCSIM_INC})
    endif()

else()

    # Not included, so build DQCsim here. The tests, mostly. setup.py is in
    # charge of full installations, and in fact doesn't even chain to CMake,
    # but directly to cargo!
    cmake_minimum_required(VERSION 3.14.0 FATAL_ERROR)

    project(dqcsim
        VERSION 0.2.1
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
        BUILD_COMMAND cargo build ${CARGO_BUILD_TYPE} --manifest-path=${CMAKE_SOURCE_DIR}/rust/Cargo.toml --features=bindings
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

endif()
