# Frontend use cases

Frontends can be subdivided into roughly three different classes: mixed
quantum/classical algorithms, interpreters, and microarchitecture simulators.

The first is the easiest. Instead of writing any kind of simulator, the source
code for the frontend is the algorithm itself. Python would usually be used for
this, since it avoids a compilation step. Whenever your algorithm has to apply
a gate, you simply call the appropriate DQCsim API to send a gate to the
downstream plugin. All that you then have to do to simulate the algorithm is
run `dqcsim my-algorithm.py [backend]`. That's it!

The second frontend plugin class is a plugin that loads a file written in some
domain-specific language for describing quantum algorithms, such as cQASM, and
interprets it into a DQCsim gate stream line by line. The command-line
interface of DQCsim has some "sugaring" built in that allows it to
automatically pick the appropriate interpreter plugin based on the algorithm's
file extension, so simulation is still as easy as
`dqcsim my-algorithm.my-dsl [backend]`.

The third class represents frontends that simulate classical hardware, either
functionally, cycle-accurately, or something in between. Such a plugin may for
instance be largely written in SystemC, or interface with other simulators such
as QuestaSim or GHDL. You can make this as easy or as complex as you need, as
long as the final output remains a gatestream.

Note that unlike most qubit simulators, DQCsim's gatestreams have a concept of
time built into them. This allows error models to model decoherence over time
in an intuitive way, without the frontend needing to insert an identity gate
for each qubit each cycle.
