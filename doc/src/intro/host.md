# Host use cases

There are two main use cases for host programs. The first – on the short
term probably the most useful – is to provide an easy way to code batch
simulations. Just write a couple nested loops around the simulation code that
swap out plugins or modify parameters and collect the results in some file or
database! DQCsim also lets you insert plugin *threads* into the simulation
pipeline, which provide you with an easy way to collect statistics from various
parts of the gatestream, without needing to compile and interface with a custom
plugin executable.

The second use case for host programs is to model quantum accelerators. In this
case, the host program is probably a larger software package, such as a genome
sequencing tool, that could be accelerated using a quantum chip. This chip may
not exist yet, but by loading DQCsim's host library you can still simulate
it... as long as it doesn't use too many qubits of course. DQCsim's host
interface is defined fairly generically, such that it should be possible to
make a drop-in replacement for the control systems of a real quantum computer
later on!
