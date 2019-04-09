#include <dqcsim_raw.hpp>
#include "gtest/gtest.h"
#include "util.h"

using namespace dqcsim;

// Sanity-check.
TEST(sim_host, sanity) {
  SIM_HEADER;
  SIM_CONSTRUCT;
  SIM_FOOTER;
}

// TODO test host communication
