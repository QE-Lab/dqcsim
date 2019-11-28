#include <dqcsim>
#include <sstream>
#include "util.hpp"
#include "gtest/gtest.h"

using namespace dqcsim;

// Test the handle API.
TEST(handle, test) {
  // There should initially not be any handles.
  wrap::check(raw::dqcs_handle_leak_check());

  // Deleting, type-querying, or dumping invalid/non-existant handles should
  // throw errors.
  {
    wrap::Handle handle(33u);
    EXPECT_EQ(handle.is_valid(), false);
    EXPECT_ERROR(handle.type(), "Invalid argument: handle 33 is invalid");
    EXPECT_ERROR(handle.free(), "Invalid argument: handle 33 is invalid");
    EXPECT_ERROR(handle.dump(), "Invalid argument: handle 33 is invalid");

    handle = wrap::Handle(0u);
    EXPECT_EQ(handle.is_valid(), false);
    EXPECT_ERROR(handle.type(), "Invalid argument: handle 0 is invalid");
    EXPECT_ERROR(handle.free(), "Invalid argument: handle 0 is invalid");
    EXPECT_ERROR(handle.dump(), "Invalid argument: handle 0 is invalid");
  }

  // Check operations on an existing handle.
  {
    wrap::Handle handle(raw::dqcs_arb_new());
    EXPECT_EQ(handle.is_valid(), true);
    EXPECT_EQ(handle.type(), raw::dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA);
    EXPECT_EQ(handle.dump(),
      "ArbData(\n"
      "    ArbData {\n"
      "        json: Map(\n"
      "            {},\n"
      "        ),\n"
      "        args: [],\n"
      "    },\n"
      ")");
    std::stringstream ss;
    ss << handle;
    EXPECT_EQ(ss.str(),
      "ArbData(\n"
      "    ArbData {\n"
      "        json: Map(\n"
      "            {},\n"
      "        ),\n"
      "        args: [],\n"
      "    },\n"
      ")");
    handle.free();
    EXPECT_EQ(handle.is_valid(), false);
    EXPECT_ERROR(handle.type(), "Invalid argument: handle 0 is invalid");
    EXPECT_ERROR(handle.dump(), "Invalid argument: handle 0 is invalid");
    wrap::check(raw::dqcs_handle_leak_check());
  }

  // Check the take operation and implicit deletion.
  {
    raw::dqcs_handle_t raw_handle;
    {
      wrap::Handle handle(raw::dqcs_arb_new());
      raw::dqcs_handle_t raw_handle_get = handle.get();
      raw_handle = handle.take();
      EXPECT_EQ(raw_handle_get, raw_handle);
      EXPECT_EQ(handle.get(), 0);
      EXPECT_EQ(handle.is_valid(), false);
    }
    {
      wrap::Handle handle(raw_handle);
      EXPECT_EQ(handle.type(), raw::dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA);
    }
    wrap::check(raw::dqcs_handle_leak_check());
  }

}
