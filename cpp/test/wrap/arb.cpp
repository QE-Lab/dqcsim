#include <dqcsim>
#include "util.hpp"
#include "gtest/gtest.h"
#include <nlohmann/json.hpp>

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

// Tests the arbitrary argument list string accessors, as well as the copy
// assignment operator and copy constructor.
TEST(arb, arg_string) {
  wrap::ArbData data;
  EXPECT_ERROR(data.get_arb_arg_string(0), "Invalid argument: index out of range: 0");
  data.set_arb_arg_strings(std::vector<std::string>({"hello", "world"}));
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

struct Numbers {
  int x[6];
};

// Tests the arbitrary argument list value accessors.
TEST(arb, arg_value) {
  wrap::ArbData data;
  EXPECT_ERROR(data.get_arb_arg_as<int>(0), "Invalid argument: index out of range: 0");
  data.push_arb_arg(33);
  data.push_arb_arg(42.0f);
  data.push_arb_arg(3.14159265);
  EXPECT_EQ(data.get_arb_arg_as<int>(0), 33);
  EXPECT_EQ(data.get_arb_arg_as<float>(1), 42.0f);
  EXPECT_EQ(data.get_arb_arg_as<double>(2), 3.14159265);
  EXPECT_ERROR(data.get_arb_arg_as<float>(2), "Arbitrary argument has incorrect size: found 8 bytes, expected 4 bytes");
  EXPECT_ERROR(data.pop_arb_arg_as<float>(), "Arbitrary argument has incorrect size: found 8 bytes, expected 4 bytes");
  data.set_arb_arg(2, 3.14159265f);
  const Numbers numbers = {{4, 8, 15, 16, 23, 42}};
  data.insert_arb_arg(1, numbers);
  EXPECT_EQ(data.pop_arb_arg_as<float>(), 3.14159265f);
  EXPECT_EQ(data.pop_arb_arg_as<float>(), 42.0f);
  EXPECT_EQ(data.pop_arb_arg_as<Numbers>().x[5], numbers.x[5]);
  EXPECT_EQ(data.pop_arb_arg_as<int>(), 33);
}
