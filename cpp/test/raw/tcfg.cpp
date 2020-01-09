#include <dqcsim.h>
#include "gtest/gtest.h"
#include "util.h"

// Sanity check the plugin thread configuration API.
TEST(tcfg, sanity) {
  // Create handle.
  dqcs_handle_t a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  a = dqcs_tcfg_new(a, "d");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that the handle is OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_FRONT_THREAD_CONFIG);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Check that the handle is no longer OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_handle_dump(a), nullptr);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(a) + " is invalid");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test construction using various plugin definitions.
TEST(tcfg, types) {
  // Frontend.
  dqcs_handle_t a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  a = dqcs_tcfg_new(a, "d");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_FRONT_THREAD_CONFIG);
  EXPECT_EQ(dqcs_tcfg_type(a), dqcs_plugin_type_t::DQCS_PTYPE_FRONT);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Operator.
  a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_OPER, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  a = dqcs_tcfg_new(a, "d");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_OPER_THREAD_CONFIG);
  EXPECT_EQ(dqcs_tcfg_type(a), dqcs_plugin_type_t::DQCS_PTYPE_OPER);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Backend.
  a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  a = dqcs_tcfg_new(a, "d");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_BACK_THREAD_CONFIG);
  EXPECT_EQ(dqcs_tcfg_type(a), dqcs_plugin_type_t::DQCS_PTYPE_BACK);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Invalid.
  EXPECT_EQ(dqcs_tcfg_new(33, "d"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 33 is invalid");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test plugin instance name.
TEST(tcfg, name) {
  char *s;

  // Valid name.
  dqcs_handle_t a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  a = dqcs_tcfg_new(a, "d");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_STREQ(s = dqcs_tcfg_name(a), "d");
  if (s) free(s);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Default name using NULL.
  a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  a = dqcs_tcfg_new(a, NULL);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_STREQ(s = dqcs_tcfg_name(a), "");
  if (s) free(s);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Default name using empty string.
  a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  a = dqcs_tcfg_new(a, "");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_STREQ(s = dqcs_tcfg_name(a), "");
  if (s) free(s);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test init cmds.
TEST(tcfg, init) {
  dqcs_handle_t a, b;
  char *s;

  // Create a fresh config.
  a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  a = dqcs_tcfg_new(a, "d");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that there are initially no init cmds.
  s = dqcs_handle_dump(a);
  EXPECT_STREQ(extract_array_from_dump("init_cmds:", s), "init_cmds: []");
  if (s) free(s);

  // Add a command.
  b = dqcs_cmd_new("a", "b");
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_tcfg_init_cmd(a, b), dqcs_return_t::DQCS_SUCCESS);
  s = dqcs_handle_dump(a);
  EXPECT_STREQ(extract_array_from_dump("init_cmds:", s), "init_cmds: [ ArbCmd { interface_identifier: \"a\", operation_identifier: \"b\", data: ArbData { json: Map( {}, ), args: [], }, },]");
  if (s) free(s);

  // Some errors.
  b = dqcs_arb_new();
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_tcfg_init_cmd(a, b), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: object does not support the cmd interface");
  EXPECT_EQ(dqcs_handle_delete(b), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(dqcs_tcfg_init_cmd(a, 0), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 0 is invalid");

  // Delete the handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test tee files.
TEST(tcfg, tee) {
  dqcs_handle_t a;
  char *s;

  // Create a fresh config.
  a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  a = dqcs_tcfg_new(a, "d");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that there are initially no tee files.
  s = dqcs_handle_dump(a);
  EXPECT_STREQ(extract_array_from_dump("tee_files:", s), "tee_files: []");
  if (s) free(s);

  // Add some tees. Note that this does notcreates the files. Note that we
  // don't test the levels at this time.
  EXPECT_EQ(dqcs_tcfg_tee(a, dqcs_loglevel_t::DQCS_LOG_WARN, "warnings"), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_tcfg_tee(a, dqcs_loglevel_t::DQCS_LOG_TRACE, "trace"), dqcs_return_t::DQCS_SUCCESS);

  // Check that the tee file configurations were added.
  s = dqcs_handle_dump(a);
  EXPECT_STREQ(extract_array_from_dump("tee_files:", s), "tee_files: [ TeeFileConfiguration { filter: Warn, file: \"warnings\", }, TeeFileConfiguration { filter: Trace, file: \"trace\", },]");
  if (s) free(s);

  // Check that we can't do silly things.
  EXPECT_EQ(dqcs_tcfg_tee(a, dqcs_loglevel_t::DQCS_LOG_INVALID, "x"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid level");
  EXPECT_EQ(dqcs_tcfg_tee(a, dqcs_loglevel_t::DQCS_LOG_PASS, "x"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid loglevel filter DQCS_LOG_PASS");

  // Delete the handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test verbosity configuration.
TEST(tcfg, verbosity) {
  dqcs_handle_t a;

  // Create a fresh config.
  a = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  a = dqcs_tcfg_new(a, "d");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check the default value. Note that this is trace because plugin loglevel
  // is automatically limited to the most verbose log message sink, and usually
  // only the sink level is set.
  EXPECT_EQ(dqcs_tcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  // Check all values.
  EXPECT_EQ(dqcs_tcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_INVALID), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid level");
  EXPECT_EQ(dqcs_tcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  EXPECT_EQ(dqcs_tcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_OFF), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_tcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_OFF);

  EXPECT_EQ(dqcs_tcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_FATAL), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_tcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_FATAL);

  EXPECT_EQ(dqcs_tcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_ERROR), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_tcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_ERROR);

  EXPECT_EQ(dqcs_tcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_WARN), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_tcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_WARN);

  EXPECT_EQ(dqcs_tcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_NOTE), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_tcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_NOTE);

  EXPECT_EQ(dqcs_tcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_INFO), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_tcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_INFO);

  EXPECT_EQ(dqcs_tcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_DEBUG), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_tcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_DEBUG);

  EXPECT_EQ(dqcs_tcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_TRACE), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_tcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  EXPECT_EQ(dqcs_tcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_PASS), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid loglevel filter DQCS_LOG_PASS");
  EXPECT_EQ(dqcs_tcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  // Delete the handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}
