cmake_minimum_required(VERSION 3.14 FATAL_ERROR)
project(dqcsim
	VERSION 0.2.5
	DESCRIPTION "C++ bindings for the Delft Quantum & Classical Simulator"
	LANGUAGES CXX
)

# Wrapper target
add_library(dqcsim SHARED IMPORTED GLOBAL)

# Build rust project
# NOTE: crate is built at build-time, not configure-time
set(TARGET_DIR ${CMAKE_SOURCE_DIR}/target)
if($<CONFIG:Release>)
	set(BUILD_DIR ${TARGET_DIR}/release)
	set(CARGO_BUILD_TYPE --release)
else()
	set(BUILD_DIR ${TARGET_DIR}/debug)
endif()

include(ExternalProject)
ExternalProject_Add(
	crate
	DOWNLOAD_COMMAND ""
	CONFIGURE_COMMAND ""
	SOURCE_DIR ${CMAKE_SOURCE_DIR}/rust
	BINARY_DIR ${TARGET_DIR}/include
	BUILD_ALWAYS ON
	BUILD_COMMAND cargo build ${CARGO_BUILD_TYPE} --manifest-path=${CMAKE_SOURCE_DIR}/rust/Cargo.toml --features=bindings
	INSTALL_COMMAND ""
)
add_dependencies(dqcsim crate)

# Add dependencies
target_include_directories(dqcsim INTERFACE
	$<BUILD_INTERFACE:${TARGET_DIR}/include>
	$<INSTALL_INTERFACE:include>
)
set_target_properties(dqcsim PROPERTIES IMPORTED_LOCATION ${BUILD_DIR}/${CMAKE_SHARED_MODULE_PREFIX}dqcsim${CMAKE_SHARED_LIBRARY_SUFFIX})

# Install header files
install(
	FILES
		${TARGET_DIR}/include/dqcsim
		${TARGET_DIR}/include/cdqcsim
		${TARGET_DIR}/include/dqcsim.h
	DESTINATION include
)

# Install shared library
get_target_property(DQCSIM_LIB dqcsim IMPORTED_LOCATION)
install(
	FILES ${DQCSIM_LIB}
	DESTINATION lib
)

# Install CMake module
include(CMakePackageConfigHelpers)
configure_package_config_file(dqcsimConfig.cmake.in
	${CMAKE_CURRENT_BINARY_DIR}/dqcsimConfig.cmake
	INSTALL_DESTINATION lib/cmake/dqcsim
)
write_basic_package_version_file(
	${CMAKE_CURRENT_BINARY_DIR}/dqcsimConfigVersion.cmake
	VERSION
	COMPATIBILITY SameMajorVersion
)
install(
	FILES
		${CMAKE_CURRENT_BINARY_DIR}/dqcsimConfig.cmake
		${CMAKE_CURRENT_BINARY_DIR}/dqcsimConfigVersion.cmake
	DESTINATION lib/cmake/dqcsim
)