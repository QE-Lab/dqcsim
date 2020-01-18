# Gate- and measurement streams

The gatestream interface connects frontends to backends, frontends to
operators, and operators to backends. There is a slight difference between
the three to do with the measurement path, but for the most part they are
the same. The following graphic shows the functions and callbacks used to
form the interface on either side.

<p style="text-align: center"><img src="gatestream.svg" /></p>

The green datapaths only exist if the upstream plugin is an operator. The red
datapaths only exist if the downstream plugin is an operator.

## Allocating and freeing qubits

DQCsim allows upstream plugins to allocate qubits within the downstream
plugins at all times. This was done to provide a means for qubit mapping
operators to determine whether upstream qubits are in use or not, and because
it seems obvious in general coming from the classical world. Similar to a
classical computer, a backend with limited resources must make map the qubits
that are actually in use appropriately.

The `alloc()` function takes the number of qubits that are to be allocated as
an argument, as well as a list of `ArbCmd`s. The `ArbCmd`s can for instance be
used for various things, such as assigning a name to qubit registers for
debugging purposes or specifying error model information on a per-qubit basis.
In response, DQCsim allocates unique indices for the requested qubits and
returns them to the algorithm immediately. It also asynchronously sends an
allocation request through the gatestream, which causes the `alloc()` callback
to be called on the other end. This function takes the qubit indices that
DQCsim allocated for the qubits and the list of `ArbCmd`s as argument.

`free()` can be used to free previously allocated qubits. If these qubits are
still entangled, the backend should collapse the state in some way; however, it
is up to the backend to decide how to do this. `free()` only takes the list of
qubit indices that are to be freed as arguments.

Qubit indices are assigned by DQCsim in a deterministic way. The first
allocated qubit receives index 1. Subsequent allocations receive the next
index. This means that freed qubit indices are never reused; if a backend wants
to reuse indices internally, it must maintain its own mapping. Index 0 is
reserved for marking invalid qubits and the end of a qubit list in the C API.

## Sending and receiving gates

DQCsim supports four kinds of gates:

 - unitary gates, defined by a matrix, one or more target qubits, and zero or
   more control qubits;
 - measurement gates, defined by one or more measured qubits and an arbitrary
   measurement basis;
 - prep gates, defined by one or more target qubits and an arbitrary basis;
 - custom gates, defined by a name and any of the above. Downstream plugins
   should reject named gates that they don't recognize.

Note that all but the latter do not have any kind of name attached to them.
In other words, detecting whether a gate is for instance an X gate is not very
trivial: it requires matching the matrix with a known X matrix. Floating point
roundoff errors could cause headaches, and global phase also becomes a thing
then. After all, the following matrices are equivalent in quantum:

\f[
\begin{bmatrix}
0 & 1 \\
1 & 0
\end{bmatrix} \_\_
\begin{bmatrix}
0 & i \\
i & 0
\end{bmatrix}
\f]

as they only differ in global phase. This also goes for the measurement and
prep gates, where the basis is also represented with a matrix such that any
basis and initial state can be described.

The good thing about this representation, however, is that there is no need for
plugins to agree on some naming scheme for gates. After all, is `Y` the same as
`y`? What about `Pauli_Y`? Or `Y180`? `cnot` versus `cx`? `ccnot` versus
`toffoli`? And so on. Not to mention arbitrary rotations. The matrix
representation of gates is pretty universally agreed upon, global phase aside
in some cases, so it serves as a sort of universal language. Which is exactly
what DQCsim is trying to do: provide a universal interface for components of
quantum simulations to communicate with each other at runtime.

It's also worth noting that simulations don't necessarily have to care about
the name of a gate; they just have to do the math. That's not always the case
though. For instance, if an operator wants to count X gates, the X matrix
equivalence problem has to be solved. For this purpose, DQCsim provides the
gate map interface.

Gate maps deal with the translation between DQCsim's internal format and any
enumeration-based format you may come up with as a plugin developer that
looks like the following:

 - Some enumeration-like type (a name, if you like) determining the type of
   gate. Think `X`, `CNOT`, `MEASURE_Z`, and so on.
 - A number of qubit arguments. Which qubit does what is up to you, depending
   on the gate type.
 - A number of classical arguments, wrapped in an `ArbData`.

Gate maps are constructed using a detector and constructor function for each
gate type. You can of course define these yourself, but unless you need some
exotic parameterized gate or custom gates, you shouldn't have to: DQCsim
provides predefined converters for a wide number of gate types. So, in
practice, you usually only have to tell DQCsim what name or enum variant you
want to use for each gate.

## Measurement results

Measurement objects in DQCsim consist of the following:

 - (usually) the index of the measured qubit;
 - the measured value, which may be zero, one, or undefined (to model a failed
   measurement);
 - an `ArbData` object that may contain additional information about the
   measurement.

The upstream plugin will store the result of the latest measurement performed
on a per-qubit basis. This storage can be queried using the `get_measurement()`
function. Measuring the same qubit twice without calling `get_measurement()` in
between is fine; in this case, the result of the first measurement is
discarded.

DQCsim requires that every qubit in the `measures` list of a gate results in
exactly one measurement being returned. Furthermore, it is illegal to return
measurement data for a qubit that was not measured. This has to do with
internal optimizations in the communication protocol. DQCsim will check whether
you fulfill these requirements, and issue warnings if you don't. The stored
measurement results become undefined in a potentionally non-deterministic way
after violating the protocol in this way, so it is important to fix these
warnings when you get them.

Note that operators do not need to return all measurement results immediately.
Specifically, if they propagate the measurement gate further downstream in some
way, the qubits measured by that gate must *not* be returned immediately.
Instead, these measurement results pass through the `modify_measurement()`
callback when they become available. `modify_measurement()` takes one
measurement result as an argument and can return zero or more measurements,
which will then be passed on to the upstream plugin. The only thing that
matters, ultimately, is that the measurements received by the upstream plugin
correspond exactly to the qubits it measured.

## Passing time

Gates in DQCsim are modeled as being performed sequentially and
instantaneously. Among other things, this allows operators to insert gates into
the gatestream at any time, without having to worry about violating boundary
conditions. However, DQCsim *does* have a basic concept of time.

Specifically, an integral cycle counter is maintained for every gatestream
interface. This cycle counter can be advanced by the specified number of cycles
using the `advance()` function, which results in the `advance()` callback being
called for the downstream plugin. Other than that, DQCsim does nothing with
this timing information.

This mechanism was introduced to provide a standardized way for upstream
plugins to specify how much time is passing to downstream plugins. This is
important specifically for error model operators, which may randomly insert
gates in response to the `advance()` callback to decohere the quantum state.

## Gatestream arbs

In addition to the above, the upstream plugin can send `ArbCmd`s to the
downstream plugin. These operate like synchronous remote procedure calls,
taking an `ArbCmd` as argument and sending an `ArbData` or error message in
response.

This mechanism can for instance be used to tell the downstream plugin to dump
its quantum state for debug purposes.
