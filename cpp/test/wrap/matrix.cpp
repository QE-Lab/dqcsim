#include <dqcsim>
#include <sstream>
#include <cstring>
#include "util.hpp"
#include "gtest/gtest.h"

using namespace dqcsim;

static std::string to_string(const wrap::Matrix &matrix) {
    std::stringstream ss;
    ss << matrix;
    return ss.str();
}

// Test matrices.
TEST(matrix, test) {
  wrap::Matrix a(2);
  EXPECT_EQ(to_string(a), "{[1, 0], [0, 1]}");
  a(1, 1) = std::complex<double>(-1.0, 0.0);
  EXPECT_EQ(to_string(a), "{[1, 0], [0, -1]}");
  a(0, 0) = std::complex<double>(0.0, 1.0);
  a(0, 1) = std::complex<double>(0.0, -1.0);
  a(1, 0) = std::complex<double>(-1.0, 1.0);
  a(1, 1) = std::complex<double>(-1.0, -1.0);
  EXPECT_EQ(to_string(a), "{[1i, -1i], [-1+1i, -1-1i]}");
  EXPECT_EQ(a.size(), 2);
  wrap::Matrix b(2, a.data());
  EXPECT_EQ(to_string(b), "{[1i, -1i], [-1+1i, -1-1i]}");
  EXPECT_TRUE(a == b);
  EXPECT_FALSE(a != b);
  b(1, 0) = std::complex<double>(3.14, 0.0);
  EXPECT_TRUE(a != b);
  EXPECT_FALSE(a == b);
  EXPECT_EQ(b(1, 0), std::complex<double>(3.14, 0.0));
  EXPECT_EQ(to_string(b), "{[1i, -1i], [3.14, -1-1i]}");
  double test[8] = {0.0, 1.0, 0.0, -1.0, 3.14, 0.0, -1.0, -1.0};
  EXPECT_EQ(std::memcmp(b.data(), test, sizeof(test)), 0);
}
