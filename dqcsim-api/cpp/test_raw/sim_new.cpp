#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"
#include "util.h"

using namespace dqcsim;

// Sanity check the simulator constructor.
TEST(sim_new, sanity) {
  dqcs_handle_t a, b;

  // Create handle.
  a = dqcs_scfg_new();
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  b = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c");
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  b = dqcs_tcfg_new(b, "d");
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_push_plugin(a, b), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  b = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, "e", "f", "g");
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  b = dqcs_tcfg_new(b, "h");
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_push_plugin(a, b), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  a = dqcs_sim_new(a);
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  // Check that the handle is OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_SIM);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Check that the handle is no longer OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_handle_dump(a), nullptr);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(a) + " is invalid");

  // Leak check.
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

#define QUAD_PLUGIN(a_typ, e_typ, i_typ, m_typ, auto_name) \
  a = dqcs_scfg_new(); \
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get(); \
  b = dqcs_pdef_new(dqcs_plugin_type_t::a_typ, "a", "b", "c"); \
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get(); \
  b = dqcs_tcfg_new(b, auto_name ? NULL : "d"); \
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_scfg_push_plugin(a, b), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get(); \
  b = dqcs_pdef_new(dqcs_plugin_type_t::e_typ, "e", "f", "g"); \
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get(); \
  b = dqcs_tcfg_new(b, auto_name ? NULL : "h"); \
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_scfg_push_plugin(a, b), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get(); \
  b = dqcs_pdef_new(dqcs_plugin_type_t::i_typ, "i", "j", "k"); \
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get(); \
  b = dqcs_tcfg_new(b, auto_name ? NULL : "l"); \
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_scfg_push_plugin(a, b), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get(); \
  b = dqcs_pdef_new(dqcs_plugin_type_t::m_typ, "m", "n", "o"); \
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get(); \
  b = dqcs_tcfg_new(b, auto_name ? NULL : "p"); \
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get(); \
  ASSERT_EQ(dqcs_scfg_push_plugin(a, b), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get(); \
  a = dqcs_sim_new(a)

// Different order and with operators. Check plugin metadata getters
// extensively.
TEST(sim_new, reordered) {
  char *s;
  dqcs_handle_t a, b;
  QUAD_PLUGIN(
    DQCS_PTYPE_BACK,
    DQCS_PTYPE_OPER,
    DQCS_PTYPE_FRONT,
    DQCS_PTYPE_OPER,
    false
  );
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  ASSERT_STREQ(s = dqcs_sim_get_name(a, "d"), "a"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_author(a, "d"), "b"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_version(a, "d"), "c"); if (s) free(s);

  ASSERT_STREQ(s = dqcs_sim_get_name(a, "h"), "e"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_author(a, "h"), "f"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_version(a, "h"), "g"); if (s) free(s);

  ASSERT_STREQ(s = dqcs_sim_get_name(a, "l"), "i"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_author(a, "l"), "j"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_version(a, "l"), "k"); if (s) free(s);

  ASSERT_STREQ(s = dqcs_sim_get_name(a, "p"), "m"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_author(a, "p"), "n"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_version(a, "p"), "o"); if (s) free(s);

  ASSERT_STREQ(s = dqcs_sim_get_name(a, "x"), NULL); if (s) free(s);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: plugin x not found");
  ASSERT_STREQ(s = dqcs_sim_get_author(a, "y"), NULL); if (s) free(s);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: plugin y not found");
  ASSERT_STREQ(s = dqcs_sim_get_version(a, "z"), NULL); if (s) free(s);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: plugin z not found");

  ASSERT_STREQ(s = dqcs_sim_get_name(a, NULL), NULL); if (s) free(s);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: unexpected NULL string");
  ASSERT_STREQ(s = dqcs_sim_get_author(a, NULL), NULL); if (s) free(s);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: unexpected NULL string");
  ASSERT_STREQ(s = dqcs_sim_get_version(a, NULL), NULL); if (s) free(s);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: unexpected NULL string");

  ASSERT_STREQ(s = dqcs_sim_get_name_idx(a, -1), "a"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_author_idx(a, -1), "b"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_version_idx(a, -1), "c"); if (s) free(s);

  ASSERT_STREQ(s = dqcs_sim_get_name_idx(a, 1), "e"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_author_idx(a, 1), "f"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_version_idx(a, 1), "g"); if (s) free(s);

  ASSERT_STREQ(s = dqcs_sim_get_name_idx(a, 0), "i"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_author_idx(a, 0), "j"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_version_idx(a, 0), "k"); if (s) free(s);

  ASSERT_STREQ(s = dqcs_sim_get_name_idx(a, 2), "m"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_author_idx(a, 2), "n"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_version_idx(a, 2), "o"); if (s) free(s);

  ASSERT_STREQ(s = dqcs_sim_get_name_idx(a, 3), "a"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_author_idx(a, 3), "b"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_version_idx(a, 3), "c"); if (s) free(s);

  ASSERT_STREQ(s = dqcs_sim_get_name_idx(a, 4), NULL); if (s) free(s);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index 4 out of range");
  ASSERT_STREQ(s = dqcs_sim_get_author_idx(a, 4), NULL); if (s) free(s);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index 4 out of range");
  ASSERT_STREQ(s = dqcs_sim_get_version_idx(a, 4), NULL); if (s) free(s);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index 4 out of range");

  ASSERT_STREQ(s = dqcs_sim_get_name_idx(a, -5), NULL); if (s) free(s);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index -5 out of range");
  ASSERT_STREQ(s = dqcs_sim_get_author_idx(a, -5), NULL); if (s) free(s);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index -5 out of range");
  ASSERT_STREQ(s = dqcs_sim_get_version_idx(a, -5), NULL); if (s) free(s);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index -5 out of range");

  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Check plugin auto-naming.
TEST(sim_new, auto_naming) {
  char *s;
  dqcs_handle_t a, b;
  QUAD_PLUGIN(
    DQCS_PTYPE_OPER,
    DQCS_PTYPE_BACK,
    DQCS_PTYPE_OPER,
    DQCS_PTYPE_FRONT,
    true
  );
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  ASSERT_STREQ(s = dqcs_sim_get_name(a, "front"), "m"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_name(a, "op1"), "a"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_name(a, "op2"), "i"); if (s) free(s);
  ASSERT_STREQ(s = dqcs_sim_get_name(a, "back"), "e"); if (s) free(s);

  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Check missing frontend.
TEST(sim_new, missing_frontend) {
  dqcs_handle_t a, b;
  QUAD_PLUGIN(
    DQCS_PTYPE_OPER,
    DQCS_PTYPE_OPER,
    DQCS_PTYPE_BACK,
    DQCS_PTYPE_OPER,
    true
  );
  ASSERT_EQ(a, 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: missing frontend");
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Check missing backend.
TEST(sim_new, missing_backend) {
  dqcs_handle_t a, b;
  QUAD_PLUGIN(
    DQCS_PTYPE_OPER,
    DQCS_PTYPE_FRONT,
    DQCS_PTYPE_OPER,
    DQCS_PTYPE_OPER,
    true
  );
  ASSERT_EQ(a, 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: missing backend");
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Check duplicate frontend.
TEST(sim_new, duplicate_frontend) {
  dqcs_handle_t a, b;
  QUAD_PLUGIN(
    DQCS_PTYPE_FRONT,
    DQCS_PTYPE_OPER,
    DQCS_PTYPE_BACK,
    DQCS_PTYPE_FRONT,
    true
  );
  ASSERT_EQ(a, 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: duplicate frontend");
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Check duplicate backend.
TEST(sim_new, duplicate_backend) {
  dqcs_handle_t a, b;
  QUAD_PLUGIN(
    DQCS_PTYPE_BACK,
    DQCS_PTYPE_FRONT,
    DQCS_PTYPE_OPER,
    DQCS_PTYPE_BACK,
    true
  );
  ASSERT_EQ(a, 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: duplicate backend");
  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

// Check duplicate name.
TEST(sim_new, duplicate_name) {
  dqcs_handle_t a, b;

  a = dqcs_scfg_new();
  ASSERT_NE(a, 0u) << "Unexpected error: " << dqcs_error_get();

  b = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_FRONT, "a", "b", "c");
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  b = dqcs_tcfg_new(b, "a");
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_push_plugin(a, b), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  b = dqcs_pdef_new(dqcs_plugin_type_t::DQCS_PTYPE_BACK, "e", "f", "g");
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  b = dqcs_tcfg_new(b, "a");
  ASSERT_NE(b, 0u) << "Unexpected error: " << dqcs_error_get();
  ASSERT_EQ(dqcs_scfg_push_plugin(a, b), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  a = dqcs_sim_new(a);
  ASSERT_EQ(a, 0u);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: duplicate plugin name 'a'");

  EXPECT_EQ(dqcs_handle_leak_check(), dqcs_return_t::DQCS_SUCCESS) << dqcs_error_get();
}

