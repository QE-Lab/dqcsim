# Debugging

Let's say we made a mistake in our hello world plugin. For instance,

```python
from dqcsim.plugin import *

@plugin("My Plugin", "Me!", "0.1")
class MyPlugin(Frontend):
    def handle_run(self):
        self.ifo('Hello, world!')

MyPlugin().run()
```

Can you spot the typo? Running the plugin in DQCsim with its default settings
will help a little bit already:


```console
$ dqcsim my-plugin.py null
...  Info dqcsim  Starting Simulation with seed: 9198852889345625466
...  Info back    Running null backend initialization callback
...  Info dqcsim  Executing 'start(...)' host call...
...  Info dqcsim  Executing 'wait()' host call...
... Error front   'MyPlugin' object has no attribute 'ifo'
...  Info dqcsim  Reproduction file written to "my-plugin.py.repro".
... Fatal dqcsim  Simulation failed: 'MyPlugin' object has no attribute 'ifo'
...  Info dqcsim  PluginProcess exited with status code: 0
...  Info dqcsim  PluginProcess exited with status code: 0
```

Apparently we typed `self.ifo` somewhere. Easily fixed in this case... but
there's no traceback, or even a line number in the log. This is because
DQCsim's error handling system only propagates the error message itself,
without any metadata. Propagating things like tracebacks properly would be
pretty hard after all, with all the potential programming language boundaries.

The Python module *does* however log tracebacks of exceptions raised from
within your callback functions. They're simply suppressed by default, as
they're printed with the trace loglevel. To see them, you'll need to change
DQCsim's command line:

```console
$ dqcsim -lt my-plugin.py null
...
... Trace ... Traceback (most recent call last):
...
... Trace ...   File "my-plugin.py", line 6, in handle_run
... Trace ...     self.ifo('Hello, world!')
... Trace ... AttributeError: 'MyPlugin' object has no attribute 'ifo'
...
```

There are quite some messages to sift through with this verbosity, but the ones
you're looking for would normally also be in there. If it's not, DQCsim's
Python module is itself confused, either because you're doing something it
didn't expect, or because there is a bug somewhere. In this case you'll have to
narrow the problem down through trial and error.

## Reproduction files

It's also possible that your exception only happens sometimes, because of some
non-deterministic quantum behavior being modelled, or because you're running
the plugin from a non-deterministic host program. In this case, you can try
using reproduction files. The command-line interface will output one by default
whenever you run a simulation, named after the frontend plugin with a `.repro`
suffix. So, in this case, the following command should reproduce the previous
run without any log message filtering.

```console
$ dqcsim -lt --reproduce-exactly my-plugin.py.repro
```
