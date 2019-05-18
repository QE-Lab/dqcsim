# Running local plugins

Besides letting DQCsim spawn plugin processes for you, you can also let DQCsim
run a plugin within a thread, or assume full control over spawning the process
or thread. To do this, you need to use a plugin *thread* configuration object
(`tcfg`) instead of a plugin *process* configuration.

## Running a plugin within a thread

This method is similar to spawning a plugin in a process. However, because
there's no process boundary, the plugin is defined within the host process, and
can access memory of the host process. This can be useful particularly when you
want to make a self-contained simulation, or want to insert a simple operator
plugin somewhere in the pipeline that you don't want to make a separate
executable for. The configuration object for this scenario is constructed using
the following function.

@@@c_api_gen ^dqcs_tcfg_new$@@@

## Assuming full control over plugin spawning

This method gives you full control over spawning a plugin process or thread.
This is useful for instance if you want to encapsulate the plugin process in a
tool like `gdb` or `valgrind`. The configuration object for this method is
constructed using the following function.

@@@c_api_gen ^dqcs_tcfg_new_raw$@@@

## Querying plugin thread configuration objects

Similar to plugin process configuration objects, the name and type of the
plugin are immutable after construction, but can be queried.

@@@c_api_gen ^dqcs_tcfg_type$@@@
@@@c_api_gen ^dqcs_tcfg_name$@@@

## Functional configuration

Like plugin process configuration objects, it's possible to send `ArbCmd`s to
the plugin's initialization callback. However, environment variables and the
working directory of the plugin cannot be set, since they're tied to the host
process.

@@@c_api_gen ^dqcs_tcfg_init_cmd$@@@

## Logging configuration

The logging behavior for a plugin thread can be configured with the following
functions, just like plugin processes. However, file descriptors are shared
between threads, so the plugin thread does not have a separate stdout/stderr
stream that can be captured.

@@@c_api_gen ^dqcs_tcfg_verbosity_set$@@@
@@@c_api_gen ^dqcs_tcfg_verbosity_get$@@@
@@@c_api_gen ^dqcs_tcfg_tee$@@@
