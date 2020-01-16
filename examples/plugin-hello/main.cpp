#define DQCSIM_SHORT_LOGGING_MACROS
#include <dqcsim>
#include <iostream>
using namespace dqcsim::wrap;
using namespace std;

ArbData run(RunningPluginState &state, ArbData &&arg) {
  INFO("Hello, World!");
  return ArbData();
}

int main(int argc, char *argv[]) {
  return Plugin::Frontend("hello", "JvS", "v1.0")
    .with_run(run)
    .run(argc, argv);
}
