# ArbData and ArbCmds

Before we get into defining the host and gatestream interfaces, we need to
define a generic objects through which data and user-defined commands can be
communicated. We've named these objects `ArbData` and `ArbCmd`s, short for
arbitrary data and arbitrary commands.

These objects are used wherever something needs to be communicated that DQCsim
doesn't necessarily need to know about. In some cases they'll be accompanied by
additional data that *is* defined/standardized by DQCsim, to prevent everyone
from rolling their own protocols for common stuff. After all, that would make
swapping out different plugins a lot of trouble.

## Arbitrary data

An `ArbData` object consists of a [JSON](https://en.wikipedia.org/wiki/JSON)-like
object (specifically [CBOR](https://en.wikipedia.org/wiki/CBOR), which is a
superset of JSON) and a list of binary strings. The JSON-like object can be
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

To test if a plugin supports a certain interface programmatically, you can send
an `ArbCmd` with the respective interface ID and a dummy operation ID, such as
a single underscore. If this returns an error, the interface is supported, per
the default behavior described above.

When you need to make a new `ArbCmd`, it is recommended to set the interface ID
to the name of your plugin. Alternatively, if you intend to add support for
your interface in multiple plugins, come up with a sane name for the interface
as a whole. Make sure to document the interfaces your plugin supports!
