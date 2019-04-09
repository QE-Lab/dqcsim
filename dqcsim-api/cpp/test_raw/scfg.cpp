#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"
#include "util.h"

using namespace dqcsim;

// Sanity check the simulator configuration API.
TEST(scfg, sanity) {
  // Create handle.
  dqcs_handle_t a = dqcs_scfg_new();
  ASSERT_NE(a, 0) << "Unexpected error: " << dqcs_error_get();

  // Check that the handle is OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_SIM_CONFIG);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Check that the handle is no longer OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_handle_dump(a), nullptr);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(a) + " is invalid");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test the simulator reproduction configuration API.
TEST(scfg, repro) {
  // Create handle.
  dqcs_handle_t a = dqcs_scfg_new();
  ASSERT_NE(a, 0) << "Unexpected error: " << dqcs_error_get();

  // Check the default value.
  EXPECT_EQ(dqcs_scfg_repro_path_style_get(a), dqcs_path_style_t::DQCS_PATH_STYLE_KEEP);

  // Check setting all possible values.
  EXPECT_EQ(dqcs_scfg_repro_path_style_set(a, dqcs_path_style_t::DQCS_PATH_STYLE_RELATIVE), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_repro_path_style_get(a), dqcs_path_style_t::DQCS_PATH_STYLE_RELATIVE);

  EXPECT_EQ(dqcs_scfg_repro_path_style_set(a, dqcs_path_style_t::DQCS_PATH_STYLE_ABSOLUTE), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_repro_path_style_get(a), dqcs_path_style_t::DQCS_PATH_STYLE_ABSOLUTE);

  EXPECT_EQ(dqcs_scfg_repro_path_style_set(a, dqcs_path_style_t::DQCS_PATH_STYLE_KEEP), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_repro_path_style_get(a), dqcs_path_style_t::DQCS_PATH_STYLE_KEEP);

  EXPECT_EQ(dqcs_scfg_repro_path_style_set(a, dqcs_path_style_t::DQCS_PATH_STYLE_INVALID), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid path style");
  EXPECT_EQ(dqcs_scfg_repro_path_style_get(a), dqcs_path_style_t::DQCS_PATH_STYLE_KEEP);

  // Check disabling reproduction.
  EXPECT_EQ(dqcs_scfg_repro_disable(a), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_repro_path_style_get(a), dqcs_path_style_t::DQCS_PATH_STYLE_INVALID);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: the reproduction system is disabled for this configuration");

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test the source verbosity configuration API.
TEST(scfg, verbosity) {
  // Create handle.
  dqcs_handle_t a = dqcs_scfg_new();
  ASSERT_NE(a, 0) << "Unexpected error: " << dqcs_error_get();

  // Check the default value. Note that this is trace because source loglevel
  // is automatically limited to the most verbose log message sink, and usually
  // only the sink level is set.
  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  // Check all values.
  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_INVALID), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid level");
  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_OFF), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_OFF);

  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_FATAL), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_FATAL);

  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_ERROR), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_ERROR);

  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_WARN), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_WARN);

  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_NOTE), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_NOTE);

  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_INFO), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_INFO);

  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_DEBUG), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_DEBUG);

  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_TRACE), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_PASS), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid loglevel filter DQCS_LOG_PASS");
  EXPECT_EQ(dqcs_scfg_dqcsim_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test the stderr verbosity configuration API.
TEST(scfg, stderr) {
  // Create handle.
  dqcs_handle_t a = dqcs_scfg_new();
  ASSERT_NE(a, 0) << "Unexpected error: " << dqcs_error_get();

  // Check the default value. Note that this is trace because source loglevel
  // is automatically limited to the most verbose log message sink, and usually
  // only the sink level is set.
  EXPECT_EQ(dqcs_scfg_stderr_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_INFO);

  // Check all values.
  EXPECT_EQ(dqcs_scfg_stderr_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_INVALID), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid level");
  EXPECT_EQ(dqcs_scfg_stderr_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_INFO);

  EXPECT_EQ(dqcs_scfg_stderr_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_OFF), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_stderr_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_OFF);

  EXPECT_EQ(dqcs_scfg_stderr_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_FATAL), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_stderr_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_FATAL);

  EXPECT_EQ(dqcs_scfg_stderr_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_ERROR), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_stderr_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_ERROR);

  EXPECT_EQ(dqcs_scfg_stderr_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_WARN), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_stderr_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_WARN);

  EXPECT_EQ(dqcs_scfg_stderr_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_NOTE), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_stderr_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_NOTE);

  EXPECT_EQ(dqcs_scfg_stderr_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_INFO), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_stderr_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_INFO);

  EXPECT_EQ(dqcs_scfg_stderr_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_DEBUG), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_stderr_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_DEBUG);

  EXPECT_EQ(dqcs_scfg_stderr_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_TRACE), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_stderr_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  EXPECT_EQ(dqcs_scfg_stderr_verbosity_set(a, dqcs_loglevel_t::DQCS_LOG_PASS), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid loglevel filter DQCS_LOG_PASS");
  EXPECT_EQ(dqcs_scfg_stderr_verbosity_get(a), dqcs_loglevel_t::DQCS_LOG_TRACE);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test tee files.
TEST(scfg, tee) {
  dqcs_handle_t a;
  char *s;

  // Create a fresh config.
  a = dqcs_scfg_new();
  ASSERT_NE(a, 0) << "Unexpected error: " << dqcs_error_get();

  // Check that there are initially no tee files.
  s = dqcs_handle_dump(a);
  EXPECT_STREQ(extract_array_from_dump("tee_files:", s), "tee_files: []");
  if (s) free(s);

  // Add some tees. Note that this does notcreates the files. Note that we
  // don't test the levels at this time.
  EXPECT_EQ(dqcs_scfg_tee(a, dqcs_loglevel_t::DQCS_LOG_WARN, "warnings"), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_scfg_tee(a, dqcs_loglevel_t::DQCS_LOG_TRACE, "trace"), dqcs_return_t::DQCS_SUCCESS);

  // Check that the tee file configurations were added.
  s = dqcs_handle_dump(a);
  EXPECT_STREQ(extract_array_from_dump("tee_files:", s), "tee_files: [ TeeFileConfiguration { filter: Warn, file: \"warnings\" }, TeeFileConfiguration { filter: Trace, file: \"trace\" }]");
  if (s) free(s);

  // Check that we can't do silly things.
  EXPECT_EQ(dqcs_scfg_tee(a, dqcs_loglevel_t::DQCS_LOG_INVALID, "x"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid level");
  EXPECT_EQ(dqcs_scfg_tee(a, dqcs_loglevel_t::DQCS_LOG_PASS, "x"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid loglevel filter DQCS_LOG_PASS");

  // Delete the handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Test seed.
TEST(scfg, seed) {
  dqcs_handle_t a;
  long long seed;

  // Check that the initial seeds are random.
  // NOTE: this will on average fail every 2^64th try. Seems astronomical
  // enough to me.
  a = dqcs_scfg_new();
  ASSERT_NE(a, 0) << "Unexpected error: " << dqcs_error_get();
  seed = dqcs_scfg_seed_get(a);
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);
  a = dqcs_scfg_new();
  ASSERT_NE(a, 0) << "Unexpected error: " << dqcs_error_get();
  ASSERT_NE(dqcs_scfg_seed_get(a), seed);

  // Check the setter.
  EXPECT_EQ(dqcs_scfg_seed_set(a, 0xDEADBEEF), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_scfg_seed_get(a), 0xDEADBEEF);

  // Delete the handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

void log_cb(
  void *user,
  const char *message,
  const char *logger,
  dqcs_loglevel_t level,
  const char *module,
  const char *file,
  uint32_t line,
  uint64_t time_s,
  uint32_t time_ns,
  uint32_t pid,
  uint64_t tid
) {
}

void free_cb(void *user) {
  (*((int*)user))++;
}

// Test log callback. We only test the user_free function here, since we never
// actually start the simulation in this suite.
TEST(scfg, log_callback) {
  dqcs_handle_t a;
  int user = 0;
  int user2 = 0;

  // Create a fresh config.
  a = dqcs_scfg_new();
  ASSERT_NE(a, 0) << "Unexpected error: " << dqcs_error_get();

  // Proper setters.
  EXPECT_EQ(dqcs_scfg_log_callback(a, dqcs_loglevel_t::DQCS_LOG_WARN, log_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(user, 0);
  EXPECT_EQ(dqcs_scfg_log_callback(a, dqcs_loglevel_t::DQCS_LOG_WARN, log_cb, NULL, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(user, 1);
  EXPECT_EQ(dqcs_scfg_log_callback(a, dqcs_loglevel_t::DQCS_LOG_WARN, log_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(user, 1);
  EXPECT_EQ(dqcs_scfg_log_callback(a, dqcs_loglevel_t::DQCS_LOG_WARN, log_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(user, 2);

  // Improper setters.
  EXPECT_EQ(dqcs_scfg_log_callback(a, dqcs_loglevel_t::DQCS_LOG_INVALID, log_cb, free_cb, &user2), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid level");
  EXPECT_EQ(user, 2);
  EXPECT_EQ(user2, 1);
  EXPECT_EQ(dqcs_scfg_log_callback(a, dqcs_loglevel_t::DQCS_LOG_PASS, log_cb, NULL, &user2), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: invalid loglevel filter DQCS_LOG_PASS");
  EXPECT_EQ(user, 2);
  EXPECT_EQ(user2, 1);

  // Callback deleter.
  EXPECT_EQ(dqcs_scfg_log_callback(a, dqcs_loglevel_t::DQCS_LOG_INFO, NULL, free_cb, &user2), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(user, 3);
  EXPECT_EQ(user2, 2);

  // Set a callback again to make sure free_cb() is called when the handle is
  // deleted.
  EXPECT_EQ(dqcs_scfg_log_callback(a, dqcs_loglevel_t::DQCS_LOG_WARN, log_cb, free_cb, &user), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(user, 3);

  // Delete the handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(user, 4);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}
