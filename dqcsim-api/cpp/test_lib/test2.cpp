#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"

using namespace dqcsim;

dqcs_handle_t cb_run(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t args) {
  printf("User data: 0x%016llX\n", (long long)user_data);
  printf("State pointer: 0x%016llX\n", (long long)state);
  printf("Args: %d\n", (int)args);
  return args;
}

TEST(Test, Test) {
  dqcs_handle_t scfg = dqcs_scfg_new();
  dqcs_scfg_dqcsim_verbosity_set(scfg, dqcs_loglevel_t::DQCS_LOG_TRACE);
  dqcs_scfg_stderr_verbosity_set(scfg, dqcs_loglevel_t::DQCS_LOG_TRACE);

  dqcs_handle_t front = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "test!", "jvs", "3.14");
  dqcs_pdef_set_run_cb(front, cb_run, NULL, (void*)0x12345);
  front = dqcs_tcfg_new(front, "");
  dqcs_scfg_push_plugin(scfg, front);

  dqcs_handle_t back = dqcs_pcfg_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, "", "target/debug/dqcsim-plugin");
  dqcs_scfg_push_plugin(scfg, back);

  dqcs_handle_t sim = dqcs_sim_init(scfg);

  dqcs_handle_t arb = dqcs_arb_new();
  dqcs_accel_start(sim, arb);
  dqcs_handle_t x = dqcs_accel_wait(sim);
  if (!x) {
    printf("Error returned by wait(): %s\n", dqcs_error_get());
  } else {
    printf("%s", dqcs_handle_dump(x));
  }

  dqcs_handle_delete(sim);
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
