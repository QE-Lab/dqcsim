# Operator use cases

Following up on the above, a prime example of an operator plugin is an error
model. Any error model that can be described by modifying the unitary matrices
of the gates requested by the frontend, by inserting gates to model decoherence
over time, by modifying measurement results, or any combination thereof, can
be described this way. Separating the error models from the (perfect) qubit
simulation allows you to mix and match both error model and simulation accuracy
by just changing DQCsim's command line.

Operators are more powerful than that, though. Imagine for instance that you're
developing a new map-and-route algorithm. You have to test it at some point,
but adding a pass to any kind of compiler is notoriously difficult, especially
if the compiler is written in a language that you're not familiar with. You may
be inclined to throw your own quick-and-dirty program together instead, to be
able to iterate quickly while you debug your algorithm. Then you write your
paper, get a new idea (or deadline), and never get around to turning the
algorithm into something that someone else can use... at least not without
trudging through undocumented code to figure out how to use it. DQCsim
operators provide another alternative: writing an operator plugin should be
easier than starting from scratch, *and* it allows other people to use your
algorithm more easily since the interface is standardized! Operators have full
control over the way gates are passed through to the next plugin, so modifying
qubit indices or inserting swap gates is not a problem.

Operators are also useful for debugging. You might want to dump the gatestream
produced by your mapping algorithm in addition to just simulating it, for
instance. Easy – just write another operator that "tee's" the gatestream to a
file.

An operator also has the power to allocate additional qubits. You can for
instance model an upstream qubit as multiple downstream qubits to model some
error correction code. You can then easily test the effect of the code easily
by swapping out some error operators and the code operator itself.
