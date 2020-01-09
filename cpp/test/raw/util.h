#pragma once

#ifndef DQCSIM_NAMESPACE
#define DQCSIM_NAMESPACE
#endif

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
  DQCSIM_NAMESPACE dqcs_handle_t front, oper, back, sim; \
  sim = DQCSIM_NAMESPACE dqcs_scfg_new(); \
  ASSERT_NE(sim, 0u) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get(); \
  ASSERT_EQ(DQCSIM_NAMESPACE dqcs_scfg_repro_disable(sim), DQCSIM_NAMESPACE dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get(); \
  /*ASSERT_EQ(DQCSIM_NAMESPACE dqcs_scfg_stderr_verbosity_set(sim, DQCSIM_NAMESPACE dqcs_loglevel_t::DQCS_LOG_TRACE), DQCSIM_NAMESPACE dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get();*/ \
  front = DQCSIM_NAMESPACE dqcs_pdef_new(DQCSIM_NAMESPACE dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c"); \
  ASSERT_NE(front, 0u) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get(); \
  oper = DQCSIM_NAMESPACE dqcs_pdef_new(DQCSIM_NAMESPACE dqcs_plugin_type_t::DQCS_PTYPE_OPER, "a", "b", "c"); \
  ASSERT_NE(oper, 0u) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get(); \
  back = DQCSIM_NAMESPACE dqcs_pdef_new(DQCSIM_NAMESPACE dqcs_plugin_type_t::DQCS_PTYPE_BACK, "a", "b", "c"); \
  ASSERT_NE(back, 0u) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get()

#define SIM_CONSTRUCT_ \
  front = DQCSIM_NAMESPACE dqcs_tcfg_new(front, NULL); \
  ASSERT_NE(front, 0u) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get(); \
  ASSERT_EQ(DQCSIM_NAMESPACE dqcs_scfg_push_plugin(sim, front), DQCSIM_NAMESPACE dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get(); \
  oper = DQCSIM_NAMESPACE dqcs_tcfg_new(oper, NULL); \
  ASSERT_NE(oper, 0u) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get(); \
  ASSERT_EQ(DQCSIM_NAMESPACE dqcs_scfg_push_plugin(sim, oper), DQCSIM_NAMESPACE dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get(); \
  back = DQCSIM_NAMESPACE dqcs_tcfg_new(back, NULL); \
  ASSERT_NE(back, 0u) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get(); \
  ASSERT_EQ(DQCSIM_NAMESPACE dqcs_scfg_push_plugin(sim, back), DQCSIM_NAMESPACE dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get(); \
  sim = DQCSIM_NAMESPACE dqcs_sim_new(sim) \

#define SIM_CONSTRUCT \
  SIM_CONSTRUCT_; \
  ASSERT_NE(sim, 0u) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get()

#define SIM_FOOTER \
  EXPECT_EQ(DQCSIM_NAMESPACE dqcs_handle_delete(sim), DQCSIM_NAMESPACE dqcs_return_t::DQCS_SUCCESS); \
  EXPECT_EQ(DQCSIM_NAMESPACE dqcs_handle_leak_check(), DQCSIM_NAMESPACE dqcs_return_t::DQCS_SUCCESS) << DQCSIM_NAMESPACE dqcs_error_get()

#define SIM_CONSTRUCT_FAIL(msg) \
  SIM_CONSTRUCT_; \
  ASSERT_EQ(sim, 0u); \
  ASSERT_STREQ(DQCSIM_NAMESPACE dqcs_error_get(), msg) \

#define MAKE_ARB(arb, json, ...) do { \
  arb = DQCSIM_NAMESPACE dqcs_arb_new(); \
  ASSERT_NE(arb, 0u) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get(); \
  ASSERT_EQ(DQCSIM_NAMESPACE dqcs_arb_json_set(arb, json), DQCSIM_NAMESPACE dqcs_return_t::DQCS_SUCCESS); \
  const char *args[] = {__VA_ARGS__}; \
  for (const char *arg : args) { \
    ASSERT_EQ(DQCSIM_NAMESPACE dqcs_arb_push_str(arb, arg), DQCSIM_NAMESPACE dqcs_return_t::DQCS_SUCCESS); \
  } \
  } while (0)

#define MAKE_CMD(cmd, iface, oper, json, ...) do { \
  cmd = DQCSIM_NAMESPACE dqcs_cmd_new(iface, oper); \
  ASSERT_NE(cmd, 0u) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get(); \
  ASSERT_EQ(DQCSIM_NAMESPACE dqcs_arb_json_set(cmd, json), DQCSIM_NAMESPACE dqcs_return_t::DQCS_SUCCESS); \
  const char *args[] = {__VA_ARGS__}; \
  for (const char *arg : args) { \
    ASSERT_EQ(DQCSIM_NAMESPACE dqcs_arb_push_str(cmd, arg), DQCSIM_NAMESPACE dqcs_return_t::DQCS_SUCCESS); \
  } \
  } while (0)

#define CHECK_ARB(arb, json, ...) do { \
  char *s; \
  EXPECT_NE(arb, 0u) << "Unexpected error: " << DQCSIM_NAMESPACE dqcs_error_get(); \
  EXPECT_STREQ(s = DQCSIM_NAMESPACE dqcs_arb_json_get(arb), json); if (s) free(s); \
  const char *exp[] = {__VA_ARGS__}; \
  int i = 0; \
  for (const char *e : exp) { \
    EXPECT_STREQ(s = DQCSIM_NAMESPACE dqcs_arb_get_str(arb, i), e); if (s) free(s); \
    i++; \
  } \
  EXPECT_EQ(DQCSIM_NAMESPACE dqcs_arb_len(arb), i); \
  EXPECT_EQ(DQCSIM_NAMESPACE dqcs_handle_delete(arb), DQCSIM_NAMESPACE dqcs_return_t::DQCS_SUCCESS); \
  } while (0)

#define CHECK_CMD(cmd, iface, oper, json, ...) do { \
  char *s; \
  EXPECT_STREQ(s = DQCSIM_NAMESPACE dqcs_cmd_iface_get(cmd), iface); if (s) free(s); \
  EXPECT_STREQ(s = DQCSIM_NAMESPACE dqcs_cmd_oper_get(cmd), oper); if (s) free(s); \
  CHECK_ARB(cmd, json, ##__VA_ARGS__) \
  } while (0)

#define CHECK_EMPTY_ARB(arb) CHECK_ARB(arb, "{}")
#define CHECK_EMPTY_CMD(cmd, iface, oper) CHECK_CMD(cmd, iface, oper, "{}")
