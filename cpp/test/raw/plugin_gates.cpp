#include <dqcsim.h>
#include "gtest/gtest.h"
#include "util.h"
#include <vector>
#include <queue>

// Because the C preprocessor is retarded and doesn't understand {}:
#define QUBITS(...) __VA_ARGS__

#define QBSET(qbset, ...) do { \
  qbset = dqcs_qbset_new(); \
  if (!qbset) return dqcs_return_t::DQCS_FAILURE; \
  dqcs_qubit_t qubits[] = {__VA_ARGS__}; \
  for (dqcs_qubit_t qubit : qubits) { \
    if (dqcs_qbset_push(qbset, qubit) != dqcs_return_t::DQCS_SUCCESS) return dqcs_return_t::DQCS_FAILURE; \
  } \
  } while (0)

#define ALLOC(n) do { \
  dqcs_handle_t cmds = dqcs_cq_new(); \
  if (!cmds) return dqcs_return_t::DQCS_FAILURE; \
  dqcs_handle_t cmd = dqcs_cmd_new("a", "b"); \
  if (!cmd) return dqcs_return_t::DQCS_FAILURE; \
  if (dqcs_cq_push(cmds, cmd) != dqcs_return_t::DQCS_SUCCESS) return dqcs_return_t::DQCS_FAILURE; \
  dqcs_handle_t qubits = dqcs_plugin_allocate(state, n, cmds); \
  if (!qubits) return dqcs_return_t::DQCS_FAILURE; \
  dqcs_handle_delete(qubits); \
  } while (0)

#define FREE(...) do { \
  dqcs_handle_t qbset; \
  QBSET(qbset, __VA_ARGS__); \
  if (dqcs_plugin_free(state, qbset) != dqcs_return_t::DQCS_SUCCESS) return dqcs_return_t::DQCS_FAILURE; \
  } while (0)

#define GATE(name, targets, controls, measures) do { \
  dqcs_handle_t targets_handle, controls_handle, measures_handle; \
  QBSET(targets_handle, targets); \
  QBSET(controls_handle, controls); \
  QBSET(measures_handle, measures); \
  dqcs_handle_t gate = dqcs_gate_new_custom(name, targets_handle, controls_handle, measures_handle, 0); \
  if (!gate) return dqcs_return_t::DQCS_FAILURE; \
  if (dqcs_plugin_gate(state, gate) != dqcs_return_t::DQCS_SUCCESS) return dqcs_return_t::DQCS_FAILURE; \
  } while (0)

#define ADVANCE(cycles) do { \
  if (!dqcs_plugin_advance(state, cycles)) return dqcs_return_t::DQCS_FAILURE; \
  } while (0)

#define EXPECT_CYCLE(cycle) do { \
  if (dqcs_plugin_get_cycle(state) != cycle) { \
    dqcs_error_set("Unexpected cycle count"); \
    return dqcs_return_t::DQCS_FAILURE; \
  } \
  } while (0)

#define EXPECT_MEASUREMENT(qubit, m, t, s) do { \
  dqcs_cycle_t cycle = dqcs_plugin_get_cycles_between_measures(state, qubit); \
  if (cycle != t) { \
    dqcs_log_fatal("Received unexpected cycles between measures: %d != %d on line %d", (int)cycle, (int)t, __LINE__); \
    dqcs_error_set("Received unexpected cycles between measures"); \
    return dqcs_return_t::DQCS_FAILURE; \
  } \
  cycle = dqcs_plugin_get_cycles_since_measure(state, qubit); \
  if (cycle != s) { \
    dqcs_log_fatal("Received unexpected cycles since measure: %d != %d on line %d", (int)cycle, (int)t, __LINE__); \
    dqcs_error_set("Received unexpected cycles since measure"); \
    return dqcs_return_t::DQCS_FAILURE; \
  } \
  dqcs_handle_t meas = dqcs_plugin_get_measurement(state, qubit); \
  if (dqcs_measurement_t::m == dqcs_measurement_t::DQCS_MEAS_INVALID) { \
    if (meas) { \
      dqcs_error_set("Unexpected measurement data"); \
      return dqcs_return_t::DQCS_FAILURE; \
    } \
  } else { \
    if (dqcs_meas_qubit_get(meas) != qubit) { \
      dqcs_error_set("Received measurement data for wrong qubit"); \
      return dqcs_return_t::DQCS_FAILURE; \
    } \
    dqcs_measurement_t value = dqcs_meas_value_get(meas); \
    if (value != dqcs_measurement_t::m) { \
      dqcs_log_fatal("Received unexpected measurement value: %d != %d on line %d", (int)value, (int)dqcs_measurement_t::m, __LINE__); \
      dqcs_error_set("Received unexpected measurement value"); \
      return dqcs_return_t::DQCS_FAILURE; \
    } \
  } \
  } while (0)

typedef struct {
  std::string gate_name;
  std::vector<dqcs_qubit_t> targets;
  std::vector<dqcs_qubit_t> controls;
  std::vector<dqcs_qubit_t> measures;
  dqcs_cycle_t advance;
} gatestream_op_t;

typedef struct {
  std::queue<gatestream_op_t> ops;
} back_data_t;

dqcs_handle_t back_gate_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t gate) {
  (void)state;
  back_data_t *data = (back_data_t*)user_data;
  gatestream_op_t op;
  dqcs_handle_t meas_data = dqcs_mset_new();
  dqcs_handle_t qubits;

  op.advance = 0;

  char *s = dqcs_gate_name(gate);
  if (!s) return 0;
  op.gate_name = std::string(s);
  free(s);

  qubits = dqcs_gate_targets(gate);
  while (dqcs_qubit_t qubit = dqcs_qbset_pop(qubits)) {
    op.targets.push_back(qubit);
  }
  dqcs_handle_delete(qubits);

  qubits = dqcs_gate_controls(gate);
  while (dqcs_qubit_t qubit = dqcs_qbset_pop(qubits)) {
    op.controls.push_back(qubit);
  }
  dqcs_handle_delete(qubits);

  qubits = dqcs_gate_measures(gate);
  while (dqcs_qubit_t qubit = dqcs_qbset_pop(qubits)) {
    op.measures.push_back(qubit);
    dqcs_mset_set(meas_data, dqcs_meas_new(qubit, dqcs_measurement_t::DQCS_MEAS_ZERO));
  }
  dqcs_handle_delete(qubits);

  dqcs_handle_delete(gate);

  data->ops.push(op);
  return meas_data;
}

dqcs_return_t back_advance_cb(void *user_data, dqcs_plugin_state_t state, dqcs_cycle_t cycles) {
  (void)state;
  back_data_t *data = (back_data_t*)user_data;
  gatestream_op_t op;

  op.gate_name = std::string("");
  op.advance = cycles;

  data->ops.push(op);
  return dqcs_return_t::DQCS_SUCCESS;
}

dqcs_return_t op_allocate_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t qubits, dqcs_handle_t alloc_cmds) {
  (void)user_data;
  size_t ds_qubits = dqcs_qbset_len(qubits) * 2;
  dqcs_handle_delete(qubits);
  return dqcs_plugin_allocate(state, ds_qubits, alloc_cmds) ? dqcs_return_t::DQCS_SUCCESS : dqcs_return_t::DQCS_FAILURE;
}

dqcs_return_t op_free_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t qubits) {
  (void)user_data;
  dqcs_handle_t ds_qubits = dqcs_qbset_new();
  while (dqcs_qubit_t qubit = dqcs_qbset_pop(qubits)) {
    dqcs_qbset_push(ds_qubits, qubit * 2 - 1);
    dqcs_qbset_push(ds_qubits, qubit * 2);
  }
  dqcs_handle_delete(qubits);
  return dqcs_plugin_free(state, ds_qubits);
}

dqcs_handle_t op_gate_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t gate) {
  back_data_t *data = (back_data_t*)user_data;
  gatestream_op_t op;
  dqcs_handle_t meas_data = dqcs_mset_new();
  dqcs_handle_t qubits, ds_targets, ds_controls, ds_measures;

  op.advance = 0;

  char *s = dqcs_gate_name(gate);
  if (!s) return 0;
  op.gate_name = std::string(s);

  ds_targets = dqcs_qbset_new();
  ds_controls = dqcs_qbset_new();
  ds_measures = dqcs_qbset_new();

  qubits = dqcs_gate_targets(gate);
  while (dqcs_qubit_t qubit = dqcs_qbset_pop(qubits)) {
    op.targets.push_back(qubit);
    dqcs_qbset_push(ds_targets, qubit * 2 - 1);
    dqcs_qbset_push(ds_targets, qubit * 2);
  }
  dqcs_handle_delete(qubits);

  qubits = dqcs_gate_controls(gate);
  while (dqcs_qubit_t qubit = dqcs_qbset_pop(qubits)) {
    op.controls.push_back(qubit);
    dqcs_qbset_push(ds_controls, qubit * 2 - 1);
    dqcs_qbset_push(ds_controls, qubit * 2);
  }
  dqcs_handle_delete(qubits);

  qubits = dqcs_gate_measures(gate);
  while (dqcs_qubit_t qubit = dqcs_qbset_pop(qubits)) {
    op.measures.push_back(qubit);
    if (qubit % 2) {
      dqcs_mset_set(meas_data, dqcs_meas_new(qubit, dqcs_measurement_t::DQCS_MEAS_ZERO));
    } else {
      dqcs_qbset_push(ds_measures, qubit * 2 - 1);
      dqcs_qbset_push(ds_measures, qubit * 2);
    }
  }
  dqcs_handle_delete(qubits);

  dqcs_handle_delete(gate);

  gate = dqcs_gate_new_custom(s, ds_targets, ds_controls, ds_measures, 0);
  dqcs_plugin_gate(state, gate);
  dqcs_plugin_advance(state, 1);

  free(s);

  data->ops.push(op);
  return meas_data;
}

dqcs_return_t op_advance_cb(void *user_data, dqcs_plugin_state_t state, dqcs_cycle_t cycles) {
  back_data_t *data = (back_data_t*)user_data;
  gatestream_op_t op;

  op.gate_name = std::string("");
  op.advance = cycles;

  dqcs_plugin_advance(state, cycles * 2);

  data->ops.push(op);
  return dqcs_return_t::DQCS_SUCCESS;
}

dqcs_handle_t op_modify_measurement_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t meas) {
  (void)user_data;
  (void)state;
  dqcs_qubit_t qubit = dqcs_meas_qubit_get(meas);
  dqcs_handle_t meas_data = dqcs_mset_new();
  if (qubit % 2 == 0) {
    dqcs_mset_set(meas_data, dqcs_meas_new(qubit / 2, dqcs_measurement_t::DQCS_MEAS_ONE));
  }
  dqcs_handle_delete(meas);
  return meas_data;
}

#define EXPECT_QUBITS(vec, ...) do { \
  dqcs_qubit_t qubits[] = {__VA_ARGS__}; \
  size_t i = 0; \
  for (dqcs_qubit_t qubit : qubits) { \
    EXPECT_EQ(vec[i], qubit); \
    i++; \
  } \
  EXPECT_EQ(vec.size(), i); \
  } while (0)

#define EXPECT_GATE(queue, n, t, c, m) do { \
  EXPECT_EQ(queue.empty(), false); \
  if (!queue.empty()) { \
    EXPECT_EQ(queue.front().gate_name, n); \
    EXPECT_QUBITS(queue.front().targets, t); \
    EXPECT_QUBITS(queue.front().controls, c); \
    EXPECT_QUBITS(queue.front().measures, m); \
    EXPECT_EQ(queue.front().advance, 0); \
    queue.pop(); \
  } \
  } while (0)

#define EXPECT_ADVANCE(queue, c) do { \
  EXPECT_EQ(queue.empty(), false); \
  if (!queue.empty()) { \
    EXPECT_EQ(queue.front().gate_name, ""); \
    EXPECT_EQ(queue.front().targets.empty(), true); \
    EXPECT_EQ(queue.front().controls.empty(), true); \
    EXPECT_EQ(queue.front().measures.empty(), true); \
    EXPECT_EQ(queue.front().advance, c); \
    queue.pop(); \
  } \
  } while (0)

#define EXPECT_DONE(queue) do { \
  EXPECT_EQ(queue.empty(), true); \
  } while (0)

// Test a stream of gates and advancements without fetching any measurement
// results.
dqcs_return_t initialize_cb_no_feedback(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t init_cmds) {
  (void)user_data;
  (void)state;
  (void)init_cmds;
  ALLOC(10);
  EXPECT_CYCLE(0);
  GATE("MEAS", QUBITS(), QUBITS(), QUBITS(1, 2, 3, 4, 5));
  ADVANCE(3);
  EXPECT_CYCLE(3);
  GATE("X", QUBITS(1), QUBITS(), QUBITS());
  GATE("Y", QUBITS(2), QUBITS(), QUBITS());
  GATE("Z", QUBITS(3), QUBITS(), QUBITS());
  EXPECT_CYCLE(3);
  ADVANCE(2);
  EXPECT_CYCLE(5);
  GATE("CNOT", QUBITS(4), QUBITS(5), QUBITS());
  EXPECT_CYCLE(5);
  ADVANCE(5);
  EXPECT_CYCLE(10);
  GATE("MEAS", QUBITS(), QUBITS(), QUBITS(3, 4, 5, 6, 7));
  EXPECT_CYCLE(10);
  FREE(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
  return dqcs_return_t::DQCS_SUCCESS;
}

TEST(plugin_gates, no_feedback) {
  back_data_t back_data;

  SIM_HEADER;
  ASSERT_EQ(dqcs_pdef_set_initialize_cb(front, initialize_cb_no_feedback, NULL, NULL), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_gate_cb(back, back_gate_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_advance_cb(back, back_advance_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  SIM_CONSTRUCT;
  SIM_FOOTER;

  EXPECT_GATE(back_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(1, 2, 3, 4, 5));
  EXPECT_ADVANCE(back_data.ops, 3);
  EXPECT_GATE(back_data.ops, "X", QUBITS(1), QUBITS(), QUBITS());
  EXPECT_GATE(back_data.ops, "Y", QUBITS(2), QUBITS(), QUBITS());
  EXPECT_GATE(back_data.ops, "Z", QUBITS(3), QUBITS(), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 2);
  EXPECT_GATE(back_data.ops, "CNOT", QUBITS(4), QUBITS(5), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 5);
  EXPECT_GATE(back_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(3, 4, 5, 6, 7));
  EXPECT_DONE(back_data.ops);
}

TEST(plugin_gates, no_feedback_with_operator) {
  back_data_t back_data, oper_data;

  SIM_HEADER;
  ASSERT_EQ(dqcs_pdef_set_initialize_cb(front, initialize_cb_no_feedback, NULL, NULL), dqcs_return_t::DQCS_SUCCESS);

  ASSERT_EQ(dqcs_pdef_set_allocate_cb(oper, op_allocate_cb, NULL, &oper_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_free_cb(oper, op_free_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_gate_cb(oper, op_gate_cb, NULL, &oper_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_advance_cb(oper, op_advance_cb, NULL, &oper_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_modify_measurement_cb(oper, op_modify_measurement_cb, NULL, &oper_data), dqcs_return_t::DQCS_SUCCESS);

  ASSERT_EQ(dqcs_pdef_set_gate_cb(back, back_gate_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_advance_cb(back, back_advance_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  SIM_CONSTRUCT;
  SIM_FOOTER;

  EXPECT_GATE(oper_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(1, 2, 3, 4, 5));
  EXPECT_ADVANCE(oper_data.ops, 3);
  EXPECT_GATE(oper_data.ops, "X", QUBITS(1), QUBITS(), QUBITS());
  EXPECT_GATE(oper_data.ops, "Y", QUBITS(2), QUBITS(), QUBITS());
  EXPECT_GATE(oper_data.ops, "Z", QUBITS(3), QUBITS(), QUBITS());
  EXPECT_ADVANCE(oper_data.ops, 2);
  EXPECT_GATE(oper_data.ops, "CNOT", QUBITS(4), QUBITS(5), QUBITS());
  EXPECT_ADVANCE(oper_data.ops, 5);
  EXPECT_GATE(oper_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(3, 4, 5, 6, 7));
  EXPECT_DONE(oper_data.ops);

  EXPECT_GATE(back_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(3, 4, 7, 8));
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_ADVANCE(back_data.ops, 6);
  EXPECT_GATE(back_data.ops, "X", QUBITS(1, 2), QUBITS(), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_GATE(back_data.ops, "Y", QUBITS(3, 4), QUBITS(), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_GATE(back_data.ops, "Z", QUBITS(5, 6), QUBITS(), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_ADVANCE(back_data.ops, 4);
  EXPECT_GATE(back_data.ops, "CNOT", QUBITS(7, 8), QUBITS(9, 10), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_ADVANCE(back_data.ops, 10);
  EXPECT_GATE(back_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(7, 8, 11, 12));
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_DONE(back_data.ops);
}

TEST(plugin_gates, no_feedback_with_broken_operator) {
  back_data_t back_data, oper_data;

  SIM_HEADER;
  ASSERT_EQ(dqcs_pdef_set_initialize_cb(front, initialize_cb_no_feedback, NULL, NULL), dqcs_return_t::DQCS_SUCCESS);

  ASSERT_EQ(dqcs_pdef_set_allocate_cb(oper, op_allocate_cb, NULL, &oper_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_free_cb(oper, op_free_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_gate_cb(oper, op_gate_cb, NULL, &oper_data), dqcs_return_t::DQCS_SUCCESS);
  // note: advance and modify_measurement missing; the latter causes
  // incorrect measurement data.

  ASSERT_EQ(dqcs_pdef_set_gate_cb(back, back_gate_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_advance_cb(back, back_advance_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  SIM_CONSTRUCT;
  SIM_FOOTER;

  EXPECT_GATE(oper_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(1, 2, 3, 4, 5));
  EXPECT_GATE(oper_data.ops, "X", QUBITS(1), QUBITS(), QUBITS());
  EXPECT_GATE(oper_data.ops, "Y", QUBITS(2), QUBITS(), QUBITS());
  EXPECT_GATE(oper_data.ops, "Z", QUBITS(3), QUBITS(), QUBITS());
  EXPECT_GATE(oper_data.ops, "CNOT", QUBITS(4), QUBITS(5), QUBITS());
  EXPECT_GATE(oper_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(3, 4, 5, 6, 7));
  EXPECT_DONE(oper_data.ops);

  EXPECT_GATE(back_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(3, 4, 7, 8));
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_ADVANCE(back_data.ops, 3);
  EXPECT_GATE(back_data.ops, "X", QUBITS(1, 2), QUBITS(), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_GATE(back_data.ops, "Y", QUBITS(3, 4), QUBITS(), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_GATE(back_data.ops, "Z", QUBITS(5, 6), QUBITS(), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_ADVANCE(back_data.ops, 2);
  EXPECT_GATE(back_data.ops, "CNOT", QUBITS(7, 8), QUBITS(9, 10), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_ADVANCE(back_data.ops, 5);
  EXPECT_GATE(back_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(7, 8, 11, 12));
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_DONE(back_data.ops);
}

// Test a stream of gates and advancements without fetching any measurement
// results.
dqcs_return_t initialize_cb_with_feedback(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t init_cmds) {
  (void)init_cmds;
  long test = (long)user_data;

  // Measure non-existant qubit.
  EXPECT_MEASUREMENT(2, DQCS_MEAS_INVALID, -1, -1);

  ALLOC(10);
  EXPECT_CYCLE(0);

  GATE("MEAS", QUBITS(), QUBITS(), QUBITS(1, 2, 3, 4, 5));

  // Results before a measurement.
  EXPECT_MEASUREMENT(6, DQCS_MEAS_INVALID, -1, -1);

  // Results after a measurement.
  switch (test) {
    case 0: EXPECT_MEASUREMENT(2, DQCS_MEAS_ZERO, -1, 0); break; // Received from backend
    case 1: EXPECT_MEASUREMENT(2, DQCS_MEAS_ONE, -1, 0); break; // Modified by op
    case 2: EXPECT_MEASUREMENT(2, DQCS_MEAS_UNDEFINED, -1, 0); break; // Missing due to broken op
  }

  // Broken operator will spuriously send data for qubits 3, 7, and 8. Make
  // sure that these are ignored.
  EXPECT_MEASUREMENT(3, DQCS_MEAS_ZERO, -1, 0);
  EXPECT_MEASUREMENT(8, DQCS_MEAS_INVALID, -1, -1);

  ADVANCE(3);
  EXPECT_CYCLE(3);

  // Results after a measurement + advance.
  EXPECT_MEASUREMENT(3, DQCS_MEAS_ZERO, -1, 3);

  GATE("X", QUBITS(1), QUBITS(), QUBITS());
  GATE("Y", QUBITS(2), QUBITS(), QUBITS());
  GATE("Z", QUBITS(3), QUBITS(), QUBITS());
  EXPECT_CYCLE(3);
  ADVANCE(2);
  EXPECT_CYCLE(5);
  GATE("CNOT", QUBITS(4), QUBITS(5), QUBITS());
  EXPECT_CYCLE(5);
  ADVANCE(5);
  EXPECT_CYCLE(10);
  GATE("MEAS", QUBITS(), QUBITS(), QUBITS(3, 4, 5, 6, 7));
  EXPECT_CYCLE(10);

  // Test measurement timer.
  EXPECT_MEASUREMENT(3, DQCS_MEAS_ZERO, 10, 0);
  ADVANCE(5);
  EXPECT_MEASUREMENT(3, DQCS_MEAS_ZERO, 10, 5);

  FREE(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

  // Measure deallocated qubit.
  EXPECT_MEASUREMENT(2, DQCS_MEAS_INVALID, -1, -1);

  return dqcs_return_t::DQCS_SUCCESS;
}

TEST(plugin_gates, with_feedback) {
  back_data_t back_data;

  SIM_HEADER;
  ASSERT_EQ(dqcs_pdef_set_initialize_cb(front, initialize_cb_with_feedback, NULL, (void*)0), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_gate_cb(back, back_gate_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_advance_cb(back, back_advance_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  SIM_CONSTRUCT;
  SIM_FOOTER;

  EXPECT_GATE(back_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(1, 2, 3, 4, 5));
  EXPECT_ADVANCE(back_data.ops, 3);
  EXPECT_GATE(back_data.ops, "X", QUBITS(1), QUBITS(), QUBITS());
  EXPECT_GATE(back_data.ops, "Y", QUBITS(2), QUBITS(), QUBITS());
  EXPECT_GATE(back_data.ops, "Z", QUBITS(3), QUBITS(), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 2);
  EXPECT_GATE(back_data.ops, "CNOT", QUBITS(4), QUBITS(5), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 5);
  EXPECT_GATE(back_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(3, 4, 5, 6, 7));
  EXPECT_ADVANCE(back_data.ops, 5);
  EXPECT_DONE(back_data.ops);
}

TEST(plugin_gates, with_feedback_with_operator) {
  back_data_t back_data, oper_data;

  SIM_HEADER;
  ASSERT_EQ(dqcs_pdef_set_initialize_cb(front, initialize_cb_with_feedback, NULL, (void*)1), dqcs_return_t::DQCS_SUCCESS);

  ASSERT_EQ(dqcs_pdef_set_allocate_cb(oper, op_allocate_cb, NULL, &oper_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_free_cb(oper, op_free_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_gate_cb(oper, op_gate_cb, NULL, &oper_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_advance_cb(oper, op_advance_cb, NULL, &oper_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_modify_measurement_cb(oper, op_modify_measurement_cb, NULL, &oper_data), dqcs_return_t::DQCS_SUCCESS);

  ASSERT_EQ(dqcs_pdef_set_gate_cb(back, back_gate_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_advance_cb(back, back_advance_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  SIM_CONSTRUCT;
  SIM_FOOTER;

  EXPECT_GATE(oper_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(1, 2, 3, 4, 5));
  EXPECT_ADVANCE(oper_data.ops, 3);
  EXPECT_GATE(oper_data.ops, "X", QUBITS(1), QUBITS(), QUBITS());
  EXPECT_GATE(oper_data.ops, "Y", QUBITS(2), QUBITS(), QUBITS());
  EXPECT_GATE(oper_data.ops, "Z", QUBITS(3), QUBITS(), QUBITS());
  EXPECT_ADVANCE(oper_data.ops, 2);
  EXPECT_GATE(oper_data.ops, "CNOT", QUBITS(4), QUBITS(5), QUBITS());
  EXPECT_ADVANCE(oper_data.ops, 5);
  EXPECT_GATE(oper_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(3, 4, 5, 6, 7));
  EXPECT_ADVANCE(oper_data.ops, 5);
  EXPECT_DONE(oper_data.ops);

  EXPECT_GATE(back_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(3, 4, 7, 8));
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_ADVANCE(back_data.ops, 6);
  EXPECT_GATE(back_data.ops, "X", QUBITS(1, 2), QUBITS(), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_GATE(back_data.ops, "Y", QUBITS(3, 4), QUBITS(), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_GATE(back_data.ops, "Z", QUBITS(5, 6), QUBITS(), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_ADVANCE(back_data.ops, 4);
  EXPECT_GATE(back_data.ops, "CNOT", QUBITS(7, 8), QUBITS(9, 10), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_ADVANCE(back_data.ops, 10);
  EXPECT_GATE(back_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(7, 8, 11, 12));
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_ADVANCE(back_data.ops, 10);
  EXPECT_DONE(back_data.ops);
}

TEST(plugin_gates, with_feedback_with_broken_operator) {
  back_data_t back_data, oper_data;

  SIM_HEADER;
  ASSERT_EQ(dqcs_pdef_set_initialize_cb(front, initialize_cb_with_feedback, NULL, (void*)2), dqcs_return_t::DQCS_SUCCESS);

  ASSERT_EQ(dqcs_pdef_set_allocate_cb(oper, op_allocate_cb, NULL, &oper_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_free_cb(oper, op_free_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_gate_cb(oper, op_gate_cb, NULL, &oper_data), dqcs_return_t::DQCS_SUCCESS);
  // note: advance and modify_measurement missing; the latter causes
  // incorrect measurement data.

  ASSERT_EQ(dqcs_pdef_set_gate_cb(back, back_gate_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_advance_cb(back, back_advance_cb, NULL, &back_data), dqcs_return_t::DQCS_SUCCESS);
  SIM_CONSTRUCT;
  SIM_FOOTER;

  EXPECT_GATE(oper_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(1, 2, 3, 4, 5));
  EXPECT_GATE(oper_data.ops, "X", QUBITS(1), QUBITS(), QUBITS());
  EXPECT_GATE(oper_data.ops, "Y", QUBITS(2), QUBITS(), QUBITS());
  EXPECT_GATE(oper_data.ops, "Z", QUBITS(3), QUBITS(), QUBITS());
  EXPECT_GATE(oper_data.ops, "CNOT", QUBITS(4), QUBITS(5), QUBITS());
  EXPECT_GATE(oper_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(3, 4, 5, 6, 7));
  EXPECT_DONE(oper_data.ops);

  EXPECT_GATE(back_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(3, 4, 7, 8));
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_ADVANCE(back_data.ops, 3);
  EXPECT_GATE(back_data.ops, "X", QUBITS(1, 2), QUBITS(), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_GATE(back_data.ops, "Y", QUBITS(3, 4), QUBITS(), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_GATE(back_data.ops, "Z", QUBITS(5, 6), QUBITS(), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_ADVANCE(back_data.ops, 2);
  EXPECT_GATE(back_data.ops, "CNOT", QUBITS(7, 8), QUBITS(9, 10), QUBITS());
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_ADVANCE(back_data.ops, 5);
  EXPECT_GATE(back_data.ops, "MEAS", QUBITS(), QUBITS(), QUBITS(7, 8, 11, 12));
  EXPECT_ADVANCE(back_data.ops, 1);
  EXPECT_ADVANCE(back_data.ops, 5);
  EXPECT_DONE(back_data.ops);
}
