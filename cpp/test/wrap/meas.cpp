#include <dqcsim>
#include "util.hpp"
#include "gtest/gtest.h"

using namespace dqcsim;

// Test measurement objects.
TEST(meas, test) {
  wrap::Measurement a(wrap::QubitRef(33), wrap::MeasurementValue::One);
  a.set_arb_cbor_string("\xBF\x65\x68\x65\x6C\x6C\x6F\x65\x77\x6F\x72\x6C\x64\xFF");

  EXPECT_EQ(a.get_qubit().get_index(), 33);
  EXPECT_EQ(a.get_value(), wrap::MeasurementValue::One);
  EXPECT_EQ(a.get_arb_cbor_string(), "\xBF\x65\x68\x65\x6C\x6C\x6F\x65\x77\x6F\x72\x6C\x64\xFF");

  wrap::Measurement b(a);
  EXPECT_EQ(b.get_qubit().get_index(), 33);
  EXPECT_EQ(b.get_value(), wrap::MeasurementValue::One);
  EXPECT_EQ(b.get_arb_cbor_string(), "\xBF\x65\x68\x65\x6C\x6C\x6F\x65\x77\x6F\x72\x6C\x64\xFF");

  b.set_qubit(wrap::QubitRef(25));
  b.set_value(wrap::MeasurementValue::Zero);
  EXPECT_EQ(b.get_qubit().get_index(), 25);
  EXPECT_EQ(b.get_value(), wrap::MeasurementValue::Zero);

  b = a;
  EXPECT_EQ(b.get_qubit().get_index(), 33);
  EXPECT_EQ(b.get_value(), wrap::MeasurementValue::One);
  EXPECT_EQ(b.get_arb_cbor_string(), "\xBF\x65\x68\x65\x6C\x6C\x6F\x65\x77\x6F\x72\x6C\x64\xFF");
}
