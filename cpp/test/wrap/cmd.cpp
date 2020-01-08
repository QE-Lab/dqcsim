#include <dqcsim>
#include "util.hpp"
#include "gtest/gtest.h"

using namespace dqcsim;

// Basic tests for `ArbCmd` objects.
TEST(cmd, basic) {
  wrap::ArbCmd cmd("a", "b");
  EXPECT_EQ(cmd.get_iface(), "a");
  EXPECT_EQ(cmd.is_iface("a"), true);
  EXPECT_EQ(cmd.is_iface("b"), false);
  EXPECT_EQ(cmd.get_oper(), "b");
  EXPECT_EQ(cmd.is_oper("a"), false);
  EXPECT_EQ(cmd.is_oper("b"), true);
}

// Test `ArbCmd` copy operations.
TEST(cmd, copy) {
  {
    wrap::ArbCmd cmd_x("a", "b");
    cmd_x.set_arb_json_string("{\"hello\": \"world\"}");

    wrap::ArbCmd cmd_y("c", "d");
    cmd_y.set_arb_json_string("{\"another\": \"value\"}");

    std::string a_b = cmd_x.dump();
    cmd_y = cmd_x;
    EXPECT_EQ(cmd_x.dump(), a_b);
    EXPECT_EQ(cmd_y.dump(), a_b);
  }
  wrap::check(raw::dqcs_handle_leak_check());
}
