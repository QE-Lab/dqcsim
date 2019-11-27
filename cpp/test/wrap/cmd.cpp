#include <dqcsim>
#include "util.hpp"
#include "gtest/gtest.h"

using namespace dqcsim;

// Test ArbCmd objects.
TEST(arb, json) {
  wrap::ArbCmd cmd("a", "b");
  EXPECT_EQ(cmd.get_iface(), "a");
  EXPECT_EQ(cmd.is_iface("a"), true);
  EXPECT_EQ(cmd.is_iface("b"), false);
  EXPECT_EQ(cmd.get_oper(), "b");
  EXPECT_EQ(cmd.is_oper("a"), false);
  EXPECT_EQ(cmd.is_oper("b"), true);
}
