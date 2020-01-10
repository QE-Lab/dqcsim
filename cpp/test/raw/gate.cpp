#include <dqcsim.h>
#include "gtest/gtest.h"

const double X_MATRIX[] = {
  0.0, 0.0,   1.0, 0.0,
  1.0, 0.0,   0.0, 0.0,
};

// Sanity check the gate API.
TEST(gate, sanity) {
  // Create handle.
  dqcs_handle_t a = dqcs_gate_new_custom("NOP", 0, 0, 0, 0);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that the handle is OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_GATE);
  EXPECT_STREQ(dqcs_handle_dump(a), "Gate(\n    Gate {\n        name: Some(\n            \"NOP\",\n        ),\n        targets: [],\n        controls: [],\n        measures: [],\n        matrix: Matrix {\n            data: [],\n            dimension: 0,\n        },\n        data: ArbData {\n            json: Map(\n                {},\n            ),\n            args: [],\n        },\n    },\n)");

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Check that the handle is no longer OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_handle_dump(a), nullptr);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(a) + " is invalid");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

#define EXPECT_QBSET(qbset, ...) \
  do { \
    const dqcs_qubit_t qubits[] = {__VA_ARGS__, 0}; \
    expect_qbset(qbset, qubits); \
  } while (0)

void expect_qbset(dqcs_handle_t qbset, const dqcs_qubit_t *qubits) {
  int len = 0;
  EXPECT_NE(qbset, 0u) << "Unexpected error: " << dqcs_error_get();
  if (qbset) {
    while (*qubits) {
      EXPECT_EQ(dqcs_qbset_contains(qbset, *qubits), dqcs_bool_return_t::DQCS_TRUE) << "Set does not contain qubit " << *qubits;
      qubits++;
      len++;
    }
    EXPECT_EQ(dqcs_qbset_len(qbset), len);
    EXPECT_EQ(dqcs_handle_delete(qbset), dqcs_return_t::DQCS_SUCCESS);
  }
}

#define EXPECT_MATRIX(gate, expected) expect_matrix(gate, expected, sizeof(expected) / 16);
#define EXPECT_NO_MATRIX(gate) expect_matrix(gate, NULL, 0);

void expect_matrix(dqcs_handle_t gate, const double *expected, int expected_len) {
  dqcs_handle_t matrix_handle = 0;
  double *matrix = NULL;
  EXPECT_EQ(dqcs_gate_has_matrix(gate), (expected_len > 0) ? dqcs_bool_return_t::DQCS_TRUE : dqcs_bool_return_t::DQCS_FALSE);
  if (expected_len) {
    EXPECT_NE(matrix_handle = dqcs_gate_matrix(gate), 0) << "Unexpected error: " << dqcs_error_get();
    EXPECT_EQ(dqcs_mat_len(matrix_handle), expected_len);
    EXPECT_NE(matrix = dqcs_mat_get(matrix_handle), (double*)NULL) << "Unexpected error: " << dqcs_error_get();
    for (int i = 0; i < expected_len; i++) {
      EXPECT_EQ(matrix[i*2+0], expected[i*2+0]) << "matrix entry " << i << " real";
      EXPECT_EQ(matrix[i*2+1], expected[i*2+1]) << "matrix entry " << i << " imag";
    }
  } else {
    EXPECT_EQ(matrix_handle = dqcs_gate_matrix(gate), 0);
    EXPECT_STREQ(dqcs_error_get(), "Invalid argument: no matrix associated with gate");
  }
  if (matrix_handle) dqcs_handle_delete(matrix_handle);
  if (matrix) free(matrix);
}

// Check X gate.
TEST(gate, x) {
  char *s;

  dqcs_handle_t targets = dqcs_qbset_new();
  ASSERT_NE(targets, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(targets, 1u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  dqcs_handle_t matrix = dqcs_mat_new(1, X_MATRIX);
  ASSERT_NE(matrix, 0u) << "Unexpected error: " << dqcs_error_get();
  dqcs_handle_t a = dqcs_gate_new_unitary(targets, 0, matrix);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  EXPECT_EQ(dqcs_gate_is_custom(a), dqcs_bool_return_t::DQCS_FALSE);
  EXPECT_STREQ(s = dqcs_gate_name(a), NULL);
  if (s) free(s);

  EXPECT_EQ(dqcs_gate_has_targets(a), dqcs_bool_return_t::DQCS_TRUE);
  EXPECT_QBSET(dqcs_gate_targets(a), 1u);

  EXPECT_EQ(dqcs_gate_has_controls(a), dqcs_bool_return_t::DQCS_FALSE);
  EXPECT_QBSET(dqcs_gate_controls(a), 0u);

  EXPECT_EQ(dqcs_gate_has_measures(a), dqcs_bool_return_t::DQCS_FALSE);
  EXPECT_QBSET(dqcs_gate_measures(a), 0u);

  EXPECT_MATRIX(a, X_MATRIX);

  EXPECT_STREQ(s = dqcs_arb_json_get(a), "{}");
  if (s) free(s);
  EXPECT_EQ(dqcs_arb_len(a), 0);

  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Check CNOT gate.
TEST(gate, cnot) {
  char *s;

  dqcs_handle_t targets = dqcs_qbset_new();
  ASSERT_NE(targets, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(targets, 1u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  dqcs_handle_t controls = dqcs_qbset_new();
  ASSERT_NE(controls, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(controls, 2u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  dqcs_handle_t matrix = dqcs_mat_new(1, X_MATRIX);
  ASSERT_NE(matrix, 0u) << "Unexpected error: " << dqcs_error_get();
  dqcs_handle_t a = dqcs_gate_new_unitary(targets, controls, matrix);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  EXPECT_EQ(dqcs_gate_is_custom(a), dqcs_bool_return_t::DQCS_FALSE);
  EXPECT_STREQ(s = dqcs_gate_name(a), NULL);
  if (s) free(s);

  EXPECT_EQ(dqcs_gate_has_targets(a), dqcs_bool_return_t::DQCS_TRUE);
  EXPECT_QBSET(dqcs_gate_targets(a), 1u);

  EXPECT_EQ(dqcs_gate_has_controls(a), dqcs_bool_return_t::DQCS_TRUE);
  EXPECT_QBSET(dqcs_gate_controls(a), 2u);

  EXPECT_EQ(dqcs_gate_has_measures(a), dqcs_bool_return_t::DQCS_FALSE);
  EXPECT_QBSET(dqcs_gate_measures(a), 0u);

  EXPECT_MATRIX(a, X_MATRIX);

  EXPECT_STREQ(s = dqcs_arb_json_get(a), "{}");
  if (s) free(s);
  EXPECT_EQ(dqcs_arb_len(a), 0);

  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Check measure gate.
TEST(gate, measure) {
  char *s;

  dqcs_handle_t measures = dqcs_qbset_new();
  ASSERT_NE(measures, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(measures, 1u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(measures, 2u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  dqcs_handle_t a = dqcs_gate_new_measurement(measures);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  EXPECT_EQ(dqcs_gate_is_custom(a), dqcs_bool_return_t::DQCS_FALSE);
  EXPECT_STREQ(s = dqcs_gate_name(a), NULL);
  if (s) free(s);

  EXPECT_EQ(dqcs_gate_has_targets(a), dqcs_bool_return_t::DQCS_FALSE);
  EXPECT_QBSET(dqcs_gate_targets(a), 0u);

  EXPECT_EQ(dqcs_gate_has_controls(a), dqcs_bool_return_t::DQCS_FALSE);
  EXPECT_QBSET(dqcs_gate_controls(a), 0u);

  EXPECT_EQ(dqcs_gate_has_measures(a), dqcs_bool_return_t::DQCS_TRUE);
  EXPECT_QBSET(dqcs_gate_measures(a), 1u, 2u);

  EXPECT_NO_MATRIX(a);

  EXPECT_STREQ(s = dqcs_arb_json_get(a), "{}");
  if (s) free(s);
  EXPECT_EQ(dqcs_arb_len(a), 0);

  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Check NOP custom gate.
TEST(gate, nop) {
  char *s;

  dqcs_handle_t a = dqcs_gate_new_custom("NOP", 0, 0, 0, 0);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  EXPECT_EQ(dqcs_gate_is_custom(a), dqcs_bool_return_t::DQCS_TRUE);
  EXPECT_STREQ(s = dqcs_gate_name(a), "NOP");
  if (s) free(s);

  EXPECT_EQ(dqcs_gate_has_targets(a), dqcs_bool_return_t::DQCS_FALSE);
  EXPECT_QBSET(dqcs_gate_targets(a), 0u);

  EXPECT_EQ(dqcs_gate_has_controls(a), dqcs_bool_return_t::DQCS_FALSE);
  EXPECT_QBSET(dqcs_gate_controls(a), 0u);

  EXPECT_EQ(dqcs_gate_has_measures(a), dqcs_bool_return_t::DQCS_FALSE);
  EXPECT_QBSET(dqcs_gate_measures(a), 0u);

  EXPECT_NO_MATRIX(a);

  EXPECT_STREQ(s = dqcs_arb_json_get(a), "{}");
  if (s) free(s);
  EXPECT_EQ(dqcs_arb_len(a), 0);

  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Check complex custom gate.
TEST(gate, discombobulate) {
  char *s;

  dqcs_handle_t targets = dqcs_qbset_new();
  ASSERT_NE(targets, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(targets, 1u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  dqcs_handle_t controls = dqcs_qbset_new();
  ASSERT_NE(controls, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(controls, 2u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  dqcs_handle_t measures = dqcs_qbset_new();
  ASSERT_NE(measures, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(measures, 1u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(measures, 2u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  dqcs_handle_t matrix = dqcs_mat_new(1, X_MATRIX);
  ASSERT_NE(matrix, 0u) << "Unexpected error: " << dqcs_error_get();
  dqcs_handle_t a = dqcs_gate_new_custom("DISCOMBOBULATE", targets, controls, measures, matrix);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_json_set(a, "{\"sequence\": [4, 8, 15, 16, 23, 42]}"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_push_str(a, "(%@#(*^"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  EXPECT_EQ(dqcs_gate_is_custom(a), dqcs_bool_return_t::DQCS_TRUE);
  EXPECT_STREQ(s = dqcs_gate_name(a), "DISCOMBOBULATE");
  if (s) free(s);

  EXPECT_EQ(dqcs_gate_has_targets(a), dqcs_bool_return_t::DQCS_TRUE);
  EXPECT_QBSET(dqcs_gate_targets(a), 1u);

  EXPECT_EQ(dqcs_gate_has_controls(a), dqcs_bool_return_t::DQCS_TRUE);
  EXPECT_QBSET(dqcs_gate_controls(a), 2u);

  EXPECT_EQ(dqcs_gate_has_measures(a), dqcs_bool_return_t::DQCS_TRUE);
  EXPECT_QBSET(dqcs_gate_measures(a), 1u, 2u);

  EXPECT_MATRIX(a, X_MATRIX);

  EXPECT_STREQ(s = dqcs_arb_json_get(a), "{\"sequence\":[4,8,15,16,23,42]}");
  if (s) free(s);
  EXPECT_EQ(dqcs_arb_len(a), 1);
  EXPECT_STREQ(s = dqcs_arb_get_str(a, 0), "(%@#(*^");
  if (s) free(s);

  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Check disallowed gates.
TEST(gate, erroneous) {
  dqcs_handle_t qbset_a = dqcs_qbset_new();
  ASSERT_NE(qbset_a, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(qbset_a, 1u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  dqcs_handle_t qbset_b = dqcs_qbset_new();
  ASSERT_NE(qbset_b, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(qbset_b, 1u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(qbset_b, 2u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(qbset_b, 3u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  dqcs_handle_t qbset_c = dqcs_qbset_new();
  ASSERT_NE(qbset_c, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(qbset_c, 6u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_qbset_push(qbset_c, 7u), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  // Invalid unitaries.
  EXPECT_EQ(dqcs_gate_new_unitary(0, 0, 0), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 0 is invalid");

  dqcs_handle_t matrix = dqcs_mat_new(1, X_MATRIX);
  ASSERT_NE(matrix, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_gate_new_unitary(qbset_a, qbset_b, matrix), 0u);
  dqcs_handle_delete(matrix);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: qubit 1 is used more than once");

  matrix = dqcs_mat_new(1, X_MATRIX);
  ASSERT_NE(matrix, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_gate_new_unitary(qbset_b, 0, matrix), 0u);
  dqcs_handle_delete(matrix);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: the matrix is expected to be of size 64 but was 4");

  EXPECT_EQ(dqcs_gate_new_unitary(qbset_a, 0, 0), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 0 is invalid");

  // Invalid measures.
  EXPECT_EQ(dqcs_gate_new_measurement(0), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 0 is invalid");

  // Invalid custom gates.
  ASSERT_EQ(dqcs_gate_new_custom(NULL, 0, 0, 0, 0), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: unexpected NULL string");

  ASSERT_EQ(dqcs_gate_new_custom("FOO", qbset_a, qbset_b, qbset_c, 0), 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: qubit 1 is used more than once");

  matrix = dqcs_mat_new(1, X_MATRIX);
  ASSERT_NE(matrix, 0u) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_gate_new_custom("BAR", qbset_b, qbset_c, qbset_a, matrix), 0u);
  dqcs_handle_delete(matrix);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: the matrix is expected to be of size 64 but was 4");

  EXPECT_EQ(dqcs_handle_delete(qbset_a), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_handle_delete(qbset_b), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_handle_delete(qbset_c), dqcs_return_t::DQCS_SUCCESS);

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

TEST(gate, control) {
  double i[] = {0.,0.,1.,0.,1.,0.,0.,0.};
  dqcs_handle_t targets = dqcs_qbset_new();
  dqcs_qbset_push(targets, 1);
  dqcs_handle_t controls = dqcs_qbset_new();
  dqcs_qbset_push(controls, 2);
  dqcs_handle_t matrix = dqcs_mat_new(1, i);
  ASSERT_NE(matrix, 0u) << "Unexpected error: " << dqcs_error_get();
  dqcs_handle_t gate = dqcs_gate_new_unitary(targets, controls, matrix);

  // expand
  dqcs_handle_t cnot = dqcs_gate_expand_control(gate);
  EXPECT_EQ(dqcs_handle_type(cnot), dqcs_handle_type_t::DQCS_HTYPE_GATE);
  EXPECT_EQ(dqcs_gate_has_controls(cnot), dqcs_bool_return_t::DQCS_FALSE);

  // reduce
  dqcs_handle_t x = dqcs_gate_reduce_control(cnot, 0.0001, false);
  EXPECT_EQ(dqcs_handle_type(x), dqcs_handle_type_t::DQCS_HTYPE_GATE);
  EXPECT_QBSET(dqcs_gate_controls(x), 2u);
}
