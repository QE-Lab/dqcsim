# Handles

The API is based upon a handle system for referring to simulator data.
Handles are like cross-language references or pointers: they point to a
piece of data in the simulator, but don't contain it.

The usage of handles implies that the complex data structures contained
within the simulator never actually leave the simulator. Instead, when the
simulator needs to pass data to you, it returns a handle, which you can use
to access the contents of the referred structure through a number of API
calls. Similarly, when you need to pass something to the simulator, you
construct an object through a number of API calls, and then pass the handle
to the simulator.

## Operating on handles

Handles can represent a number of different object types. Based on the type
of object the handle represents, different interfaces are supported. For
instance, `ArbCmd` objects support `handle`, `arb`, and `cmd`, while
`ArbData` objects only support `handle` and `arb`. You can find an
exhaustive list of all handle types and the interfaces they support in the
documentation for `dqcs_handle_type_t`. Note that all normal handles
support the `handle` interface.

The name of the API functions directly corresponds with the name of the
interface it requires the primary handle it operates on to have: the
functions have the form `dqcs_<interface>_*`.

## Functions common to all handles

The following functions are available for all handle types.

@@@c_api_gen ^dqcs_handle(?!.*_t$)@@@

## Handles and multithreading

The global state that the API calls operate on is purely *thread-local*.
This means that you can't exchange API objects/handles between threads.
However, this also makes the API perfectly thread-safe.
