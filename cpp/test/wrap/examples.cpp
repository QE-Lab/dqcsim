#include <dqcsim>
#include <sstream>
#include "util.hpp"
#include "gtest/gtest.h"

using namespace dqcsim::wrap;

// `dqcsim null null` example on doxy main page
TEST(examples, null) {
  SimulationConfiguration()
    .with_plugin(Frontend().with_spec("null"))
    .with_plugin(Backend().with_spec("null"))
    .run()
    .write_reproduction_file("null.repro");
}
