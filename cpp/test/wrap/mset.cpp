#include <dqcsim>
#include <algorithm>
#include "util.hpp"
#include "gtest/gtest.h"

using namespace dqcsim;

static bool sort_fn(const wrap::Measurement &i, const wrap::Measurement &j) {
  return i.get_qubit().get_index() < j.get_qubit().get_index();
}

// Test measurement set objects.
TEST(mset, test) {
  wrap::MeasurementSet mset;
  EXPECT_EQ(mset.size(), 0);

  EXPECT_FALSE(mset.contains(wrap::QubitRef(33)));
  {
    wrap::Measurement meas(wrap::QubitRef(33), wrap::MeasurementValue::One);
    mset.set(meas);
    EXPECT_TRUE(meas.is_valid());
  }
  EXPECT_TRUE(mset.contains(wrap::QubitRef(33)));
  EXPECT_EQ(mset.size(), 1);

  EXPECT_FALSE(mset.contains(wrap::QubitRef(25)));
  {
    wrap::Measurement meas(wrap::QubitRef(25), wrap::MeasurementValue::Undefined);
    mset.set(std::move(meas));
    EXPECT_FALSE(meas.is_valid());
  }
  EXPECT_TRUE(mset.contains(wrap::QubitRef(25)));
  EXPECT_EQ(mset.size(), 2);

  EXPECT_EQ(mset.get(wrap::QubitRef(33)).get_value(), wrap::MeasurementValue::One);
  EXPECT_EQ(mset.get(wrap::QubitRef(25)).get_value(), wrap::MeasurementValue::Undefined);
  EXPECT_ERROR(mset.get(wrap::QubitRef(42)), "Invalid argument: qubit not included in measurement set");

  std::vector<wrap::Measurement> vector = mset.copy_into_vector();
  EXPECT_EQ(mset.size(), 2);
  std::sort(vector.begin(), vector.end(), sort_fn);
  EXPECT_EQ(vector.size(), 2);
  EXPECT_EQ(vector[0].get_qubit().get_index(), 25);
  EXPECT_EQ(vector[0].get_value(), wrap::MeasurementValue::Undefined);
  EXPECT_EQ(vector[1].get_qubit().get_index(), 33);
  EXPECT_EQ(vector[1].get_value(), wrap::MeasurementValue::One);

  wrap::MeasurementSet mset_b(mset);
  mset.remove(25);
  EXPECT_ERROR(mset.remove(22), "Invalid argument: qubit not included in measurement set");
  EXPECT_EQ(mset.size(), 1);
  EXPECT_EQ(mset_b.size(), 2);

}
