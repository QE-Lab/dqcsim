set(CMAKE_CXX_FLAGS "-ggdb")

macro(package_add_test TESTNAME)
    add_executable(${TESTNAME} ${ARGN})
    add_dependencies(${TESTNAME} dqcsim)
    target_link_libraries(${TESTNAME} gtest gmock gtest_main ${LIBDQCSIM})
    target_include_directories(${TESTNAME} PUBLIC ${LIBDQCSIM_INCLUDE})
    add_test(NAME ${TESTNAME} COMMAND ${TESTNAME})
    set_target_properties(${TESTNAME} PROPERTIES FOLDER tests)
    target_compile_options(${TESTNAME} PRIVATE -Wall -Werror)
endmacro()

package_add_test(test2 test2.cpp)
