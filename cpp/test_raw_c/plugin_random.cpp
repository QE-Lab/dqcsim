#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"
#include "util.h"
#include <queue>

using namespace dqcsim;

typedef struct {
  std::queue<double> f64s;
  std::queue<uint64_t> u64s;
} samples_t;

dqcs_return_t initialize_cb_front(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t init_cmds) {
  dqcs_log_debug("initialize_cb_front");
  dqcs_handle_delete(init_cmds);

  samples_t *samples = (samples_t*)user_data;

  for (int i = 0; i < 3; i++) {
    samples->f64s.push(dqcs_plugin_random_f64(state));
    samples->u64s.push(dqcs_plugin_random_u64(state));
  }

  dqcs_handle_t qubits = dqcs_plugin_allocate(state, 3, 0);
  if (!qubits) return dqcs_return_t::DQCS_FAILURE;
  dqcs_handle_t gate = dqcs_gate_new_measurement(qubits);
  if (!gate) return dqcs_return_t::DQCS_FAILURE;
  if (dqcs_plugin_gate(state, gate) != dqcs_return_t::DQCS_SUCCESS) return dqcs_return_t::DQCS_FAILURE;
  if (!dqcs_plugin_get_measurement(state, 1)) return dqcs_return_t::DQCS_FAILURE;
  if (!dqcs_plugin_get_measurement(state, 2)) return dqcs_return_t::DQCS_FAILURE;
  if (!dqcs_plugin_get_measurement(state, 3)) return dqcs_return_t::DQCS_FAILURE;

  for (int i = 0; i < 3; i++) {
    samples->f64s.push(dqcs_plugin_random_f64(state));
    samples->u64s.push(dqcs_plugin_random_u64(state));
  }

  return dqcs_return_t::DQCS_SUCCESS;
}

dqcs_return_t initialize_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t init_cmds) {
  dqcs_log_debug("initialize_cb");
  dqcs_handle_delete(init_cmds);

  samples_t *samples = (samples_t*)user_data;

  for (int i = 0; i < 6; i++) {
    samples->f64s.push(dqcs_plugin_random_f64(state));
    samples->u64s.push(dqcs_plugin_random_u64(state));
  }

  return dqcs_return_t::DQCS_SUCCESS;
}

dqcs_handle_t modify_measurement_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t meas) {
  dqcs_log_debug("modify_measurement_cb");
  samples_t *samples = (samples_t*)user_data;
  samples->f64s.push(dqcs_plugin_random_f64(state));
  samples->u64s.push(dqcs_plugin_random_u64(state));
  return meas;
}

dqcs_handle_t gate_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t gate) {
  dqcs_log_debug("gate_cb");
  dqcs_handle_t meas_qubits = dqcs_gate_measures(gate);
  dqcs_handle_delete(gate);
  dqcs_handle_t meas_data = dqcs_mset_new();
  while (dqcs_qubit_t qubit = dqcs_qbset_pop(meas_qubits)) {
    dqcs_mset_set(meas_data, dqcs_meas_new(qubit, dqcs_measurement_t::DQCS_MEAS_ZERO));
  }
  return meas_data;
}

#define EXPECT_UNIT_RANGE(x) \
  EXPECT_TRUE(((x) >= 0.0) && ((x) < 1.0)) << "float value out of range [0,1): " << (x)

// Check that two consecutive runs with the same seed generate the same PRNG
// streams.
TEST(plugin_random, run_consistency) {
  samples_t a, b;
  uint64_t seed;

  {
    SIM_HEADER;
    seed = dqcs_scfg_seed_get(sim);
    ASSERT_EQ(dqcs_pdef_set_initialize_cb(front, initialize_cb, NULL, &a), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_initialize_cb(oper, initialize_cb, NULL, &a), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_initialize_cb(back, initialize_cb, NULL, &a), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_gate_cb(back, gate_cb, NULL, NULL), dqcs_return_t::DQCS_SUCCESS);
    SIM_CONSTRUCT;
    SIM_FOOTER;
  }

  {
    SIM_HEADER;
    ASSERT_EQ(dqcs_scfg_seed_set(sim, seed), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_initialize_cb(front, initialize_cb, NULL, &b), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_initialize_cb(oper, initialize_cb, NULL, &b), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_initialize_cb(back, initialize_cb, NULL, &b), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_gate_cb(back, gate_cb, NULL, NULL), dqcs_return_t::DQCS_SUCCESS);
    SIM_CONSTRUCT;
    SIM_FOOTER;
  }

  EXPECT_EQ(a.f64s.size(), 18u);
  EXPECT_EQ(a.u64s.size(), 18u);
  EXPECT_EQ(b.f64s.size(), 18u);
  EXPECT_EQ(b.u64s.size(), 18u);

  while (!a.f64s.empty() && !b.f64s.empty()) {
    EXPECT_UNIT_RANGE(a.f64s.front());
    EXPECT_UNIT_RANGE(b.f64s.front());
    EXPECT_EQ(a.f64s.front(), b.f64s.front());
    a.f64s.pop();
    b.f64s.pop();
  }
  EXPECT_EQ(a.f64s.empty(), b.f64s.empty());

  while (!a.u64s.empty() && !b.u64s.empty()) {
    EXPECT_EQ(a.u64s.front(), b.u64s.front());
    a.u64s.pop();
    b.u64s.pop();
  }
  EXPECT_EQ(a.u64s.empty(), b.u64s.empty());
}

// Check that the RPC PRNG stream is unaffected by the upstream response PRNG
// stream.
TEST(plugin_random, modify_meas_consistency) {
  samples_t a, b, c;

  {
    SIM_HEADER;
    ASSERT_EQ(dqcs_scfg_seed_set(sim, 33), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_initialize_cb(front, initialize_cb, NULL, &a), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_initialize_cb(oper, initialize_cb, NULL, &a), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_initialize_cb(back, initialize_cb, NULL, &a), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_gate_cb(back, gate_cb, NULL, NULL), dqcs_return_t::DQCS_SUCCESS);
    SIM_CONSTRUCT;
    SIM_FOOTER;
  }

  {
    SIM_HEADER;
    ASSERT_EQ(dqcs_scfg_seed_set(sim, 33), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_initialize_cb(front, initialize_cb_front, NULL, &b), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_initialize_cb(oper, initialize_cb, NULL, &b), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_modify_measurement_cb(oper, modify_measurement_cb, NULL, &c), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_initialize_cb(back, initialize_cb, NULL, &b), dqcs_return_t::DQCS_SUCCESS);
    ASSERT_EQ(dqcs_pdef_set_gate_cb(back, gate_cb, NULL, NULL), dqcs_return_t::DQCS_SUCCESS);
    SIM_CONSTRUCT;
    SIM_FOOTER;
  }

  EXPECT_EQ(a.f64s.size(), 18u);
  EXPECT_EQ(a.u64s.size(), 18u);
  EXPECT_EQ(b.f64s.size(), 18u);
  EXPECT_EQ(b.u64s.size(), 18u);
  EXPECT_EQ(c.f64s.size(), 3u);
  EXPECT_EQ(c.u64s.size(), 3u);

  while (!c.f64s.empty()) {
    EXPECT_UNIT_RANGE(c.f64s.front());
    c.f64s.pop();
  }

  while (!a.f64s.empty() && !b.f64s.empty()) {
    EXPECT_UNIT_RANGE(a.f64s.front());
    EXPECT_UNIT_RANGE(b.f64s.front());
    EXPECT_EQ(a.f64s.front(), b.f64s.front());
    a.f64s.pop();
    b.f64s.pop();
  }
  EXPECT_EQ(a.f64s.empty(), b.f64s.empty());

  while (!a.u64s.empty() && !b.u64s.empty()) {
    EXPECT_EQ(a.u64s.front(), b.u64s.front());
    a.u64s.pop();
    b.u64s.pop();
  }
  EXPECT_EQ(a.u64s.empty(), b.u64s.empty());
}

// Check that the PRNGs are consistent between OS's and versions, to make sure
// that reproduction files remain valid.
TEST(plugin_random, system_consistency) {
  samples_t a, b;

  SIM_HEADER;
  ASSERT_EQ(dqcs_scfg_seed_set(sim, 33), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_initialize_cb(front, initialize_cb_front, NULL, &a), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_initialize_cb(oper, initialize_cb, NULL, &a), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_modify_measurement_cb(oper, modify_measurement_cb, NULL, &b), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_initialize_cb(back, initialize_cb, NULL, &a), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_gate_cb(back, gate_cb, NULL, NULL), dqcs_return_t::DQCS_SUCCESS);
  SIM_CONSTRUCT;
  SIM_FOOTER;

  EXPECT_EQ(a.f64s.size(), 18u);
  EXPECT_EQ(*((uint64_t*)&a.f64s.front()), 4593409061353752120llu);

  EXPECT_EQ(a.u64s.size(), 18u);
  EXPECT_EQ(a.u64s.front(), 9141847719916741698llu);

  EXPECT_EQ(b.f64s.size(), 3u);
  EXPECT_EQ(*((uint64_t*)&b.f64s.front()), 4599562987221987508llu);

  EXPECT_EQ(b.u64s.size(), 3u);
  EXPECT_EQ(b.u64s.front(), 16429946247105183054llu);
}
