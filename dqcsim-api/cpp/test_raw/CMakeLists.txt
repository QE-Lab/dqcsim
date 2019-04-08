set(CMAKE_CXX_FLAGS "-ggdb")

macro(package_add_test TESTNAME)
    add_executable(${TESTNAME} ${ARGN})
    target_link_libraries(${TESTNAME} gtest gmock gtest_main dqcsim)
    add_test(NAME ${TESTNAME} COMMAND ${TESTNAME})
    set_target_properties(${TESTNAME} PROPERTIES FOLDER tests)
endmacro()

# Common
package_add_test(arb arb.cpp)
package_add_test(cmd cmd.cpp)
package_add_test(cq cq.cpp)
package_add_test(error error.cpp)
package_add_test(gate gate.cpp)
package_add_test(handle handle.cpp)
package_add_test(meas meas.cpp)
package_add_test(mset mset.cpp)
package_add_test(qbset qbset.cpp)

# Host
package_add_test(pcfg pcfg.cpp)
