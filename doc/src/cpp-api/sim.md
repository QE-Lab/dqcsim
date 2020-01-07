# Host/simulation anatomy

To run a simulation (that is, make a host process) with the C++ interface,
you start with a `SimulationConfiguration` object. The most important thing
to do with this object is to configure which plugins you want to use,
particularly the frontend and backend. You do this by adding plugin
configurations using `with_plugin()` or `add_plugin()`. These plugin
configurations are in turn constructed with the `Frontend()`, `Operator()`,
and `Backend()` shorthands, followed by the appropriate builder functions
for how you want to launch the plugins.

When you're done with your configuration, call `build()` or `run()`. The
difference is that the former only initializes the simulation and then
[passes control over to you](../intro/host-iface.md), while the latter is a
shorthand for just calling `run()`, which is usually sufficient. After this,
you may want to write a reproduction file with `write_reproduction_file()`;
this file allows you to reproduce your simulation exactly using the DQCsim
command line (as long as the plugins only use DQCsim's pseudorandom number
generator or are deterministic) without even needing your host program
anymore.

The simplest example for running a simulation is therefore as follows:

```C++
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
```

Plugins can either run as separate processes (as above), or can run as threads
within the host process. You can for instance insert the hello world frontend
we'd defined in the previous section as follows:

```C++
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
```

Note that simulations with plugins defined in-place in the host process
cannot be reproduced through a reproduction file. Therefore, the
reproduction system was turned off here.
