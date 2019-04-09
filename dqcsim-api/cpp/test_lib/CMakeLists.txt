set(CMAKE_CXX_FLAGS "-ggdb")

macro(package_add_test TESTNAME)
    add_executable(${TESTNAME} ${ARGN})
    target_link_libraries(${TESTNAME} gtest gmock gtest_main dqcsim)
    add_test(NAME ${TESTNAME} COMMAND ${TESTNAME})
    set_target_properties(${TESTNAME} PROPERTIES FOLDER tests)
    target_compile_options(${TESTNAME} PRIVATE -Wall -Werror)
endmacro()

package_add_test(test2 test2.cpp)
