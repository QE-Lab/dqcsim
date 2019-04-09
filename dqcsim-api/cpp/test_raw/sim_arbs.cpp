#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"
#include "util.h"

using namespace dqcsim;

#define SIM_HEADER \
  dqcs_handle_t front, oper, back, sim; \
  sim = dqcs_scfg_new(); \
  ASSERT_NE(sim, 0u) << "Unexpected error: " << dqcs_error_get(); \
  front = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c"); \
  ASSERT_NE(front, 0u) << "Unexpected error: " << dqcs_error_get(); \
  oper = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_OPER, "a", "b", "c"); \
  ASSERT_NE(oper, 0u) << "Unexpected error: " << dqcs_error_get(); \
  back = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, "a", "b", "c"); \
  ASSERT_NE(back, 0u) << "Unexpected error: " << dqcs_error_get()

#define SIM_CONSTRUCT \
  front = dqcs_tcfg_new(front, NULL); \
  ASSERT_NE(front, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_scfg_push_plugin(sim, front), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get(); \
  oper = dqcs_tcfg_new(oper, NULL); \
  ASSERT_NE(oper, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_scfg_push_plugin(sim, oper), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get(); \
  back = dqcs_tcfg_new(back, NULL); \
  ASSERT_NE(back, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_scfg_push_plugin(sim, back), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get(); \
  sim = dqcs_sim_new(sim); \
  ASSERT_NE(sim, 0u) << "Unexpected error: " << dqcs_error_get()

#define SIM_FOOTER \
  EXPECT_EQ(dqcs_handle_delete(sim), dqcs_return_t::DQCS_SUCCESS); \
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get()

#define MAKE_ARB(arb, json, str) do { \
  arb = dqcs_arb_new(); \
  ASSERT_NE(arb, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_arb_json_set(arb, json), dqcs_return_t::DQCS_SUCCESS); \
  ASSERT_EQ(dqcs_arb_push_str(arb, str), dqcs_return_t::DQCS_SUCCESS); \
  } while (0)

#define CHECK_ARB(arb, json, ...) do { \
  char *s; \
  EXPECT_NE(arb, 0u) << "Unexpected error: " << dqcs_error_get(); \
  EXPECT_STREQ(s = dqcs_arb_json_get(arb), json); if (s) free(s); \
  const char *exp[] = {__VA_ARGS__}; \
  int i = 0; \
  for (const char *e : exp) { \
    EXPECT_STREQ(s = dqcs_arb_get_str(arb, i), e); if (s) free(s); \
    i++; \
  } \
  EXPECT_EQ(dqcs_arb_len(arb), i); \
  EXPECT_EQ(dqcs_handle_delete(arb), dqcs_return_t::DQCS_SUCCESS); \
  } while (0)

#define CHECK_EMPTY_ARB(arb) do { \
  char *s; \
  EXPECT_NE(arb, 0u) << "Unexpected error: " << dqcs_error_get(); \
  EXPECT_STREQ(s = dqcs_arb_json_get(arb), "{}"); if (s) free(s); \
  EXPECT_EQ(dqcs_arb_len(arb), 0); \
  EXPECT_EQ(dqcs_handle_delete(arb), dqcs_return_t::DQCS_SUCCESS); \
  } while (0)

#define MAKE_CMD(cmd, iface, oper, json, str) do { \
  cmd = dqcs_cmd_new(iface, oper); \
  ASSERT_NE(cmd, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_arb_json_set(cmd, json), dqcs_return_t::DQCS_SUCCESS); \
  ASSERT_EQ(dqcs_arb_push_str(cmd, str), dqcs_return_t::DQCS_SUCCESS); \
  } while (0)

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

