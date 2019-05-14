#pragma once

/**
 * Extracts an array going by the name `marker` from a handle dump such that
 * modifications to the structure around this array don't affect this
 * function's output.
 */
inline char *extract_array_from_dump(const char *marker, char *dump) {
  int marker_len = strlen(marker);
  while (*dump) {
    if (!memcmp(dump, marker, marker_len)) {
      break;
    }
    dump++;
  }
  if (!*dump) return NULL;
  char *start = dump;
  char *out = dump;
  int level = 0;
  int after_newline = 0;
  while (*dump) {
    if (*dump == '[') {
      level++;
    } else if (*dump == ']') {
      level--;
      if (!level) {
        out[0] = ']';
        out[1] = 0;
        return start;
      }
    }
    if (after_newline) {
      if (*dump == ' ') {
        dump++;
        continue;
      } else {
        *out++ = ' ';
        after_newline = 0;
      }
    }
    if (*dump == '\n') {
      after_newline = 1;
      dump++;
      continue;
    }
    *out++ = *dump++;
  }
  return NULL;
}

#define SIM_HEADER \
  dqcs_handle_t front, oper, back, sim; \
  sim = dqcs_scfg_new(); \
  ASSERT_NE(sim, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_scfg_repro_disable(sim), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get(); \
  /*ASSERT_EQ(dqcs_scfg_stderr_verbosity_set(sim, dqcs_loglevel_t::DQCS_LOG_TRACE), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();*/ \
  front = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c"); \
  ASSERT_NE(front, 0u) << "Unexpected error: " << dqcs_error_get(); \
  oper = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_OPER, "a", "b", "c"); \
  ASSERT_NE(oper, 0u) << "Unexpected error: " << dqcs_error_get(); \
  back = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, "a", "b", "c"); \
  ASSERT_NE(back, 0u) << "Unexpected error: " << dqcs_error_get()

#define SIM_CONSTRUCT_ \
  front = dqcs_tcfg_new(front, NULL); \
  ASSERT_NE(front, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_scfg_push_plugin(sim, front), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get(); \
  oper = dqcs_tcfg_new(oper, NULL); \
  ASSERT_NE(oper, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_scfg_push_plugin(sim, oper), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get(); \
  back = dqcs_tcfg_new(back, NULL); \
  ASSERT_NE(back, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_scfg_push_plugin(sim, back), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get(); \
  sim = dqcs_sim_new(sim) \

#define SIM_CONSTRUCT \
  SIM_CONSTRUCT_; \
  ASSERT_NE(sim, 0u) << "Unexpected error: " << dqcs_error_get()

#define SIM_FOOTER \
  EXPECT_EQ(dqcs_handle_delete(sim), dqcs_return_t::DQCS_SUCCESS); \
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get()

#define SIM_CONSTRUCT_FAIL(msg) \
  SIM_CONSTRUCT_; \
  ASSERT_EQ(sim, 0u); \
  ASSERT_STREQ(dqcs_error_get(), msg) \

#define MAKE_ARB(arb, json, ...) do { \
  arb = dqcs_arb_new(); \
  ASSERT_NE(arb, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_arb_json_set(arb, json), dqcs_return_t::DQCS_SUCCESS); \
  const char *args[] = {__VA_ARGS__}; \
  for (const char *arg : args) { \
    ASSERT_EQ(dqcs_arb_push_str(arb, arg), dqcs_return_t::DQCS_SUCCESS); \
  } \
  } while (0)

#define MAKE_CMD(cmd, iface, oper, json, ...) do { \
  cmd = dqcs_cmd_new(iface, oper); \
  ASSERT_NE(cmd, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_arb_json_set(cmd, json), dqcs_return_t::DQCS_SUCCESS); \
  const char *args[] = {__VA_ARGS__}; \
  for (const char *arg : args) { \
    ASSERT_EQ(dqcs_arb_push_str(cmd, arg), dqcs_return_t::DQCS_SUCCESS); \
  } \
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

#define CHECK_CMD(cmd, iface, oper, json, ...) do { \
  char *s; \
  EXPECT_STREQ(s = dqcs_cmd_iface_get(cmd), iface); if (s) free(s); \
  EXPECT_STREQ(s = dqcs_cmd_oper_get(cmd), oper); if (s) free(s); \
  CHECK_ARB(cmd, json, ##__VA_ARGS__) \
  } while (0)

#define CHECK_EMPTY_ARB(arb) CHECK_ARB(arb, "{}")
#define CHECK_EMPTY_CMD(cmd, iface, oper) CHECK_CMD(cmd, iface, oper, "{}")
