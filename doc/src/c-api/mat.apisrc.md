# Matrices

The last component we need to describe a quantum gate is a unitary matrix.
DQCsim internally represents all normal gates (that is, everything except
measurements and custom gates) using such matrices as a universal format that
all plugins must be able to deal with. Note that [Gate maps](gm.apigen.md) can
help you with converting between this format and the format your plugin uses,
if they differ.

To prevent DQCsim from turning into a math library, its matrix API is very
basic. Matrices are constructed from a C array of its elements and are
subsequently immutable.

@@@c_api_gen ^dqcs_mat_new$@@@

The following functions can be used to query the size of a matrix.

@@@c_api_gen ^dqcs_mat_len$@@@
@@@c_api_gen ^dqcs_mat_dimension$@@@
@@@c_api_gen ^dqcs_mat_num_qubits$@@@

The C array can of course also be recovered again.

@@@c_api_gen ^dqcs_mat_get$@@@

The primary use of this is to put all the complexity of converting between the
C and internal DQCsim representation of such a matrix in a single place. This
is particularly important for some of the gate map detector and constructor
callbacks. However, DQCsim does provide some matrix operations that are common
when dealing with gate detection and construction, but not so much anywhere
else.

## Matrix equality

A very common operation in DQCsim is matrix equality. An operator plugin may
for instance want to detect whether a matrix is an X matrix. Getting this right
is unfortunately difficult, due to floating point roundoff errors, numerical
instability here and there, or (specifically to quantum gates) differences in
global phase. For this reason, DQCsim provides an equality check function.

@@@c_api_gen ^dqcs_mat_approx_eq$@@@

## Unitary check

DQCsim also exposes a unitary check, which it uses internally here and there
anyway.

@@@c_api_gen ^dqcs_mat_approx_unitary$@@@

## Predefined matrices

DQCsim provides a number of predefined gate matrices. These are identified by
the `dqcs_predefined_gate_t` enumeration.

@@@c_api_gen ^dqcs_predefined_gate_t$@@@

Given such a variant and an `ArbData` object with the parameters described in
the enum variant documentation, a matrix can be constructed.

@@@c_api_gen ^dqcs_mat_predef$@@@

DQCsim also provides the reverse operation: going from a matrix matching a
given gate type to its parameterization. This matrix detection uses the
internal equivalent of `dqcs_mat_approx_eq`, so its parameters are also needed
here.

@@@c_api_gen ^dqcs_mat_is_predef$@@@

Note that these two functions are only the most basic form for constructing and
detecting gates using some higher abstraction level. If you feel like you're
using these functions a lot, you should probably use a [gate map](gm.apigen.md)
instead.

## Control normalization

DQCsim allows controlled quantum gates to be specified either with an explicit
set of control qubits and the non-controlled submatrix, or the full controlled
matrix. The canonical form within DQCsim is the former, as operating on only
the submatrices may improve performance, and gives you controlled gates for
free. In some cases however, the user may wish to convert between the two
representations. DQCsim provides higher-level functions to do this as part of
the gate API, but you can also call the low-level matrix conversion functions
manually as follows.

@@@c_api_gen ^dqcs_mat_add_controls$@@@
@@@c_api_gen ^dqcs_mat_strip_control$@@@

## Basis matrices

TODO: someone who knows what they're talking about should check/correct this
section at some point. I'm mostly working off of hunches. - Jeroen

DQCsim uses 2x2 matrices to represent the basis for a measurement or prep gate
to operate on. The intuitive nature of these matrices is as follows for
measurements:

 - apply the Hermetian/conjugate transpose/inverse of the basis matrix as a
   gate to each measured qubit;
 - do a Z-basis measurement;
 - apply the basis matrix as a gate to each measured qubit.

Basically, the application of the inverse of the matrix rotates the state of
the qubits from the desired basis to the Z basis, then a Z measurement is
applied, then the application of the basis matrix rotates the state back to
the desired basis.

The semantics for prep gates are basically the same:

 - initialize the state of each qubit to |0>;
 - apply the basis matrix as a gate to each targeted qubit.

With this definition, the basis matrices can be written as follows.

@@@c_api_gen ^dqcs_basis_t$@@@

... and constructed as follows:

@@@c_api_gen ^dqcs_mat_basis$@@@

Basis matrices can be compared with the following function. This function
ignores any of the phase differences that don't affect the basis in its
approximate equality function.

@@@c_api_gen ^dqcs_mat_basis_approx_eq$@@@

