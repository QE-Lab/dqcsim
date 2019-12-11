//! Helper functions for common quantum Gates
use crate::common::types::{matrix_detector, Detector, Matrix, QubitRef};

use std::{
    convert::{TryFrom, TryInto},
    f64::consts::{FRAC_1_SQRT_2, PI},
};

/// Enumeration of predefined quantum gates.
#[derive(Clone, Copy, Debug)]
pub enum GateType {
    I,
    X,
    Y,
    Z,
    H,
    S,
    SDAG,
    T,
    TDAG,
    RX90,
    RXM90,
    RX180,
    RY90,
    RYM90,
    RY180,
    RZ90,
    RZM90,
    RZ180,
    RX,
    RY,
    RK,
    RZ,
    R,
    SWAP,
    SQSWAP,
    U(usize), // usize represents the number of involved qubits
}

impl GateType {
    pub fn into_detector<T: Default>(
        self,
        epsilon: f64,
        ignore_global_phase: bool,
    ) -> Box<Detector<T>> {
        let unbound_gate: Result<UnboundGate, _> = self.try_into();
        if unbound_gate.is_ok() {
            matrix_detector(unbound_gate.unwrap().into(), epsilon, ignore_global_phase)
        } else {
            match self {
                GateType::RX => unimplemented!(),
                GateType::RY => unimplemented!(),
                GateType::RK => unimplemented!(),
                GateType::RZ => unimplemented!(),
                GateType::R => unimplemented!(),
                GateType::U(_matrix) => unimplemented!(),
                _ => unreachable!(),
            }
        }
    }
}

impl From<UnboundGate> for GateType {
    fn from(unbound_gate: UnboundGate) -> Self {
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
            UnboundGate::RK(_) => GateType::RK,
            UnboundGate::RZ(_) => GateType::RZ,
            UnboundGate::R(_, _, _) => GateType::R,
            UnboundGate::SWAP => GateType::SWAP,
            UnboundGate::SQSWAP => GateType::SQSWAP,
            UnboundGate::U(matrix) => GateType::U(matrix.num_qubits().expect("bad matrix")),
        }
    }
}

impl TryFrom<GateType> for UnboundGate {
    type Error = &'static str;
    fn try_from(gate_type: GateType) -> Result<Self, Self::Error> {
        Ok(match gate_type {
            GateType::I => UnboundGate::I,
            GateType::X => UnboundGate::X,
            GateType::Y => UnboundGate::Y,
            GateType::Z => UnboundGate::Z,
            GateType::H => UnboundGate::H,
            GateType::S => UnboundGate::S,
            GateType::SDAG => UnboundGate::SDAG,
            GateType::T => UnboundGate::T,
            GateType::TDAG => UnboundGate::TDAG,
            GateType::RX90 => UnboundGate::RX90,
            GateType::RXM90 => UnboundGate::RXM90,
            GateType::RX180 => UnboundGate::RX180,
            GateType::RY90 => UnboundGate::RY90,
            GateType::RYM90 => UnboundGate::RYM90,
            GateType::RY180 => UnboundGate::RY180,
            GateType::RZ90 => UnboundGate::RZ90,
            GateType::RZM90 => UnboundGate::RZM90,
            GateType::RZ180 => UnboundGate::RZ180,
            GateType::SWAP => UnboundGate::SWAP,
            GateType::SQSWAP => UnboundGate::SQSWAP,
            _ => Err("gate is parameterized")?,
        })
    }
}

// impl From<BoundGate> for UnboundGate {}

// impl From<BoundGate> for Gate {}

// impl From<UnboundGate> for Detector<T> {

// }

// impl<T> From<GateType> for Box<Detector<T>> {
//     fn from(gate_type: GateType) -> Self {
//         if let Some(unbound_gate) = gate_type.try_into() {
//             matrix_detector(unbound_gate.into()
//         }

//             I =>
//             X =>
//             Y =>
//             Z =>
//             H =>
//             S =>
//             SDAG =>
//             T =>
//             TDAG =>
//             RX_90 =>
//             RX_M90 =>
//             RX_180 =>
//             RY_90 =>
//             RY_M90 =>
//             RY_180 =>
//             RZ_90 =>
//             RZ_M90 =>
//             RZ_180 =>
//             RX =>
//             RY =>
//             RK =>
//             RZ =>
//             R =>
//             SWAP =>
//             SQSWAP =>
//             U(usize) =>
//         }
//     }
// }

// impl From<BoundGate> for Detector<T> {}

// impl TryFrom<Gate> for BoundGate {}

pub enum UnboundGate {
    I,
    X,
    Y,
    Z,
    H,
    S,
    SDAG,
    T,
    TDAG,
    RX90,
    RXM90,
    RX180,
    RY90,
    RYM90,
    RY180,
    RZ90,
    RZM90,
    RZ180,
    RX(f64),
    RY(f64),
    RK(usize),
    RZ(f64),
    R(f64, f64, f64),
    SWAP,
    SQSWAP,
    U(Matrix),
}

pub enum BoundGate {
    I(QubitRef),
    X(QubitRef),
    Y(QubitRef),
    Z(QubitRef),
    H(QubitRef),
    S(QubitRef),
    SDAG(QubitRef),
    T(QubitRef),
    TDAG(QubitRef),
    RX90(QubitRef),
    RXM90(QubitRef),
    RX180(QubitRef),
    RY90(QubitRef),
    RYM90(QubitRef),
    RY180(QubitRef),
    RZ90(QubitRef),
    RZM90(QubitRef),
    RZ180(QubitRef),
    RX(f64, QubitRef),
    RY(f64, QubitRef),
    RK(usize, QubitRef),
    RZ(f64, QubitRef),
    R(f64, f64, f64, QubitRef),
    SWAP(QubitRef, QubitRef),
    SQSWAP(QubitRef, QubitRef),
    U(Matrix, Vec<QubitRef>), // TODO(mb): ref?
}

impl From<UnboundGate> for Matrix {
    fn from(unbound_gate: UnboundGate) -> Matrix {
        match unbound_gate {
            UnboundGate::I => vec![c!(1.), c!(0.), c!(0.), c!(1.)].into(),
            UnboundGate::X => vec![c!(0.), c!(1.), c!(1.), c!(0.)].into(),
            UnboundGate::Y => vec![c!(0.), c!(0., -1.), c!(0., 1.), c!(0.)].into(),
            UnboundGate::Z => vec![c!(1.), c!(0.), c!(0.), c!(-1.)].into(),
            UnboundGate::H => {
                let x = c!(FRAC_1_SQRT_2);
                vec![x, x, x, -x].into()
            }
            UnboundGate::S => vec![c!(1.), c!(0.), c!(0.), c!(0., 1.)].into(),
            UnboundGate::SDAG => vec![c!(1.), c!(0.), c!(0.), c!(0., -1.)].into(),
            UnboundGate::T => vec![c!(1.), c!(0.), c!(0.), c!(FRAC_1_SQRT_2, FRAC_1_SQRT_2)].into(),
            UnboundGate::TDAG => {
                vec![c!(1.), c!(0.), c!(0.), c!(FRAC_1_SQRT_2, -FRAC_1_SQRT_2)].into()
            }
            UnboundGate::RX90 => {
                let x = c!(FRAC_1_SQRT_2);
                let y = c!(0., -FRAC_1_SQRT_2);
                vec![x, y, y, x].into()
            }
            UnboundGate::RXM90 => {
                let x = c!(FRAC_1_SQRT_2);
                let y = c!(0., FRAC_1_SQRT_2);
                vec![x, y, y, x].into()
            }
            UnboundGate::RX180 => {
                let x = c!(0., -FRAC_1_SQRT_2);
                vec![c!(0.), x, x, c!(0.)].into()
            }
            UnboundGate::RY90 => {
                let x = c!(FRAC_1_SQRT_2);
                vec![x, -x, x, x].into()
            }
            UnboundGate::RYM90 => {
                let x = c!(FRAC_1_SQRT_2);
                vec![x, x, -x, x].into()
            }
            UnboundGate::RY180 => vec![c!(0.), c!(-1.), c!(0.), c!(0.)].into(),
            UnboundGate::RZ90 => {
                let x = FRAC_1_SQRT_2;
                vec![c!(x, -x), c!(0.), c!(0.), c!(x, x)].into()
            }
            UnboundGate::RZM90 => {
                let x = FRAC_1_SQRT_2;
                vec![c!(x, x), c!(0.), c!(0.), c!(x, -x)].into()
            }
            UnboundGate::RZ180 => vec![c!(0., -1.), c!(0.), c!(0.), c!(0., 1.)].into(),
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
            UnboundGate::RK(k) => {
                let theta = 2. * PI / 2usize.pow(k as u32) as f64; // TODO(mb): check
                let a = c!(0., -0.5 * theta).exp();
                let b = c!(0., 0.5 * theta).exp();
                vec![a, c!(0.), c!(0.), b].into()
            }
            UnboundGate::RZ(theta) => {
                let a = c!(0., -0.5 * theta).exp();
                let b = c!(0., 0.5 * theta).exp();
                vec![a, c!(0.), c!(0.), b].into()
            }
            UnboundGate::R(theta, phi, lambda) => {
                let a = (theta / 2.).cos();
                let b = (theta / 2.).sin();
                let c = (phi + lambda) / 2.;
                let d = (phi - lambda) / 2.;
                vec![
                    c!(0., -c).exp() * a,
                    -c!(0., -d).exp() * b,
                    c!(0., d).exp() * b,
                    c!(0., c).exp() * a,
                ]
                .into()
            }
            UnboundGate::SWAP => {
                vec![
                    c!(1.),
                    c!(0.),
                    c!(0.),
                    c!(0.),
                    //
                    c!(0.),
                    c!(0.),
                    c!(1.),
                    c!(0.),
                    //
                    c!(0.),
                    c!(1.),
                    c!(0.),
                    c!(0.),
                    //
                    c!(0.),
                    c!(0.),
                    c!(0.),
                    c!(1.),
                ]
                .into()
            }
            UnboundGate::SQSWAP => {
                vec![
                    c!(1.),
                    c!(0.),
                    c!(0.),
                    c!(0.),
                    //
                    c!(0.),
                    c!(0.5, 0.5),
                    c!(0.5, -0.5),
                    c!(0.),
                    //
                    c!(0.),
                    c!(0.5, -0.5),
                    c!(0.5, 0.5),
                    c!(0.),
                    //
                    c!(0.),
                    c!(0.),
                    c!(0.),
                    c!(1.),
                ]
                .into()
            }
            UnboundGate::U(matrix) => matrix,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: UnboundGate, b: UnboundGate) -> bool {
        Into::<Matrix>::into(a).approx_eq(&b.into(), 1e-15, true)
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
}
