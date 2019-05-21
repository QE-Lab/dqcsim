#------------------------------------------------------------------------------
# Search paths
#------------------------------------------------------------------------------
if(TEST_CPP)
    add_subdirectory(test_cpp)
endif()

if(TEST_RAW_C)
    add_subdirectory(test_raw_c)
endif()
