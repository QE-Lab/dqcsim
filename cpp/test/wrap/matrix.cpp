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
  wrap::Matrix a = wrap::Matrix::identity(2);
  EXPECT_EQ(to_string(a), "{[1, 0], [0, 1]}");
  a(1, 1) = wrap::complex(-1.0, 0.0);
  EXPECT_EQ(to_string(a), "{[1, 0], [0, -1]}");
  a(0, 0) = wrap::complex(0.0, 1.0);
  a(0, 1) = wrap::complex(0.0, -1.0);
  a(1, 0) = wrap::complex(-1.0, 1.0);
  a(1, 1) = wrap::complex(-1.0, -1.0);
  EXPECT_INV_ARG(a(2, 1) = wrap::complex(-1.0, -1.0), "matrix subscript out of bounds");
  EXPECT_INV_ARG(a(1, 2) = wrap::complex(-1.0, -1.0), "matrix subscript out of bounds");
  EXPECT_INV_ARG(a(2, 1), "matrix subscript out of bounds");
  EXPECT_INV_ARG(a(1, 2), "matrix subscript out of bounds");
  EXPECT_EQ(to_string(a), "{[1i, -1i], [-1+1i, -1-1i]}");
  EXPECT_EQ(a.n(), 2);
  EXPECT_EQ(a.size(), 4);
  EXPECT_INV_ARG(wrap::Matrix(a.n(), a.data()), "size must be a square number");
  wrap::Matrix b(a.size(), a.data());
  EXPECT_EQ(to_string(b), "{[1i, -1i], [-1+1i, -1-1i]}");
  EXPECT_TRUE(a == b);
  EXPECT_FALSE(a != b);
  b(1, 0) = wrap::complex(3.14, 0.0);
  EXPECT_TRUE(a != b);
  EXPECT_FALSE(a == b);
  EXPECT_EQ(b(1, 0), wrap::complex(3.14, 0.0));
  EXPECT_EQ(to_string(b), "{[1i, -1i], [3.14, -1-1i]}");
  double test[8] = {0.0, 1.0, 0.0, -1.0, 3.14, 0.0, -1.0, -1.0};
  EXPECT_EQ(std::memcmp(b.data(), test, sizeof(test)), 0);
}
