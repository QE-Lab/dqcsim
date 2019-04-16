#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"

using namespace dqcsim;

// Test the handle API.
TEST(handle, test) {
  // There should initially not be any handles.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();

  // Deleting, type-querying, or dumping invalid/non-existant handles should
  // throw errors.
  EXPECT_EQ(dqcs_handle_type(0u), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 0 is invalid");

  EXPECT_EQ(dqcs_handle_type(33u), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 33 is invalid");

  EXPECT_EQ(dqcs_handle_delete(0u), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 0 is invalid");

  EXPECT_EQ(dqcs_handle_delete(33u), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 33 is invalid");

  EXPECT_STREQ(dqcs_handle_dump(0u), NULL);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 0 is invalid");

  EXPECT_STREQ(dqcs_handle_dump(33u), NULL);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 33 is invalid");

  // Check dqcs_handle_delete_all() by making some random handles and then
  // checking that they don't exist anymore. We also check that handle numbers
  // monotonously increase, at least in this simple scenario.
  dqcs_handle_t a = dqcs_arb_new();
  EXPECT_EQ(a, 1u) << "Unexpected error: " << dqcs_error_get();

  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Leak check: 1 handles remain, 1 = ArbData(ArbData { json: Object({}), args: [] })");

  dqcs_handle_t b = dqcs_qbset_new();
  EXPECT_EQ(b, 2u) << "Unexpected error: " << dqcs_error_get();
  dqcs_handle_t c = dqcs_mset_new();
  EXPECT_EQ(c, 3u) << "Unexpected error: " << dqcs_error_get();
  dqcs_handle_t d = dqcs_cmd_new("a", "b");
  EXPECT_EQ(d, 4u) << "Unexpected error: " << dqcs_error_get();

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

  // Make sure that the leak message cuts off after 10 leaks.
  for (int i = 0; i < 15; i++) {
    dqcs_arb_new();
  }
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Leak check: 15 handles remain, 5 = ArbData(ArbData { json: Object({}), args: [] }), 6 = ArbData(ArbData { json: Object({}), args: [] }), 7 = ArbData(ArbData { json: Object({}), args: [] }), 8 = ArbData(ArbData { json: Object({}), args: [] }), 9 = ArbData(ArbData { json: Object({}), args: [] }), 10 = ArbData(ArbData { json: Object({}), args: [] }), 11 = ArbData(ArbData { json: Object({}), args: [] }), 12 = ArbData(ArbData { json: Object({}), args: [] }), 13 = ArbData(ArbData { json: Object({}), args: [] }), 14 = ArbData(ArbData { json: Object({}), args: [] }), and 5 more");

  // Cleanup.
  EXPECT_EQ(dqcs_handle_delete_all(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}
