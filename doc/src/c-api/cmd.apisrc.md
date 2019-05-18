# ArbCmd objects

`ArbCmd` objects are created using `dqcs_cmd_new()`:

@@@c_api_gen ^dqcs_cmd_new$@@@

The interface and operation IDs are immutable after construction. They can be
matched and read using the following functions.

@@@c_api_gen ^dqcs_cmd_@@@

In addition to the IDs, `ArbCmd` objects carry an `ArbData` object along with
them as an argument. This object is accessed by applying `dqcs_arb_*()`
functions directly to the `ArbCmd` handle.
