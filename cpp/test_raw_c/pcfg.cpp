#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"
#include <fcntl.h>
#include <math.h>
#include "util.h"

using namespace dqcsim;

// Sanity check the plugin configuration API.
TEST(pcfg, sanity) {
  // Create handle.
  dqcs_handle_t a = dqcs_pcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, "x", NULL);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that the handle is OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_FRONT_PROCESS_CONFIG);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Check that the handle is no longer OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_handle_dump(a), nullptr);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(a) + " is invalid");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test raw constructor properties.
TEST(pcfg, raw_constructor) {
  dqcs_handle_t a;
  char *s;

  a = dqcs_pcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, "x", NULL);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_FRONT_PROCESS_CONFIG);
  EXPECT_EQ(dqcs_pcfg_type(a), dqcs_plugin_type_t::DQCS_PTYPE_FRONT);
  EXPECT_STREQ(s = dqcs_pcfg_name(a), "");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_pcfg_executable(a), "x");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_pcfg_script(a), "");
  if (s) free(s);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  a = dqcs_pcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_OPER, "", "x", "y");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_OPER_PROCESS_CONFIG);
  EXPECT_EQ(dqcs_pcfg_type(a), dqcs_plugin_type_t::DQCS_PTYPE_OPER);
  EXPECT_STREQ(s = dqcs_pcfg_name(a), "");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_pcfg_executable(a), "x");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_pcfg_script(a), "y");
  if (s) free(s);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  a = dqcs_pcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_BACK, "name", "x", "");
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_BACK_PROCESS_CONFIG);
  EXPECT_EQ(dqcs_pcfg_type(a), dqcs_plugin_type_t::DQCS_PTYPE_BACK);
  EXPECT_STREQ(s = dqcs_pcfg_name(a), "name");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_pcfg_executable(a), "x");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_pcfg_script(a), "");
  if (s) free(s);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(dqcs_pcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_OPER, NULL, "", NULL), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: plugin executable must not be empty");

  EXPECT_EQ(dqcs_pcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_OPER, NULL, NULL, NULL), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: plugin executable must not be empty");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test desugaring (within reason; we don't check searching the system path).
TEST(pcfg, sugared_constructor) {
  dqcs_handle_t a;
  char *s;

  // Test native desugaring.
  unlink("dqcsfehello");
  unlink("hello");

  EXPECT_EQ(dqcs_pcfg_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, "hello"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: could not find plugin executable 'dqcsfehello', needed for plugin specification 'hello'");

  EXPECT_EQ(dqcs_pcfg_new(dqcs_plugin_type_t::DQCS_PTYPE_OPER, NULL, "hello"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: could not find plugin executable 'dqcsophello', needed for plugin specification 'hello'");

  EXPECT_EQ(dqcs_pcfg_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, NULL, "hello"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: could not find plugin executable 'dqcsbehello', needed for plugin specification 'hello'");

  close(open("dqcsfehello", O_RDWR | O_CREAT, S_IRUSR | S_IRGRP | S_IROTH));

  EXPECT_NE(a = dqcs_pcfg_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, "hello"), 0u);
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_FRONT_PROCESS_CONFIG);
  EXPECT_EQ(dqcs_pcfg_type(a), dqcs_plugin_type_t::DQCS_PTYPE_FRONT);
  EXPECT_STREQ(s = dqcs_pcfg_name(a), "");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_pcfg_executable(a), "dqcsfehello");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_pcfg_script(a), "");
  if (s) free(s);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(dqcs_pcfg_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, NULL, "hello"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: could not find plugin executable 'dqcsbehello', needed for plugin specification 'hello'");

  close(open("hello", O_RDWR | O_CREAT, S_IRUSR | S_IRGRP | S_IROTH));

  EXPECT_NE(a = dqcs_pcfg_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, "", "hello"), 0u);
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_BACK_PROCESS_CONFIG);
  EXPECT_EQ(dqcs_pcfg_type(a), dqcs_plugin_type_t::DQCS_PTYPE_BACK);
  EXPECT_STREQ(s = dqcs_pcfg_name(a), "");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_pcfg_executable(a), "hello");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_pcfg_script(a), "");
  if (s) free(s);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  unlink("dqcsfehello");
  unlink("hello");

  // Test script desugaring.
  unlink("hello.xyz");
  unlink("dqcsopxyz");

  EXPECT_EQ(dqcs_pcfg_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, "hello.xyz"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: could not find plugin executable 'dqcsfehello.xyz', needed for plugin specification 'hello.xyz'");

  EXPECT_EQ(dqcs_pcfg_new(dqcs_plugin_type_t::DQCS_PTYPE_OPER, NULL, "hello.xyz"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: could not find plugin executable 'dqcsophello.xyz', needed for plugin specification 'hello.xyz'");

  EXPECT_EQ(dqcs_pcfg_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, NULL, "hello.xyz"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: could not find plugin executable 'dqcsbehello.xyz', needed for plugin specification 'hello.xyz'");

  close(open("hello.xyz", O_RDWR | O_CREAT, S_IRUSR | S_IRGRP | S_IROTH));

  EXPECT_EQ(dqcs_pcfg_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, "hello.xyz"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: could not find plugin executable 'dqcsfexyz', needed for plugin specification 'hello.xyz'");

  EXPECT_EQ(dqcs_pcfg_new(dqcs_plugin_type_t::DQCS_PTYPE_OPER, NULL, "hello.xyz"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: could not find plugin executable 'dqcsopxyz', needed for plugin specification 'hello.xyz'");

  EXPECT_EQ(dqcs_pcfg_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, NULL, "hello.xyz"), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: could not find plugin executable 'dqcsbexyz', needed for plugin specification 'hello.xyz'");

  close(open("dqcsopxyz", O_RDWR | O_CREAT, S_IRUSR | S_IRGRP | S_IROTH));

  EXPECT_NE(a = dqcs_pcfg_new(dqcs_plugin_type_t::DQCS_PTYPE_OPER, "operator?", "hello.xyz"), 0u);
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_OPER_PROCESS_CONFIG);
  EXPECT_EQ(dqcs_pcfg_type(a), dqcs_plugin_type_t::DQCS_PTYPE_OPER);
  EXPECT_STREQ(s = dqcs_pcfg_name(a), "operator?");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_pcfg_executable(a), "dqcsopxyz");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_pcfg_script(a), "hello.xyz");
  if (s) free(s);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  unlink("hello.xyz");
  unlink("dqcsopxyz");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test working directory get/set.
TEST(pcfg, workdir) {
  dqcs_handle_t a;
  char *s;

  // Create a fresh config.
  a = dqcs_pcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, "x", NULL);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check the default value.
  ASSERT_STREQ(s = dqcs_pcfg_work_get(a), ".");
  if (s) free(s);

  // Set a different value.
  ASSERT_EQ(dqcs_pcfg_work_set(a, ".."), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_STREQ(s = dqcs_pcfg_work_get(a), "..");
  if (s) free(s);

  // Check invalid paths/nonexistant directories.
  ASSERT_EQ(dqcs_pcfg_work_set(a, "banana"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: not a directory");

  ASSERT_EQ(dqcs_pcfg_work_set(a, "/usr/bin/ls"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: not a directory");

  // Delete the handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test environment variable operators.
TEST(pcfg, env) {
  dqcs_handle_t a;
  char *s;

  // Create a fresh config.
  a = dqcs_pcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, "x", NULL);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that there are initially no env mods.
  s = dqcs_handle_dump(a);
  EXPECT_STREQ(extract_array_from_dump("env:", s), "env: []");
  if (s) free(s);

  // Override a key.
  EXPECT_EQ(dqcs_pcfg_env_set(a, "hello", "there"), dqcs_return_t::DQCS_SUCCESS);
  s = dqcs_handle_dump(a);
  EXPECT_STREQ(extract_array_from_dump("env:", s), "env: [ Set { key: \"hello\", value: \"there\" }]");
  if (s) free(s);

  // Delete a key, option A.
  EXPECT_EQ(dqcs_pcfg_env_set(a, "delete", NULL), dqcs_return_t::DQCS_SUCCESS);
  s = dqcs_handle_dump(a);
  EXPECT_STREQ(extract_array_from_dump("env:", s), "env: [ Set { key: \"hello\", value: \"there\" }, Remove { key: \"delete\" }]");
  if (s) free(s);

  // Delete a key, option B.
  EXPECT_EQ(dqcs_pcfg_env_unset(a, "unset"), dqcs_return_t::DQCS_SUCCESS);
  s = dqcs_handle_dump(a);
  EXPECT_STREQ(extract_array_from_dump("env:", s), "env: [ Set { key: \"hello\", value: \"there\" }, Remove { key: \"delete\" }, Remove { key: \"unset\" }]");
  if (s) free(s);

  // Some errors.
  EXPECT_EQ(dqcs_pcfg_env_set(a, NULL, "???"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: unexpected NULL string");
  EXPECT_EQ(dqcs_pcfg_env_unset(a, NULL), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: unexpected NULL string");

  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test init cmds.
TEST(pcfg, init) {
  dqcs_handle_t a, b;
  char *s;

  // Create a fresh config.
  a = dqcs_pcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, "x", NULL);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that there are initially no init cmds.
  s = dqcs_handle_dump(a);
  EXPECT_STREQ(extract_array_from_dump("init:", s), "init: []");
  if (s) free(s);

  // Add a command.
  b = dqcs_cmd_new("a", "b");
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_pcfg_init_cmd(a, b), dqcs_return_t::DQCS_SUCCESS);
  s = dqcs_handle_dump(a);
  EXPECT_STREQ(extract_array_from_dump("init:", s), "init: [ ArbCmd { interface_identifier: \"a\", operation_identifier: \"b\", data: ArbData { json: Object( {} ), args: [] } }]");
  if (s) free(s);

  // Some errors.
  b = dqcs_arb_new();
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_pcfg_init_cmd(a, b), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: object does not support the cmd interface");
  EXPECT_EQ(dqcs_handle_delete(b), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(dqcs_pcfg_init_cmd(a, 0), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 0 is invalid");

  // Delete the handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test tee files.
TEST(pcfg, tee) {
  dqcs_handle_t a;
  char *s;

  // Create a fresh config.
  a = dqcs_pcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, "x", NULL);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that there are initially no tee files.
  s = dqcs_handle_dump(a);
  EXPECT_STREQ(extract_array_from_dump("tee_files:", s), "tee_files: []");
  if (s) free(s);

  // Add some tees. Note that this does notcreates the files. Note that we
  // don't test the levels at this time.
  EXPECT_EQ(dqcs_pcfg_tee(a, dqcs_loglevel_t::DQCS_LOG_WARN, "warnings"), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_tee(a, dqcs_loglevel_t::DQCS_LOG_TRACE, "trace"), dqcs_return_t::DQCS_SUCCESS);

  // Check that the tee file configurations were added.
  s = dqcs_handle_dump(a);
  EXPECT_STREQ(extract_array_from_dump("tee_files:", s), "tee_files: [ TeeFileConfiguration { filter: Warn, file: \"warnings\" }, TeeFileConfiguration { filter: Trace, file: \"trace\" }]");
  if (s) free(s);

  // Check that we can't do silly things.
  EXPECT_EQ(dqcs_pcfg_tee(a, dqcs_loglevel_t::DQCS_LOG_INVALID, "x"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid level");
  EXPECT_EQ(dqcs_pcfg_tee(a, dqcs_loglevel_t::DQCS_LOG_PASS, "x"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid loglevel filter DQCS_LOG_PASS");

  // Delete the handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test process startup/shutdown timeouts.
TEST(pcfg, timeout) {
  dqcs_handle_t a;

  // Create a fresh config.
  a = dqcs_pcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, "x", NULL);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check the default timeouts.
  EXPECT_EQ(dqcs_pcfg_accept_timeout_get(a), 5.0);
  EXPECT_EQ(dqcs_pcfg_shutdown_timeout_get(a), 5.0);

  // Change the timeouts.
  EXPECT_EQ(dqcs_pcfg_accept_timeout_set(a, 3.0), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_shutdown_timeout_set(a, 16.0), dqcs_return_t::DQCS_SUCCESS);

  // Ensure that the timeouts changed.
  EXPECT_EQ(dqcs_pcfg_accept_timeout_get(a), 3.0);
  EXPECT_EQ(dqcs_pcfg_shutdown_timeout_get(a), 16.0);

  // Make sure that negative timeouts are not a thing.
  EXPECT_EQ(dqcs_pcfg_accept_timeout_set(a, -4.0), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: timeouts cannot be negative");

  EXPECT_EQ(dqcs_pcfg_shutdown_timeout_set(a, -8.0), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: timeouts cannot be negative");

  EXPECT_EQ(dqcs_pcfg_accept_timeout_get(a), 3.0);
  EXPECT_EQ(dqcs_pcfg_shutdown_timeout_get(a), 16.0);

  // Test no timeout using positive infinity.
  EXPECT_EQ(dqcs_pcfg_accept_timeout_set(a, INFINITY), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_shutdown_timeout_set(a, INFINITY), dqcs_return_t::DQCS_SUCCESS);

  // Ensure that the timeouts changed.
  EXPECT_EQ(dqcs_pcfg_accept_timeout_get(a), INFINITY);
  EXPECT_EQ(dqcs_pcfg_shutdown_timeout_get(a), INFINITY);

  // Delete the handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test stderr/stdout modes.
TEST(pcfg, stream_capture_mode) {
  dqcs_handle_t a;

  // Create a fresh config.
  a = dqcs_pcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, "x", NULL);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check the default values.
  EXPECT_EQ(dqcs_pcfg_stdout_mode_get(a), dqcs_loglevel_t::DQCS_LOG_INFO);
  EXPECT_EQ(dqcs_pcfg_stderr_mode_get(a), dqcs_loglevel_t::DQCS_LOG_INFO);

  // Check all values.
  EXPECT_EQ(dqcs_pcfg_stdout_mode_set(a, dqcs_loglevel_t::DQCS_LOG_INVALID), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid level");
  EXPECT_EQ(dqcs_pcfg_stdout_mode_get(a), dqcs_loglevel_t::DQCS_LOG_INFO);

  EXPECT_EQ(dqcs_pcfg_stderr_mode_set(a, dqcs_loglevel_t::DQCS_LOG_OFF), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_stderr_mode_get(a), dqcs_loglevel_t::DQCS_LOG_OFF);

  EXPECT_EQ(dqcs_pcfg_stdout_mode_set(a, dqcs_loglevel_t::DQCS_LOG_FATAL), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_stdout_mode_get(a), dqcs_loglevel_t::DQCS_LOG_FATAL);

  EXPECT_EQ(dqcs_pcfg_stderr_mode_set(a, dqcs_loglevel_t::DQCS_LOG_ERROR), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_stderr_mode_get(a), dqcs_loglevel_t::DQCS_LOG_ERROR);

  EXPECT_EQ(dqcs_pcfg_stdout_mode_set(a, dqcs_loglevel_t::DQCS_LOG_WARN), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_stdout_mode_get(a), dqcs_loglevel_t::DQCS_LOG_WARN);

  EXPECT_EQ(dqcs_pcfg_stderr_mode_set(a, dqcs_loglevel_t::DQCS_LOG_NOTE), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_stderr_mode_get(a), dqcs_loglevel_t::DQCS_LOG_NOTE);

  EXPECT_EQ(dqcs_pcfg_stdout_mode_set(a, dqcs_loglevel_t::DQCS_LOG_INFO), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_stdout_mode_get(a), dqcs_loglevel_t::DQCS_LOG_INFO);

  EXPECT_EQ(dqcs_pcfg_stderr_mode_set(a, dqcs_loglevel_t::DQCS_LOG_DEBUG), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_stderr_mode_get(a), dqcs_loglevel_t::DQCS_LOG_DEBUG);

  EXPECT_EQ(dqcs_pcfg_stdout_mode_set(a, dqcs_loglevel_t::DQCS_LOG_TRACE), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_stdout_mode_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  EXPECT_EQ(dqcs_pcfg_stderr_mode_set(a, dqcs_loglevel_t::DQCS_LOG_PASS), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_stderr_mode_get(a), dqcs_loglevel_t::DQCS_LOG_PASS);

  // Delete the handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test verbosity configuration.
TEST(pcfg, verbosity) {
  dqcs_handle_t a;

  // Create a fresh config.
  a = dqcs_pcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, "x", NULL);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check the default value. Note that this is trace because plugin loglevel
  // is automatically limited to the most verbose log message sink, and usually
  // only the sink level is set.
  EXPECT_EQ(dqcs_pcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  // Check all values.
  EXPECT_EQ(dqcs_pcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_INVALID), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid level");
  EXPECT_EQ(dqcs_pcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  EXPECT_EQ(dqcs_pcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_OFF), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_OFF);

  EXPECT_EQ(dqcs_pcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_FATAL), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_FATAL);

  EXPECT_EQ(dqcs_pcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_ERROR), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_ERROR);

  EXPECT_EQ(dqcs_pcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_WARN), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_WARN);

  EXPECT_EQ(dqcs_pcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_NOTE), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_NOTE);

  EXPECT_EQ(dqcs_pcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_INFO), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_INFO);

  EXPECT_EQ(dqcs_pcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_DEBUG), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_DEBUG);

  EXPECT_EQ(dqcs_pcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_TRACE), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_pcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  EXPECT_EQ(dqcs_pcfg_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_PASS), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid loglevel filter DQCS_LOG_PASS");
  EXPECT_EQ(dqcs_pcfg_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  // Delete the handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

