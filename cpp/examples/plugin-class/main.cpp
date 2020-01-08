#define DQCSIM_SHORT_LOGGING_MACROS
#include <dqcsim>
using namespace dqcsim::wrap;

class GateCounter {
public:

  int counter = 0;

  MeasurementSet gate(PluginState &state, Gate &&gate) {
    counter++;
    state.gate(std::move(gate));
    return MeasurementSet();
  }

  void drop(PluginState &state) {
    NOTE("%d gate(s) were transferred!", counter);
  }

};

int main(int argc, char *argv[]) {
  GateCounter gateCounter;
  return Plugin::Operator("GateCounter", "JvS", "v1.0")
    .with_gate(&gateCounter, &GateCounter::gate)
    .with_drop(&gateCounter, &GateCounter::drop)
    .run(argc, argv);
}
