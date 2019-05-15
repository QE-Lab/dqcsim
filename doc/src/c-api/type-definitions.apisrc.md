# Type definitions

DQCsim defines some types and enumerations. These are documented below. Note
that DQCsim does *not* define any structures; all types used on the interface
are primitive. This should hopefully simplify using the bindings from languages
other than C, which may not support such things.

## Return codes

Almost all functions in DQCsim can fail. They indicate failure through their
return value. For some types this return value is obvious; for instance, `NULL`
is used for functions that return a string or another kind of pointer. For
enumerations, the failure return value is usually 0 or -1. In other cases, the
failure return value will be listed in the function documentation.

There are two special cases: functions that return a boolean and functions that
don't otherwise return a value. These have the following two special
enumerations defined for them:

@@@c_api_gen ^dqcs_return_t$@@@
@@@c_api_gen ^dqcs_bool_return_t$@@@

## Simulator object references

The following types are used to refer to simulator objects.

@@@c_api_gen ^dqcs_handle_t$@@@
@@@c_api_gen ^dqcs_qubit_t$@@@
@@@c_api_gen ^dqcs_plugin_state_t$@@@

## Timekeeping

DQCsim supports timed simulation using integral cycle numbers as a unit. The
following typedef is used to refer to such timestamps.

@@@c_api_gen ^dqcs_cycle_t$@@@

## Misc. enumerations

The following enumerations are used for various purposes within the API.

@@@c_api_gen _t$@@@
