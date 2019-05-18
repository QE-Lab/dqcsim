# The components of a simulation

DQCsim divides a mixed quantum-classical simulation up into up to four
different types of components. Put briefly, these are:

 - the [*frontend*](frontend.html), which deals with the
   microarchitecture-specific classical part of the simulation;
 - the [*backend*](backend.html), which deals with simulating the quantum
   mechanics behind the qubits, usually in a microarchitecture-agnostic way;
 - any number of [*operators*](operator.html), which sit between the frontend
   and backend to monitor or modify the gate and measurement streams flowing
   between them;
 - and an optional [*host*](host.html) program, which treats the simulated
   quantum computer as an accelerator.

The frontend, backend, and operators are collectively referred to as *plugins*.
Frontend and operator plugins produce a stream of gates (*gatestream source*),
while operator and backend plugins consume such a stream (*gatestream sinks*).
The measurement stream flows in the opposite direction, in response to the
execution of a measurement gate. We'll commonly use *downstream* and *upstream*
to refer to stream directions between the plugins (or the next plugin over in
said direction); in this case the gatestream is used for the direction
reference. That is, in a simulation with just a frontend and a backend, the
backend is the downstream plugin for the frontend.

Which plugins are used for a simulation is decided by the host program when it
initializes the simulation. If you don't need any special host logic, you can
also use DQCsim's [command-line interface](../cli/index.html) to start the
simulation, which normally behaves like a host that just starts and stops a
simulation without interacting with it.

## Process boundaries

The plugins and host programs are usually all separate processes. The host
side of DQCsim spawns these plugin processes and handles all communication
between them for you, so you don't need to think about it. This requires you to
fundamentally separate your host, frontend, and backend logic, which will help
others understand and reuse parts of your code later on. It also prevents
invalid memory accesses caused by bugs from propagating into other parts of the
simulation, which helps you locate the source of the problem faster.

Perhaps most importantly, the process boundaries let you program the components
in different languages. After all, different problems have different
requirements and different programmers solving them; there is no one language
that can solve everything efficiently! Normally such language boundaries are so
cumbersome that you have to pick a single language for the whole, but DQCsim
handles it all for you. There is absolutely zero difference between a plugin
written in C, C++, Python, or Rust from the perspective of another plugin in
the same simulation!

## Null plugins

To simplify testing a little bit, particularly the regression tests of DQCsim
itself, DQCsim also provides a so-called "null" plugin of each type. The
frontend simply does nothing, the operator passes all gates and measurements
through unchanged, and the backend returns 50/50 measurement results but
otherwise does nothing.
