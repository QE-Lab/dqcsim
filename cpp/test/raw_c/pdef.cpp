#include <dqcsim.h>
#include "gtest/gtest.h"
#include <fcntl.h>
#include <math.h>
#include "util.h"

// Sanity check the plugin definition API.
TEST(pdef, sanity) {
  // Create handle.
  dqcs_handle_t a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that the handle is OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_FRONT_DEF);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Check that the handle is no longer OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_handle_dump(a), nullptr);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(a) + " is invalid");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test the plugin types.
TEST(pdef, types) {
  // Frontend.
  dqcs_handle_t a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_FRONT_DEF);
  EXPECT_EQ(dqcs_pdef_type(a), dqcs_plugin_type_t::DQCS_PTYPE_FRONT);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Operator.
  a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_OPER, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_OPER_DEF);
  EXPECT_EQ(dqcs_pdef_type(a), dqcs_plugin_type_t::DQCS_PTYPE_OPER);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Backend.
  a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_BACK_DEF);
  EXPECT_EQ(dqcs_pdef_type(a), dqcs_plugin_type_t::DQCS_PTYPE_BACK);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Invalid.
  EXPECT_EQ(dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_INVALID, "a", "b", "c"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid plugin type");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test the metadata.
TEST(pdef, metadata) {
  char *s;

  // Create handle.
  dqcs_handle_t a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check metadata.
  EXPECT_STREQ(s = dqcs_pdef_name(a), "a");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_pdef_author(a), "b");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_pdef_version(a), "c");
  if (s) free(s);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Check that the metadata is required.
  EXPECT_EQ(dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "", "b", "c"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: plugin name is required");
  EXPECT_EQ(dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, "b", "c"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: plugin name is required");
  EXPECT_EQ(dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "", "c"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: author name is required");
  EXPECT_EQ(dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", NULL, "c"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: author name is required");
  EXPECT_EQ(dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", ""), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: version string is required");
  EXPECT_EQ(dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", NULL), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: version string is required");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

void free_cb(void *user_data) {
  (*((int*)user_data))++;
}

namespace pdef {

  dqcs_return_t initialize_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t init_cmds) {
    return dqcs_return_t::DQCS_FAILURE;
  }

}

dqcs_return_t drop_cb(void *user_data, dqcs_plugin_state_t state) {
  return dqcs_return_t::DQCS_FAILURE;
}

dqcs_handle_t run_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t args) {
  return 0u;
}

dqcs_return_t allocate_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t qubits, dqcs_handle_t alloc_cmds) {
  return dqcs_return_t::DQCS_FAILURE;
}

dqcs_return_t free_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t qubits) {
  return dqcs_return_t::DQCS_FAILURE;
}

dqcs_handle_t gate_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t gate) {
  return 0u;
}

dqcs_handle_t modify_measurement_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t meas) {
  return 0u;
}

dqcs_return_t advance_cb(void *user_data, dqcs_plugin_state_t state,  dqcs_cycle_t cycles) {
  return dqcs_return_t::DQCS_FAILURE;
}

dqcs_handle_t upstream_arb_cb(void *user_data, dqcs_plugin_state_t state,  dqcs_handle_t cmd) {
  return 0u;
}

dqcs_handle_t host_arb_cb(void *user_data, dqcs_plugin_state_t state,  dqcs_handle_t cmd) {
  return 0u;
}

// Test frontend callback setters.
TEST(pdef, frontend_cb) {
  int user = 0;

  // Create handle.
  dqcs_handle_t a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Try setting all the supported callbacks.
  EXPECT_EQ(dqcs_pdef_set_initialize_cb(a, pdef::initialize_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_drop_cb(a, drop_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_run_cb(a, run_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_host_arb_cb(a, host_arb_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(user, 0);

  // Try setting nonsensical callbacks.
  EXPECT_EQ(dqcs_pdef_set_allocate_cb(a, allocate_cb, free_cb, &user), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid operation: the allocate() callback is not supported for frontends");
  EXPECT_EQ(dqcs_pdef_set_free_cb(a, free_cb, free_cb, &user), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid operation: the free() callback is not supported for frontends");
  EXPECT_EQ(dqcs_pdef_set_gate_cb(a, gate_cb, free_cb, &user), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid operation: the gate() callback is not supported for frontends");
  EXPECT_EQ(dqcs_pdef_set_modify_measurement_cb(a, modify_measurement_cb, free_cb, &user), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid operation: the modify_measurement() callback is only supported for operators");
  EXPECT_EQ(dqcs_pdef_set_advance_cb(a, advance_cb, free_cb, &user), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid operation: the advance() callback is not supported for frontends");
  EXPECT_EQ(dqcs_pdef_set_upstream_arb_cb(a, upstream_arb_cb, free_cb, &user), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid operation: the upstream_arb() callback is not supported for frontends");
  EXPECT_EQ(user, 6);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(user, 10);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test operator callback setters.
TEST(pdef, operator_cb) {
  int user = 0;

  // Create handle.
  dqcs_handle_t a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_OPER, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Try setting all the supported callbacks.
  EXPECT_EQ(dqcs_pdef_set_initialize_cb(a, pdef::initialize_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_drop_cb(a, drop_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_allocate_cb(a, allocate_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_free_cb(a, free_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_gate_cb(a, gate_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_modify_measurement_cb(a, modify_measurement_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_advance_cb(a, advance_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_upstream_arb_cb(a, upstream_arb_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_host_arb_cb(a, host_arb_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(user, 0);

  // Try setting nonsensical callbacks.
  EXPECT_EQ(dqcs_pdef_set_run_cb(a, run_cb, free_cb, &user), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid operation: the run() callback is only supported for frontends");
  EXPECT_EQ(user, 1);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(user, 10);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test backend callback setters.
TEST(pdef, backend_cb) {
  int user = 0;

  // Create handle.
  dqcs_handle_t a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Try setting all the supported callbacks.
  EXPECT_EQ(dqcs_pdef_set_initialize_cb(a, pdef::initialize_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_drop_cb(a, drop_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_allocate_cb(a, allocate_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_free_cb(a, free_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_gate_cb(a, gate_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_advance_cb(a, advance_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_upstream_arb_cb(a, upstream_arb_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pdef_set_host_arb_cb(a, host_arb_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(user, 0);

  // Try setting nonsensical callbacks.
  EXPECT_EQ(dqcs_pdef_set_modify_measurement_cb(a, modify_measurement_cb, free_cb, &user), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid operation: the modify_measurement() callback is only supported for operators");
  EXPECT_EQ(dqcs_pdef_set_run_cb(a, run_cb, free_cb, &user), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid operation: the run() callback is only supported for frontends");
  EXPECT_EQ(user, 2);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(user, 10);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

