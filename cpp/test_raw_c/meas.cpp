#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"

using namespace dqcsim;

// Sanity check the measurement result API.
TEST(meas, sanity) {
  // Create handle.
  dqcs_handle_t a = dqcs_meas_new(1, dqcs_measurement_t::DQCS_MEAS_ONE);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that the handle is OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_MEAS);
  EXPECT_STREQ(dqcs_handle_dump(a), "QubitMeasurementResult(\n    QubitMeasurementResult {\n        qubit: QubitRef(\n            1,\n        ),\n        value: One,\n        data: ArbData {\n            json: Map(\n                {},\n            ),\n            args: [],\n        },\n    },\n)");

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Check that the handle is no longer OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_handle_dump(a), nullptr);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(a) + " is invalid");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test the measurement result API.
TEST(meas, test) {
  char *s;

  // Check constructor errors.
  EXPECT_EQ(dqcs_meas_new(0, dqcs_measurement_t::DQCS_MEAS_ONE), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: 0 is not a valid qubit reference");

  EXPECT_EQ(dqcs_meas_new(1, dqcs_measurement_t::DQCS_MEAS_INVALID), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid measurement value specified");

  // Create handle.
  dqcs_handle_t a = dqcs_meas_new(2, dqcs_measurement_t::DQCS_MEAS_ZERO);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_json_set(a, "{\"probability_one\": 0.234}"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  // Check the contents.
  EXPECT_EQ(dqcs_meas_qubit_get(a), 2u);
  EXPECT_EQ(dqcs_meas_value_get(a), dqcs_measurement_t::DQCS_MEAS_ZERO);
  EXPECT_STREQ(s = dqcs_arb_json_get(a), "{\"probability_one\":0.234}");
  if (s) free(s);

  // Mutate the contents.
  EXPECT_EQ(dqcs_meas_qubit_set(a, 5u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_meas_value_set(a, dqcs_measurement_t::DQCS_MEAS_ONE), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  // Check the contents.
  EXPECT_EQ(dqcs_meas_qubit_get(a), 5u);
  EXPECT_EQ(dqcs_meas_value_get(a), dqcs_measurement_t::DQCS_MEAS_ONE);

  // Make sure we can't mutate the contents to something invalid.
  EXPECT_EQ(dqcs_meas_qubit_set(a, 0u), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: 0 is not a valid qubit reference");

  EXPECT_EQ(dqcs_meas_value_set(a, dqcs_measurement_t::DQCS_MEAS_INVALID), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid measurement value specified");

  // Check the contents.
  EXPECT_EQ(dqcs_meas_qubit_get(a), 5u);
  EXPECT_EQ(dqcs_meas_value_get(a), dqcs_measurement_t::DQCS_MEAS_ONE);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Check the contents.
  EXPECT_EQ(dqcs_meas_qubit_get(a), 0u);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(a) + " is invalid");

  EXPECT_EQ(dqcs_meas_value_get(a), dqcs_measurement_t::DQCS_MEAS_INVALID);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(a) + " is invalid");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}
