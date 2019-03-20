#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"

TEST(Test, Test) {
  ASSERT_STREQ("Test", "Test");
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
