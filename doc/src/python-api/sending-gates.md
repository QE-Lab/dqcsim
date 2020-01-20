# Sending some gates

Let's change our plugin to make it do something more useful now. We'll
implement the
[Deutsch–Jozsa algorithm](https://en.wikipedia.org/wiki/Deutsch%E2%80%93Jozsa_algorithm)
because it's nice and simple. Put briefly, this algorithm determines whether
a one-qubit to one-qubit oracle function is constant (`x -> 0` or `x -> 1`)
or balanced (`x -> x` or `x -> !x`) by only evaluating the function once.

Here's the plugin code:

```python
from dqcsim.plugin import *

@plugin("Deutsch-Jozsa", "Tutorial", "0.1")
class MyPlugin(Frontend):

    def oracle_constant_0(self, qi, qo):
        """x -> 0 oracle function."""
        pass

    def oracle_constant_1(self, qi, qo):
        """x -> 1 oracle function."""
        self.x_gate(qo)

    def oracle_passthrough(self, qi, qo):
        """x -> x oracle function."""
        self.cnot_gate(qi, qo)

    def oracle_invert(self, qi, qo):
        """x -> !x oracle function."""
        self.cnot_gate(qi, qo)
        self.x_gate(qo)

    def deutsch_jozsa(self, qi, qo, oracle):
        """Runs the Deutsch-Jozsa algorithm on the given oracle. The oracle is
        called with the input and output qubits as positional arguments."""

        # Prepare the input qubit.
        self.prepare(qi)
        self.h_gate(qi)

        # Prepare the output qubit.
        self.prepare(qo)
        self.x_gate(qo)
        self.h_gate(qo)

        # Run the oracle function.
        oracle(qi, qo)

        # Measure the input.
        self.h_gate(qi)
        self.measure(qi)
        if self.get_measurement(qi).value:
            self.info('Oracle was balanced!')
        else:
            self.info('Oracle was constant!')

    def handle_run(self):
        qi, qo = self.allocate(2)

        self.info('Running Deutsch-Jozsa on x -> 0...')
        self.deutsch_jozsa(qi, qo, self.oracle_constant_0)

        self.info('Running Deutsch-Jozsa on x -> 1...')
        self.deutsch_jozsa(qi, qo, self.oracle_constant_1)

        self.info('Running Deutsch-Jozsa on x -> x...')
        self.deutsch_jozsa(qi, qo, self.oracle_passthrough)

        self.info('Running Deutsch-Jozsa on x -> !x...')
        self.deutsch_jozsa(qi, qo, self.oracle_invert)

        self.free(qi, qo)

MyPlugin().run()
```

The plugin runs the algorithm on all possible oracles in sequence. Observe that
we hardly had to duplicate any code to do that, by making clever use of
Python's expressivity! Also note that DQCsim's Python plugin provides you with
a number of built-in gate types. You can find the full list
[here](../py_/dqcsim/plugin/index.html#dqcsim.plugin.GateStreamSource).

You can run this example with the null backend provided by DQCsim as follows.

```console
$ dqcsim my-plugin.py null
... Info dqcsim  Starting Simulation with seed: 17518393962103239508
... Info back    Running null backend initialization callback
... Info dqcsim  Executing 'start(...)' host call...
... Info dqcsim  Executing 'wait()' host call...
... Info front   Running Deutsch-Jozsa on x -> 0...
... Info front   Oracle was constant!
... Info front   Running Deutsch-Jozsa on x -> 1...
... Info front   Oracle was balanced!
... Info front   Running Deutsch-Jozsa on x -> x...
... Info front   Oracle was balanced!
... Info front   Running Deutsch-Jozsa on x -> !x...
... Info front   Oracle was constant!
... Note dqcsim  'wait()' returned {}
... Info dqcsim  Reproduction file written to "my-plugin.py.repro".
... Info dqcsim  Simulation completed successfully.
... Info dqcsim  PluginProcess exited with status code: 0
... Info dqcsim  PluginProcess exited with status code: 0
```

Note that the null backend ignores all gates and just returns random
measurements, so the algorithm also just returns random results. To make the
algorithm work, you'll have to use a real backend.
