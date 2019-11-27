#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"

using namespace dqcsim;

// Sanity-check ArbCmd queue handles.
TEST(cq, sanity) {
  // Create handle.
  dqcs_handle_t a = dqcs_cq_new();
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that the handle is OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_ARB_CMD_QUEUE);
  EXPECT_STREQ(dqcs_handle_dump(a), "ArbCmdQueue(\n    [],\n)");

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Check that the handle is no longer OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_handle_dump(a), nullptr);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(a) + " is invalid");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test the ArbCmd queue API.
TEST(cq, test) {
  // Create handle.
  dqcs_handle_t a = dqcs_cq_new();
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Assert that length starts at 0.
  EXPECT_EQ(dqcs_cq_len(a), 0);

  // Push a command.
  dqcs_handle_t b = dqcs_cmd_new("a", "b");
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_cq_push(a, b), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_handle_type(b), dqcs_handle_type_t::DQCS_HTYPE_INVALID);

  // Assert that length is now 1.
  EXPECT_EQ(dqcs_cq_len(a), 1);

  // Second command.
  b = dqcs_cmd_new("c", "d");
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_cq_push(a, b), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_handle_type(b), dqcs_handle_type_t::DQCS_HTYPE_INVALID);

  // Assert that length is now 2.
  EXPECT_EQ(dqcs_cq_len(a), 2);

  // Try to push nonsense.
  b = dqcs_arb_new();
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_cq_push(a, b), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: object does not support the cmd interface");
  EXPECT_EQ(dqcs_handle_type(b), dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA);
  EXPECT_EQ(dqcs_handle_delete(b), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_cq_push(a, b), dqcs_return_t::DQCS_FAILURE);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(b) + " is invalid");

  // Assert that length is still 2.
  EXPECT_EQ(dqcs_cq_len(a), 2);

  // We should be able to treat this handle as an ArbCmd, which should return
  // the first one.
  char *s;
  EXPECT_STREQ(s = dqcs_cmd_iface_get(a), "a") << "Unexpected error: " << dqcs_error_get();
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_cmd_oper_get(a), "b") << "Unexpected error: " << dqcs_error_get();
  if (s) free(s);

  // Pop the first one.
  EXPECT_EQ(dqcs_cq_next(a), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_cq_len(a), 1);

  // Now we should see the second one.
  EXPECT_STREQ(s = dqcs_cmd_iface_get(a), "c") << "Unexpected error: " << dqcs_error_get();
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_cmd_oper_get(a), "d") << "Unexpected error: " << dqcs_error_get();
  if (s) free(s);

  // Pop the second one.
  EXPECT_EQ(dqcs_cq_next(a), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_cq_len(a), 0);

  // See what happens when we try to use the cmd interface on an empty queue.
  EXPECT_EQ(s = dqcs_cmd_iface_get(a), (char*)NULL);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: empty command queue does not support cmd interface");
  if (s) free(s);

  // See what happens when we pop too much.
  EXPECT_EQ(dqcs_cq_next(a), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: the command queue is already empty");
  EXPECT_EQ(dqcs_cq_len(a), 0);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}
