# Controlling simulations

Right now, our Deutsch-Jozsa algorithm always executes all oracles once and
logs the results. This is fine when you just want to run it once, but what if
you want to run it 1000 times, maybe with an error model in between, and apply
some statistics to the results? You could of course make a script that just
calls the DQCsim command line 1000 times and parses the stderr stream for every
call... but that would be a very fragile solution, and it'd be annoying to
make.

Instead, we can use DQCsim's host interface. Let's change the frontend plugin
to make use of this feature.

```python
from dqcsim.plugin import *

@plugin("Deutsch-Jozsa", "Tutorial", "0.2")
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
            self.send(result='balanced')
        else:
            self.send(result='constant')

    def handle_run(self, oracle='', runs=1):

        oracle = {
            '0': self.oracle_constant_0,
            '1': self.oracle_constant_1,
            'x': self.oracle_passthrough,
            '!x': self.oracle_invert,
        }.get(oracle, None)

        if oracle is None:
            raise ValueError('Please specify an oracle!')

        qi, qo = self.allocate(2)

        for _ in range(runs):
            self.deutsch_jozsa(qi, qo, oracle)

        self.free(qi, qo)

MyPlugin().run()
```

When we try to run this frontend with DQCsim's command line, you'll get an
error:

```console
$ dqcsim my-plugin.py null
...  Info dqcsim  Starting Simulation with seed: 15043164643727486506
...  Info back    Running null backend initialization callback
...  Info dqcsim  Executing 'start(...)' host call...
...  Info dqcsim  Executing 'wait()' host call...
... Error front   Please specify an oracle!
...  Info dqcsim  Reproduction file written to "my-plugin.py.repro".
... Fatal dqcsim  Simulation failed: Please specify an oracle!
...  Info dqcsim  PluginProcess exited with status code: 0
...  Info dqcsim  PluginProcess exited with status code: 0
```

Indeed, we've changed the algorithm such that our `handle_run()` callback wants
some arguments.

Remember how the [`start()` call takes an `ArbData` as argument](../intro/host-iface.html#algorithm-execution)?
The Python module passes the contents of this `ArbData` as arguments to
`handle_run()`. Specifically, the binary strings are passed as positional
arguments (`*args`), and the toplevel entries in the JSON/CBOR object are
passed as keyword arguments (`**kwargs`). This abstraction is used all over
the place in the Python layer, including when you have to send an `ArbData`
with it, because it gives the callbacks a nice, Pythonic interface.

It's possible to tell the command-line interface to pass arguments to the
`start()` call as follows, though the syntax isn't very friendly:

```console
$ dqcsim -C 'start:{"oracle":"x"}' my-plugin.py null
... Info dqcsim  Starting Simulation with seed: 13451103132954817086
... Info back    Running null backend initialization callback
... Info dqcsim  Executing 'start(...)' host call...
... Info dqcsim  Executing 'wait()' host call...
... Note dqcsim  'wait()' returned {}
... Info dqcsim  Reproduction file written to "my-plugin.py.repro".
... Info dqcsim  Simulation completed successfully.
... Info dqcsim  PluginProcess exited with status code: 0
... Info dqcsim  PluginProcess exited with status code: 0
```

Now we don't get an error anymore... but we also don't get any output. Look
again at the revised Python script: instead of logging the results, we `send()`
them. `send()` takes an `ArbData` as argument, and thus, like the syntax we
used for our `handle_run()` callback, we can construct it by passing positional
and keyword arguments.

To see the result, we need to make DQCsim call `recv()` before it exits. We can
do that through the command line as well:

```console
$ dqcsim -C 'start:{"oracle":"x"}' -C recv my-plugin.py null
... Info dqcsim  Starting Simulation with seed: 8371252716093296353
... Info back    Running null backend initialization callback
... Info dqcsim  Executing 'start(...)' host call...
... Info dqcsim  Executing 'recv()' host call...
... Note dqcsim  'recv()' returned {"result":"balanced"}
... Info dqcsim  Executing 'wait()' host call...
... Note dqcsim  'wait()' returned {}
... Info dqcsim  Reproduction file written to "my-plugin.py.repro".
... Info dqcsim  Simulation completed successfully.
... Info dqcsim  PluginProcess exited with status code: 0
... Info dqcsim  PluginProcess exited with status code: 0
```

Now `dqcsim` notes that the `recv()` call returned `{"result":"balanced"}`,
which coincidentally is the correct output of the algorithm. Since we're still
using the `null` backend, the result is just 50/50.

## Constructing an accompanying host program

It makes little sense to do all that work only to get a less convenient
command-line interface. Indeed, this mechanism is not intended to be used this
way. You should use it with a host program instead.

You can write this in any of the languages DQCsim supports, since it runs in a
different process. But since this is a Python tutorial, we'll use Python.

Here's an example of a host program that makes use of our modified frontend.

```python
from dqcsim.host import *

runs = 1000
oracle = 'x'

with Simulator('my-plugin.py', 'null') as sim:
    sim.start(oracle=oracle, runs=runs)
    results = [sim.recv()['result'] for _ in range(runs)]
    sim.wait()

print('Number of balanced outcomes:', results.count('balanced'))
print('Number of constant outcomes:', results.count('constant'))
```

Calling this script may yield the following:

```console
$ python3 simulate.py
... Info dqcsim  Starting Simulation with seed: 14750381695807274720
... Info back    Running null backend initialization callback
... Info dqcsim  PluginProcess exited with status code: 0
... Info dqcsim  PluginProcess exited with status code: 0
Number of balanced outcomes: 493
Number of constant outcomes: 507
```

Let's dissect this script.

```python
from dqcsim.host import *
```

This line loads DQCsim's host library and brings it into scope. Specifically,
we're using the `Simulator` class.

```python
runs = 1000
oracle = 'x'
```

These variables set the number of runs and the oracle under test.

```python
with Simulator('my-plugin.py', 'null') as sim:
```

This line starts a simulation. You can call the `Simulator()` constructor in
[many different ways](../py_/dqcsim/host/index.html);
the one we're using here is very similar to the command line we used before.
However, instead of using strings to try to hack a list of host calls in there
(which isn't even supported by the constructor), we'll interact with the
simulation while it's running.

The `with` syntax used here is shorthand notation for the following:

```python
sim = Simulator('my-plugin.py', 'null')
sim.simulate()
# block contents
sim.stop()
```

Calling the `Simulator()` constructor doesn't really do anything yet; it just
*configures* a simulation. In fact, you're free to use it more than once, as
long as you don't try to have multiple simulations running at the same time.

The `simulate()` function actually starts the simulation, in the sense that it
spawns the plugins and calls their initialization callbacks (if applicable),
but it doens't call our `handle_run()` callback yet. The `stop()` function
performs the inverse operation of `simulate()`; it ensures that all the spawned
processes get cleaned up.

```python
sim.start(oracle=oracle, runs=runs)
```

This line actually starts the simulation. The keyword arguments specified are
used to construct the `ArbData` argument, which the plugin library unpacks into
keyword arguments again when it calls `handle_run()`.

```python
results = [sim.recv()['result'] for _ in range(runs)]
```

This [list comprehension](https://docs.python.org/3/tutorial/datastructures.html#list-comprehensions)
calls `recv()` just as many times as the algorithm runs for, and records the
`result` key of the returned `ArbData` object into a list. These entries should
be either `'balanced'` or `'constant'`.

Note that there is a possibility of a deadlock here: if the algorithm wouldn't
call `send()` as often as we expect, everything would logically lock up.
DQCsim detects all possible kinds of deadlocks on the host interface however,
and immediately throws an exception instead. Try it out!


```python
sim.wait()
```

This line waits for the `handle_run()` callback to return. `wait()` returns an
`ArbData` object corresponding to what the `handle_run()` returned, but we're
not using that feature, so we just discard it.

```python
print('Number of balanced outcomes:', results.count('balanced'))
print('Number of constant outcomes:', results.count('constant'))
```

These two lines count the number of `'balanced'` vs. `'constant'` occurrences
in the result list. As we can see when we run the script, the chance is 50/50.
With a perfect qubit simulator backend you should get 1000 balanced and zero
constant outcomes. When you add an error model to the simulation, you might get
something in between.

Note that we're using regular print statements here, versus using DQCsim's log
system. In fact, we have to do this: DQCsim's log system is intended for the
plugins only. You can, however, instruct the `Simulator()` object to forward
the messages it receives to Python's `logging` library to get uniform output.
This is beyond the scope of this tutorial.
