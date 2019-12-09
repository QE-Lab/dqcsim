# Gates

The state of a quantum system is modified by means of quantum gates.

## Constructing gates

DQCsim provides three types of gates:

 - Unitary gates: these apply a Pauli matrix on one or more qubits.
 - Measurement gates: these cause the state of a qubit to be collapsed along
   and measured in the Z basis.
 - Custom gates: anything else that the downstream plugin supports.

These are constructed using the following functions.

@@@c_api_gen ^dqcs_gate_new_unitary$@@@
@@@c_api_gen ^dqcs_gate_new_measurement$@@@
@@@c_api_gen ^dqcs_gate_new_custom$@@@

## Control qubit representation

A gatestream source is allowed to specify controlled gates either using
DQCsim's separate list of control qubits (this is the recommended way), by
using an explicitly controlled gate matrix and using only the target qubit
list, or a even mix of the two. The following two functions, primarily intended
for gatestream sinks, can be used to convert between these representations.

@@@c_api_gen ^dqcs_gate_reduce_control@@@
@@@c_api_gen ^dqcs_gate_expand_control@@@

## Attached classical data

Classical information can be attached to any gate using the `ArbData`
protocol: gate handles support all the `dqcs_arb_*()` API calls. This is
primarily intended for custom gates.

## Interpreting gates

Backend and operator plugins have to process incoming gates using the following
algorithm to be compliant with DQCsim's interface.

 - If the gate has a name (equivalent to it being a custom gate), defer to the
   custom gate logic identified by this name. Name matching should be
   case-sensitive. If an unsupported/unknown gate is requested, an error must
   be generated. The custom gate logic may make use of the `ArbData` attached
   to the gate object.

 - If the gate doesn't have a name and doesn't have any `ArbData` attached:

    - If the gate has target qubits, no control qubits, and a Pauli matrix,
      apply the Pauli matrix to the target qubits.

    - If the gate has target qubits, control qubits, and a Pauli matrix,
      convert the Pauli matrix into a controlled gate with the appropriate
      number of control qubits, and then apply it to the concatenation of the
      control and target qubit sets.

    - If the gate has measurement qubits, collapse the state of these qubits in
      the Z basis and return the measurements. The random number generator
      provided by DQCsim (or another PRNG seeded by DQCsim's RNG) should be
      used to collapse the state.

 - If the gate has no name but does have `ArbData`, the gate *may* be
   interpreted in a customized way. For instance, this data can be used to
   apply random errors to the gate. However, it is recommended to not change
   the functionality too much; that's what custom gates are for.

Note that the above implies that a gate can consist of both a Pauli gate and
one or more measurements, to be applied in that order. It is currently however
impossible to construct such a gate using the C API.

The following functions can be used to read the data associated with a gate.

@@@c_api_gen ^dqcs_gate_@@@
