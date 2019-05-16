# Configuring plugins

Before we can build a simulation, we need to configure the individual plugins
that make up the simulation. This is usually done using a plugin process
configuration (`pcfg`).

## Constructing a plugin process configuration

There are two ways to construct the configuration. `dqcs_pcfg_new()` is the
easy one: it will look for the plugin executable in the same way that the
command-line interface does it. It works using a single string, which can be:

 - a valid path to the plugin executable;
 - the basename of the plugin executable with implicit `dqcsfe`/`dqcsop`/`dqcsbe`
   prefix, searched for in A) the current working directory, B) the directory
   that the binary for the current process resides in, and C) the system
   `$PATH` (in that order);
 - a valid path to a script file with a file extension. In this case, the above
   rule is run for a plugin named by the file extension of the script file. For
   instance, if `test.py` is specified for a frontend plugin, DQCsim will look
   for an executable named `dqcsfepy`. The script filename is passed to the
   plugin through the first command-line argument, moving the simulator
   endpoint string to the second slot.

Alternatively, you can bypass this algorithm by specifying the full path to the
plugin and (optionally) the script file directly using `dqcs_pcfg_new_raw()`.

@@@c_api_gen ^dqcs_pcfg_new$@@@
@@@c_api_gen ^dqcs_pcfg_new_raw$@@@

After construction, the plugin type, name, executable path, and optional script
path become immutable. However, their values can be queried using the following
functions.

@@@c_api_gen ^dqcs_pcfg_type$@@@
@@@c_api_gen ^dqcs_pcfg_name$@@@
@@@c_api_gen ^dqcs_pcfg_executable$@@@
@@@c_api_gen ^dqcs_pcfg_script$@@@

## Functional configuration

A plugin's behavior can be augmented using a list of `ArbCmd`s passed to its
initialization callback and through environment variables. These can be set
using the following functions.

@@@c_api_gen ^dqcs_pcfg_init_cmd$@@@
@@@c_api_gen ^dqcs_pcfg_env_set$@@@
@@@c_api_gen ^dqcs_pcfg_env_unset$@@@

It's also possible to assign a different working directory to a plugin process
using the following functions.

@@@c_api_gen ^dqcs_pcfg_work_set$@@@
@@@c_api_gen ^dqcs_pcfg_work_get$@@@

These configuration parameters are recorded in reproduction files, since they
may modify the behavior of the plugin.

## Logging configuration

DQCsim allows log message filtering to be performed independently for each
plugin. The following functions can be used to configure this per-plugin
filter. The verbosity defaults to trace to pass all messages through; the
messages will also be filtered by DQCsim itself.

@@@c_api_gen ^dqcs_pcfg_verbosity_set$@@@
@@@c_api_gen ^dqcs_pcfg_verbosity_get$@@@

You can also let DQCsim pipe only the messages of a specific plugin to a file.
This behavior can be configured using `dqcs_pcfg_tee()`.

@@@c_api_gen ^dqcs_pcfg_tee$@@@

Finally, DQCsim will by default capture the stdout and stderr streams of the
plugin process and convert each received line into a log message. The following
functions can be used to configure the loglevels used for these messages, to
disable capturing, or to void the streams altogether.

@@@c_api_gen ^dqcs_pcfg_stdout_mode_set$@@@
@@@c_api_gen ^dqcs_pcfg_stdout_mode_get$@@@
@@@c_api_gen ^dqcs_pcfg_stderr_mode_set$@@@
@@@c_api_gen ^dqcs_pcfg_stderr_mode_get$@@@

## Timeouts

DQCsim uses a timeout mechanism when spawning a plugin and shutting it down to
detect deadlocks due to misconfiguration. This timeout defaults to 5 seconds.
If your plugin needs more time to start up or shut down gracefully for some
reason, you can modify the timeouts using the following functions.

@@@c_api_gen ^dqcs_pcfg_accept_timeout_set$@@@
@@@c_api_gen ^dqcs_pcfg_accept_timeout_get$@@@
@@@c_api_gen ^dqcs_pcfg_shutdown_timeout_set$@@@
@@@c_api_gen ^dqcs_pcfg_shutdown_timeout_get$@@@
