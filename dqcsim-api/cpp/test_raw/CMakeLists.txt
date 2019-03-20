macro(package_add_test TESTNAME)
    add_executable(${TESTNAME} ${ARGN})
    target_link_libraries(${TESTNAME} gtest gmock gtest_main dqcsim)
    add_test(NAME ${TESTNAME} COMMAND ${TESTNAME})
    set_target_properties(${TESTNAME} PROPERTIES FOLDER tests)
endmacro()

package_add_test(test1 test1.cpp)
