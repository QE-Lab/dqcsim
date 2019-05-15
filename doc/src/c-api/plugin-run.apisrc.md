# Running a plugin

Once a plugin has been defined, it can be started. Normally, DQCsim will do
this in one way or another. There are four ways in which DQCsim can do this:

 - by spawning a process;
 - by letting you spawn the process;
 - by spawning a thread within the simulation process;
 - by letting you spawn the thread.

We'll go into this in greater depths here (TODO). For now, I'll assume either
the first or second thing has already happened, and the process that was
launched is the one that you're working on right now.

DQCsim will spawn a plugin process with a single command-line argument (in
addition to the process name as per convention). This argument identifies a
FIFO file or address of some kind that tells the DQCsim instance running in
the plugin process how to connect to the DQCsim instance that controls the
simulation. You don't need to know anything about the syntax of this argument;
all you need to do is pass it to `dqcs_plugin_run()`, along with a plugin
definition.

@@@c_api_gen ^dqcs_plugin_run$@@@

If the function fails, your process should return a nonzero exit code; if it
succeeds, it should return 0. And that's it!

Some notes, though:

 - If you spawned the process manually, you're in control of how the connection
   endpoint string is passed. So you can do something else if you like, but
   keep in mind that the command-line interface won't be able to use your
   plugin in that case.
 - The connection endpoint is currently a FIFO file, so it cannot connect to
   another machine.
 - `dqcs_plugin_run()` is a blocking function call. If you're making some
   awesome plugin server that needs to be able to run multiple plugin threads
   at a time, you can also use the asynchronous equivalents below. This is
   particularly useful in case you want to construct the plugin definition in
   your main thread for some reason. However, you can currently only use a
   plugin definition once; it is deleted by `dqcs_plugin_start()` as well as
   `dqcs_plugin_run()`.

@@@c_api_gen ^dqcs_plugin_start$@@@
@@@c_api_gen ^dqcs_plugin_wait$@@@
