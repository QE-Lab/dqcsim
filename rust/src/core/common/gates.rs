//! Gate types and definitions.
//!
//! The types defined here are provided to facilitate plugin developers. They
//! are not to be confused with the [`Gate`] type used in gatestream
//! [`protocol`], and are not relied on in the core of DQCsim.
//!
//! The following gate types are defined in this module:
//!
//! - [`UnitaryGateType`]: An abstract gate type. The variants in this type
//!                        carry no additional parameters or information about
//!                        the target qubits, with the exception of the
//!                        [`UnitaryGateType::U`] variant that encodes an
//!                        abstract unitary gate with the number of qubits
//!                        involved specified.
//!
//! - [`UnboundUnitaryGate`]: An unbound gate type. The variants in this type
//!                           specify all parameters to determine the behavior
//!                           of the gate. However, these variants carry no
//!                           information about the target qubits.
//!
//! - [`BoundUnitaryGate`]: A bound gate type. The variants in this type
//!                         specify all parameters and qubits targets to
//!                         determine the behaviour of the gate.
//!
//! The [`BoundUnitaryGate`] can always be converted to an
//! [`UnboundUnitaryGate`] that in turn can always be converted to a
//! [`UnitaryGateType`]. [`UnboundUnitaryGate`] variants without additional
//! parameters can be converted to their [`UnitaryGateType`] variants.
//! [`BoundUnitaryGate`] instances can be converted to [`Gate`] instances.
//!
//!
//! [`Gate`]: ../types/struct.Gate.html
//! [`protocol`]: ../protocol/index.html
//!
//! [`UnitaryGateType`]: ./enum.UnitaryGateType.html
//! [`UnitaryGateType::U`]: ./enum.UnitaryGateType.html#variant.U
//! [`UnboundUnitaryGate`]: ./enum.UnboundUnitaryGate.html
//! [`BoundUnitaryGate`]: ./enum.BoundUnitaryGate.html

use crate::common::{
    converter::{
        Converter, FixedMatrixConverter, MatrixConverterArb, PhaseKMatrixConverter,
        PhaseMatrixConverter, RMatrixConverter, RxMatrixConverter, RyMatrixConverter,
        RzMatrixConverter, UMatrixConverter, UnitaryConverter, UnitaryGateConverter,
    },
    types::{ArbData, Gate, Matrix, QubitRef},
};
use std::{
    convert::{TryFrom, TryInto},
    f64::consts::{FRAC_1_SQRT_2, PI},
};

/// An abstract gate type.
///
/// The variants in this type carry no additional parameters or information
/// about the target qubits, with the exception of the [`UnitaryGateType::U`] variant
/// that encodes an abstract Unitary gate with the number of qubits involved
/// specified.
///
/// [`UnitaryGateType::U`]: ./enum.UnitaryGateType.html#variant.U
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum UnitaryGateType {
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
pub enum UnboundUnitaryGate<'matrix> {
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
pub enum BoundUnitaryGate<'matrix, 'qref> {
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

impl<'matrix> From<BoundUnitaryGate<'matrix, '_>> for UnboundUnitaryGate<'matrix> {
    fn from(bound_gate: BoundUnitaryGate<'matrix, '_>) -> UnboundUnitaryGate<'matrix> {
        match bound_gate {
            BoundUnitaryGate::I(_) => UnboundUnitaryGate::I,
            BoundUnitaryGate::X(_) => UnboundUnitaryGate::X,
            BoundUnitaryGate::Y(_) => UnboundUnitaryGate::Y,
            BoundUnitaryGate::Z(_) => UnboundUnitaryGate::Z,
            BoundUnitaryGate::H(_) => UnboundUnitaryGate::H,
            BoundUnitaryGate::S(_) => UnboundUnitaryGate::S,
            BoundUnitaryGate::SDAG(_) => UnboundUnitaryGate::SDAG,
            BoundUnitaryGate::T(_) => UnboundUnitaryGate::T,
            BoundUnitaryGate::TDAG(_) => UnboundUnitaryGate::TDAG,
            BoundUnitaryGate::RX90(_) => UnboundUnitaryGate::RX90,
            BoundUnitaryGate::RXM90(_) => UnboundUnitaryGate::RXM90,
            BoundUnitaryGate::RX180(_) => UnboundUnitaryGate::RX180,
            BoundUnitaryGate::RY90(_) => UnboundUnitaryGate::RY90,
            BoundUnitaryGate::RYM90(_) => UnboundUnitaryGate::RYM90,
            BoundUnitaryGate::RY180(_) => UnboundUnitaryGate::RY180,
            BoundUnitaryGate::RZ90(_) => UnboundUnitaryGate::RZ90,
            BoundUnitaryGate::RZM90(_) => UnboundUnitaryGate::RZM90,
            BoundUnitaryGate::RZ180(_) => UnboundUnitaryGate::RZ180,
            BoundUnitaryGate::RX(theta, _) => UnboundUnitaryGate::RX(theta),
            BoundUnitaryGate::RY(theta, _) => UnboundUnitaryGate::RY(theta),
            BoundUnitaryGate::RZ(theta, _) => UnboundUnitaryGate::RZ(theta),
            BoundUnitaryGate::Phase(theta, _) => UnboundUnitaryGate::Phase(theta),
            BoundUnitaryGate::PhaseK(k, _) => UnboundUnitaryGate::PhaseK(k),
            BoundUnitaryGate::R(theta, phi, lambda, _) => UnboundUnitaryGate::R(theta, phi, lambda),
            BoundUnitaryGate::SWAP(_, _) => UnboundUnitaryGate::SWAP,
            BoundUnitaryGate::SQSWAP(_, _) => UnboundUnitaryGate::SQSWAP,
            BoundUnitaryGate::U(matrix, _) => UnboundUnitaryGate::U(matrix),
        }
    }
}

impl From<BoundUnitaryGate<'_, '_>> for UnitaryGateType {
    fn from(bound_gate: BoundUnitaryGate<'_, '_>) -> UnitaryGateType {
        UnboundUnitaryGate::from(bound_gate).into()
    }
}

impl From<BoundUnitaryGate<'_, '_>> for Matrix {
    fn from(bound_gate: BoundUnitaryGate<'_, '_>) -> Matrix {
        UnboundUnitaryGate::from(bound_gate).into()
    }
}

impl From<BoundUnitaryGate<'_, '_>> for Gate {
    fn from(bound_gate: BoundUnitaryGate<'_, '_>) -> Gate {
        let matrix = Matrix::from(bound_gate);
        match bound_gate {
            BoundUnitaryGate::I(q)
            | BoundUnitaryGate::X(q)
            | BoundUnitaryGate::Y(q)
            | BoundUnitaryGate::Z(q)
            | BoundUnitaryGate::H(q)
            | BoundUnitaryGate::S(q)
            | BoundUnitaryGate::SDAG(q)
            | BoundUnitaryGate::T(q)
            | BoundUnitaryGate::TDAG(q)
            | BoundUnitaryGate::RX90(q)
            | BoundUnitaryGate::RXM90(q)
            | BoundUnitaryGate::RX180(q)
            | BoundUnitaryGate::RY90(q)
            | BoundUnitaryGate::RYM90(q)
            | BoundUnitaryGate::RY180(q)
            | BoundUnitaryGate::RZ90(q)
            | BoundUnitaryGate::RZM90(q)
            | BoundUnitaryGate::RZ180(q)
            | BoundUnitaryGate::RX(_, q)
            | BoundUnitaryGate::RY(_, q)
            | BoundUnitaryGate::RZ(_, q)
            | BoundUnitaryGate::Phase(_, q)
            | BoundUnitaryGate::PhaseK(_, q)
            | BoundUnitaryGate::R(_, _, _, q) => Gate::new_unitary(vec![q], vec![], matrix),
            BoundUnitaryGate::SWAP(q1, q2) | BoundUnitaryGate::SQSWAP(q1, q2) => {
                Gate::new_unitary(vec![q1, q2], vec![], matrix)
            }
            BoundUnitaryGate::U(matrix, q) => Gate::new_unitary(q.to_vec(), vec![], matrix.clone()),
        }
        .unwrap()
    }
}

impl From<UnboundUnitaryGate<'_>> for UnitaryGateType {
    fn from(unbound_gate: UnboundUnitaryGate<'_>) -> UnitaryGateType {
        match unbound_gate {
            UnboundUnitaryGate::I => UnitaryGateType::I,
            UnboundUnitaryGate::X => UnitaryGateType::X,
            UnboundUnitaryGate::Y => UnitaryGateType::Y,
            UnboundUnitaryGate::Z => UnitaryGateType::Z,
            UnboundUnitaryGate::H => UnitaryGateType::H,
            UnboundUnitaryGate::S => UnitaryGateType::S,
            UnboundUnitaryGate::SDAG => UnitaryGateType::SDAG,
            UnboundUnitaryGate::T => UnitaryGateType::T,
            UnboundUnitaryGate::TDAG => UnitaryGateType::TDAG,
            UnboundUnitaryGate::RX90 => UnitaryGateType::RX90,
            UnboundUnitaryGate::RXM90 => UnitaryGateType::RXM90,
            UnboundUnitaryGate::RX180 => UnitaryGateType::RX180,
            UnboundUnitaryGate::RY90 => UnitaryGateType::RY90,
            UnboundUnitaryGate::RYM90 => UnitaryGateType::RYM90,
            UnboundUnitaryGate::RY180 => UnitaryGateType::RY180,
            UnboundUnitaryGate::RZ90 => UnitaryGateType::RZ90,
            UnboundUnitaryGate::RZM90 => UnitaryGateType::RZM90,
            UnboundUnitaryGate::RZ180 => UnitaryGateType::RZ180,
            UnboundUnitaryGate::RX(_) => UnitaryGateType::RX,
            UnboundUnitaryGate::RY(_) => UnitaryGateType::RY,
            UnboundUnitaryGate::RZ(_) => UnitaryGateType::RZ,
            UnboundUnitaryGate::Phase(_) => UnitaryGateType::Phase,
            UnboundUnitaryGate::PhaseK(_) => UnitaryGateType::PhaseK,
            UnboundUnitaryGate::R(_, _, _) => UnitaryGateType::R,
            UnboundUnitaryGate::SWAP => UnitaryGateType::SWAP,
            UnboundUnitaryGate::SQSWAP => UnitaryGateType::SQSWAP,
            UnboundUnitaryGate::U(matrix) => UnitaryGateType::U(matrix.num_qubits().unwrap_or(0)),
        }
    }
}

impl From<UnboundUnitaryGate<'_>> for Matrix {
    fn from(unbound_gate: UnboundUnitaryGate<'_>) -> Matrix {
        match unbound_gate {
            UnboundUnitaryGate::I => matrix!(
                1., 0.;
                0., 1.
            ),

            UnboundUnitaryGate::X => matrix!(
                0., 1.;
                1., 0.
            ),

            UnboundUnitaryGate::Y => matrix!(
                0.,      (0.,-1.);
                (0., 1.), 0.
            ),

            UnboundUnitaryGate::Z => matrix!(
                1.,  0.;
                0., (-1.)
            ),

            UnboundUnitaryGate::H => matrix!(
                FRAC_1_SQRT_2,   FRAC_1_SQRT_2;
                FRAC_1_SQRT_2, (-FRAC_1_SQRT_2)
            ),

            UnboundUnitaryGate::S => matrix!(
                1.,  0.;
                0., (0., 1.)
            ),

            UnboundUnitaryGate::SDAG => matrix!(
                1.,  0.;
                0., (0., -1.)
            ),

            UnboundUnitaryGate::T => matrix!(
                1.,  0.;
                0., (FRAC_1_SQRT_2, FRAC_1_SQRT_2)
            ),

            UnboundUnitaryGate::TDAG => matrix!(
                1.,  0.;
                0., (FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
            ),

            UnboundUnitaryGate::RX90 => matrix!(
                 FRAC_1_SQRT_2,      (0., -FRAC_1_SQRT_2);
                (0., -FRAC_1_SQRT_2), FRAC_1_SQRT_2
            ),

            UnboundUnitaryGate::RXM90 => matrix!(
                FRAC_1_SQRT_2,      (0., FRAC_1_SQRT_2);
                (0., FRAC_1_SQRT_2), FRAC_1_SQRT_2
            ),

            UnboundUnitaryGate::RX180 => matrix!(
                 0.,      (0., -1.);
                (0., -1.), 0.
            ),

            UnboundUnitaryGate::RY90 => matrix!(
                FRAC_1_SQRT_2, (-FRAC_1_SQRT_2);
                FRAC_1_SQRT_2,   FRAC_1_SQRT_2
            ),

            UnboundUnitaryGate::RYM90 => matrix!(
                  FRAC_1_SQRT_2,  FRAC_1_SQRT_2;
                (-FRAC_1_SQRT_2), FRAC_1_SQRT_2
            ),

            UnboundUnitaryGate::RY180 => matrix!(
                0., (-1.);
                1.,  0.
            ),

            UnboundUnitaryGate::RZ90 => matrix!(
                (FRAC_1_SQRT_2, -FRAC_1_SQRT_2),  0.;
                0.,                              (FRAC_1_SQRT_2, FRAC_1_SQRT_2)
            ),

            UnboundUnitaryGate::RZM90 => matrix!(
                (FRAC_1_SQRT_2, FRAC_1_SQRT_2),  0.;
                0.,                             (FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
            ),

            UnboundUnitaryGate::RZ180 => matrix!(
                (0., -1.),  0.;
                0.,        (0., 1.)
            ),

            UnboundUnitaryGate::SWAP => matrix!(
                1., 0., 0., 0.;
                0., 0., 1., 0.;
                0., 1., 0., 0.;
                0., 0., 0., 1.
            ),

            UnboundUnitaryGate::SQSWAP => matrix!(
                1., 0.,           0.,         0.;
                0., (0.5, 0.5),  (0.5, -0.5), 0.;
                0., (0.5, -0.5), (0.5, 0.5),  0.;
                0., 0.,           0.,         1.
            ),

            UnboundUnitaryGate::RX(theta) => {
                let a = c!((0.5 * theta).cos());
                let b = c!(0., -1.) * (0.5 * theta).sin();
                vec![a, b, b, a].try_into().unwrap()
            }

            UnboundUnitaryGate::RY(theta) => {
                let a = c!((0.5 * theta).cos());
                let b = c!((0.5 * theta).sin());
                vec![a, -b, b, a].try_into().unwrap()
            }

            UnboundUnitaryGate::RZ(theta) => {
                let a = c!(0., -0.5 * theta).exp();
                let b = c!(0., 0.5 * theta).exp();
                vec![a, c!(0.), c!(0.), b].try_into().unwrap()
            }

            UnboundUnitaryGate::Phase(theta) => vec![c!(1.), c!(0.), c!(0.), c!(0., theta).exp()]
                .try_into()
                .unwrap(),

            UnboundUnitaryGate::PhaseK(k) => {
                let theta = PI / 2usize.pow(k as u32) as f64;
                vec![c!(1.), c!(0.), c!(0.), c!(0., theta).exp()]
                    .try_into()
                    .unwrap()
            }

            UnboundUnitaryGate::R(theta, phi, lambda) => {
                let a = (theta / 2.).cos();
                let b = (theta / 2.).sin();
                vec![
                    c!(0., 0.).exp() * a,
                    -c!(0., lambda).exp() * b,
                    c!(0., phi).exp() * b,
                    c!(0., lambda + phi).exp() * a,
                ]
                .try_into()
                .unwrap()
            }

            UnboundUnitaryGate::U(matrix) => matrix.clone(),
        }
    }
}

impl TryFrom<UnitaryGateType> for UnboundUnitaryGate<'_> {
    type Error = &'static str;
    fn try_from(gate_type: UnitaryGateType) -> Result<Self, Self::Error> {
        match gate_type {
            UnitaryGateType::RX
            | UnitaryGateType::RY
            | UnitaryGateType::RZ
            | UnitaryGateType::Phase
            | UnitaryGateType::PhaseK
            | UnitaryGateType::R => Err("gate is parameterized"),
            UnitaryGateType::U(_) => Err("gate is parameterized"),
            UnitaryGateType::I => Ok(UnboundUnitaryGate::I),
            UnitaryGateType::X => Ok(UnboundUnitaryGate::X),
            UnitaryGateType::Y => Ok(UnboundUnitaryGate::Y),
            UnitaryGateType::Z => Ok(UnboundUnitaryGate::Z),
            UnitaryGateType::H => Ok(UnboundUnitaryGate::H),
            UnitaryGateType::S => Ok(UnboundUnitaryGate::S),
            UnitaryGateType::SDAG => Ok(UnboundUnitaryGate::SDAG),
            UnitaryGateType::T => Ok(UnboundUnitaryGate::T),
            UnitaryGateType::TDAG => Ok(UnboundUnitaryGate::TDAG),
            UnitaryGateType::RX90 => Ok(UnboundUnitaryGate::RX90),
            UnitaryGateType::RXM90 => Ok(UnboundUnitaryGate::RXM90),
            UnitaryGateType::RX180 => Ok(UnboundUnitaryGate::RX180),
            UnitaryGateType::RY90 => Ok(UnboundUnitaryGate::RY90),
            UnitaryGateType::RYM90 => Ok(UnboundUnitaryGate::RYM90),
            UnitaryGateType::RY180 => Ok(UnboundUnitaryGate::RY180),
            UnitaryGateType::RZ90 => Ok(UnboundUnitaryGate::RZ90),
            UnitaryGateType::RZM90 => Ok(UnboundUnitaryGate::RZM90),
            UnitaryGateType::RZ180 => Ok(UnboundUnitaryGate::RZ180),
            UnitaryGateType::SWAP => Ok(UnboundUnitaryGate::SWAP),
            UnitaryGateType::SQSWAP => Ok(UnboundUnitaryGate::SQSWAP),
        }
    }
}

impl TryFrom<UnitaryGateType> for Matrix {
    type Error = &'static str;
    fn try_from(gate_type: UnitaryGateType) -> Result<Self, Self::Error> {
        UnboundUnitaryGate::try_from(gate_type).map(|unbound_gate| unbound_gate.into())
    }
}

impl From<UnitaryGateType> for Box<dyn MatrixConverterArb> {
    fn from(gate_type: UnitaryGateType) -> Box<dyn MatrixConverterArb> {
        match gate_type {
            UnitaryGateType::RX => Box::new(RxMatrixConverter::default()),
            UnitaryGateType::RY => Box::new(RyMatrixConverter::default()),
            UnitaryGateType::RZ => Box::new(RzMatrixConverter::default()),
            UnitaryGateType::Phase => Box::new(PhaseMatrixConverter::default()),
            UnitaryGateType::PhaseK => Box::new(PhaseKMatrixConverter::default()),
            UnitaryGateType::R => Box::new(RMatrixConverter::default()),
            UnitaryGateType::U(num_qubits) => Box::new(UMatrixConverter::new(Some(num_qubits))),
            _ => Box::new(FixedMatrixConverter::from(
                Matrix::try_from(gate_type).unwrap(),
            )),
        }
    }
}

impl UnitaryGateType {
    pub fn into_gate_converter(
        self,
        num_controls: Option<usize>,
        epsilon: f64,
        ignore_global_phase: bool,
    ) -> Box<dyn Converter<Input = Gate, Output = (Vec<QubitRef>, ArbData)>> {
        match self {
            UnitaryGateType::RX => Box::new(UnitaryGateConverter::from(UnitaryConverter::new(
                RxMatrixConverter::default(),
                num_controls,
                epsilon,
                ignore_global_phase,
            ))),
            UnitaryGateType::RY => Box::new(UnitaryGateConverter::from(UnitaryConverter::new(
                RyMatrixConverter::default(),
                num_controls,
                epsilon,
                ignore_global_phase,
            ))),
            UnitaryGateType::RZ => Box::new(UnitaryGateConverter::from(UnitaryConverter::new(
                RzMatrixConverter::default(),
                num_controls,
                epsilon,
                ignore_global_phase,
            ))),
            UnitaryGateType::Phase => Box::new(UnitaryGateConverter::from(UnitaryConverter::new(
                PhaseMatrixConverter::default(),
                num_controls,
                epsilon,
                ignore_global_phase,
            ))),
            UnitaryGateType::PhaseK => Box::new(UnitaryGateConverter::from(UnitaryConverter::new(
                PhaseKMatrixConverter::default(),
                num_controls,
                epsilon,
                ignore_global_phase,
            ))),
            UnitaryGateType::R => Box::new(UnitaryGateConverter::from(UnitaryConverter::new(
                RMatrixConverter::default(),
                num_controls,
                epsilon,
                ignore_global_phase,
            ))),
            UnitaryGateType::U(num_qubits) => {
                Box::new(UnitaryGateConverter::from(UnitaryConverter::new(
                    UMatrixConverter::new(Some(num_qubits)),
                    num_controls,
                    epsilon,
                    ignore_global_phase,
                )))
            }
            _ => Box::new(UnitaryGateConverter::from(UnitaryConverter::new(
                FixedMatrixConverter::from(Matrix::try_from(self).unwrap()),
                num_controls,
                epsilon,
                ignore_global_phase,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: UnboundUnitaryGate, b: UnboundUnitaryGate) -> bool {
        Matrix::from(a).approx_eq(&b.into(), 1e-15, true)
    }

    #[test]
    fn gates_as_rotation() {
        assert!(check(
            UnboundUnitaryGate::I,
            UnboundUnitaryGate::R(0., 0., 0.)
        ));
        assert!(check(
            UnboundUnitaryGate::X,
            UnboundUnitaryGate::R(PI, 0., PI)
        ));
        assert!(check(
            UnboundUnitaryGate::Y,
            UnboundUnitaryGate::R(PI, PI / 2., PI / 2.)
        ));
        assert!(check(
            UnboundUnitaryGate::Z,
            UnboundUnitaryGate::R(0., 0., PI)
        ));
        assert!(check(
            UnboundUnitaryGate::H,
            UnboundUnitaryGate::R(PI / 2., 0., PI)
        ));
        assert!(check(
            UnboundUnitaryGate::S,
            UnboundUnitaryGate::R(0., 0., PI / 2.)
        ));
        assert!(check(
            UnboundUnitaryGate::SDAG,
            UnboundUnitaryGate::R(0., 0., -PI / 2.)
        ));
        assert!(check(
            UnboundUnitaryGate::T,
            UnboundUnitaryGate::R(0., 0., PI / 4.)
        ));
        assert!(check(
            UnboundUnitaryGate::TDAG,
            UnboundUnitaryGate::R(0., 0., -PI / 4.)
        ));
    }

    #[test]
    fn gates_coversions() {
        for gate in vec![
            (UnboundUnitaryGate::I, UnitaryGateType::I),
            (UnboundUnitaryGate::X, UnitaryGateType::X),
            (UnboundUnitaryGate::Y, UnitaryGateType::Y),
            (UnboundUnitaryGate::Z, UnitaryGateType::Z),
            (UnboundUnitaryGate::H, UnitaryGateType::H),
            (UnboundUnitaryGate::S, UnitaryGateType::S),
            (UnboundUnitaryGate::SDAG, UnitaryGateType::SDAG),
            (UnboundUnitaryGate::T, UnitaryGateType::T),
            (UnboundUnitaryGate::TDAG, UnitaryGateType::TDAG),
            (UnboundUnitaryGate::RX90, UnitaryGateType::RX90),
            (UnboundUnitaryGate::RXM90, UnitaryGateType::RXM90),
            (UnboundUnitaryGate::RX180, UnitaryGateType::RX180),
            (UnboundUnitaryGate::RY90, UnitaryGateType::RY90),
            (UnboundUnitaryGate::RYM90, UnitaryGateType::RYM90),
            (UnboundUnitaryGate::RY180, UnitaryGateType::RY180),
            (UnboundUnitaryGate::RZ90, UnitaryGateType::RZ90),
            (UnboundUnitaryGate::RZM90, UnitaryGateType::RZM90),
            (UnboundUnitaryGate::RZ180, UnitaryGateType::RZ180),
            (UnboundUnitaryGate::RX(1.), UnitaryGateType::RX),
            (UnboundUnitaryGate::RY(1.), UnitaryGateType::RY),
            (UnboundUnitaryGate::Phase(1.), UnitaryGateType::Phase),
            (UnboundUnitaryGate::PhaseK(1), UnitaryGateType::PhaseK),
            (UnboundUnitaryGate::RZ(1.), UnitaryGateType::RZ),
            (UnboundUnitaryGate::R(1., 1., 1.), UnitaryGateType::R),
            (UnboundUnitaryGate::SWAP, UnitaryGateType::SWAP),
            (UnboundUnitaryGate::SQSWAP, UnitaryGateType::SQSWAP),
            (
                UnboundUnitaryGate::U(&Matrix::new(vec![c!(1.), c!(1.), c!(1.), c!(1.)]).unwrap()),
                UnitaryGateType::U(1),
            ),
        ]
        .into_iter()
        {
            let gate_type: UnitaryGateType = gate.0.into();
            assert_eq!(gate_type, gate.1);
        }

        let a = QubitRef::from_foreign(1).unwrap();
        let b = QubitRef::from_foreign(2).unwrap();
        for gate in vec![
            (
                BoundUnitaryGate::I(a),
                UnboundUnitaryGate::I,
                UnitaryGateType::I,
            ),
            (
                BoundUnitaryGate::X(a),
                UnboundUnitaryGate::X,
                UnitaryGateType::X,
            ),
            (
                BoundUnitaryGate::Y(a),
                UnboundUnitaryGate::Y,
                UnitaryGateType::Y,
            ),
            (
                BoundUnitaryGate::Z(a),
                UnboundUnitaryGate::Z,
                UnitaryGateType::Z,
            ),
            (
                BoundUnitaryGate::H(a),
                UnboundUnitaryGate::H,
                UnitaryGateType::H,
            ),
            (
                BoundUnitaryGate::S(a),
                UnboundUnitaryGate::S,
                UnitaryGateType::S,
            ),
            (
                BoundUnitaryGate::SDAG(a),
                UnboundUnitaryGate::SDAG,
                UnitaryGateType::SDAG,
            ),
            (
                BoundUnitaryGate::T(a),
                UnboundUnitaryGate::T,
                UnitaryGateType::T,
            ),
            (
                BoundUnitaryGate::TDAG(a),
                UnboundUnitaryGate::TDAG,
                UnitaryGateType::TDAG,
            ),
            (
                BoundUnitaryGate::RX90(a),
                UnboundUnitaryGate::RX90,
                UnitaryGateType::RX90,
            ),
            (
                BoundUnitaryGate::RXM90(a),
                UnboundUnitaryGate::RXM90,
                UnitaryGateType::RXM90,
            ),
            (
                BoundUnitaryGate::RX180(a),
                UnboundUnitaryGate::RX180,
                UnitaryGateType::RX180,
            ),
            (
                BoundUnitaryGate::RY90(a),
                UnboundUnitaryGate::RY90,
                UnitaryGateType::RY90,
            ),
            (
                BoundUnitaryGate::RYM90(a),
                UnboundUnitaryGate::RYM90,
                UnitaryGateType::RYM90,
            ),
            (
                BoundUnitaryGate::RY180(a),
                UnboundUnitaryGate::RY180,
                UnitaryGateType::RY180,
            ),
            (
                BoundUnitaryGate::RZ90(a),
                UnboundUnitaryGate::RZ90,
                UnitaryGateType::RZ90,
            ),
            (
                BoundUnitaryGate::RZM90(a),
                UnboundUnitaryGate::RZM90,
                UnitaryGateType::RZM90,
            ),
            (
                BoundUnitaryGate::RZ180(a),
                UnboundUnitaryGate::RZ180,
                UnitaryGateType::RZ180,
            ),
            (
                BoundUnitaryGate::RX(1., a),
                UnboundUnitaryGate::RX(1.),
                UnitaryGateType::RX,
            ),
            (
                BoundUnitaryGate::RY(1., a),
                UnboundUnitaryGate::RY(1.),
                UnitaryGateType::RY,
            ),
            (
                BoundUnitaryGate::Phase(1., a),
                UnboundUnitaryGate::Phase(1.),
                UnitaryGateType::Phase,
            ),
            (
                BoundUnitaryGate::PhaseK(1, a),
                UnboundUnitaryGate::PhaseK(1),
                UnitaryGateType::PhaseK,
            ),
            (
                BoundUnitaryGate::RZ(1., a),
                UnboundUnitaryGate::RZ(1.),
                UnitaryGateType::RZ,
            ),
            (
                BoundUnitaryGate::R(1., 1., 1., a),
                UnboundUnitaryGate::R(1., 1., 1.),
                UnitaryGateType::R,
            ),
            (
                BoundUnitaryGate::SWAP(a, b),
                UnboundUnitaryGate::SWAP,
                UnitaryGateType::SWAP,
            ),
            (
                BoundUnitaryGate::SQSWAP(a, b),
                UnboundUnitaryGate::SQSWAP,
                UnitaryGateType::SQSWAP,
            ),
            (
                BoundUnitaryGate::U(
                    &Matrix::new(vec![c!(1.), c!(1.), c!(1.), c!(1.)]).unwrap(),
                    &[a],
                ),
                UnboundUnitaryGate::U(&Matrix::new(vec![c!(1.), c!(1.), c!(1.), c!(1.)]).unwrap()),
                UnitaryGateType::U(1),
            ),
        ]
        .into_iter()
        {
            let unbound_gate: UnboundUnitaryGate = gate.0.into();
            let gate_type: UnitaryGateType = gate.0.into();
            assert_eq!(unbound_gate, gate.1);
            assert_eq!(gate_type, gate.2);
        }

        for bound_gate in vec![
            BoundUnitaryGate::I(a),
            BoundUnitaryGate::X(a),
            BoundUnitaryGate::Y(a),
            BoundUnitaryGate::Z(a),
            BoundUnitaryGate::H(a),
            BoundUnitaryGate::S(a),
            BoundUnitaryGate::SDAG(a),
            BoundUnitaryGate::T(a),
            BoundUnitaryGate::TDAG(a),
            BoundUnitaryGate::RX90(a),
            BoundUnitaryGate::RXM90(a),
            BoundUnitaryGate::RX180(a),
            BoundUnitaryGate::RY90(a),
            BoundUnitaryGate::RYM90(a),
            BoundUnitaryGate::RY180(a),
            BoundUnitaryGate::RZ90(a),
            BoundUnitaryGate::RZM90(a),
            BoundUnitaryGate::RZ180(a),
            BoundUnitaryGate::RX(1., a),
            BoundUnitaryGate::RY(1., a),
            BoundUnitaryGate::RZ(1., a),
            BoundUnitaryGate::R(1., 2., 3., a),
            BoundUnitaryGate::Phase(1., a),
            BoundUnitaryGate::PhaseK(2, a),
        ]
        .into_iter()
        {
            assert_eq!(
                Gate::new_unitary(
                    vec![a],
                    vec![],
                    Matrix::from(UnboundUnitaryGate::from(bound_gate))
                )
                .unwrap(),
                Gate::from(bound_gate)
            );
        }
        for bound_gate in
            vec![BoundUnitaryGate::SWAP(a, b), BoundUnitaryGate::SQSWAP(a, b)].into_iter()
        {
            assert_eq!(
                Gate::new_unitary(
                    vec![a, b],
                    vec![],
                    Matrix::from(UnboundUnitaryGate::from(bound_gate))
                )
                .unwrap(),
                Gate::from(bound_gate)
            );
        }

        assert_eq!(
            Gate::from(BoundUnitaryGate::U(&Matrix::new_identity(2), &[a])),
            Gate::new_unitary(vec![a], vec![], Matrix::new_identity(2)).unwrap()
        );
    }
}
