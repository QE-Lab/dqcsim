# Inserting an operator

Now that we have a simulation to play around with, let's add an operator.

There are many types of operators conceivable. To keep things simple here,
we'll use an operator that just affects measurements as an example. Here's
the code:

```python
from dqcsim.plugin import *

@plugin("Measurement-Error", "Tutorial", "0.1")
class MyOperator(Operator):

    def __init__(self):
        super().__init__()
        self.one_error = 0.0
        self.zero_error = 0.0

    def handle_host_measurementError_setOneError(self, value=0.0):
        self.one_error = value

    def handle_host_measurementError_setZeroError(self, value=0.0):
        self.zero_error = value

    def handle_measurement(self, measurement):
        if measurement.value == 1:
            measurement.value = self.random_float() < self.one_error
        elif measurement.value == 0:
            measurement.value = self.random_float() >= self.zero_error
        return measurement

MyOperator().run()
```

To use it within the simulation created in the previous section, replace
the following line:

```python
with Simulator('my-plugin.py', 'null') as sim:
```

With this:

```python
with Simulator('my-plugin.py', 'my-operator.py', 'null') as sim:
```

What this does should be obvious – it sticks the operator we just made in
between, assuming you used the same filename.

Running it doesn't change much though:

```console
$ python3 simulate.py 
... Info dqcsim  Starting Simulation with seed: 14673804979996191647
... Info back    Running null backend initialization callback
... Info dqcsim  PluginProcess exited with status code: 0
... Info dqcsim  PluginProcess exited with status code: 0
... Info dqcsim  PluginProcess exited with status code: 0
Number of balanced outcomes: 498
Number of constant outcomes: 502
```

Still 50/50. Of course, that's because we never set any error rates.

## Dissecting the new plugin

```python
from dqcsim.plugin import *

@plugin("Measurement-Error", "Tutorial", "0.1")
class MyOperator(Operator):
    ...

MyOperator().run()
```

This part is the same as what we've seen before, except we've renamed some
things and are deriviing from `Operator` instead of `Frontend`. You'll see
these lines in every DQCsim plugin; it's the boilerplate code that turns an
otherwise regular Python script into a valid plugin.

```python
def __init__(self):
    super().__init__()
    self.one_error = 0.0
    self.zero_error = 0.0
```

We've built the plugin such that we can set the measurement error rates.
Specifically, we specify two probabilities:

 - `one_error` is the chance that a qubit observed to be in the one state is
   measured as zero;
 - `zero_error` is the chance that a qubit observed to be in the zero state is
   measured as one.

We can store these parameters within our operator class by defining them in its
initializer, as done here.

Note specifically the `super().__init__()` line. It's often forgotten, but in
fact, you *have* to call this any time you override any Python class'
constructor, or your superclass' constructor will simply never be called. In
this case, not doing it will break DQCsim, although it will tell you what you
need to do to fix it in the error message.

```python
def handle_host_measurementError_setOneError(self, value=0.0):
    self.one_error = value

def handle_host_measurementError_setZeroError(self, value=0.0):
    self.zero_error = value
```

These two functions provide entry points for any `ArbCmd`s sent to us by the
host or by the initialization callback (since we didn't override it). The
interface and operation IDs are in the handler's name, as is the source of
the command, which can be `host` or `upstream`.

The Python module detects which interfaces your plugin supports by looking for
handlers of the form `handle_host_<interface>_<operation>()` inside the plugin
constructor. Interface and operation IDs preferably don't contain any
underscores, because this makes the split between the interface and operation
ID ambiguous. In this case, the automatic detection algorithm will throw an
error, and ask you to specify which interfaces your plugin supports through a
keyword argument to the plugin's constructor.

In this case, the detection algorithm works just fine, and determines that the
plugin supports the `measurementError` interface. Within the interface, two
operations are defined, `setOneError` and `setZeroError`; any other operation
will return an error.

```python
def handle_measurement(self, measurement):
    if measurement.value == 1:
        measurement.value = self.random_float() < self.one_error
    elif measurement.value == 0:
        measurement.value = self.random_float() >= self.zero_error
    return measurement
```

This function is the core of the plugin. If it exists, it is called by DQCsim
whenever a measurement passes through the operator, to allow the operator to
return a modified measurement instead. It can also turn a single measurement
into a list of measurements or block propagation of the measurement, but you
wouldn't need to do this unless you're making some complex mapping algorithm.

The implementation provided here modifies the measurement value based on the
perfect measurement received from downstream, DQCsim's random generator, and
the configured probabilities.

## Testing the error model

Let's change the `Simulator()` arguments again. It's getting a little complex
now, so we'll fold it apart for clarity:

```python
with Simulator(
    'my-plugin.py',
    (
        'my-operator.py',
        {
            'init': [
                ArbCmd('measurementError', 'setZeroError', value=0.5)
            ]
        }
    ),
    'null'
) as sim:
```

This adds an initialization command to our error model operator, that sets the
measurement error probability for a zero observation to `0.5`. When we run the
program now, we get something like this:

```console
$ python3 simulate.py
... Info dqcsim  Starting Simulation with seed: 8830780357769760084
... Info back    Running null backend initialization callback
... Info dqcsim  PluginProcess exited with status code: 0
... Info dqcsim  PluginProcess exited with status code: 0
... Info dqcsim  PluginProcess exited with status code: 0
Number of balanced outcomes: 266
Number of constant outcomes: 734
```

Success – this is clearly not 50/50 anymore! Half of the balanced outcomes are
converted to constant by the error model, so the probability is 25/75 now.

To make the `Simulator()` syntax a bit more aesthetically pleasing, we can
configure the simulation in multiple steps, like this:

```python
sim_config = Simulator('my-plugin.py', 'null')
sim_config.with_operator(
    'my-operator.py',
    init=ArbCmd('measurementError', 'setZeroError', value=0.5))
with sim_config as sim:
```

Functionally, this is exactly the same.

## Changing error model parameters at runtime

Because our operator allows the values to be set with run-time `ArbCmd`s as
well, we can also do the following:

```python
with Simulator('my-plugin.py', 'my-operator.py', 'null') as sim:
    sim.start(oracle=oracle, runs=runs)
    results = [sim.recv()['result'] for _ in range(runs)]
    sim.wait()

    sim.arb('op1', 'measurementError', 'setZeroError', value=0.5)

    sim.start(oracle=oracle, runs=runs)
    results += [sim.recv()['result'] for _ in range(runs)]
    sim.wait()
```

Now half of our simulation runs at 50/50 probability, and half of it at 25/75.
So we expect to get around 38/62. Indeed:

```console
$ python3 simulate.py 
... Info dqcsim  Starting Simulation with seed: 6379995085809620608
... Info back    Running null backend initialization callback
... Info dqcsim  PluginProcess exited with status code: 0
... Info dqcsim  PluginProcess exited with status code: 0
... Info dqcsim  PluginProcess exited with status code: 0
Number of balanced outcomes: 722
Number of constant outcomes: 1278
```

Note however, that sending a new probability in the middle of our `recv()`
calls will *not* work. This is because the frontend does not wait for `recv()`
to be called; `send()` is asynchronous! To make that work, you'd have to have
the frontend wait for the host through a `recv()` call, or have the frontend
update the operator's parameters while it's running. In the latter case, you
also have to define `handle_upstream_*()` equivalents for the `handle_host_*()`
callbacks.
