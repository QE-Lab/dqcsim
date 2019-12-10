# Matrix maps

In most cases, having gates defined by their unitary matrix is good enough for
a simulation. Only the frontend needs to know which gate is a Pauli X gate, or
a Hadamard, or some exotic rotation; the backend just has to perform the math.
However, in some cases, operators or even the backend may need to know which
gate is which, for instance to record statistics or to optimize some
calculations. This is what matrix maps are for.

Put simply, a matrix map is used to match a set of known gates against an
incoming matrix from the upstream plugin in an intelligent way. You can match
incoming matrices against a set of gates provided to you by DQCsim, define your
own detection matrices, or define callback functions that check for
parameterized gates. Results are cached, so receiving the exact same gate
multiple times (as is often the case) is efficient even if the callback
functions are complex. Finally, an API is provided for doing numerically stable
fuzzy matrix equality checks, optionally ignoring differences in global phase,
for use in your own callback functions.

More formally, a matrix map consists of a number of callback functions, each
matching one gate type, possibly parameterized. Each callback function is
associated with an immutable `void*` key, which can be returned as part of the
match result. You can use this key to point to any data structure you may want
to define to identify the gates; anything from a number to a string to a full
AST node will do. The callback functions can also return an `ArbData` object,
called the parameter data. As the name implies, this is intended for storing
the parameters for parameterized gates, such as the angle for an RX gate.

When a match is performed, the matrix, key, and parameter data is stored in a
hashmap with the matrix as the key. Matrices that do not match any of the gate
detector functions are also stored in the map. Whenever a gate is to be
detected, the hashmap is checked for an exact, bitwise match first, allowing
the callback functions to be short-circuited altogether. **It is therefore
important that the callback functions are completely stateless;** when
receiving the same matrix twice, they must return the same result twice.

## Built-in gates

As mentioned, DQCsim provides a number of common gate detector functions for
you to use. These gate detectors are selected by way of the following `enum`.

@@@c_api_gen ^dqcs_internal_gate_t$@@@

## Constructing a matrix map

Matrix maps are constructed using a temporary object called a matrix map
builder. First, the builder itself must be constructed with `dqcs_mmb_new()`.

@@@c_api_gen ^dqcs_mmb_new$@@@

Once constructed, you can add detectors to the map with the following
functions.

@@@c_api_gen ^dqcs_mmb_add_defaults$@@@
@@@c_api_gen ^dqcs_mmb_add_internal$@@@
@@@c_api_gen ^dqcs_mmb_add_fixed$@@@
@@@c_api_gen ^dqcs_mmb_add_user$@@@

All the supplied callback functions are exclusively called from the thread
in which the matrix map is constructed.

The non-parameterized built-in matrix detectors all make use of the same
fuzzy matrix comparison algorithm. You can also use this algorithm in your own
callback function using the following API call.

@@@c_api_gen ^dqcs_mat_compare$@@@

Note that the order in which the `dqcs_mmb_add_*()` functions are called is
important: when a new matrix is encountered, the matrix map will call the
callback functions in insertion order and return the first match. Therefore,
more specialized gate detectors should be added first. For example, a Pauli X
gate detector must be inserted before a parameterized RX gate, otherwise the
Pauli X will be detected as RX(pi).

Finally, complete the construction using `dqcs_mm_new()`.

@@@c_api_gen ^dqcs_mm_new$@@@

## Using a matrix map

Once the matrix map is contructed, matrices can be identified using the
following functions.

@@@c_api_gen ^dqcs_mm_map_@@@

Finally, the following function is available to clear the detection cache.

@@@c_api_gen ^dqcs_mm_clear_cache$@@@
