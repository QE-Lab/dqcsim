#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"

using namespace dqcsim;

// Sanity check the measurement set API.
TEST(mset, sanity) {
  // Create handle.
  dqcs_handle_t a = dqcs_mset_new();
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that the handle is OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_MEAS_SET);
  EXPECT_STREQ(dqcs_handle_dump(a), "QubitMeasurementResultSet(\n    {}\n)");

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Check that the handle is no longer OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_handle_dump(a), nullptr);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(a) + " is invalid");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test the measurement set API.
TEST(mset, test) {
  // Create handle.
  dqcs_handle_t a = dqcs_mset_new();
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that the set is initially empty.
  EXPECT_EQ(dqcs_mset_len(a), 0);

  // Push some measurements into the set.
  dqcs_handle_t b = dqcs_meas_new(1, dqcs_measurement_t::DQCS_MEAS_ZERO);
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_mset_set(a, b), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  b = dqcs_meas_new(2, dqcs_measurement_t::DQCS_MEAS_ONE);
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_mset_set(a, b), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  b = dqcs_meas_new(3, dqcs_measurement_t::DQCS_MEAS_UNDEFINED);
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_mset_set(a, b), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  // Verify the number of measurements.
  EXPECT_EQ(dqcs_mset_len(a), 3);

  // Check contains.
  EXPECT_EQ(dqcs_mset_contains(a, 2u), dqcs_bool_return_t::DQCS_TRUE);
  EXPECT_EQ(dqcs_mset_contains(a, 4u), dqcs_bool_return_t::DQCS_FALSE);

  // Check getter.
  EXPECT_NE(b = dqcs_mset_get(a, 2u), 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_meas_qubit_get(b), 2u);
  EXPECT_EQ(dqcs_meas_value_get(b), dqcs_measurement_t::DQCS_MEAS_ONE);
  EXPECT_EQ(dqcs_handle_delete(b), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(dqcs_mset_get(a, 4u), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: qubit not included in measurement set");

  // Check taker.
  EXPECT_NE(b = dqcs_mset_take(a, 2u), 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_meas_qubit_get(b), 2u);
  EXPECT_EQ(dqcs_meas_value_get(b), dqcs_measurement_t::DQCS_MEAS_ONE);
  EXPECT_EQ(dqcs_handle_delete(b), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(dqcs_mset_take(a, 2u), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: qubit not included in measurement set");

  EXPECT_EQ(dqcs_mset_len(a), 2);

  // Check remover.
  EXPECT_EQ(dqcs_mset_remove(a, 3u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_mset_remove(a, 3u), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: qubit not included in measurement set");

  EXPECT_EQ(dqcs_mset_len(a), 1);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}
