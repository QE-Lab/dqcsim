#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"
#include "util.h"

using namespace dqcsim;

typedef struct {
  bool start_in_thread;
  bool omit_join;
  dqcs_plugin_type_t plugin_type;
  dqcs_handle_type_t join_handle_type;
  std::string join_handle_dump;
  dqcs_return_t retval;
  std::string errval;
} cfg_t;

void plugin_run_cb(void *user_data, const char *simulator) {
  cfg_t *cfg = (cfg_t*)user_data;
  dqcs_handle_t plugin = dqcs_pdef_new(cfg->plugin_type, "a", "b", "c");
  if (cfg->start_in_thread) {
    dqcs_handle_t join = dqcs_plugin_start(plugin, simulator);
    cfg->join_handle_type = dqcs_handle_type(join);
    cfg->join_handle_dump = std::string(dqcs_handle_dump(join));
    if (!cfg->omit_join) {
      cfg->retval = dqcs_plugin_wait(join);
    }
  } else {
    cfg->retval = dqcs_plugin_run(plugin, simulator);
  }
  const char *s = dqcs_error_get();
  cfg->errval = std::string(s ? s : "<NONE>");
}

// Test starting the frontend manually.
TEST(sim_special_start, run_front) {
  cfg_t cfg = {
    false,
    false,
    dqcs_plugin_type_t::DQCS_PTYPE_FRONT,
    dqcs_handle_type_t::DQCS_HTYPE_INVALID,
    std::string("?"),
    dqcs_return_t::DQCS_FAILURE,
    std::string("?"),
  };

  dqcs_handle_t sim, plugin;
  sim = dqcs_scfg_new();
  ASSERT_NE(sim, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_repro_disable(sim), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  plugin = dqcs_tcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, plugin_run_cb, NULL, &cfg);
  ASSERT_NE(plugin, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_push_plugin(sim, plugin), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  plugin = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, "a", "b", "c");
  ASSERT_NE(plugin, 0u) << "Unexpected error: " << dqcs_error_get();
  plugin = dqcs_tcfg_new(plugin, NULL);
  ASSERT_NE(plugin, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_push_plugin(sim, plugin), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  sim = dqcs_sim_new(sim);
  ASSERT_NE(sim, 0u) << "Unexpected error: " << dqcs_error_get();

  SIM_FOOTER;

  EXPECT_EQ(cfg.retval, dqcs_return_t::DQCS_SUCCESS);
}

// Test starting the frontend manually in a worker thread.
TEST(sim_special_start, start_wait_front) {
  cfg_t cfg = {
    true,
    false,
    dqcs_plugin_type_t::DQCS_PTYPE_FRONT,
    dqcs_handle_type_t::DQCS_HTYPE_INVALID,
    std::string("?"),
    dqcs_return_t::DQCS_FAILURE,
    std::string("?"),
  };

  dqcs_handle_t sim, plugin;
  sim = dqcs_scfg_new();
  ASSERT_NE(sim, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_repro_disable(sim), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  plugin = dqcs_tcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, plugin_run_cb, NULL, &cfg);
  ASSERT_NE(plugin, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_push_plugin(sim, plugin), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  plugin = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, "a", "b", "c");
  ASSERT_NE(plugin, 0u) << "Unexpected error: " << dqcs_error_get();
  plugin = dqcs_tcfg_new(plugin, NULL);
  ASSERT_NE(plugin, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_push_plugin(sim, plugin), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  sim = dqcs_sim_new(sim);
  ASSERT_NE(sim, 0u) << "Unexpected error: " << dqcs_error_get();

  SIM_FOOTER;

  EXPECT_EQ(cfg.join_handle_type, dqcs_handle_type_t::DQCS_HTYPE_PLUGIN_JOIN);
  EXPECT_EQ(cfg.join_handle_dump, "PluginJoinHandle(\n    JoinHandle { .. }\n)");
  EXPECT_EQ(cfg.retval, dqcs_return_t::DQCS_SUCCESS);
}

// Test starting the frontend manually in a worker thread, without waiting for
// the worker thread to complete.
TEST(sim_special_start, start_front) {
  cfg_t cfg = {
    true,
    true,
    dqcs_plugin_type_t::DQCS_PTYPE_FRONT,
    dqcs_handle_type_t::DQCS_HTYPE_INVALID,
    std::string("?"),
    dqcs_return_t::DQCS_FAILURE,
    std::string("?"),
  };

  dqcs_handle_t sim, plugin;
  sim = dqcs_scfg_new();
  ASSERT_NE(sim, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_repro_disable(sim), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  plugin = dqcs_tcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, plugin_run_cb, NULL, &cfg);
  ASSERT_NE(plugin, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_push_plugin(sim, plugin), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  plugin = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, "a", "b", "c");
  ASSERT_NE(plugin, 0u) << "Unexpected error: " << dqcs_error_get();
  plugin = dqcs_tcfg_new(plugin, NULL);
  ASSERT_NE(plugin, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_push_plugin(sim, plugin), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  sim = dqcs_sim_new(sim);
  ASSERT_NE(sim, 0u) << "Unexpected error: " << dqcs_error_get();

  SIM_FOOTER;

  EXPECT_EQ(cfg.join_handle_type, dqcs_handle_type_t::DQCS_HTYPE_PLUGIN_JOIN);
  EXPECT_EQ(cfg.join_handle_dump, "PluginJoinHandle(\n    JoinHandle { .. }\n)");
}

// Test starting the wrong type of plugin.
TEST(sim_special_start, run_wrong) {
  cfg_t cfg = {
    false,
    false,
    dqcs_plugin_type_t::DQCS_PTYPE_OPER,
    dqcs_handle_type_t::DQCS_HTYPE_INVALID,
    std::string("?"),
    dqcs_return_t::DQCS_FAILURE,
    std::string("?"),
  };

  dqcs_handle_t sim, plugin;
  sim = dqcs_scfg_new();
  ASSERT_NE(sim, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_repro_disable(sim), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  plugin = dqcs_tcfg_new_raw(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, NULL, plugin_run_cb, NULL, &cfg);
  ASSERT_NE(plugin, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_push_plugin(sim, plugin), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  plugin = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, "a", "b", "c");
  ASSERT_NE(plugin, 0u) << "Unexpected error: " << dqcs_error_get();
  plugin = dqcs_tcfg_new(plugin, NULL);
  ASSERT_NE(plugin, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_push_plugin(sim, plugin), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  sim = dqcs_sim_new(sim);
  ASSERT_EQ(sim, 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid operation: host is expecting a plugin of type Frontend, but we're a plugin of type Operator");
}
