# C API

The C API allows any language that supports C bindings to use DQCsim for making
plugins or host programs. It consists of a shared object and a header file,
which are automatically installed along with the DQCsim Python package (more
detailed notes [here](../install/index.html)).

## How to read this chapter

The sections form a somewhat coherent story that runs you through the entire
API, starting with [some conceptual things](concepts.apigen.md), and followed
by a [walkthrough](type-definitions.apigen.md) of the objects encapsulated by
the API using a bottom-up approach. The [final section](reference.apigen.md)
summarizes all the functions/types exposed by the API in alphabetical order, in
case you're just looking for a searchable list.

The documentation assumes that you already know what DQCsim is, and have a
decent understanding of the basic concepts. If you don't, start
[here](../index.md).

## Contents

 - [Usage](usage.apigen.md)
 - [Concepts](concepts.apigen.md)
 - [Type definitions](type-definitions.apigen.md)
 - [ArbData and ArbCmd objects](arb-cmd-cq.apigen.md)
 - [Qubits](qbset.apigen.md)
 - [Matrices](mat.apigen.md)
 - [Gates](gate.apigen.md)
 - [Gate maps](gm.apigen.md)
 - [Measurements](measurements.apigen.md)
 - [Plugins](plugins.apigen.md)
 - [Simulations](simulations.apigen.md)
 - [Reference](reference.apigen.md)
