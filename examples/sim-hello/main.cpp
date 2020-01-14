#define DQCSIM_SHORT_LOGGING_MACROS
#include <dqcsim>
using namespace dqcsim::wrap;

ArbData run(RunningPluginState &state, ArbData &&arg) {
  INFO("Hello, World!");
  return ArbData();
}

int main() {

  SimulationConfiguration()
    .without_reproduction()
    .with_plugin(Frontend().with_callbacks(
      Plugin::Frontend("hello", "JvS", "v1.0")
        .with_run(run)
    ))
    .with_plugin(Backend().with_spec("null"))
    .run();

  return 0;
}
