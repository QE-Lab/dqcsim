cmake_minimum_required(VERSION 3.14.0 FATAL_ERROR)

project(dqcsim
    VERSION 0.0.1
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

include(ExternalProject)
ExternalProject_Add(
    dqcsim_crate
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    SOURCE_DIR "${CMAKE_SOURCE_DIR}/rust"
    BINARY_DIR "${CMAKE_SOURCE_DIR}/target/include"
    BUILD_ALWAYS ON
    BUILD_COMMAND cargo build ${CARGO_BUILD_TYPE} --all-features
    INSTALL_COMMAND ""
)
add_library(dqcsim SHARED IMPORTED)
add_dependencies(dqcsim dqcsim_crate)
target_include_directories(dqcsim INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/target/include)
set_target_properties(dqcsim PROPERTIES IMPORTED_LOCATION ${CMAKE_CURRENT_SOURCE_DIR}/target/${BUILD_DIR}/${CMAKE_SHARED_MODULE_PREFIX}dqcsim${CMAKE_SHARED_LIBRARY_SUFFIX})

include(FetchContent)
FetchContent_Declare(cmake-modules
  GIT_REPOSITORY  https://github.com/abs-tudelft/cmake-modules.git
  GIT_TAG         master
)
FetchContent_MakeAvailable(cmake-modules)

include(CompileUnits)

add_subdirectory(cpp)

compile_units()
