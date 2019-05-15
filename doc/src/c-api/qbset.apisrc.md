# Qubits

While DQCsim does not perform any quantum simulation on its own, and therefore
does not maintain any kind of state space to refer to or measure, it is still
very important to be able to *refer* to qubits. This is done using integers,
specifically of the type `dqcs_qubit_t`, in a way that is not dissimilar from
handles. The biggest difference compared to handles is that the entity that is
referred to is not owned by DQCsim, but rather by the downstream plugin.

## Allocating and freeing qubits

To be as flexible as possible, DQCsim allows qubits to be allocated and freed
at any time. There is no physical analogue to this, but then again, a
conventional computer cannot create and destroy physical bits out of thin air,
either. This dynamic allocation intends to help solve two problems:

 - Operator plugins that perform a logical-to-physical mapping need to know
   which logical qubits are actually in use.
 - By doing allocation on-the-fly, a quantum algorithm doesn't need to specify
   how many qubits it's going to need before it's started. It can for instance
   interpret a file first, perform some classical computations, and determine
   the qubit requirements based on that.
 - Quantum simulations take up a lot of memory. Depending on the
   implementation, they may be able to use liveness information for
   optimizations.

The numbers assigned to the qubit references/handles are guaranteed to be
sequential and unique within a simulation. Zero is reserved, so the first
allocated qubit is always qubit 1. The second is qubit 2, and so on. Even if
we free both of those, the next qubit will still be qubit 3. Implementation
code can rely on this behavior if it wants.

Since qubits are owned by the downstream plugin, there is no function to ask
DQCsim itself for a new qubit. Instead, this is done using
`dqcs_plugin_allocate()`. Its inverse is `dqcs_plugin_free()`. These functions
require a plugin state, which is only available from within plugin callback
functions. Specifically, the functions can only be called by running frontend
or operator plugins.

## Sets of qubits

Many things within DQCsim operate not on a single qubit, but on a set of
qubits. Such a set is represented through a `qbset` handle.

### Constructing a qubit set

A qubit set can be constructed as follows. It is assumed that the qubits have
already been allocated.

```C
dqcs_handle_t qbset = dqcs_qbset_new();
for (qubit = ...; ...; ...) {
    dqcs_qbset_push(qbset, qubit);
}
```

@@@c_api_gen ^dqcs_qbset_new$@@@
@@@c_api_gen ^dqcs_qbset_push$@@@

### Iterating over a qubit set

Iterating over a qubit set can be done as follows.

```C
dqcs_handle_t qbset = ...;
dqcs_qubit_t qubit = 0;
while (qubit = dqcs_qbset_pop(qbset)) {
    ...
}
dqcs_handle_delete(qbset);
```

@@@c_api_gen ^dqcs_qbset_pop$@@@

Note that insertion order is maintained (in FIFO order). This means that qubit
sets can be used for specifying the target qubits of a multi-qubit gate.

Note also that iteration is destructive: the set will be empty when iteration
completes. If you need to iterate over a set multiple times, you can make a
copy first.

@@@c_api_gen ^dqcs_qbset_copy$@@@

### Querying qubit sets

You can also query qubit sets non-destructively using the following two
functions.

@@@c_api_gen ^dqcs_qbset_contains$@@@
@@@c_api_gen ^dqcs_qbset_len$@@@
