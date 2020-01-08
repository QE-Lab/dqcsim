#include <dqcsim>
#include <sstream>
#include "util.hpp"
#include "gtest/gtest.h"

using namespace dqcsim;

// Tests for qubit reference objects.
TEST(qubit, ref) {
  wrap::QubitRef qa(1);
  wrap::QubitRef qb(1);
  wrap::QubitRef qc(2);

  EXPECT_EQ(qa.get_index(), 1);
  EXPECT_EQ(qb.get_index(), 1);
  EXPECT_EQ(qc.get_index(), 2);

  EXPECT_TRUE(qa == qb);
  EXPECT_FALSE(qa != qb);
  EXPECT_FALSE(qa == qc);
  EXPECT_TRUE(qa != qc);

  qa = qc;

  EXPECT_FALSE(qa == qb);
  EXPECT_TRUE(qa != qb);
  EXPECT_TRUE(qa == qc);
  EXPECT_FALSE(qa != qc);

  std::stringstream ss;
  ss << qa << qb << qc;
  EXPECT_EQ(ss.str(), "q2q1q2");
}

// Tests for qubit set objects.
TEST(qubit, set) {
  wrap::QubitSet qs;
  EXPECT_EQ(qs.size(), 0);
  qs = wrap::QubitSet::from_iter(std::vector<wrap::QubitRef>({3, 1, 4, 15, 9, 2, 6, 5}));
  EXPECT_EQ(qs.size(), 8);
  EXPECT_TRUE(qs.contains(wrap::QubitRef(1)));
  EXPECT_TRUE(qs.contains(wrap::QubitRef(2)));
  EXPECT_TRUE(qs.contains(wrap::QubitRef(3)));
  EXPECT_TRUE(qs.contains(wrap::QubitRef(4)));
  EXPECT_TRUE(qs.contains(wrap::QubitRef(5)));
  EXPECT_TRUE(qs.contains(wrap::QubitRef(6)));
  EXPECT_FALSE(qs.contains(wrap::QubitRef(7)));
  EXPECT_FALSE(qs.contains(wrap::QubitRef(8)));
  EXPECT_TRUE(qs.contains(wrap::QubitRef(9)));
  EXPECT_ERROR(qs.push(wrap::QubitRef(3)), "Invalid argument: the specified qubit is already part of the set");
  qs.push(wrap::QubitRef(35));
  EXPECT_EQ(qs.pop(), wrap::QubitRef(3));
  EXPECT_EQ(qs.pop(), wrap::QubitRef(1));
  EXPECT_EQ(qs.copy_into_vector(), std::vector<wrap::QubitRef>({4, 15, 9, 2, 6, 5, 35}));
  EXPECT_EQ(qs.drain_into_vector(), std::vector<wrap::QubitRef>({4, 15, 9, 2, 6, 5, 35}));
  EXPECT_EQ(qs.drain_into_vector(), std::vector<wrap::QubitRef>());
}
