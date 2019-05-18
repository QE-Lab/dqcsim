# Running a simulation

When you've finished building a simulation configuration object, you can turn
it into a real simulation as described in this section.

## Constructing a simulation

To run the simulation, all you have to do is pass the simulation configuration
object to `dqcs_sim_new()`. This function will return when all the plugins have
finished initializing as configured in the configuration object, and return a
handle to the simulation.

@@@c_api_gen ^dqcs_sim_new$@@@

Note that it is currently not possible to have more than one simulation handle
within a single thread at the same time. This has to do with DQCsim's log
system, which uses thread-local storage to determine where log messages should
go. If you want to run multiple simulations in parallel, you'll have to run
them from different threads.

## Interacting with a simulation

After constructing the simulation, you have to explicitly tell the frontend
plugin to start executing a quantum algorithm. This is done using
`dqcs_sim_start()`. This function is asynchronous: the simulation request is
only sent to the frontend when a blocking function is called. To get the
result/return value of a previously started quantum algorithm, you can use
`dqcs_sim_stop()`. In fact, you *have* to do this for every call to
`dqcs_sim_start()`, and you can't have more than one quantum algorithm running
at a time within the context of a single simulation.

@@@c_api_gen ^dqcs_sim_start$@@@
@@@c_api_gen ^dqcs_sim_wait$@@@

While a quantum algorithm is running, you can interact with it using `ArbData`
message queues. You can send and receive data to and from these queues using
the following functions. The send function is asynchronous, while the receive
function will block if no messages are available.

@@@c_api_gen ^dqcs_sim_send$@@@
@@@c_api_gen ^dqcs_sim_recv$@@@

At any time, you can force DQCsim to pass control to the frontend plugin using
the following function. This is primarily useful for debugging, when you for
instance want to see the results of a single sent message in the log message
stream without calling a blocking function that actually does something.

@@@c_api_gen ^dqcs_sim_yield$@@@

You can also send `ArbCmd`s to plugins at any time. This corresponds to calling
the `host_arb` callback within a plugin. This is always synchronous; any
requests queued through `dqcs_sim_start()` and `dqcs_sim_send()` are processed
before the `ArbCmd`, and the function waits for the `ArbCmd` to finish
executing in order for it to return its result.

@@@c_api_gen ^dqcs_sim_arb$@@@
@@@c_api_gen ^dqcs_sim_arb_idx$@@@

## Querying plugin information

You can query the metadata associated with the plugins that make up a
simulation using the following functions.

@@@c_api_gen ^dqcs_sim_get_name$@@@
@@@c_api_gen ^dqcs_sim_get_name_idx$@@@
@@@c_api_gen ^dqcs_sim_get_author$@@@
@@@c_api_gen ^dqcs_sim_get_author_idx$@@@
@@@c_api_gen ^dqcs_sim_get_version$@@@
@@@c_api_gen ^dqcs_sim_get_version_idx$@@@

## Shutting a simulation down

When you're done with a simulation, you can just use `dqcs_handle_delete()` to
shut it down. Before doing that, though, it is strongly recommended to output a
reproduction file. This file lets you reproduce the simulation exactly without
needing the host executable (or needing it to be deterministic) with just
DQCsim's command-line interface. You never know when you might need this for
debugging!

@@@c_api_gen ^dqcs_sim_write_reproduction_file$@@@
