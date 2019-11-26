cmake_minimum_required(VERSION 3.14.0 FATAL_ERROR)

project(dqcsim
    VERSION 0.0.1
    DESCRIPTION "C++ bindings for the Delft Quantum & Classical Simulator"
    LANGUAGES CXX
)

include(ExternalProject)
ExternalProject_Add(
    dqcsim_crate
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    SOURCE_DIR "${CMAKE_SOURCE_DIR}/rust"
    BUILD_IN_SOURCE ON
    BUILD_ALWAYS ON
    BUILD_COMMAND cargo build --release --all-features
    INSTALL_COMMAND ""
)
add_library(dqcsim SHARED IMPORTED)
add_dependencies(dqcsim dqcsim_crate)
target_include_directories(dqcsim INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/target/include)
set_target_properties(dqcsim PROPERTIES IMPORTED_LOCATION ${CMAKE_CURRENT_SOURCE_DIR}/target/release/${CMAKE_SHARED_MODULE_PREFIX}dqcsim${CMAKE_SHARED_LIBRARY_SUFFIX})

include(FetchContent)
FetchContent_Declare(cmake-modules
  GIT_REPOSITORY  https://github.com/abs-tudelft/cmake-modules.git
  GIT_TAG         master
)
FetchContent_MakeAvailable(cmake-modules)

include(CompileUnits)

add_subdirectory(cpp)

compile_units()
