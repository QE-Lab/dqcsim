
#include <queue>
#include <string>
#include "gtest/gtest.h"
#define DQCSIM_NAMESPACE dqcsim::raw::
#include "util.h"

// Test the cdqcsim include; this should place everything in the ::dqcsim::raw
// namespace.
#include <cdqcsim>

typedef struct {
  std::string message;
  std::string logger;
  dqcsim::raw::dqcs_loglevel_t level;
} log_msg_t;

typedef struct {
  std::queue<log_msg_t> msgs;
  std::queue<std::string> aborts;
} data_t;

#define EXPECT_MSG(data, m, lo, le) do { \
  EXPECT_EQ(data.msgs.empty(), false); \
  EXPECT_EQ(data.msgs.front().message, m); \
  EXPECT_EQ(data.msgs.front().logger, lo); \
  EXPECT_EQ(data.msgs.front().level, dqcsim::raw::dqcs_loglevel_t::le); \
  data.msgs.pop(); \
  } while (0)

#define EXPECT_ABORT_FAILED(data, m) do { \
  EXPECT_EQ(data.aborts.empty(), false); \
  EXPECT_EQ(data.aborts.front(), m); \
  data.aborts.pop(); \
  } while (0)

#define EXPECT_NO_MORE_MSGS(data) do { \
  EXPECT_EQ(data.msgs.empty(), true); \
  EXPECT_EQ(data.aborts.empty(), true); \
  } while (0)

dqcsim::raw::dqcs_return_t initialize_cb(void *user_data, dqcsim::raw::dqcs_plugin_state_t state, dqcsim::raw::dqcs_handle_t init_cmds) {
  dqcsim::raw::dqcs_log_info("!@#$ Initialize: %s", (const char*)user_data);

  dqcsim::raw::dqcs_log_trace("!@#$ Trace");
  dqcsim::raw::dqcs_log_debug("!@#$ Debug");
  dqcsim::raw::dqcs_log_info("!@#$ Info");
  dqcsim::raw::dqcs_log_note("!@#$ Note");
  dqcsim::raw::dqcs_log_warn("!@#$ Warn");
  dqcsim::raw::dqcs_log_error("!@#$ Error");
  dqcsim::raw::dqcs_log_fatal("!@#$ Fatal");

  return dqcsim::raw::dqcs_return_t::DQCS_SUCCESS;
}

dqcsim::raw::dqcs_return_t initialize_cb_simple(void *user_data, dqcsim::raw::dqcs_plugin_state_t state, dqcsim::raw::dqcs_handle_t init_cmds) {
  dqcsim::raw::dqcs_log_info("!@#$ Initialize: %s", (const char*)user_data);
  return dqcsim::raw::dqcs_return_t::DQCS_SUCCESS;
}

dqcsim::raw::dqcs_return_t initialize_cb_fail(void *user_data, dqcsim::raw::dqcs_plugin_state_t state, dqcsim::raw::dqcs_handle_t init_cmds) {
  dqcsim::raw::dqcs_log_info("!@#$ Initialize: %s", (const char*)user_data);
  std::string s = std::string("Here's an error from ") + (const char*)user_data;
  dqcsim::raw::dqcs_error_set(s.c_str());
  return dqcsim::raw::dqcs_return_t::DQCS_FAILURE;
}

dqcsim::raw::dqcs_return_t drop_cb(void *user_data, dqcsim::raw::dqcs_plugin_state_t state) {
  dqcsim::raw::dqcs_log_info("!@#$ Drop: %s", (const char*)user_data);
  return dqcsim::raw::dqcs_return_t::DQCS_SUCCESS;
}

dqcsim::raw::dqcs_return_t drop_cb_fail(void *user_data, dqcsim::raw::dqcs_plugin_state_t state) {
  dqcsim::raw::dqcs_log_info("!@#$ Drop: %s", (const char*)user_data);
  std::string s = std::string("Here's an error from ") + (const char*)user_data;
  dqcsim::raw::dqcs_error_set(s.c_str());
  return dqcsim::raw::dqcs_return_t::DQCS_FAILURE;
}

void log_cb(
  void *user,
  const char *message,
  const char *logger,
  dqcsim::raw::dqcs_loglevel_t level,
  const char *module,
  const char *file,
  uint32_t line,
  uint64_t time_s,
  uint32_t time_ns,
  uint32_t pid,
  uint64_t tid
) {
  data_t *data = (data_t*)user;
  if (!strncmp(message, "!@#$ ", 5)) {
    log_msg_t msg = {
      std::string(message + 5),
      std::string(logger),
      level
    };
    data->msgs.push(msg);
  } else if (strstr(message, "failed to abort:")) {
    data->aborts.push(std::string(message));
  }
}

// Test normal flow.
TEST(cpp_header, test) {
  data_t data;
  SIM_HEADER;
  dqcsim::raw::dqcs_scfg_log_callback(sim, dqcsim::raw::dqcs_loglevel_t::DQCS_LOG_TRACE, log_cb, NULL, &data);
  dqcsim::raw::dqcs_pdef_set_initialize_cb(front, initialize_cb, NULL, (void*)"front");
  dqcsim::raw::dqcs_pdef_set_drop_cb(front, drop_cb, NULL, (void*)"front");
  dqcsim::raw::dqcs_pdef_set_initialize_cb(oper, initialize_cb, NULL, (void*)"oper");
  dqcsim::raw::dqcs_pdef_set_drop_cb(oper, drop_cb, NULL, (void*)"oper");
  dqcsim::raw::dqcs_pdef_set_initialize_cb(back, initialize_cb, NULL, (void*)"back");
  dqcsim::raw::dqcs_pdef_set_drop_cb(back, drop_cb, NULL, (void*)"back");
  SIM_CONSTRUCT;
  SIM_FOOTER;

  EXPECT_MSG(data, "Initialize: back", "back", DQCS_LOG_INFO);
  EXPECT_MSG(data, "Trace", "back", DQCS_LOG_TRACE);
  EXPECT_MSG(data, "Debug", "back", DQCS_LOG_DEBUG);
  EXPECT_MSG(data, "Info", "back", DQCS_LOG_INFO);
  EXPECT_MSG(data, "Note", "back", DQCS_LOG_NOTE);
  EXPECT_MSG(data, "Warn", "back", DQCS_LOG_WARN);
  EXPECT_MSG(data, "Error", "back", DQCS_LOG_ERROR);
  EXPECT_MSG(data, "Fatal", "back", DQCS_LOG_FATAL);

  EXPECT_MSG(data, "Initialize: oper", "op1", DQCS_LOG_INFO);
  EXPECT_MSG(data, "Trace", "op1", DQCS_LOG_TRACE);
  EXPECT_MSG(data, "Debug", "op1", DQCS_LOG_DEBUG);
  EXPECT_MSG(data, "Info", "op1", DQCS_LOG_INFO);
  EXPECT_MSG(data, "Note", "op1", DQCS_LOG_NOTE);
  EXPECT_MSG(data, "Warn", "op1", DQCS_LOG_WARN);
  EXPECT_MSG(data, "Error", "op1", DQCS_LOG_ERROR);
  EXPECT_MSG(data, "Fatal", "op1", DQCS_LOG_FATAL);

  EXPECT_MSG(data, "Initialize: front", "front", DQCS_LOG_INFO);
  EXPECT_MSG(data, "Trace", "front", DQCS_LOG_TRACE);
  EXPECT_MSG(data, "Debug", "front", DQCS_LOG_DEBUG);
  EXPECT_MSG(data, "Info", "front", DQCS_LOG_INFO);
  EXPECT_MSG(data, "Note", "front", DQCS_LOG_NOTE);
  EXPECT_MSG(data, "Warn", "front", DQCS_LOG_WARN);
  EXPECT_MSG(data, "Error", "front", DQCS_LOG_ERROR);
  EXPECT_MSG(data, "Fatal", "front", DQCS_LOG_FATAL);

  EXPECT_MSG(data, "Drop: front", "front", DQCS_LOG_INFO);
  EXPECT_MSG(data, "Drop: oper", "op1", DQCS_LOG_INFO);
  EXPECT_MSG(data, "Drop: back", "back", DQCS_LOG_INFO);

  EXPECT_NO_MORE_MSGS(data);
}
