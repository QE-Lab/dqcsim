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

@@@c_api_gen ^dqcs_mat_approx_eq$@@@
@@@c_api_gen ^dqcs_mat_add_controls$@@@
@@@c_api_gen ^dqcs_mat_strip_control$@@@
