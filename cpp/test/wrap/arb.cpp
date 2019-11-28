#include <dqcsim>
#include "util.hpp"
#include "gtest/gtest.h"
#include "../json.hpp"

using namespace dqcsim;
using json = nlohmann::json;

// Test JSON access by means of JSON strings.
TEST(arb, json) {
  wrap::ArbData data;
  EXPECT_EQ(data.get_arb_json_string(), "{}");
  data.set_arb_json_string("{\"hello\": \"world\"}");
  EXPECT_ERROR(data.set_arb_json_string("invalid JSON"), "Invalid argument: expected value at line 1 column 1");
  EXPECT_EQ(data.get_arb_json_string(), "{\"hello\":\"world\"}");
  EXPECT_EQ(data.get_arb_cbor_string(), "\xBF\x65\x68\x65\x6C\x6C\x6F\x65\x77\x6F\x72\x6C\x64\xFF");
}

// Test JSON access by means of CBOR objects.
TEST(arb, cbor) {
  wrap::ArbData data;
  EXPECT_EQ(data.get_arb_cbor_string(), "\xBF\xFF");
  data.set_arb_cbor_string("\xBF\x65\x68\x65\x6C\x6C\x6F\x65\x77\x6F\x72\x6C\x64\xFF");
  EXPECT_ERROR(data.set_arb_cbor_string("\xFF"), "Invalid argument: unexpected code at offset 1");
  EXPECT_EQ(data.get_arb_json_string(), "{\"hello\":\"world\"}");
  EXPECT_EQ(data.get_arb_cbor_string(), "\xBF\x65\x68\x65\x6C\x6C\x6F\x65\x77\x6F\x72\x6C\x64\xFF");
}

// Test JSON access by means of `nlohmann::json` objects.
TEST(arb, nlohmann_json) {
  wrap::ArbData data;
  EXPECT_EQ(data.get_arb_json<json>(), "{}"_json);
  data.set_arb_json("{\"hello\": \"world\"}"_json);
  EXPECT_EQ(data.get_arb_json<json>(), "{\"hello\": \"world\"}"_json);
  EXPECT_EQ(data.get_arb_json_string(), "{\"hello\":\"world\"}");
}

// Tests the unstructured string list accessors, as well as the copy assignment
// operator and copy constructor.
TEST(arb, unstructured) {
  wrap::ArbData data;
  EXPECT_ERROR(data.get_arb_arg_string(0), "Invalid argument: index out of range: 0");
  data.push_arb_arg_string("hello");
  data.push_arb_arg_string("world");
  EXPECT_EQ(data.get_arb_arg_string(0), "hello");
  EXPECT_EQ(data.get_arb_arg_string(-1), "world");
  EXPECT_EQ(data.pop_arb_arg_string(), "world");
  EXPECT_EQ(data.get_arb_arg_string(-1), "hello");
  data.remove_arb_arg(-1);
  EXPECT_EQ(data.get_arb_arg_count(), 0);

  data.insert_arb_arg_string(0, "A");
  data.insert_arb_arg_string(-1, "C");
  data.insert_arb_arg_string(1, "B");
  EXPECT_EQ(data.get_arb_arg_count(), 3);
  EXPECT_EQ(data.get_arb_arg_string(0), "A");
  EXPECT_EQ(data.get_arb_arg_string(1), "B");
  EXPECT_EQ(data.get_arb_arg_string(2), "C");
  EXPECT_ERROR(data.get_arb_arg_string(3), "Invalid argument: index out of range: 3");

  wrap::ArbData data2(data);
  data.clear_arb_args();
  EXPECT_EQ(data.get_arb_arg_count(), 0);
  data = data2;
  data.push_arb_arg_string("D");
  data2.push_arb_arg_string("E");
  EXPECT_EQ(data.get_arb_arg_string(3), "D");
  EXPECT_EQ(data2.get_arb_arg_string(3), "E");
}
