#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"

using namespace dqcsim;

// Sanity check the handle API.
TEST(mm, sanity) {
  // Create handle.
  dqcs_handle_t a = dqcs_mmb_new();
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that the handle is OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_MATRIX_MAP_BUILDER);
  EXPECT_STREQ(dqcs_handle_dump(a), "MatrixMapBuilderC(\n    MatrixMapBuilder,\n)");

  // Create handle.
  dqcs_handle_t b = dqcs_mm_new(a);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that the builder handle is no longer OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_handle_dump(a), nullptr);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(a) + " is invalid");

  // Check that the b handle is OK.
  EXPECT_EQ(dqcs_handle_type(b), dqcs_handle_type_t::DQCS_HTYPE_MATRIX_MAP);
  EXPECT_STREQ(dqcs_handle_dump(b), "MatrixMapC(\n    MatrixMap,\n)");

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(b), dqcs_return_t::DQCS_SUCCESS);

  // Check that the handle is no longer OK.
  EXPECT_EQ(dqcs_handle_type(b), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_handle_dump(b), nullptr);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(b) + " is invalid");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

TEST(mm, cache) {
  dqcs_handle_t mmb = dqcs_mmb_new();
  dqcs_handle_t mm = dqcs_mm_new(mmb);
  EXPECT_EQ(dqcs_mm_clear_cache(mm), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_handle_delete_all(), dqcs_return_t::DQCS_SUCCESS);
}

// Builder check.
TEST(mm, check) {
  // Create handle.
  dqcs_handle_t mmb = dqcs_mmb_new();

  // Create key function
  int key_data = 42;
  int key_data_r = 43;

  // Add internals.
  EXPECT_EQ(dqcs_mmb_add_internal(mmb, nullptr, &key_data, dqcs_internal_gate_t::DQCS_GATE_PAULI_I, 0.00001, true), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_mmb_add_internal(mmb, nullptr, &key_data_r, dqcs_internal_gate_t::DQCS_GATE_R, 0.00001, true), dqcs_return_t::DQCS_SUCCESS);

  // Test check.
  dqcs_handle_t mm = dqcs_mm_new(mmb);

  {
    double i[] = {1.,0.,0.,0.,0.,0.,1.,0.};
    const void *output;
    dqcs_handle_t param;
    EXPECT_EQ(dqcs_mm_map_matrix(mm, i, 4, &output, &param), dqcs_bool_return_t::DQCS_TRUE);
    EXPECT_EQ(*(int*)output, 42);
    EXPECT_EQ(dqcs_handle_type(param), dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA);
  }
  {
    double i[] = {0.,1.,1.,0.,1.,0.,0.,0.};
    const void *output = (void *)1234;
    dqcs_handle_t param = 42;
    EXPECT_EQ(dqcs_mm_map_matrix(mm, i, 4, &output, &param), dqcs_bool_return_t::DQCS_FALSE);
    EXPECT_EQ(output, (void *)1234);
    EXPECT_EQ(param, 0);
    EXPECT_EQ(dqcs_handle_type(param), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  }
  {
    double i[] = {0.,0.,1.,0.,1.,0.,0.,0.};
    const void *output = (void *)1234;
    dqcs_handle_t param = 42;
    EXPECT_EQ(dqcs_mm_map_matrix(mm, i, 4, &output, &param), dqcs_bool_return_t::DQCS_TRUE);
    EXPECT_EQ(*(int*)output, 43);

    double theta, phi, lambda;
    EXPECT_EQ(dqcs_arb_get_raw(param, 0, &theta, sizeof(double)), sizeof(double));
    EXPECT_EQ(dqcs_arb_get_raw(param, 1, &phi, sizeof(double)), sizeof(double));
    EXPECT_EQ(dqcs_arb_get_raw(param, 2, &lambda, sizeof(double)), sizeof(double));
    EXPECT_EQ(theta, 3.14159265358979323846);
    EXPECT_EQ(phi, 0);
    EXPECT_EQ(lambda, 3.14159265358979323846);
  }
  {
    double i[] = {0.,0.,1.,0.,1.,0.,0.,0.};
    const void *output = (void *)1234;
    dqcs_handle_t param = 42;
    EXPECT_EQ(dqcs_mm_map_matrix(mm, i, 3, &output, &param), dqcs_bool_return_t::DQCS_BOOL_FAILURE);
    EXPECT_EQ(output, (void *)1234);
    EXPECT_EQ(param, 0);
    EXPECT_EQ(dqcs_error_get(), std::string("Invalid argument: invalid matrix size"));
  }
  {
    double i[] = {1.,0.};
    const void *output = (void *)1234;
    dqcs_handle_t param = 42;
    EXPECT_EQ(dqcs_mm_map_matrix(mm, i, 1, &output, &param), dqcs_bool_return_t::DQCS_BOOL_FAILURE);
    EXPECT_EQ(output, (void *)1234);
    EXPECT_EQ(param, 0);
    EXPECT_EQ(dqcs_error_get(), std::string("Invalid argument: invalid matrix size"));
  }

  EXPECT_EQ(dqcs_handle_delete_all(), dqcs_return_t::DQCS_SUCCESS);
}

// Mat compare
TEST(mm, mat_compare) {
  double i[] = {0.,0.,1.,0.,1.,0.,0.,0.};
  double j[] = {0.,0.,1.,0.,1.,0.,1.,1.};
  ASSERT_EQ(dqcs_mat_compare(i, j, 2, 0.0001, true), dqcs_bool_return_t::DQCS_BOOL_FAILURE);
  ASSERT_EQ(dqcs_mat_compare(i, j, 4, 0.0001, true), dqcs_bool_return_t::DQCS_FALSE);
  ASSERT_EQ(dqcs_mat_compare(i, i, 4, 0.0001, true), dqcs_bool_return_t::DQCS_TRUE);
  ASSERT_EQ(dqcs_mat_compare(i, i, 4, 0.001, false), dqcs_bool_return_t::DQCS_TRUE);
}

// Test gate
TEST(mm, gate) {
  // Create handle.
  dqcs_handle_t mmb = dqcs_mmb_new();

  // Create key function
  int key_data = 42;
  int key_data_r = 43;

  // Add internals.
  EXPECT_EQ(dqcs_mmb_add_internal(mmb, nullptr, &key_data, dqcs_internal_gate_t::DQCS_GATE_PAULI_I, 0.00001, true), dqcs_return_t::DQCS_SUCCESS);
  EXPECT_EQ(dqcs_mmb_add_internal(mmb, nullptr, &key_data_r, dqcs_internal_gate_t::DQCS_GATE_R, 0.00001, true), dqcs_return_t::DQCS_SUCCESS);

  // Test check.
  dqcs_handle_t mm = dqcs_mm_new(mmb);



  {
    double i[] = {1.,0.,0.,0.,0.,0.,1.,0.};
    const void *output;
    dqcs_handle_t param;
    dqcs_handle_t targets = dqcs_qbset_new();
    dqcs_qbset_push(targets, 1);
    dqcs_handle_t gate = dqcs_gate_new_unitary(
      targets,
      0,
      i,
      4
    );
    EXPECT_EQ(dqcs_mm_map_gate(mm, gate, &output, &param), dqcs_bool_return_t::DQCS_TRUE);
    EXPECT_EQ(*(int*)output, 42);
    EXPECT_EQ(dqcs_handle_type(param), dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA);
  }
  {
    double i[] = {0.,0.,1.,0.,1.,0.,0.,0.};
    const void *output = (void *)1234;
    dqcs_handle_t param = 42;
    dqcs_handle_t targets = dqcs_qbset_new();
    dqcs_qbset_push(targets, 1);
    dqcs_handle_t gate = dqcs_gate_new_unitary(
      targets,
      0,
      i,
      4
    );
    EXPECT_EQ(dqcs_mm_map_gate(mm, gate, &output, &param), dqcs_bool_return_t::DQCS_TRUE);
    EXPECT_EQ(*(int*)output, 43);

    double theta, phi, lambda;
    EXPECT_EQ(dqcs_arb_get_raw(param, 0, &theta, sizeof(double)), sizeof(double));
    EXPECT_EQ(dqcs_arb_get_raw(param, 1, &phi, sizeof(double)), sizeof(double));
    EXPECT_EQ(dqcs_arb_get_raw(param, 2, &lambda, sizeof(double)), sizeof(double));
    EXPECT_EQ(theta, 3.14159265358979323846);
    EXPECT_EQ(phi, 0);
    EXPECT_EQ(lambda, 3.14159265358979323846);
  }
  {
    double i[] = {0.,0.,1.,0.,1.,0.,0.,0.};
    const void *output = (void *)1234;
    dqcs_handle_t param = 42;
    EXPECT_EQ(dqcs_mm_map_gate(mm, 0, &output, &param), dqcs_bool_return_t::DQCS_BOOL_FAILURE);
    EXPECT_EQ(output, (void *)1234);
    EXPECT_EQ(param, 0);
    EXPECT_EQ(dqcs_error_get(), std::string("Invalid argument: handle 0 is invalid"));
  }

  EXPECT_EQ(dqcs_handle_delete_all(), dqcs_return_t::DQCS_SUCCESS);

}

TEST(mm, fixed) {
  dqcs_handle_t mmb = dqcs_mmb_new();
  int key_data = 42;
  double x[] = {0.,0.,1.,0.,1.,0.,0.,0.};
  ASSERT_EQ(dqcs_mmb_add_fixed(mmb, 0, &key_data, x, 4, 0.0001, false), dqcs_return_t::DQCS_SUCCESS);

  int key_data_2 = 43;
  ASSERT_EQ(dqcs_mmb_add_fixed(mmb, 0, &key_data_2, x, 4, 0.0001, true), dqcs_return_t::DQCS_SUCCESS);

  dqcs_handle_t mm = dqcs_mm_new(mmb);

  {
    double x[] = {0.,0.,1.,0.,1.,0.,0.,0.};
    const void *output;
    dqcs_handle_t param;
    EXPECT_EQ(dqcs_mm_map_matrix(mm, x, 4, &output, &param), dqcs_bool_return_t::DQCS_TRUE);
    EXPECT_EQ(*(int*)output, 42);
    EXPECT_EQ(dqcs_handle_type(param), dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA);
  }
  {
    double nx[] = {1.,0.,0.,0.,0.,0.,1.,0.};
    const void *output = (void*) 1234;
    dqcs_handle_t param = 1234;
    EXPECT_EQ(dqcs_mm_map_matrix(mm, nx, 4, &output, &param), dqcs_bool_return_t::DQCS_FALSE);
    EXPECT_EQ(output, (void*)1234);
    EXPECT_EQ(param, 0);
  }
  {
    double x[] = {0.,0.,-1.,0.,-1.,0.,0.,0.};
    const void *output;
    dqcs_handle_t param;
    EXPECT_EQ(dqcs_mm_map_matrix(mm, x, 4, &output, &param), dqcs_bool_return_t::DQCS_TRUE);
    EXPECT_EQ(*(int*)output, 43);
    EXPECT_EQ(dqcs_handle_type(param), dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA);
  }
  {
    double x[] = {0.,0.,-0.99998,0.,-1.,0.,0.,0.};
    const void *output;
    dqcs_handle_t param;
    EXPECT_EQ(dqcs_mm_map_matrix(mm, x, 4, &output, &param), dqcs_bool_return_t::DQCS_TRUE);
    EXPECT_EQ(*(int*)output, 43);
    EXPECT_EQ(dqcs_handle_type(param), dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA);
  }
  {
    double x[] = {0.,0.,-0.9997,0.,-1.,0.,0.,0.};
    const void *output = (void*) 1234;
    dqcs_handle_t param = 1234;
    EXPECT_EQ(dqcs_mm_map_matrix(mm, x, 4, &output, &param), dqcs_bool_return_t::DQCS_FALSE);
    EXPECT_EQ(output, (void*)1234);
    EXPECT_EQ(param, 0);
  }

}

dqcs_bool_return_t detector_true(const void *user_data, const double *matrix, size_t matrix_len, dqcs_handle_t *param_data) {
  return dqcs_bool_return_t::DQCS_TRUE;
}


TEST(mm, user) {
  dqcs_handle_t mmb = dqcs_mmb_new();

  int key_data = 43;
  EXPECT_EQ(dqcs_mmb_add_user(mmb, 0, &key_data, detector_true, 0, 0), dqcs_return_t::DQCS_SUCCESS);

  dqcs_handle_t mm = dqcs_mm_new(mmb);

  double x[] = {0.,0.,1.,0.,1.,0.,0.,0.};
  const void *output;
  dqcs_handle_t param;
  EXPECT_EQ(dqcs_mm_map_matrix(mm, x, 4, &output, &param), dqcs_bool_return_t::DQCS_TRUE);
  EXPECT_EQ(*(int*)output, 43);
  EXPECT_EQ(dqcs_handle_type(param), dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA);

}
