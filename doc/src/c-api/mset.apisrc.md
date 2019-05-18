# Measurement sets

A measurement set encapsulates measurement results for zero or more qubits. It
is therefore actually more like a map/dictionary than a set.

## Constructing measurement sets

A measurement set can be constructed as follows.

```C
dqcs_handle_t mset = dqcs_mset_new();
for (qubit, value = ...; ...; ...) {
    dqcs_handle_t meas = dqcs_meas_new(qubit, value);
    dqcs_mset_set(mset, meas);
    dqcs_handle_delete(meas);
}
```

@@@c_api_gen ^dqcs_mset_new$@@@
@@@c_api_gen ^dqcs_mset_set$@@@

## Iterating over measurement sets

Destructive iteration can be performed as follows if needed.

```C
dqcs_handle_t mset = ...;
while ((dqcs_handle_t meas = dqcs_mset_take_any(mset))) {
    dqcs_qubit_t qubit = dqcs_meas_qubit_get(meas);
    dqcs_measurement_t value = dqcs_meas_value_get(meas);
    dqcs_handle_delete(meas);
    ...
}
```

To iterate nondestructively, one would have to construct a new measurement set
while iterating.

@@@c_api_gen ^dqcs_mset_take_any$@@@

Note that insertion order is *not* preserved. Measurements can also be removed
from a measurement set in a controlled order using the following functions.

@@@c_api_gen ^dqcs_mset_take$@@@
@@@c_api_gen ^dqcs_mset_remove$@@@

## Querying measurement sets

Measurement sets can be queried nondestructively using the following functions.

@@@c_api_gen ^dqcs_mset_@@@
