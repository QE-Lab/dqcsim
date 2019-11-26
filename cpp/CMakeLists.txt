add_compile_unit(
  NAME dqcsim::cpp
  TYPE INTERFACE
)

add_compile_unit(
  NAME dqcsim::cpp::tests
  TYPE TESTS
  PRPS
    CXX_STANDARD 11
    CXX_STANDARD_REQUIRED ON
  SRCS
    test/test2.cpp
  DEPS
    dqcsim
    dqcsim::cpp
)

add_compile_unit(
  NAME dqcsim::c::tests::common
  TYPE TESTS
  PRPS
    CXX_STANDARD 11
    CXX_STANDARD_REQUIRED ON
  SRCS
    test/raw_c/arb.cpp
    test/raw_c/cmd.cpp
    test/raw_c/cq.cpp
    test/raw_c/error.cpp
    test/raw_c/gate.cpp
    test/raw_c/handle.cpp
    test/raw_c/meas.cpp
    test/raw_c/mset.cpp
    test/raw_c/qbset.cpp
  DEPS
    dqcsim
)

add_compile_unit(
  NAME dqcsim::c::tests::host
  TYPE TESTS
  PRPS
    CXX_STANDARD 11
    CXX_STANDARD_REQUIRED ON
  SRCS
    test/raw_c/pcfg.cpp
    test/raw_c/tcfg.cpp
    test/raw_c/scfg.cpp
    test/raw_c/sim_new.cpp
  DEPS
    dqcsim
)

add_compile_unit(
  NAME dqcsim::c::tests::plugin
  TYPE TESTS
  PRPS
    CXX_STANDARD 11
    CXX_STANDARD_REQUIRED ON
  SRCS
    test/raw_c/pdef.cpp
  DEPS
    dqcsim
)

#TODO(mb): fix tests
add_compile_unit(
  NAME dqcsim::c::tests::integration
  TYPE TESTS
  PRPS
    CXX_STANDARD 11
    CXX_STANDARD_REQUIRED ON
  SRCS
    test/raw_c/sim_arbs.cpp
    test/raw_c/sim_host.cpp
    test/raw_c/sim_special_start.cpp
    test/raw_c/plugin_init_drop_log.cpp
    test/raw_c/plugin_alloc_free.cpp
    test/raw_c/plugin_gates.cpp
    test/raw_c/plugin_random.cpp
  DEPS
    dqcsim
)
