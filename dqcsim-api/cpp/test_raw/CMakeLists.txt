set(CMAKE_CXX_FLAGS "-ggdb")

macro(package_add_test TESTNAME)
    add_executable(${TESTNAME} ${ARGN})
    target_link_libraries(${TESTNAME} gtest gmock gtest_main dqcsim)
    add_test(NAME ${TESTNAME} COMMAND ${TESTNAME})
    set_target_properties(${TESTNAME} PROPERTIES FOLDER tests)
    target_compile_options(${TESTNAME} PRIVATE -Wall -Werror)
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
package_add_test(tcfg tcfg.cpp)
package_add_test(scfg scfg.cpp)
package_add_test(sim_new sim_new.cpp)

# Plugin
package_add_test(pdef pdef.cpp)

# Integration
package_add_test(sim_arbs sim_arbs.cpp)
package_add_test(sim_host sim_host.cpp)
package_add_test(sim_init_drop_log sim_init_drop_log.cpp)
package_add_test(sim_special_start sim_special_start.cpp)
package_add_test(sim_random sim_random.cpp)
