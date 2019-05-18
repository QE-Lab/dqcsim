# Miscellaneous interfaces

Besides the interfaces described previously, DQCsim provides some
miscellaneous services.

## Plugin construction and destruction

Each type of plugin can define an `initialize()` and a `drop()` callback.
DQCsim will ensure that these callbacks are called respectively before and
after all other callbacks.

The `initialize()` callback takes a list of `ArbCmd`s as an argument. These
are configured by the host when the plugin is constructed. They can be regarded
as a specialization of host arbs that deals with initialization specifically.

## Host arbs to operators and backends

In addition to the host being able to send arbs to the frontend, it can also
send arbs to the operator(s) and backend. The mechanism is the same.
Synchronization with respect to the rest of the simulation is guaranteed at all
times.

## Logging

Each plugin has a `log()` function of some kind that allows it to send log
messages through DQCsim's centralized logging framework. Using this system
versus simply writing to stdout or stderr has the benefit of being
synchronized (since I/O streams are not thread-safe), and allows users to
filter out messages that they're not interested in.
