# Plugin distribution

Since DQCsim is only a framework, installing *just* DQCsim doesn't let you do
much besides development. To get started with it quickly, you'll also need to
install some plugins. At the time of writing, the following plugins are
available or planned.

## QuantumSim backend

 - Install: `sudo pip3 install dqcsim-quantumsim`
 - Source/docs: [GitHub](https://github.com/QE-Lab/dqcsim-quantumsim)

A very lightweight connection to the
[QuantumSim](https://gitlab.com/quantumsim/quantumsim) simulator. Very suitable
as a more in-depth example for Python plugin development, as it's Python-only.

## OpenQL mapper operator

 - Install: `sudo pip3 install dqcsim-openql-mapper`
 - Source/docs: [GitHub](https://github.com/QE-Lab/dqcsim-openql-mapper)

This plugin converts an incoming gatestream using virtual qubits to physical
qubits in the outgoing gatestream, inserting swaps and moves along the way to
satisfy configurable connectivity constraints, using the
[OpenQL](https://github.com/QE-Lab/OpenQL) mapper.

## QX backend

 - Install: `sudo pip3 install dqcsim-qx`
 - Source/docs: [GitHub](https://github.com/QE-Lab/dqcsim-qx)

A connection to the [QX](https://github.com/QE-Lab/qx-simulator/) simulator.

## OpenQASM frontend

 - Install: `cargo install dqcsim-openqasm`
 - Source: [Github](https://github.com/mbrobbel/dqcsim-openqasm)

Allows execution of [OpenQASM](https://github.com/Qiskit/openqasm) algorithm
descriptions.

## cQASM frontend (planned)

Allows execution of cQASM algorithm descriptions, giving DQCsim access to the
full capability of the [OpenQL compiler](https://github.com/QE-Lab/OpenQL), as
cQASM is its primary output format.

## cQASM output operator (planned)

Interprets the gatestream passing through it to write a cQASM file.

## Metrics operator (planned)

Interprets the gatestream passing through it to calculate statistics, like
circuit depths, number of swap gates, and so on.

## [Your plugin here]

Developing plugins for DQCsim is very easy once you get the hang of it! Most of
the plugins listed above were written in about a day, initial debugging of
DQCsim aside. Keep reading!
