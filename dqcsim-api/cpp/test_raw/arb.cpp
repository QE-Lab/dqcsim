#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"

using namespace dqcsim;

// Sanity check the handle API.
TEST(handle, sanity) {
  // Create handle.
  dqcs_handle_t a = dqcs_arb_new();
  ASSERT_NE(a, 0) << "Unexpected error: " << dqcs_error_get();

  // Check that the handle is OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA);
  EXPECT_STREQ(dqcs_handle_dump(a), "ArbData(\n    ArbData {\n        json: Object(\n            {}\n        ),\n        args: []\n    }\n)");

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Check that the handle is no longer OK.
  EXPECT_EQ(dqcs_handle_type(a), dqcs_handle_type_t::DQCS_HTYPE_INVALID);
  EXPECT_STREQ(dqcs_handle_dump(a), nullptr);
  EXPECT_EQ(dqcs_error_get(), "Invalid argument: handle " + std::to_string(a) + " is invalid");
}

// Test JSON access by means of JSON strings.
TEST(json, string) {
  unsigned char cbor_buffer[256];

  // Create handle.
  dqcs_handle_t a = dqcs_arb_new();
  ASSERT_NE(a, 0) << "Unexpected error: " << dqcs_error_get();

  // Check default.
  EXPECT_STREQ(dqcs_arb_json_get(a), "{}");

  // Check proper object.
  EXPECT_EQ(dqcs_arb_json_set(a, "{\"hello\": \"world\"}"), dqcs_return_t::DQCS_SUCCESS);

  // Check proper object but wrong handle.
  EXPECT_EQ(dqcs_arb_json_set(0, "{\"hello\": \"world\"}"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 0 is invalid");

  // Check improper object.
  EXPECT_EQ(dqcs_arb_json_set(a, "invalid JSON"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: expected value at line 1 column 1");

  // Check that the ArbData object is what we expect.
  EXPECT_STREQ(dqcs_handle_dump(a), "ArbData(\n    ArbData {\n        json: Object(\n            {\n                String(\n                    \"hello\"\n                ): String(\n                    \"world\"\n                )\n            }\n        ),\n        args: []\n    }\n)");
  EXPECT_STREQ(dqcs_arb_json_get(a), "{\"hello\":\"world\"}");
  EXPECT_EQ(dqcs_arb_cbor_get(a, cbor_buffer, 256), 14);
//   for (int i = 0; i < 14; i++) {
//     fprintf(stderr, "\\x%02X", (unsigned int)cbor_buffer[i]);
//   }
  EXPECT_EQ(memcmp(cbor_buffer, "\xBF\x65\x68\x65\x6C\x6C\x6F\x65\x77\x6F\x72\x6C\x64\xFF", 14), 0);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);
}

// Test JSON access by means of CBOR objects.
TEST(json, cbor) {
  unsigned char cbor_buffer[256];

  // Create handle.
  dqcs_handle_t a = dqcs_arb_new();
  ASSERT_NE(a, 0) << "Unexpected error: " << dqcs_error_get();

  // Check default.
  EXPECT_EQ(dqcs_arb_cbor_get(a, cbor_buffer, 256), 2);
  EXPECT_EQ(memcmp(cbor_buffer, "\xBF\xFF", 2), 0);

  // Check proper object.
  EXPECT_EQ(dqcs_arb_cbor_set(a, "\xBF\x65\x68\x65\x6C\x6C\x6F\x65\x77\x6F\x72\x6C\x64\xFF", 14), dqcs_return_t::DQCS_SUCCESS);

  // Check proper object but wrong handle.
  EXPECT_EQ(dqcs_arb_cbor_set(0, "\xBF\x65\x68\x65\x6C\x6C\x6F\x65\x77\x6F\x72\x6C\x64\xFF", 14), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: handle 0 is invalid");

  // Check improper object.
  EXPECT_EQ(dqcs_arb_cbor_set(a, "\xFF", 1), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: unexpected code at offset 1");

  // Check that the ArbData object is what we expect.
  EXPECT_STREQ(dqcs_handle_dump(a), "ArbData(\n    ArbData {\n        json: Object(\n            {\n                String(\n                    \"hello\"\n                ): String(\n                    \"world\"\n                )\n            }\n        ),\n        args: []\n    }\n)");
  EXPECT_STREQ(dqcs_arb_json_get(a), "{\"hello\":\"world\"}");
  EXPECT_EQ(dqcs_arb_cbor_get(a, cbor_buffer, 256), 14);
  EXPECT_EQ(memcmp(cbor_buffer, "\xBF\x65\x68\x65\x6C\x6C\x6F\x65\x77\x6F\x72\x6C\x64\xFF", 14), 0);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);
}

// Tests the following functions: push_str, push_raw, pop_str, pop_raw, pop,
// len, clear, assign.
TEST(args, test1) {
  // Create handle.
  dqcs_handle_t a = dqcs_arb_new();
  ASSERT_NE(a, 0) << "Unexpected error: " << dqcs_error_get();

  // Length should be 0 initially.
  EXPECT_EQ(dqcs_arb_len(a), 0);

  // Push some correct string arguments.
  EXPECT_EQ(dqcs_arb_push_str(a, "First argument"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_push_str(a, "2nd argument"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_push_str(a, "3rd argument"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_push_str(a, ")*#$()&#$"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_push_str(a, ""), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  // Pushing null is not okay.
  EXPECT_EQ(dqcs_arb_push_str(a, nullptr), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: unexpected NULL string");

  // Length should be 5 now.
  EXPECT_EQ(dqcs_arb_len(a), 5);

  // Pop some strings.
  char *s;
  EXPECT_STREQ(s = dqcs_arb_pop_str(a), "");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_arb_pop_str(a), ")*#$()&#$");
  if (s) free(s);

  // Length should be 2 now.
  EXPECT_EQ(dqcs_arb_len(a), 3);

  // Push some data.
  int i = 42*33;
  EXPECT_EQ(dqcs_arb_push_raw(a, &i, sizeof(i)), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_push_raw(a, nullptr, 0), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  // Pushing null with nonzero length is not okay.
  EXPECT_EQ(dqcs_arb_push_raw(a, nullptr, 10), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: unexpected NULL data");

  // Length should be 5 now.
  EXPECT_EQ(dqcs_arb_len(a), 5);

  // Make a copy of this ArbData object.
  dqcs_handle_t b = dqcs_arb_new();
  ASSERT_NE(b, 0) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_assign(b, a), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  // Check the (massive) debug string.
  EXPECT_STREQ(dqcs_handle_dump(a), "ArbData(\n    ArbData {\n        json: Object(\n            {}\n        ),\n        args: [\n            [\n                70,\n                105,\n                114,\n                115,\n                116,\n                32,\n                97,\n                114,\n                103,\n                117,\n                109,\n                101,\n                110,\n                116\n            ],\n            [\n                50,\n                110,\n                100,\n                32,\n                97,\n                114,\n                103,\n                117,\n                109,\n                101,\n                110,\n                116\n            ],\n            [\n                51,\n                114,\n                100,\n                32,\n                97,\n                114,\n                103,\n                117,\n                109,\n                101,\n                110,\n                116\n            ],\n            [\n                106,\n                5,\n                0,\n                0\n            ],\n            []\n        ]\n    }\n)");

  // Do some correct pops.
  char buf[9] = {33, 33, 33, 33, 33, 33, 33, 33, 0};
  EXPECT_EQ(dqcs_arb_pop_raw(a, buf, 8), 0);
  EXPECT_EQ(dqcs_arb_pop_raw(a, buf, 8), sizeof(i));
  EXPECT_EQ(*(int*)buf, i);
  EXPECT_EQ(dqcs_arb_pop_raw(a, buf, 8), 12);
  EXPECT_STREQ(buf, "3rd argu");
  EXPECT_EQ(dqcs_arb_pop_raw(a, nullptr, 0), 12);

  // Do an incorrect pop.
  EXPECT_EQ(dqcs_arb_pop_raw(a, nullptr, 10), -1);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: unexpected NULL buffer");

  // Length should be 0 now, because even though the previous call failed, the
  // data is still lost (meh, but documented behavior).
  EXPECT_EQ(dqcs_arb_len(a), 0);

  // This pop would be okay, but the list is empty.
  EXPECT_EQ(dqcs_arb_pop_raw(a, buf, 8), -1);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: pop from empty list");

  // Same here for a string pop.
  EXPECT_EQ(dqcs_arb_pop_str(a), nullptr);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: pop from empty list");

  // And for the no-return pop.
  EXPECT_EQ(dqcs_arb_pop(a), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: pop from empty list");

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);

  // Check that the copy was not mutated.
  EXPECT_EQ(dqcs_arb_len(b), 5);
  EXPECT_STREQ(dqcs_handle_dump(b), "ArbData(\n    ArbData {\n        json: Object(\n            {}\n        ),\n        args: [\n            [\n                70,\n                105,\n                114,\n                115,\n                116,\n                32,\n                97,\n                114,\n                103,\n                117,\n                109,\n                101,\n                110,\n                116\n            ],\n            [\n                50,\n                110,\n                100,\n                32,\n                97,\n                114,\n                103,\n                117,\n                109,\n                101,\n                110,\n                116\n            ],\n            [\n                51,\n                114,\n                100,\n                32,\n                97,\n                114,\n                103,\n                117,\n                109,\n                101,\n                110,\n                116\n            ],\n            [\n                106,\n                5,\n                0,\n                0\n            ],\n            []\n        ]\n    }\n)");

  // Try a no-return pop.
  EXPECT_EQ(dqcs_arb_pop(b), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  // Now there should be 4 entries.
  EXPECT_EQ(dqcs_arb_len(b), 4);

  // Check that pop_str fails properly when there are nulls in the data.
  EXPECT_EQ(dqcs_arb_pop_str(b), nullptr);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: nul byte found in provided data at position: 2");

  // Now there should be 3 entries, because even though the previous call
  // failed, the data is still lost (meh, but documented behavior).
  EXPECT_EQ(dqcs_arb_len(b), 3);

  // Try the clear function.
  EXPECT_EQ(dqcs_arb_clear(b), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  // Now we should be empty too.
  EXPECT_EQ(dqcs_arb_len(b), 0);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(b), dqcs_return_t::DQCS_SUCCESS);
}

// Tests the following functions: insert_str, remove, set_str, get_str.
TEST(args, test2) {
  // Create handle.
  dqcs_handle_t a = dqcs_arb_new();
  ASSERT_NE(a, 0) << "Unexpected error: " << dqcs_error_get();

  // We can't get/set/remove anything in an empty list.
  EXPECT_EQ(dqcs_arb_get_str(a, 0), nullptr);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: 0");
  EXPECT_EQ(dqcs_arb_get_str(a, -1), nullptr);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: -1");
  EXPECT_EQ(dqcs_arb_get_str(a, 1), nullptr);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: 1");

  EXPECT_EQ(dqcs_arb_set_str(a, 0, "hi"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: 0");
  EXPECT_EQ(dqcs_arb_set_str(a, -1, "hi"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: -1");
  EXPECT_EQ(dqcs_arb_set_str(a, 1, "hi"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: 1");

  EXPECT_EQ(dqcs_arb_set_raw(a, 0, "hi", 2), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: 0");
  EXPECT_EQ(dqcs_arb_set_raw(a, -1, "hi", 2), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: -1");
  EXPECT_EQ(dqcs_arb_set_raw(a, 1, "hi", 2), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: 1");

  EXPECT_EQ(dqcs_arb_remove(a, 0), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: 0");
  EXPECT_EQ(dqcs_arb_remove(a, -1), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: -1");
  EXPECT_EQ(dqcs_arb_remove(a, 1), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: 1");

  // But we CAN insert, at both 0 and -1...
  EXPECT_EQ(dqcs_arb_insert_str(a, 0, "hi"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_len(a), 1);
  EXPECT_EQ(dqcs_arb_clear(a), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_insert_str(a, -1, "hi"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_len(a), 1);
  EXPECT_EQ(dqcs_arb_clear(a), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  // But not at -2 or 1.
  EXPECT_EQ(dqcs_arb_insert_str(a, -2, "hi"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: -2");
  EXPECT_EQ(dqcs_arb_insert_str(a, 1, "hi"), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: 1");

  // Insert some stuff using positive indices.
  EXPECT_EQ(dqcs_arb_insert_str(a, 0, "2"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_insert_str(a, 1, "4"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_insert_str(a, 0, "1"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  // And some using negative indices.
  EXPECT_EQ(dqcs_arb_insert_str(a, -2, "3"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_insert_str(a, -1, "5"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_insert_str(a, -6, "0"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();

  // Check that the insertions did what we expected and that get_str works.
  char expected[2] = {0, 0};
  for (int i = 0; i < 6; i++) {
    expected[0] = i + '0';
    char *s;
    EXPECT_STREQ(s = dqcs_arb_get_str(a, i), expected);
    if (s) free(s);
    EXPECT_STREQ(s = dqcs_arb_get_str(a, i - 6), expected);
    if (s) free(s);
  }

  // Check out of range accesses using get_str.
  EXPECT_EQ(dqcs_arb_remove(a, 6), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: 6");
  EXPECT_EQ(dqcs_arb_remove(a, -7), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: -7");

  // Check that remove works as expected.
  EXPECT_EQ(dqcs_arb_remove(a, 2), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_remove(a, 0), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_remove(a, -2), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_remove(a, 3), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: 3");
  EXPECT_EQ(dqcs_arb_remove(a, -4), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: -4");

  // Check that the removals did what we expected.
  for (int i = 0; i < 3; i++) {
    expected[0] = (i * 2 + 1) + '0';
    char *s;
    EXPECT_STREQ(s = dqcs_arb_get_str(a, i), expected);
    if (s) free(s);
  }

  // Make some changes using set_str.
  EXPECT_EQ(dqcs_arb_set_str(a, 0, "hello"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_set_str(a, -1, "world"), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_set_raw(a, 1, ", ", 2), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_set_raw(a, -4, "world", 5), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: -4");
  EXPECT_EQ(dqcs_arb_set_str(a, 3, ", "), dqcs_return_t::DQCS_FAILURE);
  EXPECT_STREQ(dqcs_error_get(), "Invalid argument: index out of range: 3");

  // Check that the setters did what we expected.
  char *s;
  EXPECT_STREQ(s = dqcs_arb_get_str(a, 0), "hello");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_arb_get_str(a, 1), ", ");
  if (s) free(s);
  EXPECT_STREQ(s = dqcs_arb_get_str(a, 2), "world");
  if (s) free(s);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);
}

// We assume that insert_raw, set_raw, get_raw, and get_size are just
// combinations of the behavior of their string and push/pop counterparts, so
// we only test these API calls very briefly.
TEST(args, test3) {
  // Create handle.
  dqcs_handle_t a = dqcs_arb_new();
  ASSERT_NE(a, 0) << "Unexpected error: " << dqcs_error_get();

  // Check insert_raw, get_raw, and get_size.
  unsigned int value = 0xDEADC0DE;
  EXPECT_EQ(dqcs_arb_insert_raw(a, 0, &value, sizeof(value)), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_get_raw(a, 0, &value, sizeof(value)), 4);
  EXPECT_EQ(value, 0xDEADC0DE);
  EXPECT_EQ(dqcs_arb_get_size(a, 0), 4);

  // Override the value.
  unsigned short value2 = 0xBEEF;
  EXPECT_EQ(dqcs_arb_set_raw(a, 0, &value2, sizeof(value2)), dqcs_return_t::DQCS_SUCCESS) << "Unexpected error: " << dqcs_error_get();
  EXPECT_EQ(dqcs_arb_get_raw(a, 0, &value2, sizeof(value2)), 2);
  EXPECT_EQ(value2, 0xBEEF);
  EXPECT_EQ(dqcs_arb_get_size(a, 0), 2);

  // Delete handle.
  EXPECT_EQ(dqcs_handle_delete(a), dqcs_return_t::DQCS_SUCCESS);
}
