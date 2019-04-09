#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"

using namespace dqcsim;

// Sanity check the qubit set API.
TEST(qbset, sanity) {
  // Create handle.
  dqcs_handle_t a = dqcs_qbset_new();
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that the handle is OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_QUBIT_SET);
  EXPECT_STREQ(dqcs_handle_dump(a), "QubitReferenceSet(\n    []\n)");

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Check that the handle is no longer OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_handle_dump(a), nullptr);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(a) + " is invalid");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test the qubit set API.
TEST(qbset, test) {
  // Create handle.
  dqcs_handle_t a = dqcs_qbset_new();
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that the set is initially empty.
  EXPECT_EQ(dqcs_qbset_len(a), 0);

  // Add some qubits.
  EXPECT_EQ(dqcs_qbset_push(a, 4), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(a, 42), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(a, 16), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(a, 15), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(a, 8), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(a, 23), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  // We cannot add the same qubit twice.
  EXPECT_EQ(dqcs_qbset_push(a, 8), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: the specified qubit is already part of the set");

  // Check the length.
  EXPECT_EQ(dqcs_qbset_len(a), 6);

  // Check the contains function.
  EXPECT_EQ(dqcs_qbset_contains(a, 3), dqcs_bool_return_t::DQCS_FALSE);
  EXPECT_EQ(dqcs_qbset_contains(a, 4), dqcs_bool_return_t::DQCS_TRUE);

  // Make a copy for iteration.
  dqcs_handle_t it = dqcs_qbset_copy(a);
  ASSERT_NE(it, 0u) << "Unexpected error: " << dqcs_error_get();

  // Deplete the "iterator"
  EXPECT_EQ(dqcs_qbset_pop(it), 4u);
  EXPECT_EQ(dqcs_qbset_pop(it), 42u);
  EXPECT_EQ(dqcs_qbset_pop(it), 16u);
  EXPECT_EQ(dqcs_qbset_pop(it), 15u);
  EXPECT_EQ(dqcs_qbset_pop(it), 8u);
  EXPECT_EQ(dqcs_qbset_pop(it), 23u);
  EXPECT_EQ(dqcs_qbset_pop(it), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: the qubit set is already empty");
  EXPECT_EQ(dqcs_handle_delete(it), dqcs_return_t::DQCS_SUCCESS);

  // The original set should still be intact.
  EXPECT_STREQ(dqcs_handle_dump(a), "QubitReferenceSet(\n    [\n        QubitRef(\n            4\n        ),\n        QubitRef(\n            42\n        ),\n        QubitRef(\n            16\n        ),\n        QubitRef(\n            15\n        ),\n        QubitRef(\n            8\n        ),\n        QubitRef(\n            23\n        )\n    ]\n)");

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

