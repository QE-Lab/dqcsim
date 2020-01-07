#include <dqcsim.h>
#include "gtest/gtest.h"
#include "util.h"
#include <queue>
#include <vector>

typedef struct {
  bool alloc;
  std::vector<dqcs_qubit_t> qubits;
} operation_t;

typedef struct {
  std::queue<operation_t> ops;
} operations_t;

void log_op(void *user_data, bool alloc, dqcs_handle_t qubits) {
  operations_t *ops = (operations_t*)user_data;
  operation_t op;
  op.alloc = alloc;
  while (dqcs_qubit_t qubit = dqcs_qbset_pop(qubits)) {
    op.qubits.push_back(qubit);
  }
  ops->ops.push(op);
  dqcs_handle_delete(qubits);
}

dqcs_return_t allocate_cb_oper(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t qubits, dqcs_handle_t alloc_cmds) {
  dqcs_plugin_allocate(state, dqcs_qbset_len(qubits), alloc_cmds);
  log_op(user_data, true, qubits);
  dqcs_handle_delete(alloc_cmds);
  return dqcs_return_t::DQCS_SUCCESS;
}

dqcs_return_t free_cb_oper(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t qubits) {
  dqcs_plugin_free(state, dqcs_qbset_copy(qubits));
  log_op(user_data, false, qubits);
  return dqcs_return_t::DQCS_SUCCESS;
}

dqcs_return_t allocate_cb_back(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t qubits, dqcs_handle_t alloc_cmds) {
  (void)state;
  log_op(user_data, true, qubits);
  dqcs_handle_delete(alloc_cmds);
  return dqcs_return_t::DQCS_SUCCESS;
}

dqcs_return_t free_cb_back(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t qubits) {
  (void)state;
  log_op(user_data, false, qubits);
  return dqcs_return_t::DQCS_SUCCESS;
}

#define ALLOC(n) do { \
  dqcs_handle_t cmds = dqcs_cq_new(); \
  if (!cmds) return dqcs_return_t::DQCS_FAILURE; \
  dqcs_handle_t cmd = dqcs_cmd_new("a", "b"); \
  if (!cmd) return dqcs_return_t::DQCS_FAILURE; \
  if (dqcs_cq_push(cmds, cmd) != dqcs_return_t::DQCS_SUCCESS) return dqcs_return_t::DQCS_FAILURE; \
  dqcs_handle_t qubits = dqcs_plugin_allocate(state, n, cmds); \
  if (!qubits) return dqcs_return_t::DQCS_FAILURE; \
  if (dqcs_qbset_len(qubits) != n) { \
    dqcs_error_set("Received incorrect number of qubits from dqcs_plugin_allocate()!"); \
    return dqcs_return_t::DQCS_FAILURE; \
  } \
  log_op(user_data, true, qubits); \
  } while (0)

#define _FREE(...) \
  dqcs_handle_t qubits_handle = dqcs_qbset_new(); \
  if (!qubits_handle) return dqcs_return_t::DQCS_FAILURE; \
  dqcs_qubit_t qubits[] = {__VA_ARGS__}; \
  for (dqcs_qubit_t qubit : qubits) { \
    if (dqcs_qbset_push(qubits_handle, qubit) != dqcs_return_t::DQCS_SUCCESS) return dqcs_return_t::DQCS_FAILURE; \
  } \
  result = dqcs_plugin_free(state, dqcs_qbset_copy(qubits_handle));

#define FREE_OK(...) do { \
  dqcs_return_t result; \
  _FREE(__VA_ARGS__) \
  if (result != dqcs_return_t::DQCS_SUCCESS) return dqcs_return_t::DQCS_FAILURE; \
  log_op(user_data, false, qubits_handle); \
  } while (0)

#define FREE_FAIL(msg, ...) do { \
  dqcs_return_t result; \
  _FREE(__VA_ARGS__) \
  if (result == dqcs_return_t::DQCS_SUCCESS) { \
    dqcs_error_set("Unexpected success, expected " msg); \
    return dqcs_return_t::DQCS_FAILURE; \
  } \
  if (strcmp(dqcs_error_get(), msg)) { \
    return dqcs_return_t::DQCS_FAILURE; \
  } \
  } while (0)

namespace alloc_free {

  dqcs_return_t initialize_cb(void *user_data, dqcs_plugin_state_t state, dqcs_handle_t init_cmds) {
    (void)init_cmds;
    ALLOC(2);
    FREE_FAIL("Invalid argument: qubit 3 is not allocated", 1, 2, 3);
    ALLOC(3);
    FREE_OK(3, 2, 1);
    FREE_OK();
    ALLOC(0);
    FREE_FAIL("Invalid argument: qubit 6 is not allocated", 4, 6, 5);
    FREE_OK(4, 5);
    return dqcs_return_t::DQCS_SUCCESS;
  }

}


TEST(plugin_alloc_free, test) {
  operations_t ops_front, ops_oper, ops_back;

  SIM_HEADER;
  ASSERT_EQ(dqcs_pdef_set_initialize_cb(front, alloc_free::initialize_cb, NULL, &ops_front), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_allocate_cb(oper, allocate_cb_oper, NULL, &ops_oper), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_free_cb(oper, free_cb_oper, NULL, &ops_oper), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_allocate_cb(back, allocate_cb_back, NULL, &ops_back), dqcs_return_t::DQCS_SUCCESS);
  ASSERT_EQ(dqcs_pdef_set_free_cb(back, free_cb_back, NULL, &ops_back), dqcs_return_t::DQCS_SUCCESS);
  SIM_CONSTRUCT;
  SIM_FOOTER;

  dqcs_qubit_t q = 1;

  while (!ops_front.ops.empty() && !ops_oper.ops.empty() && !ops_back.ops.empty()) {
    EXPECT_EQ(ops_front.ops.front().alloc, ops_oper.ops.front().alloc);
    EXPECT_EQ(ops_front.ops.front().alloc, ops_back.ops.front().alloc);

    EXPECT_EQ(ops_front.ops.front().qubits.size(), ops_oper.ops.front().qubits.size());
    EXPECT_EQ(ops_front.ops.front().qubits.size(), ops_back.ops.front().qubits.size());

    for (size_t i = 0; i < ops_front.ops.front().qubits.size(); i++) {
      if (ops_front.ops.front().alloc) {
        EXPECT_EQ(ops_front.ops.front().qubits[i], q);
        q++;
      }
      EXPECT_EQ(ops_front.ops.front().qubits[i], ops_oper.ops.front().qubits[i]);
      EXPECT_EQ(ops_front.ops.front().qubits[i], ops_back.ops.front().qubits[i]);
    }

    ops_front.ops.pop();
    ops_oper.ops.pop();
    ops_back.ops.pop();
  }
  EXPECT_EQ(ops_front.ops.empty(), true);
  EXPECT_EQ(ops_oper.ops.empty(), true);
  EXPECT_EQ(ops_back.ops.empty(), true);
}
