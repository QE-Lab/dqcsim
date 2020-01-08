# Plugin anatomy

To define your own plugin, you can use the `Plugin` class. The workflow is
as follows:

 - Use `Plugin::Frontend()`, `Plugin::Operator()`, or `Plugin::Backend()`
   to start defining a plugin.
 - Assign callback functions at your leisure using the `with_*` functions.
   You can pass any combination of arguments supported by
   `dqcsim::wrap::Callback()` to these builder functions, provided that the
   callback function signature is correct, of course.
 - Either:
    - run the plugin in the current thread using `Plugin::run()`;
    - start the plugin in a DQCsim-managed worker thread using
      `Plugin::start()`;
    - or pass the plugin definition object to
      `PluginConfigurationBuilder::with_callbacks()` to directly add it to
      a simulation.

Here's an example of a simple frontend plugin that just logs `"Hello, World!"`:

```C++
#define DQCSIM_SHORT_LOGGING_MACROS
#include <dqcsim>
using namespace dqcsim::wrap;

ArbData run(RunningPluginState &state, ArbData &&arg) {
  INFO("Hello, World!");
  return ArbData();
}

int main(int argc, char *argv[]) {
  return Plugin::Frontend("hello", "JvS", "v1.0")
    .with_run(run)
    .run(argc, argv);
}
```

The `Plugin` class is equivalent to the
[`pdef` C API](../c-api/pdef.apigen.md), but, as you can see, is much more
succinct. Note for instance that the example includes all error handling
implicitly thanks to exceptions, and that only a single statement is needed
thanks to the builder pattern.

The C++ API supports a few different styles for the callback functions. The one
used here is the most basic one; even more basic than the one in the C API as
it does not take a user-defined parameter to store state or initialization
parameters in. You can add that quite simply:

```C++
#include <string>

#define DQCSIM_SHORT_LOGGING_MACROS
#include <dqcsim>
using namespace dqcsim::wrap;

ArbData run(std::string *message, RunningPluginState &state, ArbData &&arg) {
  INFO("%s", message->c_str());
  *message = "I was run!";
  return ArbData();
}

int main(int argc, char *argv[]) {
  std::string message = "Hello!";
  int code = Plugin::Frontend("hello", "JvS", "v1.0")
    .with_run(run, &message)
    .run(argc, argv);
  INFO("%s", message.c_str());
  return code;
}
```

You can also abstract your plugin into a class. For instance, the following
more complex example counts the number of gates passing through the operator.

```C++
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
```

Note the `std::move()` in the `gate()` callback. Handles in DQCsim are
generally not efficiently copyable, thus `move()`ing stuff around with rvalue
references becomes important at times. If you don't know what that means, you
should probably look it up at some point, but the C++ API provides
(inefficient) copy constructors and overloads with regular references for most
handle types, so usually things will just work without the `std::move()`.
