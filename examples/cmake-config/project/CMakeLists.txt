cmake_minimum_required(VERSION 3.11.0 FATAL_ERROR)

project(dqcsim-cmake-example
    VERSION 0.0.1
    DESCRIPTION "Example for linking to DQCsim with CMake"
    LANGUAGES CXX
)

# Let's say you have a C++ executable that needs DQCsim...
set(CMAKE_CXX_STANDARD 11) # <-- DQCsim's header needs C++11
add_executable(main main.cpp)
# ... then the most straightforward way to make it depend on DQCsim is as
# follows.

# Find dqcsim using the installed config file.
find_package(dqcsim REQUIRED)

# Now we just need to tell CMake to link to dqcsim.
target_link_libraries(main dqcsim)
