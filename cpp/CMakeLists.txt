add_compile_unit(
  NAME dqcsim::cpp
  TYPE INTERFACE
)

file(GLOB_RECURSE TESTS test/**.cpp)
foreach(TEST ${TESTS})
  file(RELATIVE_PATH TEST_NAME ${CMAKE_CURRENT_SOURCE_DIR}/test ${TEST})
  string(REPLACE / :: TEST_NAME ${TEST_NAME})
  get_filename_component(TEST_NAME ${TEST_NAME} NAME_WE)
  add_compile_unit(
    NAME dqcsim::tests::${TEST_NAME}
    TYPE TESTS
    PRPS
      CXX_STANDARD 11
      CXX_STANDARD_REQUIRED ON
    SRCS
      "${TEST}"
    DEPS
      dqcsim
      dqcsim::cpp
  )
endforeach()
