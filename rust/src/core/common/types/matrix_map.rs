use crate::core::common::{
    error::{inv_arg, inv_op, Result},
    gates::{GateType, UnboundGate},
    types::Matrix,
};
use num_complex::Complex64;
use std::{
    cell::RefCell,
    collections::HashMap,
    f64::consts::PI,
    fmt,
    fmt::{Debug, Formatter},
};

/// MatrixMap type to detect gates based on their matrices.
/// Users can add a key for every registered detector and link this to a type
/// T. A MatrixMap can be constructed using a MatrixMapBuilder.
pub struct MatrixMap<T, K> {
    detectors: Vec<(K, Box<Detector<T>>)>,
    map: RefCell<HashMap<Matrix, Option<(K, T)>>>,
}

impl<T, K> Debug for MatrixMap<T, K> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "MatrixMap")
    }
}

/// A Detector is a function which gets a reference to a Matrix and returns an
/// Result with an Option of T.
pub type Detector<T> = dyn Fn(&Matrix) -> Result<Option<T>>;

impl<T: Clone, K: Clone> MatrixMap<T, K> {
    /// Returns a new MatrixMapBuilder to construxt a MatrixMap.
    pub fn builder() -> MatrixMapBuilder<T, K> {
        MatrixMapBuilder::new()
    }
    /// Returns the number of detectors in this MatrixMap.
    pub fn len(&self) -> usize {
        self.detectors.len()
    }
    /// Returns true if the MatrixMap is empty.
    pub fn is_empty(&self) -> bool {
        self.detectors.is_empty()
    }
    /// Returns a slice to the detectors in this MatrixMap.
    pub fn detectors(&self) -> &[(K, Box<Detector<T>>)] {
        self.detectors.as_slice()
    }
    /// Clear the cache.
    pub fn clear_cache(&self) {
        self.map.borrow_mut().clear();
    }
}

impl<'a, T: 'a + Clone, K: 'a + Clone + PartialEq> MatrixMap<T, K> {
    /// Check this MatrixMap for the provided Matrix, using the detectors with
    /// provided keys. This never uses or updates the internal cache.
    pub fn detect_with(&self, input: &Matrix, keys: &[K]) -> Result<Option<(K, T)>> {
        self.run_detectors(
            input,
            self.detectors.iter().filter(|(k, _)| keys.contains(k)),
            false,
        )
    }
}

impl<'a, T: 'a + Clone, K: 'a + Clone> MatrixMap<T, K> {
    /// Check this MatrixMap for the provided Matrix.
    pub fn detect(&self, input: &Matrix) -> Result<Option<(K, T)>> {
        {
            let hit = self.map.borrow().get(input).cloned();
            if let Some(hit) = hit {
                // TODO(mb): option_flattening
                return Ok(hit);
            }
        }
        self.run_detectors(input, self.detectors.iter(), true)
    }

    /// Internal method to run detectors in iterator for the given Matrix.
    /// Caching can be enabled by setting the cache parameter.
    fn run_detectors(
        &self,
        input: &Matrix,
        mut detectors: impl Iterator<Item = &'a (K, Box<Detector<T>>)>,
        cache: bool,
    ) -> Result<Option<(K, T)>> {
        detectors
            .find_map(|(k, f)| {
                f(input)
                    .map(|res| res.map(|opt| (k.clone(), opt)))
                    .transpose()
            })
            .transpose()
            .and_then(|opt| {
                if cache {
                    self.map.borrow_mut().insert(input.clone(), opt.clone());
                }
                Ok(opt)
            })
            .or_else(|e| inv_op(format!("Detector function failed: {}", e)))
    }
}

impl<T: 'static + Clone + Default> Default for MatrixMap<T, GateType> {
    fn default() -> Self {
        MatrixMap::builder()
            .with_defaults(0, 1e-6, true)
            .unwrap()
            .finish()
    }
}

pub struct MatrixMapBuilder<T, K> {
    map: MatrixMap<T, K>,
}

impl<T, K> Debug for MatrixMapBuilder<T, K> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "MatrixMapBuilder")
    }
}

impl<T: 'static + Default> MatrixMapBuilder<T, GateType> {
    /// Adds default detectors to this MatrixMapBuilder.
    pub fn with_defaults(
        mut self,
        version: usize,
        epsilon: f64,
        ignore_global_phase: bool,
    ) -> Result<Self> {
        self.add_defaults(version, epsilon, ignore_global_phase)?;
        Ok(self)
    }

    /// Adds default detectors to this MatrixMapBuilder.
    pub(crate) fn add_defaults(
        &mut self,
        version: usize,
        epsilon: f64,
        ignore_global_phase: bool,
    ) -> Result<()> {
        if version != 0 {
            inv_arg("Version should be set to zero.")
        } else {
            for gate_type in &[
                GateType::I,
                GateType::X,
                GateType::Y,
                GateType::Z,
                GateType::H,
                GateType::S,
                GateType::SDAG,
                GateType::T,
                GateType::TDAG,
                GateType::RX90,
                GateType::RXM90,
                GateType::RX180,
                GateType::RY90,
                GateType::RYM90,
                GateType::RY180,
                GateType::RZ90,
                GateType::RZM90,
                GateType::RZ180,
                GateType::SWAP,
                GateType::SQSWAP,
            ] {
                self.add_detector(
                    *gate_type,
                    gate_type.into_detector(epsilon, ignore_global_phase),
                );
            }
            Ok(())
        }
    }
}

impl<T, K> MatrixMapBuilder<T, K> {
    /// Returns a new MatrixMapBuilder.
    pub fn new() -> Self {
        MatrixMapBuilder {
            map: MatrixMap {
                detectors: vec![],
                map: RefCell::new(HashMap::new()),
            },
        }
    }

    /// Adds a detector to this MatrixMapBuilder.
    pub fn with_detector<F: Fn(&Matrix) -> Result<Option<T>> + 'static>(
        mut self,
        key: K,
        callback: F,
    ) -> Self {
        self.add_detector(key, callback);
        self
    }

    /// Adds a detector to this MatrixMapBuilder.
    pub(crate) fn add_detector<F: Fn(&Matrix) -> Result<Option<T>> + 'static>(
        &mut self,
        key: K,
        callback: F,
    ) {
        self.map.detectors.push((key, Box::new(callback)));
    }

    /// Returns the constructed MatrixMap.
    pub fn finish(self) -> MatrixMap<T, K> {
        self.map
    }
}

impl<T, K> Default for MatrixMapBuilder<T, K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, K> From<MatrixMapBuilder<T, K>> for MatrixMap<T, K> {
    fn from(builder: MatrixMapBuilder<T, K>) -> Self {
        builder.finish()
    }
}

/// Returns a detector function which detects the given Matrix.
pub fn matrix_detector<T: Default>(
    matrix: Matrix,
    epsilon: f64,
    ignore_global_phase: bool,
) -> Box<dyn Fn(&Matrix) -> Result<Option<T>>> {
    Box::new(move |input: &Matrix| -> Result<Option<T>> {
        Ok(if matrix.approx_eq(input, epsilon, ignore_global_phase) {
            Some(T::default())
        } else {
            None
        })
    })
}

/// Assuming that there is an x and y for which the inputs are equal to the
/// following equations:
///
/// a = sin(x) sin(y) + cos(x) cos(y) = Re(e^(iy - ix))
/// b = cos(x) sin(y) - sin(x) cos(y) = Im(e^(iy - ix))
/// c = cos(x) cos(y) - sin(x) sin(y) = Re(e^(iy + ix))
/// d = sin(x) cos(y) + cos(x) sin(y) = Im(e^(iy + ix))
///
/// computes and returns x.
fn detect_angle(a: f64, b: f64, c: f64, d: f64) -> f64 {
    (Complex64::new(a, -b) * Complex64::new(c, d)).arg()
}

/// Normalizes the given complex number, defaulting to 1 if the norm is zero.
fn try_normalize(x: Complex64) -> Complex64 {
    let x = x.unscale(x.norm());
    if x.is_nan() {
        Complex64::new(1.0, 0.0)
    } else {
        x
    }
}

/// Detector function for RX gates.
pub fn detect_rx(matrix: &Matrix, epsilon: f64, ignore_global_phase: bool) -> Option<f64> {
    let cc = matrix[(0, 0)].re;
    let cs = matrix[(0, 0)].im;
    let ss = matrix[(1, 0)].re;
    let sc = -matrix[(1, 0)].im;
    let theta = detect_angle(ss + cc, cs - sc, cc - ss, sc + cs);
    let expected: Matrix = UnboundGate::RX(theta).into();
    if matrix.approx_eq(&expected, epsilon, ignore_global_phase) {
        Some(theta)
    } else {
        None
    }
}

/// Detector function for RY gates.
pub fn detect_ry(matrix: &Matrix, epsilon: f64, ignore_global_phase: bool) -> Option<f64> {
    let cc = matrix[(0, 0)].re;
    let cs = matrix[(0, 0)].im;
    let ss = -matrix[(1, 0)].im;
    let sc = -matrix[(1, 0)].re;
    let theta = -detect_angle(ss + cc, cs - sc, cc - ss, sc + cs);
    let expected: Matrix = UnboundGate::RY(theta).into();
    if matrix.approx_eq(&expected, epsilon, ignore_global_phase) {
        Some(theta)
    } else {
        None
    }
}

/// Detector function for RZ gates.
pub fn detect_rz(matrix: &Matrix, epsilon: f64, ignore_global_phase: bool) -> Option<f64> {
    let theta = detect_angle(
        matrix[(0, 0)].re,
        matrix[(0, 0)].im,
        matrix[(1, 1)].re,
        matrix[(1, 1)].im,
    );
    let expected: Matrix = UnboundGate::RZ(theta).into();
    if matrix.approx_eq(&expected, epsilon, ignore_global_phase) {
        Some(theta)
    } else {
        None
    }
}

/// Detector function for RK gates.
pub fn detect_rk(matrix: &Matrix, epsilon: f64, ignore_global_phase: bool) -> Option<usize> {
    let theta = detect_angle(
        matrix[(0, 0)].re,
        matrix[(0, 0)].im,
        matrix[(1, 1)].re,
        matrix[(1, 1)].im,
    );
    let k = if theta <= 0.0 {
        0usize
    } else {
        (-(theta / PI).log(2.0).round()) as usize
    };
    let expected: Matrix = UnboundGate::RK(k).into();
    if matrix.approx_eq(&expected, epsilon, ignore_global_phase) {
        Some(k)
    } else {
        None
    }
}

/// Detector function for R (= IBM U) gates.
pub fn detect_r(
    matrix: &Matrix,
    epsilon: f64,
    ignore_global_phase: bool,
) -> Option<(f64, f64, f64)> {
    let m00 = matrix[(0, 0)];
    let m01 = matrix[(0, 1)];
    let m10 = matrix[(1, 0)];
    let m11 = matrix[(1, 1)];

    let theta = Complex64::new(m00.norm() + m11.norm(), m01.norm() + m10.norm()).arg() * 2.0;

    let phi_phase = try_normalize(m10 * m00.conj());
    let lambda_phase = if theta < 0.5 * PI {
        try_normalize(m11 * m00.conj()) * phi_phase.conj()
    } else {
        try_normalize(-m01 * m10.conj()) * phi_phase
    };
    let lambda = lambda_phase.arg();
    let phi = phi_phase.arg();

    let theta = if (m10 * m00.conj() * phi_phase.conj()).re < 0.0 {
        -theta
    } else {
        theta
    };

    let expected: Matrix = UnboundGate::R(theta, phi, lambda).into();
    if matrix.approx_eq(&expected, epsilon, ignore_global_phase) {
        Some((theta, phi, lambda))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::{gates::UnboundGate, util::log_2};
    use std::convert::TryInto;

    #[test]
    fn default() {
        let mm: MatrixMap<bool, GateType> = MatrixMap::default();
        let gate = GateType::X;
        let matrix: UnboundGate = gate.try_into().unwrap();
        assert_eq!(mm.detect(&matrix.into()).unwrap().unwrap().0, GateType::X)
    }

    #[test]
    fn matrix_map_builder() {
        // Construct a map which always returns x.
        let matrix_map = MatrixMap::builder()
            .with_detector("key", |_| Ok(Some("x")))
            .finish();
        let matrix = Matrix::new(vec![c!(1.)]);
        let detect = matrix_map.detect_with(&matrix, &["unknown"]);
        assert!(detect.is_ok());
        assert!(detect.unwrap().is_none());

        let detect = matrix_map.detect_with(&matrix, &["key"]);
        assert!(detect.is_ok());
        assert!(detect.as_ref().unwrap().is_some());
        assert_eq!(detect.unwrap(), Some(("key", "x")));

        let detect = matrix_map.detect(&matrix);
        assert!(detect.is_ok());
        assert!(detect.as_ref().unwrap().is_some());
        assert_eq!(detect.unwrap(), Some(("key", "x")));

        let matrix = Matrix::new(vec![c!(1.)]);
        let matrix_map = MatrixMapBuilder::default()
            .with_detector("key", |input| {
                if input.approx_eq(&Matrix::new(vec![c!(1.)]), 0., true) {
                    Ok(Some(1))
                } else {
                    Ok(None)
                }
            })
            .finish();
        let detect = matrix_map.detect(&matrix);
        assert!(detect.is_ok());
        assert!(detect.as_ref().unwrap().is_some());
        assert_eq!(detect.unwrap(), Some(("key", 1)));

        let detect = matrix_map.detect(&matrix);
        assert!(detect.is_ok());
        assert!(detect.as_ref().unwrap().is_some());
        assert_eq!(detect.unwrap(), Some(("key", 1)));

        let detect = matrix_map.detect_with(&matrix, &["unknown"]);
        assert!(detect.is_ok());
        assert!(detect.as_ref().unwrap().is_none());

        let matrix = Matrix::new(vec![c!(0.)]);
        let detect = matrix_map.detect(&matrix);
        assert!(detect.is_ok());
        assert!(detect.as_ref().unwrap().is_none());
    }

    fn unitary(theta: f64, phi: f64, lambda: f64, alpha: f64) -> Matrix {
        let a = (theta / 2.).cos();
        let b = (theta / 2.).sin();
        let c = (phi + lambda) / 2.;
        let d = (phi - lambda) / 2.;
        vec![
            Complex64::new(0., -c + alpha).exp() * a,
            -Complex64::new(0., -d + alpha).exp() * b,
            Complex64::new(0., d + alpha).exp() * b,
            Complex64::new(0., c + alpha).exp() * a,
        ]
        .into()
    }

    #[test]
    fn detectors() {
        // must be power of two, 2 or more
        let steps = 8i32;

        let l2steps = log_2(steps as usize).unwrap();
        for th in -steps + 1..steps {
            let theta: f64 = th as f64 * PI / steps as f64;
            for ph in -steps + 1..steps {
                let phi: f64 = ph as f64 * PI / steps as f64;
                for lm in -steps + 1..steps {
                    let lambda: f64 = lm as f64 * PI / steps as f64;
                    for al in -steps + 1..steps {
                        let alpha: f64 = al as f64 * PI / steps as f64;
                        let matrix = unitary(theta, phi, lambda, alpha);

                        // Test RX detection
                        let rx = detect_rx(&matrix, 0.0001, true);
                        if ph == -steps / 2 && lm == steps / 2 {
                            assert_eq!((rx.unwrap() * steps as f64 / PI).round() as i32, th);
                        } else if ph == steps / 2 && lm == -steps / 2 {
                            assert_eq!((rx.unwrap() * steps as f64 / PI).round() as i32, -th);
                        } else if th == 0 && ph == -lm {
                            assert_eq!((rx.unwrap() * steps as f64 / PI).round() as i32, 0);
                        } else if !rx.is_none() {
                            panic!("{} {} {} {} = rx?", th, ph, lm, al);
                        }

                        // Test RY detection
                        let ry = detect_ry(&matrix, 0.0001, true);
                        if ph == 0 && lm == 0 {
                            assert_eq!((ry.unwrap() * steps as f64 / PI).round() as i32, th);
                        } else if th == 0 && ph == -lm {
                            assert_eq!((ry.unwrap() * steps as f64 / PI).round() as i32, 0);
                        } else if !ry.is_none() {
                            panic!("{} {} {} {} = ry?", th, ph, lm, al);
                        }

                        // Test RZ detection
                        let rz = detect_rz(&matrix, 0.0001, true);
                        if th == 0 {
                            let x = lm + ph - (rz.unwrap() * steps as f64 / PI).round() as i32;
                            assert_eq!(x % (steps * 2), 0);
                        } else if !rz.is_none() {
                            panic!("{} {} {} {} = rz?", th, ph, lm, al);
                        }

                        // Test RK detection
                        let rk = detect_rk(&matrix, 0.0001, true);
                        if th == 0 {
                            let x = ((lm + ph).rem_euclid(steps * 2)) as usize;
                            let k = log_2(x).map(|x| l2steps - x);
                            assert_eq!(k, rk);
                        } else if !rk.is_none() {
                            panic!("{} {} {} {} = rk?", th, ph, lm, al);
                        }

                        // Test R detection
                        let r = detect_r(&matrix, 0.1, true);
                        if r.is_none() {
                            panic!("{} {} {} {} != r?", th, ph, lm, al);
                        }
                    }
                }
            }
        }
    }
}
