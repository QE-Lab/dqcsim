# Gate maps

Representing gates with matrices is a great way to ensure plugin compatibility;
unlike using for example names for gates, the matrix unambiguously represents
the mathematical operation that is to be performed. However, there are cases
where you may want to distinguish whether a gate is for instance one of the
Pauli gates or something else. You could do this with `dqcs_mat_approx_eq()`,
but if you have a lot of gates your code will explode, be error-prone
(especially if you want to do the reverse operation as well), and may not be
very efficient. More generally, a plugin may want to use its own higher-level
internal gate representation, and convert between that and DQCsim's
matrix-based representation.

Gate maps intend to solve this problem. You can define any data structure to
represent your gates as long as it can map to/from the following:

 - any kind of key (a `void*`) defining the type of gate.
 - a number of qubit arguments.
 - optionally, an `ArbData` representing parameters if your definition of a
   gate type is parameterized.

You can then use a gate map to convert between that representation and DQCsim's
representation, typically in both directions. Going from DQCsim's matrix
representation to your plugin's representation is called detection, while the
opposite direction is called construction. Once you have a gate map, you can
use the following functions to do this.

@@@c_api_gen ^dqcs_gm_detect$@@@
@@@c_api_gen ^dqcs_gm_construct_one$@@@
@@@c_api_gen ^dqcs_gm_construct_two$@@@
@@@c_api_gen ^dqcs_gm_construct_three$@@@
@@@c_api_gen ^dqcs_gm_construct$@@@

## Converters

Conceptually, a gate map consists of a number of converter objects, each
typically consisting of a detector function and a constructor function.
In the most generic case, the detector takes a DQCsim gate as its input, and
converts it to a qubit set and an `ArbData` if it recognizes the gate, while
the constructor performs the inverse operation. Detection is usually fuzzy to
account for floating-point inaccuracies, while construction is as exact as
possible.

These converter objects are stored in the gate map as the values of an ordered
map, for which the key is the user-defined `void*` key defining the type of
gate. Thus, each converter represents a single gate type. When a gate is to be
detected, DQCsim will call each converter's detector function in insertion
order until one of the detectors returns a match. When a gate is to be
constructed, it simply maps the gate type key to the appropriate converter and
calls only its constructor.

The most generic converter described above can be added to a map with the
following function, but it is also the most complicated to implement.

@@@c_api_gen ^dqcs_gm_add_custom$@@@

There is also a specialized version that detects unitary gate matrices instead
of complete gates. This version deals with distinguishing between unitary,
measurement, and custom gates for you. It also converts between DQCsim's
seperate target/control qubit set and the single gate-type-sensitive qubit set
in the plugin representation for you.

@@@c_api_gen ^dqcs_gm_add_custom_unitary$@@@

More likely, though, you just want to detect the usual gates, like X, H, swap,
and so on. To help you do this, DQCsim includes built-in converters for every
`dqcs_predefined_gate_t`, which you can add to the map with the following,
much simpler function.

@@@c_api_gen ^dqcs_gm_add_predef_unitary$@@@

You can also easily detect gates with a special, fixed matrix.

@@@c_api_gen ^dqcs_gm_add_fixed_unitary$@@@

Finally, you can detect Z-axis measurement gates with the following built-in
detector.

@@@c_api_gen ^dqcs_gm_add_measure$@@@

## Caching

The detection logic in a gate map includes a cache to improve performance
when the same gate is received a number of times. This is quite typical, as
algorithms typically use only a small subset of gates frequently. There are
four different caching strategies:

 - The cache key is an exact copy of the incoming gate, so the cache only hits
   when the exact gate is received twice. In this case, the cache maps directly
   to the result of the previous detection, so no detector function needs to be
   run. This is the safest option.

 - The cache ignores differing `ArbData` attachments to the incoming gates.
   This is valid only if whether a detector function matches or not is not
   sensitive to this `ArbData`. However, the cache only maps to the matching
   detection function, and calls it again for every cache hit. Thus, the
   `ArbData` returned by the detector can still depend on the gate's `ArbData`.

 - The cache key is a copy of the incoming gate, but without the qubit
   references. That is, for example an X gate applied to q1 is considered equal
   to an X gate applied to q2. This is valid only if whether a detector
   function matches or not is not sensitive to the qubit references. It can,
   however, be sensitive to the *amount* of qubits, and as with the `ArbData`
   insensitivity described above, the detector is still called for each cache
   hit and can thus still return a different qubit set.

 - A combination of the latter two, i.e., the cache is not sensitive to either
   the gate's `ArbData` or to the qubit references.

## Preprocessing

To be as compatible with other plugins as possible, you may want to preprocess
the incoming gates with either `dqcs_gate_reduce_control()` or
`dqcs_gate_expand_control()`. We already went over these in the previous
section, but their description is repeated here for convenience.

@@@c_api_gen ^dqcs_gate_reduce_control$@@@
@@@c_api_gen ^dqcs_gate_expand_control$@@@

When you apply `dqcs_gate_reduce_control()` to each incoming gate before
passing it to `dqcs_gm_detect()`, you ensure that if the upstream plugin is
sending for instance a CNOT using a complete two-qubit gate matrix and two
target qubits, it will still be detected as a controlled X gate with one
control qubit, instead of some different gate.

You can also choose to do the opposite, converting from for instance DQCsim's
controlled X representation to a full CNOT matrix using
`dqcs_gate_expand_control`. However, in this case you'll have to detect
controlled matrices with `dqcs_gm_add_fixed_unitary()` or a fully custom
implementation, as DQCsim only provided predefined matrices for the
non-controlled (sub)matrices.

## Constructing a gate map

Having read all of the above, you should be ready to construct a new gate map.
The construction function takes the caching strategy as its parameters, as well
as two optional callback functions used to compare and hash your plugin's key
type, needed for the internal hashmap mapping from key to converter object.

@@@c_api_gen ^dqcs_gm_new$@@@
