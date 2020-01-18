# Gates

The state of a quantum system is modified by means of quantum gates.

## Constructing gates

DQCsim provides four types of gates.

 - Unitary gates: these apply a gate matrix on one or more qubits.
 - Measurement gates: these cause the state of a qubit to be collapsed along
   and measured in some basis.
 - Prep gates: these set the state of a qubit to some value.
 - Custom gates: anything else that the downstream plugin supports.

These are constructed using the following functions. The predefined gates are
[as described earlier](mat.apigen.md).

@@@c_api_gen ^dqcs_gate_new_predef$@@@
@@@c_api_gen ^dqcs_gate_new_predef_one$@@@
@@@c_api_gen ^dqcs_gate_new_predef_two$@@@
@@@c_api_gen ^dqcs_gate_new_predef_three$@@@
@@@c_api_gen ^dqcs_gate_new_unitary$@@@
@@@c_api_gen ^dqcs_gate_new_measurement$@@@
@@@c_api_gen ^dqcs_gate_new_prep$@@@
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

DQCsim provides two ways for interpreting incoming gates: manually querying the
parameters and gate maps. The latter is quite advanced and deserves its own
section (the next one), but let's deal with the manual method first.

The first step for any incoming gate is to query its type.

@@@c_api_gen ^dqcs_gate_type$@@@

This results in the following enumeration. The exact semantics of each type of
gate is listed in the documentation of each enum variant.

@@@c_api_gen ^dqcs_gate_type_t$@@@

The following functions can be used to read the remaining parameters associated
with a gate.

@@@c_api_gen ^dqcs_gate_has_targets$@@@
@@@c_api_gen ^dqcs_gate_targets$@@@
@@@c_api_gen ^dqcs_gate_has_controls$@@@
@@@c_api_gen ^dqcs_gate_controls$@@@
@@@c_api_gen ^dqcs_gate_has_measures$@@@
@@@c_api_gen ^dqcs_gate_measures$@@@
@@@c_api_gen ^dqcs_gate_has_matrix$@@@
@@@c_api_gen ^dqcs_gate_matrix$@@@
@@@c_api_gen ^dqcs_gate_has_name$@@@
@@@c_api_gen ^dqcs_gate_name$@@@
