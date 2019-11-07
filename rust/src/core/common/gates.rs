//! Helper functions for common quantum Gates

use crate::common::types::{Gate, QubitRef};
use num_complex::Complex64;
use std::f64::consts::{FRAC_1_SQRT_2, PI};

macro_rules! c {
    ($re:expr, $im:expr) => {
        Complex64::new($re, $im);
    };
    ($re:expr) => {
        Complex64::new($re, 0.)
    };
}

/// Enumeration of predefined quantum gates.
pub enum Gates {
    I(QubitRef),
    RX(QubitRef, f64),
    RY(QubitRef, f64),
    RZ(QubitRef, f64),
    SWAP(QubitRef, QubitRef),
    SQSWAP(QubitRef, QubitRef),
    X(QubitRef),
    X90(QubitRef),
    MX90(QubitRef),
    Y(QubitRef),
    Y90(QubitRef),
    MY90(QubitRef),
    Z(QubitRef),
    Z90(QubitRef),
    MZ90(QubitRef),
    S(QubitRef),
    SDAG(QubitRef),
    T(QubitRef),
    TDAG(QubitRef),
    H(QubitRef),
    CNOT(QubitRef, QubitRef),
    TOFFOLI(QubitRef, QubitRef, QubitRef),
    FREDKIN(QubitRef, QubitRef, QubitRef),
}

impl Into<Gate> for Gates {
    fn into(self) -> Gate {
        match self {
            Gates::I(q) => i_gate(q),
            Gates::RX(q, theta) => rx_gate(q, theta),
            Gates::RY(q, theta) => ry_gate(q, theta),
            Gates::RZ(q, theta) => rz_gate(q, theta),
            Gates::SWAP(q1, q2) => swap_gate(q1, q2),
            Gates::SQSWAP(q1, q2) => sqswap_gate(q1, q2),
            Gates::X(q) => x_gate(q),
            Gates::X90(q) => x90_gate(q),
            Gates::MX90(q) => mx90_gate(q),
            Gates::Y(q) => y_gate(q),
            Gates::Y90(q) => y90_gate(q),
            Gates::MY90(q) => my90_gate(q),
            Gates::Z(q) => z_gate(q),
            Gates::Z90(q) => z90_gate(q),
            Gates::MZ90(q) => mz90_gate(q),
            Gates::S(q) => s_gate(q),
            Gates::SDAG(q) => sdag_gate(q),
            Gates::T(q) => t_gate(q),
            Gates::TDAG(q) => tdag_gate(q),
            Gates::H(q) => h_gate(q),
            Gates::CNOT(c, q) => cnot_gate(c, q),
            Gates::TOFFOLI(c1, c2, q) => toffoli_gate(c1, c2, q),
            Gates::FREDKIN(c, q1, q2) => fredkin_gate(c, q1, q2),
        }
    }
}

/// Returns an I gate.
pub fn i_gate(target: QubitRef) -> Gate {
    Gate::unitary(vec![target], vec![], vec![c!(1.), c!(0.), c!(0.), c!(1.)])
}

/// Returns an arbitrary X rotation gate.
/// Theta is the rotation angle in radians.
pub fn rx_gate(target: QubitRef, theta: f64) -> Gate {
    let a = c!((0.5 * theta).cos());
    let b = c!(0., -1.) * (0.5 * theta).sin();
    Gate::unitary(vec![target], vec![], vec![a, b, b, a])
}

/// Returns an arbitrary Y rotation gate.
/// Theta is the rotation angle in radians.
pub fn ry_gate(target: QubitRef, theta: f64) -> Gate {
    let a = c!((0.5 * theta).cos());
    let b = c!((0.5 * theta).sin());
    Gate::unitary(vec![target], vec![], vec![a, -b, b, a])
}

/// Returns an arbitrary Y rotation gate.
/// Theta is the rotation angle in radians.
pub fn rz_gate(target: QubitRef, theta: f64) -> Gate {
    let a = c!(0., -0.5 * theta).exp();
    let b = c!(0., 0.5 * theta).exp();
    Gate::unitary(vec![target], vec![], vec![a, c!(0.), c!(0.), b])
}

// TODO(mb): r_gate

/// Returns a swap gate on provided target qubits a and b.
pub fn swap_gate(a: QubitRef, b: QubitRef) -> Gate {
    Gate::unitary(
        vec![a, b],
        vec![],
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
        ],
    )
}

/// Returns a square-root-of-swap gate on provided target qubits a and b.
pub fn sqswap_gate(a: QubitRef, b: QubitRef) -> Gate {
    Gate::unitary(
        vec![a, b],
        vec![],
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
        ],
    )
}

/// Returns an X gate.
pub fn x_gate(target: QubitRef) -> Gate {
    rx_gate(target, PI)
}

/// Returns a 90-degree X gate.
pub fn x90_gate(target: QubitRef) -> Gate {
    rx_gate(target, 0.5 * PI)
}

/// Returns a negative 90-degree X gate.
pub fn mx90_gate(target: QubitRef) -> Gate {
    rx_gate(target, -0.5 * PI)
}

/// Returns a Y gate.
pub fn y_gate(target: QubitRef) -> Gate {
    ry_gate(target, PI)
}

/// Returns a 90-degree Y gate.
pub fn y90_gate(target: QubitRef) -> Gate {
    ry_gate(target, 0.5 * PI)
}

/// Returns a negative 90-degree Y gate.
pub fn my90_gate(target: QubitRef) -> Gate {
    ry_gate(target, -0.5 * PI)
}

/// Returns a Z gate.
pub fn z_gate(target: QubitRef) -> Gate {
    rz_gate(target, PI)
}

/// Returns a 90-degree Z gate.
pub fn z90_gate(target: QubitRef) -> Gate {
    rz_gate(target, 0.5 * PI)
}

/// Returns a negative 90-degree Z gate.
pub fn mz90_gate(target: QubitRef) -> Gate {
    rz_gate(target, -0.5 * PI)
}

/// Returns an S gate.
pub fn s_gate(target: QubitRef) -> Gate {
    z90_gate(target)
}

/// Returns an S-dagger gate.
pub fn sdag_gate(target: QubitRef) -> Gate {
    mz90_gate(target)
}

/// Returns a T gate.
pub fn t_gate(target: QubitRef) -> Gate {
    rz_gate(target, 0.25 * PI)
}

/// Returns a T-dagger gate.
pub fn tdag_gate(target: QubitRef) -> Gate {
    rz_gate(target, -0.25 * PI)
}

/// Returns a Hadamard gate.
pub fn h_gate(target: QubitRef) -> Gate {
    let x = c!(FRAC_1_SQRT_2);
    Gate::unitary(vec![target], vec![], vec![x, x, x, -x])
}

/// Returns a CNOT gate.
pub fn cnot_gate(control: QubitRef, target: QubitRef) -> Gate {
    Gate::unitary(
        vec![target],
        vec![control],
        vec![c!(0.), c!(1.), c!(1.), c!(0.)],
    )
}

/// Returns a Toffoli gate.
pub fn toffoli_gate(c1: QubitRef, c2: QubitRef, target: QubitRef) -> Gate {
    Gate::unitary(
        vec![target],
        vec![c1, c2],
        vec![c!(0.), c!(1.), c!(1.), c!(0.)],
    )
}

/// Returns a Fredking gate.
pub fn fredkin_gate(control: QubitRef, a: QubitRef, b: QubitRef) -> Gate {
    Gate::unitary(
        vec![a, b],
        vec![control],
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
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_gates() {
        let q = QubitRef::from_foreign(1u64).unwrap();
        let q1 = QubitRef::from_foreign(2u64).unwrap();
        let q2 = QubitRef::from_foreign(3u64).unwrap();
        let c = QubitRef::from_foreign(4u64).unwrap();
        let c1 = QubitRef::from_foreign(5u64).unwrap();
        let c2 = QubitRef::from_foreign(6u64).unwrap();

        let gate: Gate = Gates::I(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::RX(q, 0.5 * PI).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::RY(q, 0.5 * PI).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::RZ(q, 0.5 * PI).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::SQSWAP(q1, q2).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::X(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::X90(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::MX90(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::Y(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::Y90(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::MY90(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::Z(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::Z90(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::MZ90(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::S(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::SDAG(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::T(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::TDAG(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::H(q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::CNOT(c, q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::TOFFOLI(c1, c2, q).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());

        let gate: Gate = Gates::FREDKIN(c, q1, q2).into();
        assert!(Gate::new_unitary(
            gate.get_targets().to_vec(),
            gate.get_controls().to_vec(),
            gate.get_matrix().unwrap()
        )
        .is_ok());
    }
}
