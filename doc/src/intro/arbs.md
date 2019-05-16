# Extensibility through arbs

DQCsim allows users to come up with their own features/extensions through
`ArbData` and `ArbCmd` objects, short for arbitrary data and arbitrary
commands. These objects encapsulate both structured and unstructured data
that DQCsim just passes along between the plugins, without caring about what
the data or command actually represents.

## Arbitrary data

An `ArbData` object consists of a JSON-like object (specifically CBOR, which is
a superset of JSON) and a list of binary strings. The JSON-like object can be
used to represent structured data, whereas the binary strings are intended for
unstructured data, or data that's structured in a way that doesn't mesh well
with JSON.

The advantage of JSON data is that it's "annotated" through dictionary keys. It
therefore lends itself better for backward- and forward-compatibility. JSON
objects are also easy to print and make sense of. However, they're rather
heavyweight. This introduces complexity into the plugins, and may therefore
slow down the simulation. In cases where this is a concern, using the binary
string list is probably a better option.

## Arbitrary commands

`ArbCmd`s expand on `ArbData` objects to not only communicate a bit of data,
but also intent. Think of them as remote procedure calls. The desired procedure
to be called is identified by two strings, called the interface ID and the
operation ID. Both of these must match exactly (case-sensitive) for a command
to be accepted. The difference between the two lies within how unsupported
procedures are handled:

 - If the interface ID is known but the operation ID isn't, the target plugin
   must return an error.
 - If the interface ID is not known to the target plugin, it must acknowledge
   the request with an empty return value.

This allows `ArbCmd`s to be used as hints (also known as pragmas) that get
silently ignored if they're not available or applicable.
