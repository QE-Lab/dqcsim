#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"

using namespace dqcsim;

// Test the handle API.
TEST(handle, test) {
  // There should initially not be any handles.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();

  // Deleting, type-querying, or dumping invalid/non-existant handles should
  // throw errors.
  EXPECT_EQ(dqcs_handle_type(0), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 0 is invalid");

  EXPECT_EQ(dqcs_handle_type(33), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 33 is invalid");

  EXPECT_EQ(dqcs_handle_delete(0), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 0 is invalid");

  EXPECT_EQ(dqcs_handle_delete(33), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 33 is invalid");

  EXPECT_STREQ(dqcs_handle_dump(0), NULL);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 0 is invalid");

  EXPECT_STREQ(dqcs_handle_dump(33), NULL);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 33 is invalid");

  // Check dqcs_handle_delete_all() by making some random handles and then
  // checking that they don't exist anymore. We also check that handle numbers
  // monotonously increase, at least in this simple scenario.
  dqcs_handle_t a = dqcs_arb_new();
  EXPECT_EQ(a, 1) << "Unexpected error: " << dqcs_error_get();
  dqcs_handle_t b = dqcs_qbset_new();
  EXPECT_EQ(b, 2) << "Unexpected error: " << dqcs_error_get();
  dqcs_handle_t c = dqcs_mset_new();
  EXPECT_EQ(c, 3) << "Unexpected error: " << dqcs_error_get();
  dqcs_handle_t d = dqcs_cmd_new("a", "b");
  EXPECT_EQ(d, 4) << "Unexpected error: " << dqcs_error_get();

  EXPECT_EQ(dqcs_handle_delete_all(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();

  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 1 is invalid");
  EXPECT_EQ(dqcs_handle_type(b), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 2 is invalid");
  EXPECT_EQ(dqcs_handle_type(c), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 3 is invalid");
  EXPECT_EQ(dqcs_handle_type(d), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 4 is invalid");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}
