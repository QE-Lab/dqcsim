//! Gate types and definitions.
//!
//! The types defined here are provided to facilitate plugin developers. They
//! are not to be confused with the [`Gate`] type used in gatestream
//! [`protocol`], and are not relied on in the core of DQCsim.
//!
//! The following gate types are defined in this module:
//!
//! - [`GateType`]: An abstract gate type. The variants in this type carry no
//!                 additional parameters or information about the target
//!                 qubits, with the exception of the [`GateType::U`] variant
//!                 that encodes an abstract unitary gate with the number of
//!                 qubits involved specified.
//!
//! - [`UnboundGate`]: An unbound gate type. The variants in this type specify
//!                    all parameters to determine the behaviour of the gate.
//!                    However, these variants carry no information about the
//!                    target qubits.
//!
//! - [`BoundGate`]: A bound gate type. The variants in this type specify all
//!                  parameters and qubits targets to determine the behaviour
//!                  of the gate.
//!
//! The [`BoundGate`] can always be converted to an [`UnboundGate`] that in
//! turn can always be converted to a [`GateType`]. [`UnboundGate`] variants
//! without additional parameters can be converted to their [`GateType`]
//! variants. [`BoundGate`] instances can be converted to [`Gate`] instances.
//!
//!
//! [`Gate`]: ../types/struct.Gate.html
//! [`protocol`]: ../protocol/index.html
//!
//! [`GateType`]: ./enum.GateType.html
//! [`GateType::U`]: ./enum.GateType.html#variant.U
//! [`UnboundGate`]: ./enum.UnboundGate.html
//! [`BoundGate`]: ./enum.BoundGate.html

use crate::common::types::{Gate, Matrix, QubitRef};
use std::{
    convert::TryFrom,
    f64::consts::{FRAC_1_SQRT_2, PI},
};

/// An abstract gate type.
///
/// The variants in this type carry no additional parameters or information
/// about the target qubits, with the exception of the [`GateType::U`] variant
/// that encodes an abstract Unitary gate with the number of qubits involved
/// specified.
///
/// [`GateType::U`]: ./enum.GateType.html#variant.U
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GateType {
    /// Identity.
    I,
    /// Pauli-X.
    X,
    /// Pauli-Y.
    Y,
    /// Pauli-Z.
    Z,
    /// Hadamard.
    H,
    /// Phase.
    S,
    /// S† (conjugate transpose of S).
    SDAG,
    /// T.
    T,
    /// T† (conjugate transpose of T).
    TDAG,
    /// 90 degree rotation around X-axis.
    RX90,
    /// minus 90 degree rotation around X-axis.
    RXM90,
    /// 180 degree rotation around X-axis.
    RX180,
    /// 90 degree rotation around Y-axis.
    RY90,
    /// minus 90 degree rotation around Y-axis.
    RYM90,
    /// 180 degree rotation around Y-axis.
    RY180,
    /// 90 degree rotation around Y-axis.
    RZ90,
    /// minus 90 degree rotation around Z-axis.
    RZM90,
    /// 180 degree rotation around Z-axis.
    RZ180,
    /// Arbitrary rotation around X-axis.
    RX,
    /// Arbitrary rotation around Y-axis.
    RY,
    /// Arbitrary rotation around Z-axis.
    RZ,
    /// Arbitrary rotation around Z-axis, global phase chosen such that it
    /// works as a submatrix for controlled phase operations.
    Phase,
    /// Same as Phase, but with θ = π/2^k​.
    PhaseK,
    /// Arbitrary rotation around X-, Y- and Z-axis.
    R,
    /// Swap.
    SWAP,
    /// Square root of Swap.
    SQSWAP,
    /// Abstract unitary gate with number of target qubits specified.
    U(usize),
}

/// An unbound gate type.
///
/// The variants in this type specify all parameters to determine the behaviour
/// of the gate. However, these variants carry no information about the target
/// qubits.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnboundGate<'matrix> {
    /// Identity.
    I,
    /// Pauli-X.
    X,
    /// Pauli-Y.
    Y,
    /// Pauli-Z.
    Z,
    /// Hadamard.
    H,
    /// Phase.
    S,
    /// S† (conjugate transpose of S).
    SDAG,
    /// T.
    T,
    /// T† (conjugate transpose of T).
    TDAG,
    /// 90 degree rotation around X-axis.
    RX90,
    /// minus 90 degree rotation around X-axis.
    RXM90,
    /// 180 degree rotation around X-axis.
    RX180,
    /// 90 degree rotation around Y-axis.
    RY90,
    /// minus 90 degree rotation around Y-axis.
    RYM90,
    /// 180 degree rotation around Y-axis.
    RY180,
    /// 90 degree rotation around Y-axis.
    RZ90,
    /// minus 90 degree rotation around Z-axis.
    RZM90,
    /// 180 degree rotation around Z-axis.
    RZ180,
    /// Arbitrary rotation around X-axis with specified angle (θ).
    RX(f64),
    /// Arbitrary rotation around Y-axis with specified angle (θ).
    RY(f64),
    /// Arbitrary rotation around Z-axis with specified angle (θ).
    RZ(f64),
    /// Arbitrary rotation around Z-axis, global phase chosen such that it
    /// works as a submatrix for controlled phase operations.
    Phase(f64),
    /// Same as Phase, but with θ = π/2^k​.
    PhaseK(u64),
    /// Arbitrary rotation around X-, Y- and Z-axis with specified angles
    /// (θ, φ, λ).
    R(f64, f64, f64),
    /// Swap.
    SWAP,
    /// Square root of Swap.
    SQSWAP,
    /// Abstract unitary gate with a reference to specified unitary matrix.
    U(&'matrix Matrix),
}

/// A bound gate type.
///
/// The variants in this type specify all parameters and qubits targets to
/// determine the behaviour of the gate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoundGate<'matrix, 'qref> {
    /// Identity with specified qubit target.
    I(QubitRef),
    /// Pauli-X with specified qubit target.
    X(QubitRef),
    /// Pauli-Y with specified qubit target.
    Y(QubitRef),
    /// Pauli-Z with specified qubit target.
    Z(QubitRef),
    /// Hadamard with specified qubit target.
    H(QubitRef),
    /// Phase with specified qubit target.
    S(QubitRef),
    /// S† (conjugate transpose of S) with specified qubit target.
    SDAG(QubitRef),
    /// T with specified qubit target.
    T(QubitRef),
    /// T† (conjugate transpose of T) with specified qubit target.
    TDAG(QubitRef),
    /// 90 degree rotation around X-axis with specified qubit target.
    RX90(QubitRef),
    /// minus 90 degree rotation around X-axi with specified qubit target.
    RXM90(QubitRef),
    /// 180 degree rotation around X-axis with specified qubit target.
    RX180(QubitRef),
    /// 90 degree rotation around Y-axis with specified qubit target.
    RY90(QubitRef),
    /// minus 90 degree rotation around Y-axis with specified qubit target.
    RYM90(QubitRef),
    /// 180 degree rotation around Y-axis with specified qubit target.
    RY180(QubitRef),
    /// 90 degree rotation around Y-axis with specified qubit target.
    RZ90(QubitRef),
    /// minus 90 degree rotation around Z-axis with specified qubit target.
    RZM90(QubitRef),
    /// 180 degree rotation around Z-axis with specified qubit target.
    RZ180(QubitRef),
    /// Arbitrary rotation around X-axis with specified angle (θ) and qubit
    /// target.
    RX(f64, QubitRef),
    /// Arbitrary rotation around Y-axis with specified angle (θ) and qubit
    /// target.
    RY(f64, QubitRef),
    /// Arbitrary rotation around Z-axis with specified angle (θ) and qubit
    /// target.
    RZ(f64, QubitRef),
    /// Arbitrary rotation around Z-axis, global phase chosen such that it
    /// works as a submatrix for controlled phase operations.
    Phase(f64, QubitRef),
    /// Same as Phase, but with θ = π/2^k​.
    PhaseK(u64, QubitRef),
    /// Arbitrary rotation around X-, Y- and Z-axis with specified angles
    /// (θ, φ, λ) and qubit target.
    R(f64, f64, f64, QubitRef),
    /// Swap with specified qubit targets.
    SWAP(QubitRef, QubitRef),
    /// Square root of Swap with specified qubit targets.
    SQSWAP(QubitRef, QubitRef),
    /// Abstract unitary gate with a reference to specified unitary matrix and
    /// qubit targets.
    U(&'matrix Matrix, &'qref [QubitRef]),
}

impl<'matrix> From<BoundGate<'matrix, '_>> for UnboundGate<'matrix> {
    fn from(bound_gate: BoundGate<'matrix, '_>) -> UnboundGate<'matrix> {
        match bound_gate {
            BoundGate::I(_) => UnboundGate::I,
            BoundGate::X(_) => UnboundGate::X,
            BoundGate::Y(_) => UnboundGate::Y,
            BoundGate::Z(_) => UnboundGate::Z,
            BoundGate::H(_) => UnboundGate::H,
            BoundGate::S(_) => UnboundGate::S,
            BoundGate::SDAG(_) => UnboundGate::SDAG,
            BoundGate::T(_) => UnboundGate::T,
            BoundGate::TDAG(_) => UnboundGate::TDAG,
            BoundGate::RX90(_) => UnboundGate::RX90,
            BoundGate::RXM90(_) => UnboundGate::RXM90,
            BoundGate::RX180(_) => UnboundGate::RX180,
            BoundGate::RY90(_) => UnboundGate::RY90,
            BoundGate::RYM90(_) => UnboundGate::RYM90,
            BoundGate::RY180(_) => UnboundGate::RY180,
            BoundGate::RZ90(_) => UnboundGate::RZ90,
            BoundGate::RZM90(_) => UnboundGate::RZM90,
            BoundGate::RZ180(_) => UnboundGate::RZ180,
            BoundGate::RX(theta, _) => UnboundGate::RX(theta),
            BoundGate::RY(theta, _) => UnboundGate::RY(theta),
            BoundGate::RZ(theta, _) => UnboundGate::RZ(theta),
            BoundGate::Phase(theta, _) => UnboundGate::Phase(theta),
            BoundGate::PhaseK(k, _) => UnboundGate::PhaseK(k),
            BoundGate::R(theta, phi, lambda, _) => UnboundGate::R(theta, phi, lambda),
            BoundGate::SWAP(_, _) => UnboundGate::SWAP,
            BoundGate::SQSWAP(_, _) => UnboundGate::SQSWAP,
            BoundGate::U(matrix, _) => UnboundGate::U(matrix),
        }
    }
}

impl From<BoundGate<'_, '_>> for GateType {
    fn from(bound_gate: BoundGate<'_, '_>) -> GateType {
        UnboundGate::from(bound_gate).into()
    }
}

impl From<BoundGate<'_, '_>> for Matrix {
    fn from(bound_gate: BoundGate<'_, '_>) -> Matrix {
        UnboundGate::from(bound_gate).into()
    }
}

impl From<BoundGate<'_, '_>> for Gate {
    fn from(bound_gate: BoundGate<'_, '_>) -> Gate {
        let matrix = Matrix::from(bound_gate);
        match bound_gate {
            BoundGate::I(q)
            | BoundGate::X(q)
            | BoundGate::Y(q)
            | BoundGate::Z(q)
            | BoundGate::H(q)
            | BoundGate::S(q)
            | BoundGate::SDAG(q)
            | BoundGate::T(q)
            | BoundGate::TDAG(q)
            | BoundGate::RX90(q)
            | BoundGate::RXM90(q)
            | BoundGate::RX180(q)
            | BoundGate::RY90(q)
            | BoundGate::RYM90(q)
            | BoundGate::RY180(q)
            | BoundGate::RZ90(q)
            | BoundGate::RZM90(q)
            | BoundGate::RZ180(q)
            | BoundGate::RX(_, q)
            | BoundGate::RY(_, q)
            | BoundGate::RZ(_, q)
            | BoundGate::Phase(_, q)
            | BoundGate::PhaseK(_, q)
            | BoundGate::R(_, _, _, q) => Gate::new_unitary(vec![q], vec![], matrix),
            BoundGate::SWAP(q1, q2) | BoundGate::SQSWAP(q1, q2) => {
                Gate::new_unitary(vec![q1, q2], vec![], matrix)
            }
            BoundGate::U(matrix, q) => Gate::new_unitary(q.to_vec(), vec![], matrix.clone()),
        }
        .unwrap()
    }
}

impl From<UnboundGate<'_>> for GateType {
    fn from(unbound_gate: UnboundGate<'_>) -> GateType {
        match unbound_gate {
            UnboundGate::I => GateType::I,
            UnboundGate::X => GateType::X,
            UnboundGate::Y => GateType::Y,
            UnboundGate::Z => GateType::Z,
            UnboundGate::H => GateType::H,
            UnboundGate::S => GateType::S,
            UnboundGate::SDAG => GateType::SDAG,
            UnboundGate::T => GateType::T,
            UnboundGate::TDAG => GateType::TDAG,
            UnboundGate::RX90 => GateType::RX90,
            UnboundGate::RXM90 => GateType::RXM90,
            UnboundGate::RX180 => GateType::RX180,
            UnboundGate::RY90 => GateType::RY90,
            UnboundGate::RYM90 => GateType::RYM90,
            UnboundGate::RY180 => GateType::RY180,
            UnboundGate::RZ90 => GateType::RZ90,
            UnboundGate::RZM90 => GateType::RZM90,
            UnboundGate::RZ180 => GateType::RZ180,
            UnboundGate::RX(_) => GateType::RX,
            UnboundGate::RY(_) => GateType::RY,
            UnboundGate::RZ(_) => GateType::RZ,
            UnboundGate::Phase(_) => GateType::Phase,
            UnboundGate::PhaseK(_) => GateType::PhaseK,
            UnboundGate::R(_, _, _) => GateType::R,
            UnboundGate::SWAP => GateType::SWAP,
            UnboundGate::SQSWAP => GateType::SQSWAP,
            UnboundGate::U(matrix) => GateType::U(matrix.num_qubits().unwrap_or(0)),
        }
    }
}

impl From<UnboundGate<'_>> for Matrix {
    fn from(unbound_gate: UnboundGate<'_>) -> Matrix {
        match unbound_gate {
            UnboundGate::I => matrix!(
                1., 0.;
                0., 1.
            ),

            UnboundGate::X => matrix!(
                0., 1.;
                1., 0.
            ),

            UnboundGate::Y => matrix!(
                0.,      (0.,-1.);
                (0., 1.), 0.
            ),

            UnboundGate::Z => matrix!(
                1.,  0.;
                0., (-1.)
            ),

            UnboundGate::H => matrix!(
                FRAC_1_SQRT_2,   FRAC_1_SQRT_2;
                FRAC_1_SQRT_2, (-FRAC_1_SQRT_2)
            ),

            UnboundGate::S => matrix!(
                1.,  0.;
                0., (0., 1.)
            ),

            UnboundGate::SDAG => matrix!(
                1.,  0.;
                0., (0., -1.)
            ),

            UnboundGate::T => matrix!(
                1.,  0.;
                0., (FRAC_1_SQRT_2, FRAC_1_SQRT_2)
            ),

            UnboundGate::TDAG => matrix!(
                1.,  0.;
                0., (FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
            ),

            UnboundGate::RX90 => matrix!(
                 FRAC_1_SQRT_2,      (0., -FRAC_1_SQRT_2);
                (0., -FRAC_1_SQRT_2), FRAC_1_SQRT_2
            ),

            UnboundGate::RXM90 => matrix!(
                FRAC_1_SQRT_2,      (0., FRAC_1_SQRT_2);
                (0., FRAC_1_SQRT_2), FRAC_1_SQRT_2
            ),

            UnboundGate::RX180 => matrix!(
                 0.,                 (0., -FRAC_1_SQRT_2);
                (0., -FRAC_1_SQRT_2), 0.
            ),

            UnboundGate::RY90 => matrix!(
                FRAC_1_SQRT_2, (-FRAC_1_SQRT_2);
                FRAC_1_SQRT_2,   FRAC_1_SQRT_2
            ),

            UnboundGate::RYM90 => matrix!(
                  FRAC_1_SQRT_2,  FRAC_1_SQRT_2;
                (-FRAC_1_SQRT_2), FRAC_1_SQRT_2
            ),

            UnboundGate::RY180 => matrix!(
                0., (-1.);
                1.,  0.
            ),

            UnboundGate::RZ90 => matrix!(
                (FRAC_1_SQRT_2, -FRAC_1_SQRT_2),  0.;
                0.,                              (FRAC_1_SQRT_2, FRAC_1_SQRT_2)
            ),

            UnboundGate::RZM90 => matrix!(
                (FRAC_1_SQRT_2, FRAC_1_SQRT_2),  0.;
                0.,                             (FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
            ),

            UnboundGate::RZ180 => matrix!(
                (0., -1.),  0.;
                0.,        (0., 1.)
            ),

            UnboundGate::SWAP => matrix!(
                1., 0., 0., 0.;
                0., 0., 1., 0.;
                0., 1., 0., 0.;
                0., 0., 0., 1.
            ),

            UnboundGate::SQSWAP => matrix!(
                1., 0.,           0.,         0.;
                0., (0.5, 0.5),  (0.5, -0.5), 0.;
                0., (0.5, -0.5), (0.5, 0.5),  0.;
                0., 0.,           0.,         1.
            ),

            UnboundGate::RX(theta) => {
                let a = c!((0.5 * theta).cos());
                let b = c!(0., -1.) * (0.5 * theta).sin();
                vec![a, b, b, a].into()
            }

            UnboundGate::RY(theta) => {
                let a = c!((0.5 * theta).cos());
                let b = c!((0.5 * theta).sin());
                vec![a, -b, b, a].into()
            }

            UnboundGate::RZ(theta) => {
                let a = c!(0., -0.5 * theta).exp();
                let b = c!(0., 0.5 * theta).exp();
                vec![a, c!(0.), c!(0.), b].into()
            }

            UnboundGate::Phase(theta) => vec![c!(1.), c!(0.), c!(0.), c!(0., theta).exp()].into(),

            UnboundGate::PhaseK(k) => {
                let theta = PI / 2usize.pow(k as u32) as f64;
                vec![c!(1.), c!(0.), c!(0.), c!(0., theta).exp()].into()
            }

            UnboundGate::R(theta, phi, lambda) => {
                let a = (theta / 2.).cos();
                let b = (theta / 2.).sin();
                vec![
                    c!(0., 0.).exp() * a,
                    -c!(0., lambda).exp() * b,
                    c!(0., phi).exp() * b,
                    c!(0., lambda + phi).exp() * a,
                ]
                .into()
            }

            UnboundGate::U(matrix) => matrix.clone(),
        }
    }
}

impl TryFrom<GateType> for UnboundGate<'_> {
    type Error = &'static str;
    fn try_from(gate_type: GateType) -> Result<Self, Self::Error> {
        match gate_type {
            GateType::RX
            | GateType::RY
            | GateType::RZ
            | GateType::Phase
            | GateType::PhaseK
            | GateType::R => Err("gate is parameterized"),
            GateType::U(_) => Err("gate is parameterized"),
            GateType::I => Ok(UnboundGate::I),
            GateType::X => Ok(UnboundGate::X),
            GateType::Y => Ok(UnboundGate::Y),
            GateType::Z => Ok(UnboundGate::Z),
            GateType::H => Ok(UnboundGate::H),
            GateType::S => Ok(UnboundGate::S),
            GateType::SDAG => Ok(UnboundGate::SDAG),
            GateType::T => Ok(UnboundGate::T),
            GateType::TDAG => Ok(UnboundGate::TDAG),
            GateType::RX90 => Ok(UnboundGate::RX90),
            GateType::RXM90 => Ok(UnboundGate::RXM90),
            GateType::RX180 => Ok(UnboundGate::RX180),
            GateType::RY90 => Ok(UnboundGate::RY90),
            GateType::RYM90 => Ok(UnboundGate::RYM90),
            GateType::RY180 => Ok(UnboundGate::RY180),
            GateType::RZ90 => Ok(UnboundGate::RZ90),
            GateType::RZM90 => Ok(UnboundGate::RZM90),
            GateType::RZ180 => Ok(UnboundGate::RZ180),
            GateType::SWAP => Ok(UnboundGate::SWAP),
            GateType::SQSWAP => Ok(UnboundGate::SQSWAP),
        }
    }
}

impl TryFrom<GateType> for Matrix {
    type Error = &'static str;
    fn try_from(gate_type: GateType) -> Result<Self, Self::Error> {
        UnboundGate::try_from(gate_type).map(|unbound_gate| unbound_gate.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: UnboundGate, b: UnboundGate) -> bool {
        Matrix::from(a).approx_eq(&b.into(), 1e-15, true)
    }

    #[test]
    fn gates_as_rotation() {
        assert!(check(UnboundGate::I, UnboundGate::R(0., 0., 0.)));
        assert!(check(UnboundGate::X, UnboundGate::R(PI, 0., PI)));
        assert!(check(UnboundGate::Y, UnboundGate::R(PI, PI / 2., PI / 2.)));
        assert!(check(UnboundGate::Z, UnboundGate::R(0., 0., PI)));
        assert!(check(UnboundGate::H, UnboundGate::R(PI / 2., 0., PI)));
        assert!(check(UnboundGate::S, UnboundGate::R(0., 0., PI / 2.)));
        assert!(check(UnboundGate::SDAG, UnboundGate::R(0., 0., -PI / 2.)));
        assert!(check(UnboundGate::T, UnboundGate::R(0., 0., PI / 4.)));
        assert!(check(UnboundGate::TDAG, UnboundGate::R(0., 0., -PI / 4.)));
    }

    #[test]
    fn gates_coversions() {
        for gate in vec![
            (UnboundGate::I, GateType::I),
            (UnboundGate::X, GateType::X),
            (UnboundGate::Y, GateType::Y),
            (UnboundGate::Z, GateType::Z),
            (UnboundGate::H, GateType::H),
            (UnboundGate::S, GateType::S),
            (UnboundGate::SDAG, GateType::SDAG),
            (UnboundGate::T, GateType::T),
            (UnboundGate::TDAG, GateType::TDAG),
            (UnboundGate::RX90, GateType::RX90),
            (UnboundGate::RXM90, GateType::RXM90),
            (UnboundGate::RX180, GateType::RX180),
            (UnboundGate::RY90, GateType::RY90),
            (UnboundGate::RYM90, GateType::RYM90),
            (UnboundGate::RY180, GateType::RY180),
            (UnboundGate::RZ90, GateType::RZ90),
            (UnboundGate::RZM90, GateType::RZM90),
            (UnboundGate::RZ180, GateType::RZ180),
            (UnboundGate::RX(1.), GateType::RX),
            (UnboundGate::RY(1.), GateType::RY),
            (UnboundGate::Phase(1.), GateType::Phase),
            (UnboundGate::PhaseK(1), GateType::PhaseK),
            (UnboundGate::RZ(1.), GateType::RZ),
            (UnboundGate::R(1., 1., 1.), GateType::R),
            (UnboundGate::SWAP, GateType::SWAP),
            (UnboundGate::SQSWAP, GateType::SQSWAP),
            (
                UnboundGate::U(&Matrix::new(vec![c!(1.), c!(1.), c!(1.), c!(1.)])),
                GateType::U(1),
            ),
        ]
        .into_iter()
        {
            let gate_type: GateType = gate.0.into();
            assert_eq!(gate_type, gate.1);
        }

        let a = QubitRef::from_foreign(1).unwrap();
        let b = QubitRef::from_foreign(2).unwrap();
        for gate in vec![
            (BoundGate::I(a), UnboundGate::I),
            (BoundGate::X(a), UnboundGate::X),
            (BoundGate::Y(a), UnboundGate::Y),
            (BoundGate::Z(a), UnboundGate::Z),
            (BoundGate::H(a), UnboundGate::H),
            (BoundGate::S(a), UnboundGate::S),
            (BoundGate::SDAG(a), UnboundGate::SDAG),
            (BoundGate::T(a), UnboundGate::T),
            (BoundGate::TDAG(a), UnboundGate::TDAG),
            (BoundGate::RX90(a), UnboundGate::RX90),
            (BoundGate::RXM90(a), UnboundGate::RXM90),
            (BoundGate::RX180(a), UnboundGate::RX180),
            (BoundGate::RY90(a), UnboundGate::RY90),
            (BoundGate::RYM90(a), UnboundGate::RYM90),
            (BoundGate::RY180(a), UnboundGate::RY180),
            (BoundGate::RZ90(a), UnboundGate::RZ90),
            (BoundGate::RZM90(a), UnboundGate::RZM90),
            (BoundGate::RZ180(a), UnboundGate::RZ180),
            (BoundGate::RX(1., a), UnboundGate::RX(1.)),
            (BoundGate::RY(1., a), UnboundGate::RY(1.)),
            (BoundGate::PhaseK(1, a), UnboundGate::PhaseK(1)),
            (BoundGate::RZ(1., a), UnboundGate::RZ(1.)),
            (BoundGate::R(1., 1., 1., a), UnboundGate::R(1., 1., 1.)),
            (BoundGate::SWAP(a, b), UnboundGate::SWAP),
            (BoundGate::SQSWAP(a, b), UnboundGate::SQSWAP),
            (
                BoundGate::U(&Matrix::new(vec![c!(1.), c!(1.), c!(1.), c!(1.)]), &[a]),
                UnboundGate::U(&Matrix::new(vec![c!(1.), c!(1.), c!(1.), c!(1.)])),
            ),
        ]
        .into_iter()
        {
            let unbound_gate: UnboundGate = gate.0.into();
            assert_eq!(unbound_gate, gate.1);
        }
    }
}
