# ArbCmd objects

`ArbCmd` objects are used to communicate intent between plugins. Think of them
as remote procedure calls. The desired procedure to be called is identified by
two strings, called the interface ID and the operation ID. Both of these must
match exactly (case-sensitive) for a command to be accepted. The difference
between the two lies within how unsupported procedures are handled:

 - If the interface ID is known but the operation ID isn't, the target plugin
   must return an error.
 - If the interface ID is not known to the target plugin, it must acknowledge
   the request with an empty return value.

This allows `ArbCmd`s to be used as hints (also known as pragmas) that get
silently ignored if they're not available or applicable.

`ArbCmd` objects are created using `dqcs_cmd_new()`:

@@@c_api_gen ^dqcs_cmd_new$@@@

The interface and operation IDs are immutable after construction. They can be
matched and read using the following functions.

@@@c_api_gen ^dqcs_cmd_@@@

In addition to the IDs, `ArbCmd` objects carry an `ArbData` object along with
them as an argument. This object is accessed by applying `dqcs_arb_*()`
functions directly to the `ArbCmd` handle.
