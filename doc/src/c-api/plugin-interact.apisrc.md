# Interacting with DQCsim

When you start a plugin with `dqcs_plugin_run()` or `dqcs_plugin_start()`,
DQCsim will start calling the callbacks you provided. Each of these callbacks
takes a `dqcs_plugin_state_t` handle, which can be used to interact with
DQCsim and the downstream plugin(s) using the functions listed in this section.

## Frontend to host communication

Within a frontend's `run` callback, the following two functions can be used to
send and receive `ArbData` messages to and from the host.

@@@c_api_gen ^dqcs_plugin_send$@@@
@@@c_api_gen ^dqcs_plugin_recv$@@@

The send function always returns immediately, but the receive function may
block to return control to the host if no messages were in the buffer. That
means that the latter can call into a different callback, such as `host_arb`.

## Upstream to downstream communication

The following functions can be used by upstream plugins (frontends and
operators) to perform an operation on the downstream plugin. They correspond
one-to-one with the downstream callbacks.

@@@c_api_gen ^dqcs_plugin_allocate$@@@
@@@c_api_gen ^dqcs_plugin_free$@@@
@@@c_api_gen ^dqcs_plugin_gate$@@@
@@@c_api_gen ^dqcs_plugin_advance$@@@
@@@c_api_gen ^dqcs_plugin_arb$@@@

For performance reasons, all of the above functions except `dqcs_plugin_arb()`
are asynchronous. They send the associated request immediately, but it is down
to OS thread/process scheduling when the request is actually executed. This
means the following:

 - The ordering of log messages sent by differing plugins depends on OS
   scheduling.
 - Errors caused by these asynchronous functions cannot be propagated upstream.
   Therefore, any error that `does` occur is necessarily fatal.

`dqcs_plugin_arb()` is exempt from this since it returns a value, so `ArbCmd`
errors are not necessarily fatal.

## Querying the state of the downstream plugin

Measurement results requested through measurement gates need to be explicitly
fetched when they are needed through the following function. It always returns
the result of the most recent measurement gate for a specific qubit.

@@@c_api_gen ^dqcs_plugin_get_measurement$@@@

DQCsim also records some timing information whenever a measurement is
performed. This may be useful for calculating fidelity information within an
algorithm running in the presence of errors.

@@@c_api_gen ^dqcs_plugin_get_cycles_since_measure$@@@
@@@c_api_gen ^dqcs_plugin_get_cycles_between_measures$@@@

Finally, a simulation cycle counter is maintained. This is just an accumulation
of all the `dqcs_plugin_advance()` calls since the start of the simulation.

@@@c_api_gen ^dqcs_plugin_get_cycle$@@@

## Random number generation

To ensure that a DQCsim simulation can be deterministically reproduced, it is
strongly recommended to use the following random number generation functions.

@@@c_api_gen ^dqcs_plugin_random_f64$@@@
@@@c_api_gen ^dqcs_plugin_random_u64$@@@

Particularly, these generators use a separate PRNG stream depending on whether
the callback they are executed from is synchronous to the upstream channel
(`modify_measurement`) or the downstream channel (all other callbacks). This is
important, because the ordering of upstream callbacks with respect to
downstream callbacks is dependent on OS scheduling.

If you only use downstream callbacks, it's also fine to seed your own PRNG
using the first number returned by `dqcs_plugin_random_u64()` in the `init`
callback. However, using a randomly seeded PRNG is strongly discouraged, since
it prevents a user from using a fixed random seed for reproduction.
