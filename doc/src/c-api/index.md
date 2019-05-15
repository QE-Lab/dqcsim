# C API

The C API allows any language that supports C bindings to use DQCsim for making
plugins or host programs. It consists of a header file (`dqcsim.h`) and an
associated dynamic library (`libdqcsim.so` on Linux, `dqcsim.dylib` on macOS).
These will be installed automatically in the `include` and `lib` directories
that Python is aware of when DQCsim is installed using `pip3 install dqcsim`.

## How to read this chapter

The sections form a somewhat coherent story that runs you through the entire
API, starting with [some conceptual things](concepts.apigen.md), and followed
by a [walkthrough](type-definitions.apigen.md) of the objects encapsulated by
the API using a bottom-up approach. The [final section](reference.apigen.md)
summarizes all the functions/types exposed by the API in alphabetical order, in
case you're just looking for a searchable list.

The documentation assumes that you already know what DQCsim is, and have a
decent understanding of the basic concepts. If you don't, start
[here](https://github.com/mbrobbel/dqcsim-rs/blob/master/README.md).

## Contents

 - [Concepts](concepts.apigen.md)
 - [Type definitions](type-definitions.apigen.md)
 - [ArbData and ArbCmd objects](arb-cmd-cq.apigen.md)
 - [Qubits](qbset.apigen.md)
 - [Gates](gate.apigen.md)
 - [Measurements](measurements.apigen.md)
 - [Plugins](plugins.apigen.md)
 - [Simulations](simulations.apigen.md)
 - [Logging](log.apigen.md)
 - [Reference](reference.apigen.md)
