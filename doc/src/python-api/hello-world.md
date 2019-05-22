# Hello, world!

Let's start with a basic frontend plugin, `Hello, world!` style. This is all
you need:

```python
from dqcsim.plugin import *

@plugin("My Plugin", "Me!", "0.1")
class MyPlugin(Frontend):
    def handle_run(self):
        self.info('Hello, world!')

MyPlugin().run()
```

This frontend just logs `Hello, world!` with the info loglevel when run. You
can use the command line to test it as follows:

```shell
$ dqcsim my-plugin.py null
... Info dqcsim  Starting Simulation with seed: ...
... Info back    Running null backend initialization callback
... Info dqcsim  Executing 'start(...)' host call...
... Info dqcsim  Executing 'wait()' host call...
... Info front   Hello, world!
... Note dqcsim  'wait()' returned {}
... Info dqcsim  Reproduction file written to "my-plugin.py.repro".
... Info dqcsim  Simulation completed successfully.
... Info dqcsim  PluginProcess exited with status code: 0
... Info dqcsim  PluginProcess exited with status code: 0
```

Note the `Hello, world!` line in the middle.

While you're debugging a plugin, you might want to change the log verbosities
around a little. For instance, the following will set the verbosity of your
plugin to the debug level, and the verbosity of the other plugins and DQCsim
itself to error.

```shell
$ dqcsim -ld --plugin-level e --dqcsim-level e my-plugin.py -ld null
... Info front   Hello, world!
```

Take a few minutes to look through `dqcsim --long-help` to see what those
options mean and what else it can do for you, specifically on the subject of
logging!

## How it works

Let's dissect the hello world plugin line by line.

```python
from dqcsim.plugin import *
```

This loads the DQCsim plugin library and pulls it into our module's scope.
Specifically, we're using `Frontend` and `plugin` in this script.

```python
@plugin("My Plugin", "Me!", "0.1")
class MyPlugin(Frontend):
```

These lines define a new plugin. Any plugin must derive from either `Frontend`,
`Operator`, or `Backend`; we're deriving from `Frontend` here.

The `@plugin` annotation specifies some metadata for the plugin, namely the
plugin's name, author, and version. The host can access this metadata to verify
that it loaded the right plugin. While DQCsim requires you to specify these
three strings, it doesn't actually do anything with it on its own.


```python
def handle_run(self):
```

This function is called by DQCsim in response to the `start()` command from the
host. There are a couple more callbacks that frontend plugins can define, but
this is the most important one. It's also the only one that's required for a
frontend.

```python
self.info('Hello, world!')
```

This function sends a log message back to DQCsim. You can also use `trace`,
`debug`, `note`, `warn`, `error`, or `fatal` to get a different loglevel. Any
arguments specified in addition to the first are passed to
[`str.format()`](https://docs.python.org/3.6/library/stdtypes.html#str.format),
so you could for instance also call `self.info('Hello, {}!', username)` (if
`username` would be a thing here).

```python
MyPlugin().run()
```

This line actually turns the Python script into a DQCsim plugin. Without it,
DQCsim would crash:

```plaintext
Fatal dqcsim  plugin did not connect within specified timeout
```

After all, just defining a class doesn't really make a Python script do
anything!

The first part of the line, `MyPlugin()`, makes an instance of the plugin, but
doesn't start it yet. This is because there are multiple ways to start a
plugin, and it's also useful to pass instantiated but not-yet-started plugins
around during initialization.

The second part, `.run()`, actually starts the plugin. It also waits for it to
finish, and either returns `None` to indicate success or throws a
`RuntimeError` to indicate failure.

Plugins need a simulator to connect to. DQCsim passes this endpoint as a string
to the first command-line argument of a plugin process. `run()` let's you
specify the endpoint manually if you like, but if you don't, it takes it from
`sys.argv[1]` automatically, with appropriate error checking.
