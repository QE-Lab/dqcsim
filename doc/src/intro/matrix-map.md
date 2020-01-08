# Matrix and MatrixMap

To facilitate working with gate operations defined by unitary matrices DQCsim
provides the `Matrix` and `MatrixMap` types.

## Matrix

Qubit gate operations encoded using their unitary matrix wrap their complex
elements in the `Matrix` type. This type provides a comparison method that can
be used to approximately compare matrices. This comparison method enables a
mechanism to link matrices to user-defined types. This mechanism is provided by
the `MatrixMap` type.

## MatrixMap

A `MatrixMap` is constructed with a set of so-called detector functions, that
given a `Matrix` return a user-defined type, possibly indicating a gate type or
name.

A `MatrixMap` is constructed with a `MatrixMapBuilder` that allows adding
detector functions to the map. Users can add keys to detector functions and
define a return type for the detector functions in their map.
Detector functions match an input `Matrix` to a user-defined output type. With
the `MatrixMap` users can try all registered detectors or a subset based on the
detector function keys.

DQCsim provides a default `MatrixMap` that returns variants of the
`UnboundGate` type based on the incoming matrices. Plugin developers can choose
to map these variants to their gate definitions instead of defining their own
gate detection mechanism.
