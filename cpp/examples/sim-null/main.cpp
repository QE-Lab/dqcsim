#include <dqcsim>
using namespace dqcsim::wrap;

int main() {

  SimulationConfiguration()
    .with_plugin(Frontend().with_spec("null"))
    .with_plugin(Backend().with_spec("null"))
    .run()
    .write_reproduction_file("null.repro");

  return 0;
}

