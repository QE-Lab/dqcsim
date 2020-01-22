cmake_minimum_required(VERSION 3.11.0 FATAL_ERROR)

project(dqcsim-cmake-example
    VERSION 0.0.1
    DESCRIPTION "Example for linking to DQCsim with CMake"
    LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 11)

# Let's say you have an executable that needs DQCsim...
add_executable(main main.cpp)
# ... then the most straightforward way to make it depend on DQCsim is as
# follows.

# First of all, we need to find the DQCsim shared object file. CMake can handle
# this for you like this:
find_library(DQCSIM_LIB dqcsim)

# Similarly, we need to find the header files. You can tell CMake to do this by
# having it search for dqcsim.h.
find_path(DQCSIM_INC dqcsim.h)
# Even though they are valid header files, don't change this to just dqcsim or
# cdqcsim. find_path will search all kinds of directories, and may find the
# dqcsim command-line executable first.

# Now you should probably check if both were found. Otherwise you'll get weird
# messages later if they weren't.
if(NOT DQCSIM_LIB OR NOT DQCSIM_INC)
    message(FATAL_ERROR "DQCsim could not be found!)")
endif()

# Now we just need to tell CMake to use the library and headers when compiling
# your program. You do this as follows. This relies on your compiler being able
# to find the library as well; there's no good way around this unfortunately.
# You could also link against ${DQCSIM_LIB} directly, but then the path will be
# hardcoded, so the binary might not work on other systems.
get_filename_component(DQCSIM_LIB_PATH ${DQCSIM_LIB} DIRECTORY)
target_link_options(main PUBLIC -L${DQCSIM_LIB_PATH} -ldqcsim)
target_include_directories(main PRIVATE ${DQCSIM_INC})

# While the above works and is relatively easy to understand, you may want to
# look at the cmake-fetchcontent example as well. It needs a newer version of
# CMake, an internet connection, and may not be as easy to understand, but it's
# more concise, and lets you do something that you can't do with this: compile
# against a version of DQCsim that's built from source. This is great if you
# ever want to add debug statements to DQCsim's codebase while debugging your
# own stuff, or if you want to test with a specific DQCsim version that either
# isn't released yet or that you don't want to install in your root.
