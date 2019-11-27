#include <dqcsim>
#include "util.hpp"
#include "gtest/gtest.h"

using namespace dqcsim;

// Test JSON access by means of JSON strings.
TEST(arb, json) {
  wrap::Arb data;
  EXPECT_EQ(data.get_json(), "{}");
  data.set_json("{\"hello\": \"world\"}");
  EXPECT_ERROR(data.set_json("invalid JSON"), "Invalid argument: expected value at line 1 column 1");
  EXPECT_EQ(data.get_json(), "{\"hello\":\"world\"}");
  EXPECT_EQ(data.get_cbor(), "\xBF\x65\x68\x65\x6C\x6C\x6F\x65\x77\x6F\x72\x6C\x64\xFF");
}

// Test JSON access by means of CBOR objects.
TEST(arb, cbor) {
  wrap::Arb data;
  EXPECT_EQ(data.get_cbor(), "\xBF\xFF");
  data.set_cbor("\xBF\x65\x68\x65\x6C\x6C\x6F\x65\x77\x6F\x72\x6C\x64\xFF");
  EXPECT_ERROR(data.set_cbor("\xFF"), "Invalid argument: unexpected code at offset 1");
  EXPECT_EQ(data.get_json(), "{\"hello\":\"world\"}");
  EXPECT_EQ(data.get_cbor(), "\xBF\x65\x68\x65\x6C\x6C\x6F\x65\x77\x6F\x72\x6C\x64\xFF");
}

// Tests the unstructured string list accessors, as well as the copy assignment
// operator and copy constructor.
TEST(arb, unstructured) {
  wrap::Arb data;
  EXPECT_ERROR(data.get_string(0), "Invalid argument: index out of range: 0");
  data.push_string("hello");
  data.push_string("world");
  EXPECT_EQ(data.get_string(0), "hello");
  EXPECT_EQ(data.get_string(-1), "world");
  EXPECT_EQ(data.pop_string(), "world");
  EXPECT_EQ(data.get_string(-1), "hello");
  data.remove_string(-1);
  EXPECT_EQ(data.get_string_count(), 0);

  data.insert_string(0, "A");
  data.insert_string(-1, "C");
  data.insert_string(1, "B");
  EXPECT_EQ(data.get_string_count(), 3);
  EXPECT_EQ(data.get_string(0), "A");
  EXPECT_EQ(data.get_string(1), "B");
  EXPECT_EQ(data.get_string(2), "C");
  EXPECT_ERROR(data.get_string(3), "Invalid argument: index out of range: 3");

  wrap::Arb data2(data);
  data.clear_strings();
  EXPECT_EQ(data.get_string_count(), 0);
  data = data2;
  data.push_string("D");
  data2.push_string("E");
  EXPECT_EQ(data.get_string(3), "D");
  EXPECT_EQ(data2.get_string(3), "E");
}
