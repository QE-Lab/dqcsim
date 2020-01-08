#include <dqcsim.h>
#include "gtest/gtest.h"
#include "util.h"

// Sanity-check.
TEST(sim_arbs, sanity) {
  SIM_HEADER;
  SIM_CONSTRUCT;
  SIM_FOOTER;
}

// Test the default host arb behavior.
TEST(sim_arbs, host_defaults) {
  SIM_HEADER;
  SIM_CONSTRUCT;

  dqcs_handle_t arb;

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "front", arb);
  CHECK_EMPTY_ARB(arb);

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "op1", arb);
  CHECK_EMPTY_ARB(arb);

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "back", arb);
  CHECK_EMPTY_ARB(arb);

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  ASSERT_EQ(dqcs_sim_arb(sim, "derp", arb), 0u);
  EXPECT_EQ(dqcs_handle_delete(arb), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: plugin derp not found");

  SIM_FOOTER;
}

dqcs_handle_t test_arb_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t cmd) {
  (void)state;
  char *s;
  std::string json = "";
  json += "{\"x\":[\"";

  s = dqcs_cmd_iface_get(cmd);
  if (!s) return 0;
  if (strcmp(s, "return_error") == 0) {
    s = dqcs_cmd_oper_get(cmd);
    if (!s) return 0;
    dqcs_error_set(s);
    return 0;
  }
  json += s;
  free(s);

  json += "\",\"";

  s = dqcs_cmd_oper_get(cmd);
  if (!s) return 0;
  json += s;
  free(s);

  json += "\",";

  s = dqcs_arb_json_get(cmd);
  if (!s) return 0;
  json += s;
  free(s);

  json += "]}";

  if (dqcs_arb_json_set(cmd, json.c_str()) == dqcs_return_t::DQCS_FAILURE) return 0;
  if (dqcs_arb_push_str(cmd, (const char*)user_data) == dqcs_return_t::DQCS_FAILURE) return 0;

  return cmd;
}

// Test non-default host arb behavior.
TEST(sim_arbs, host_nondefault) {
  SIM_HEADER;

  dqcs_pdef_set_host_arb_cb(front, test_arb_cb, NULL, (void*)"front");
  dqcs_pdef_set_host_arb_cb(oper, test_arb_cb, NULL, (void*)"oper");
  dqcs_pdef_set_host_arb_cb(back, test_arb_cb, NULL, (void*)"back");

  SIM_CONSTRUCT;

  dqcs_handle_t arb;

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "front", arb);
  CHECK_ARB(arb, "{\"x\":[\"a\",\"b\",{\"a\":\"b\"}]}", "test", "front");

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "op1", arb);
  CHECK_ARB(arb, "{\"x\":[\"a\",\"b\",{\"a\":\"b\"}]}", "test", "oper");

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "back", arb);
  CHECK_ARB(arb, "{\"x\":[\"a\",\"b\",{\"a\":\"b\"}]}", "test", "back");

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  ASSERT_EQ(dqcs_sim_arb(sim, "derp", arb), 0u);
  EXPECT_EQ(dqcs_handle_delete(arb), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: plugin derp not found");

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  ASSERT_EQ(dqcs_sim_arb_idx(sim, -4, arb), 0u);
  EXPECT_EQ(dqcs_handle_delete(arb), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index -4 out of range");

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb_idx(sim, -3, arb);
  CHECK_ARB(arb, "{\"x\":[\"a\",\"b\",{\"a\":\"b\"}]}", "test", "front");

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb_idx(sim, -2, arb);
  CHECK_ARB(arb, "{\"x\":[\"a\",\"b\",{\"a\":\"b\"}]}", "test", "oper");

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb_idx(sim, -1, arb);
  CHECK_ARB(arb, "{\"x\":[\"a\",\"b\",{\"a\":\"b\"}]}", "test", "back");

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb_idx(sim, 0, arb);
  CHECK_ARB(arb, "{\"x\":[\"a\",\"b\",{\"a\":\"b\"}]}", "test", "front");

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb_idx(sim, 1, arb);
  CHECK_ARB(arb, "{\"x\":[\"a\",\"b\",{\"a\":\"b\"}]}", "test", "oper");

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb_idx(sim, 2, arb);
  CHECK_ARB(arb, "{\"x\":[\"a\",\"b\",{\"a\":\"b\"}]}", "test", "back");

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  ASSERT_EQ(dqcs_sim_arb_idx(sim, 3, arb), 0u);
  EXPECT_EQ(dqcs_handle_delete(arb), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index 3 out of range");

  MAKE_CMD(arb, "return_error", "hello", "{}", "");
  ASSERT_EQ(dqcs_sim_arb_idx(sim, 0, arb), 0u);
  EXPECT_EQ(dqcs_handle_delete(arb), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_STREQ(dqcs_error_get(), "hello");

  SIM_FOOTER;
}

dqcs_handle_t fwd_arb_downstream_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t cmd) {
  (void)user_data;
  return dqcs_plugin_arb(state, cmd);
}

// Test default upstream arb behavior.
TEST(sim_arbs, upstream_default_back) {
  SIM_HEADER;

  dqcs_pdef_set_host_arb_cb(front, fwd_arb_downstream_cb, NULL, NULL);
  dqcs_pdef_set_host_arb_cb(oper, fwd_arb_downstream_cb, NULL, NULL);

  SIM_CONSTRUCT;

  dqcs_handle_t arb;

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "front", arb);
  CHECK_EMPTY_ARB(arb);

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "op1", arb);
  CHECK_EMPTY_ARB(arb);

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "back", arb);
  CHECK_EMPTY_ARB(arb);

  MAKE_CMD(arb, "return_error", "hello", "{}", "");
  arb = dqcs_sim_arb(sim, "front", arb);
  CHECK_EMPTY_ARB(arb);

  MAKE_CMD(arb, "return_error", "hello", "{}", "");
  arb = dqcs_sim_arb(sim, "op1", arb);
  CHECK_EMPTY_ARB(arb);

  MAKE_CMD(arb, "return_error", "hello", "{}", "");
  arb = dqcs_sim_arb(sim, "back", arb);
  CHECK_EMPTY_ARB(arb);

  SIM_FOOTER;
}

// Test default upstream arb behavior.
TEST(sim_arbs, upstream_default_oper) {
  SIM_HEADER;

  dqcs_pdef_set_host_arb_cb(front, fwd_arb_downstream_cb, NULL, NULL);
  dqcs_pdef_set_upstream_arb_cb(back, test_arb_cb, NULL, (void*)"back");

  SIM_CONSTRUCT;

  dqcs_handle_t arb;

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "front", arb);
  CHECK_ARB(arb, "{\"x\":[\"a\",\"b\",{\"a\":\"b\"}]}", "test", "back");

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "op1", arb);
  CHECK_EMPTY_ARB(arb);

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "back", arb);
  CHECK_EMPTY_ARB(arb);

  MAKE_CMD(arb, "return_error", "hello", "{}", "");
  ASSERT_EQ(dqcs_sim_arb(sim, "front", arb), 0u);
  EXPECT_EQ(dqcs_handle_delete(arb), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_STREQ(dqcs_error_get(), "hello");

  MAKE_CMD(arb, "return_error", "hello", "{}", "");
  arb = dqcs_sim_arb(sim, "op1", arb);
  CHECK_EMPTY_ARB(arb);

  MAKE_CMD(arb, "return_error", "hello", "{}", "");
  arb = dqcs_sim_arb(sim, "back", arb);
  CHECK_EMPTY_ARB(arb);

  SIM_FOOTER;
}

// Test non-default upstream arb behavior.
TEST(sim_arbs, upstream_nondefault) {
  SIM_HEADER;

  dqcs_pdef_set_host_arb_cb(front, fwd_arb_downstream_cb, NULL, NULL);
  dqcs_pdef_set_upstream_arb_cb(oper, test_arb_cb, NULL, (void*)"oper");

  dqcs_pdef_set_host_arb_cb(oper, fwd_arb_downstream_cb, NULL, NULL);
  dqcs_pdef_set_upstream_arb_cb(back, test_arb_cb, NULL, (void*)"back");

  SIM_CONSTRUCT;

  dqcs_handle_t arb;

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "front", arb);
  CHECK_ARB(arb, "{\"x\":[\"a\",\"b\",{\"a\":\"b\"}]}", "test", "oper");

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "op1", arb);
  CHECK_ARB(arb, "{\"x\":[\"a\",\"b\",{\"a\":\"b\"}]}", "test", "back");

  MAKE_CMD(arb, "a", "b", "{\"a\": \"b\"}", "test");
  arb = dqcs_sim_arb(sim, "back", arb);
  CHECK_EMPTY_ARB(arb);

  MAKE_CMD(arb, "return_error", "hello", "{}", "");
  ASSERT_EQ(dqcs_sim_arb(sim, "front", arb), 0u);
  EXPECT_EQ(dqcs_handle_delete(arb), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_STREQ(dqcs_error_get(), "hello");

  MAKE_CMD(arb, "return_error", "hello", "{}", "");
  ASSERT_EQ(dqcs_sim_arb(sim, "op1", arb), 0u);
  EXPECT_EQ(dqcs_handle_delete(arb), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_STREQ(dqcs_error_get(), "hello");

  MAKE_CMD(arb, "return_error", "hello", "{}", "");
  arb = dqcs_sim_arb(sim, "back", arb);
  CHECK_EMPTY_ARB(arb);

  SIM_FOOTER;
}

