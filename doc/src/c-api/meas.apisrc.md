# Singular measurements

Singular measurements, i.e. a single measurement for a single qubit, are
represented through `meas` handles.

## Constructing measurement objects

Measurement objects are constructed using `dqcs_meas_new()`.

@@@c_api_gen ^dqcs_meas_new$@@@

They are also mutable after construction.

@@@c_api_gen ^dqcs_meas_.*_set$@@@

## Attaching custom data

Measurement objects support the `ArbData` protocol for attaching custom data.
That is, all `dqcs_arb_*()` API calls can be applied to measurement handles.

## Querying measurement objects

The measurement value and qubit reference can be queried from a measurement
object as follows.

@@@c_api_gen ^dqcs_meas_.*_get$@@@
