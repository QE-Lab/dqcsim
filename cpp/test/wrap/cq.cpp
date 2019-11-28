#include <dqcsim>
#include "util.hpp"
#include "gtest/gtest.h"

using namespace dqcsim;

// Basic tests for `ArbCmd` objects.
TEST(cq, basic) {
  {
    std::vector<wrap::ArbCmd> vec_in;
    vec_in.emplace_back("a", "b");
    vec_in.back().set_arb_json_string("{\"hello\": \"world\"}");
    std::string a_b = vec_in.back().dump();
    vec_in.emplace_back("c", "d");
    vec_in.back().set_arb_json_string("{\"another\": \"value\"}");
    std::string c_d = vec_in.back().dump();

    wrap::ArbCmdQueue cq(std::move(vec_in));
    std::string cq_dump = cq.dump();

    {
      std::vector<wrap::ArbCmd> vec_out = cq.copy_into_vector();
      EXPECT_EQ(vec_out.size(), 2);
      EXPECT_EQ(vec_out[0].dump(), a_b);
      EXPECT_EQ(vec_out[1].dump(), c_d);
    }

    EXPECT_EQ(cq.dump(), cq_dump);

    std::vector<wrap::ArbCmd> vec_out = cq.drain_into_vector();
    EXPECT_EQ(vec_out.size(), 2);
    EXPECT_EQ(vec_out[0].dump(), a_b);
    EXPECT_EQ(vec_out[1].dump(), c_d);

    EXPECT_EQ(cq.size(), 0);
  }

  wrap::check(raw::dqcs_handle_leak_check());
}
