use crate::common::{
    error::{inv_arg, Error, Result},
    util::log_2,
};
use integer_sqrt::IntegerSquareRoot;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
#[cfg(feature = "bindings")]
use std::os::raw::c_double;
use std::{
    collections::HashSet,
    convert::TryFrom,
    f64::consts::FRAC_1_SQRT_2,
    fmt,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
};

/// Matrix wrapper for `Gate` matrices.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Matrix {
    /// The elements in the matrix stored as a `Vec<Complex64>`.
    #[serde(with = "complex_serde")]
    data: Vec<Complex64>,
    /// Cached dimension of inner data.
    #[serde(skip)]
    dimension: usize,
}

/// This mod provides ser/de for Vec<Complex64>.
mod complex_serde {
    use super::Complex64;
    use serde::{
        ser::SerializeSeq,
        {Deserialize, Deserializer, Serialize, Serializer},
    };

    #[derive(Serialize, Deserialize)]
    #[serde(remote = "Complex64")]
    struct Complex64Def {
        re: f64,
        im: f64,
    }

    pub fn serialize<S>(value: &[Complex64], serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Wrapper<'a>(#[serde(with = "Complex64Def")] &'a Complex64);
        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for c in value.iter().map(Wrapper) {
            seq.serialize_element(&c)?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<Vec<Complex64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper(#[serde(with = "Complex64Def")] Complex64);
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|Wrapper(c)| c).collect())
    }
}

impl Hash for Matrix {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // f64 is not Hash and not Eq.
        // However, since Hash here is used to cache results false negatives
        // don't break anything.
        self.data.iter().for_each(|c| {
            c.re.to_le_bytes().hash(state);
            c.im.to_le_bytes().hash(state);
        });
    }
}

// Byte-wise PartialEq for Matrix.
impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        // This is a byte-wise comparison to determine Eq.
        self.len() == other.len()
            && self.data.iter().zip(other.data.iter()).all(|(x, y)| {
                x.re.to_le_bytes() == y.re.to_le_bytes() && x.im.to_le_bytes() == y.im.to_le_bytes()
            })
    }
}

// Byte-wise Eq for Matrix.
impl Eq for Matrix {}

impl Index<(usize, usize)> for Matrix {
    type Output = Complex64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.dimension + index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.dimension + index.1]
    }
}

impl Index<usize> for Matrix {
    type Output = Complex64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl TryFrom<Vec<Complex64>> for Matrix {
    type Error = Error;
    fn try_from(elements: Vec<Complex64>) -> Result<Self> {
        Matrix::new(elements)
    }
}

impl IntoIterator for Matrix {
    type Item = Complex64;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in 0..self.dimension() {
            for col in 0..self.dimension() {
                let e = self[(row, col)];
                write!(f, "{:6.3}{:+6.3}i  ", e.re, e.im)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Matrix {
    /// Returns a new Matrix with provided elements.
    pub fn new(elements: impl IntoIterator<Item = Complex64>) -> Result<Self> {
        let elements = elements.into_iter().collect::<Vec<Complex64>>();
        let dimension = elements.len().integer_sqrt();
        if elements.len() != dimension * dimension {
            return inv_arg("matrix is not square");
        }
        Ok(Matrix {
            dimension: elements.len().integer_sqrt(),
            data: elements,
        })
    }

    /// Returns a new identity Matrix with given dimension.
    pub fn new_identity(dimension: usize) -> Self {
        let mut output = Matrix::new(vec![c!(0.); dimension.pow(2)]).unwrap();
        for i in 0..dimension {
            output[(i, i)] = c!(1.);
        }
        output
    }

    /// Approximately compares this Matrix with another Matrix.
    /// `epsilon` specifies the maximum element-wise root-mean-square error
    /// between the matrices that results in a positive match. `ignore_phase`
    /// specifies whether the aforementioned check should ignore global phase.
    pub fn approx_eq(&self, other: &Matrix, epsilon: f64, ignore_global_phase: bool) -> bool {
        // Sizes must match
        if self.len() != other.len() {
            return false;
        }
        let phase_delta = if ignore_global_phase {
            let phase_delta =
                self.data
                    .iter()
                    .zip(other.data.iter())
                    .fold(c!(0.), |mut phase_delta, (a, b)| {
                        phase_delta += a * b.conj();
                        phase_delta
                    });
            phase_delta / phase_delta.norm()
        } else {
            c!(1.)
        };
        self.data
            .iter()
            .zip(other.data.iter())
            .try_fold(epsilon * epsilon, |mut tolerance, (a, b)| {
                tolerance -= (a - b * phase_delta).norm_sqr();
                if tolerance.is_sign_negative() {
                    None
                } else {
                    Some(tolerance)
                }
            })
            .is_some()
    }

    /// Approximately compares two matrices representing bases with each other.
    ///
    /// The basis representation works as follows:
    ///
    ///  - (apply hermetian of matrix as gate if measurement)
    ///  - measure/prep Z
    ///  - apply matrix as gate
    ///
    /// Any rotation around the Z axis is don't care before and after the
    /// prep/measurement, because that information is thrown away when the
    /// prep/measurement occurs. Global phase is also don't care as usual. That
    /// means the following must hold for the bases to be equal:
    ///
    /// ```text
    ///               / e^ix   0  \  ~
    ///     basis_a * |           |  =  basis_b
    ///               \  0   e^iy /
    ///
    ///     / a_a e^ix   a_b e^iy \  ~  / b_a   b_b \
    ///     |                     |  =  |           |
    ///     \ a_c e^ix   a_d e^iy /     \ b_c   b_d /
    /// ```
    ///
    /// with x and y being free variables.
    pub fn basis_approx_eq(&self, other: &Matrix, epsilon: f64) -> bool {
        // Sizes must both be 2x2
        if self.dimension() != 2 || other.dimension() != 2 {
            return false;
        }
        let mut tolerance = epsilon * epsilon;
        for col in 0..2 {
            let phase_delta =
                self[(0, col)] * other[(0, col)].conj() + self[(1, col)] * other[(1, col)].conj();
            let phase_delta = phase_delta.unscale(phase_delta.norm());
            for row in 0..2 {
                tolerance -= (self[(row, col)] - other[(row, col)] * phase_delta).norm_sqr();
                if tolerance < 0. {
                    return false;
                }
            }
        }
        true
    }

    /// Checks whether this Matrix is approximately unitary by multiplying
    /// itself with its hermetian, and measuring the RMS difference of the
    /// result and the identity matrix. If this difference is more than epsilon
    /// the matrix is not (sufficiently) unitary and this function returns
    /// false, otherwise it returns true.
    pub fn approx_unitary(&self, epsilon: f64) -> bool {
        let mut tolerance = epsilon * epsilon;

        // Check if result of matrix multiplication of self and our hermetian
        // is identity.
        for i in 0..self.dimension {
            for j in 0..self.dimension {
                let element = (0..self.dimension).fold(Complex64::new(0., 0.), |acc, k| {
                    acc + (self[i * self.dimension + k] * self[j * self.dimension + k].conj())
                });
                let expected = if i == j {
                    Complex64::new(1., 0.)
                } else {
                    Complex64::new(0., 0.)
                };
                tolerance -= (expected - element).norm_sqr();
                if tolerance < 0. {
                    return false;
                }
            }
        }
        true
    }

    /// Returns new Matrix with `number_of_control` qubits added.
    pub fn add_controls(&self, number_of_controls: usize) -> Self {
        let dimension = self.dimension() * 2usize.pow(number_of_controls as u32);
        let mut output = Matrix::new_identity(dimension);
        for row in 0..self.dimension() {
            for col in 0..self.dimension() {
                output[(
                    row + dimension - self.dimension(),
                    col + dimension - self.dimension(),
                )] = self[(row, col)];
            }
        }
        output
    }

    /// Returns new Matrix with control behavior removed from the Matrix, and
    /// the control indices corresponding to the target qubits acting as
    /// control in the original Matrix.
    ///
    /// `epsilon` specifies the maximum magitude of the difference between the
    /// column vectors of the input matrix and the identity matrix (after
    /// dephasing if `ignore_phase` is set) for the column vector to be
    /// considered to not affect the respective entry in the quantum state
    /// vector. Note that if this is greater than zero, the resulting gate may
    /// not be exactly equivalent. If `ignore_global_phase` is set, any global
    /// phase in the matrix is ignored, but note that if control qubits are
    /// stripped the "global" phase of the resulting submatrix is always
    /// significant.
    ///
    /// This function assumes that the incoming matrix is unitary (within
    /// `epsilon`) without verifying that this is the case. The results may
    /// thus be invalid if it was not.
    ///
    /// The identity matrix special case is handled by interpreting all qubits
    /// as non-control (any set of qubits would satisfy the controlled matrix
    /// criterium).
    pub fn strip_control(
        &self,
        epsilon: f64,
        ignore_global_phase: bool,
    ) -> (HashSet<usize>, Matrix) {
        // If we're to ignore the global phase of the matrix, we dephase using
        // the phase of the first matrix entry; for any controlled matrix, this
        // entry will have a unit magnitude.
        let phase = if ignore_global_phase {
            Complex64::from_polar(&1.0, &self[(0, 0)].arg())
        } else {
            c!(1.0)
        };

        // Determine which qubits are control qubits. This is done by detecting
        // which column vectors match the column vectors of the identity matrix
        // phased by `phase`. If a column vector is not approximately equal, it
        // may indicate that some qubits cannot be controls. Note that the last
        // column vector is never checked, because it never rules out anything.
        // Note also that we short-circuit to returning the original matrix
        // when all qubits are ruled out as being controls.
        let epsilon_sqr = epsilon.powi(2);
        let mut controls_int = self.dimension() - 1;
        for i in 0..self.dimension() - 1 {
            let mut error_sqr = 0.0;
            for j in 0..self.dimension() {
                if i == j {
                    error_sqr += (self[(i, j)] - phase).norm_sqr();
                } else {
                    error_sqr += self[(i, j)].norm_sqr();
                }
                if error_sqr > epsilon_sqr {
                    controls_int &= i;
                    if controls_int == 0 {
                        return (HashSet::new(), self.clone());
                    }
                    break;
                }
            }
        }

        // If all qubits could be control qubits, we're actually dealing with
        // an identity matrix.
        if controls_int == self.dimension() - 1 {
            return (HashSet::new(), self.clone());
        }

        // Construct a HashSet of the qubits that were found to be controls.
        // Note that the qubit indices used in DQCsim are reversed with respect
        // to the matrix indices, so we do said reversal here.
        let mut controls = HashSet::new();
        let num_qubits = self.num_qubits().unwrap();
        for q in 0..num_qubits {
            if controls_int & (1 << q) != 0 {
                controls.insert(num_qubits - q - 1);
            }
        }

        // Compute the submatrix.
        let mut entries =
            Vec::with_capacity((self.dimension() / 2_usize.pow(controls.len() as u32)).pow(2));
        for row in 0..self.dimension() {
            if row & controls_int == controls_int {
                for col in 0..self.dimension() {
                    if col & controls_int == controls_int {
                        entries.push(self[(row, col)] * phase.conj());
                    }
                }
            }
        }

        (controls, Matrix::new(entries).unwrap())
    }

    /// Returns the number of elements in the Matrix.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Returns true if the Matrix is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the dimension of the Matrix.
    /// The dimension equals the square root of the number of elements.
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Returns the number of qubits for this Matrix.
    pub fn num_qubits(&self) -> Option<usize> {
        log_2(self.dimension)
    }

    /// Returns the element at given row and colum index.
    /// Returns `None` for out of bound indices.
    pub fn get(&self, row: usize, column: usize) -> Option<&Complex64> {
        self.data.get(row * self.dimension + column)
    }

    #[cfg(feature = "bindings")]
    pub(crate) fn as_ptr(&self) -> *const c_double {
        self.data.as_ptr() as *const c_double
    }
}

/// Predefined measurement/prep bases.
#[derive(Clone, Copy, Debug)]
pub enum Basis {
    X,
    Y,
    Z,
}

impl From<Basis> for Matrix {
    fn from(basis: Basis) -> Matrix {
        match basis {
            Basis::X => matrix!(
                (FRAC_1_SQRT_2), (-FRAC_1_SQRT_2);
                (FRAC_1_SQRT_2), (FRAC_1_SQRT_2);
            ),
            Basis::Y => matrix!(
                (FRAC_1_SQRT_2), (0., FRAC_1_SQRT_2);
                (0., FRAC_1_SQRT_2), (FRAC_1_SQRT_2);
            ),
            Basis::Z => matrix!(
                1., 0.;
                0., 1.;
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::gates::UnboundUnitaryGate;
    use std::iter::FromIterator;

    #[test]
    fn matrix() {
        let mut a = matrix!(
            1., 1.;
            1., (-1.);
        );
        let b = matrix!(
            (0., 1.), 0.;
            1., (-1., -1.);
        );
        let c = matrix!(
            1., (-1., -1.);
            (0., 1.), 0.;
        );
        assert_eq!(a, a);
        assert_eq!(b, b);
        assert_eq!(c, c);
        assert_ne!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
        assert_eq!(a.len(), 4);
        assert_eq!(b.dimension(), 2);
        assert_eq!(b.get(0, 0).unwrap(), &b[(0, 0)]);
        assert!(b.get(4, 4).is_none());
        assert_eq!(b[3], c[1]);
        a[0] = b[3];
        assert_eq!(b[3], a[0]);
        assert_eq!(a[(0, 0)], b[(1, 1)]);
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        a.hash(&mut hasher);
        let h = hasher.finish();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        a.hash(&mut hasher);
        let hh = hasher.finish();
        assert_eq!(h, hh);
    }

    #[test]
    fn matrix_approx_eq() {
        let x1 = matrix!(
            0., 1.;
            1., 0.;
        );
        let x2 = matrix!(
            0., (-1.);
            (-1.), 0.;
        );
        assert!(x1.approx_eq(&x2, 0., true));
        assert!(x2.approx_eq(&x1, 0., true));

        let h1 = matrix!(
            (FRAC_1_SQRT_2), (FRAC_1_SQRT_2);
            (FRAC_1_SQRT_2), (-FRAC_1_SQRT_2);
        );
        let h2 = matrix!(
            (-FRAC_1_SQRT_2), (-FRAC_1_SQRT_2);
            (-FRAC_1_SQRT_2), (FRAC_1_SQRT_2);
        );
        let h3 = matrix!(
            (0., FRAC_1_SQRT_2), (0., FRAC_1_SQRT_2);
            (0., FRAC_1_SQRT_2), (0., -FRAC_1_SQRT_2);
        );
        let h4 = matrix!(
            (0., -FRAC_1_SQRT_2), (0., -FRAC_1_SQRT_2);
            (0., -FRAC_1_SQRT_2), (0., FRAC_1_SQRT_2);
        );
        assert!(h1.approx_eq(&h2, 0., true));
        assert!(h1.approx_eq(&h3, 0., true));
        assert!(h1.approx_eq(&h4, 0., true));
        assert!(h2.approx_eq(&h1, 0., true));
        assert!(h2.approx_eq(&h3, 0., true));
        assert!(h2.approx_eq(&h4, 0., true));
        assert!(h3.approx_eq(&h1, 0., true));
        assert!(h3.approx_eq(&h2, 0., true));
        assert!(h3.approx_eq(&h4, 0., true));
        assert!(h4.approx_eq(&h1, 0., true));
        assert!(h3.approx_eq(&h2, 0., true));
        assert!(h3.approx_eq(&h3, 0., true));
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn basis_approx_eq() {
        let x: Matrix = Basis::X.into();
        let y: Matrix = Basis::Y.into();
        let z: Matrix = Basis::Z.into();
        let x2: Matrix = matrix!(
            (FRAC_1_SQRT_2), (FRAC_1_SQRT_2);
            (FRAC_1_SQRT_2), (-FRAC_1_SQRT_2);
        );
        let y2: Matrix = matrix!(
            (FRAC_1_SQRT_2), (FRAC_1_SQRT_2);
            (0., FRAC_1_SQRT_2), (0., -FRAC_1_SQRT_2);
        );

        assert!(x.basis_approx_eq(&x, 0.));
        assert!(!x.basis_approx_eq(&y, 0.));
        assert!(!x.basis_approx_eq(&z, 0.));
        assert!(x.basis_approx_eq(&x2, 0.));
        assert!(!x.basis_approx_eq(&y2, 0.));

        assert!(!y.basis_approx_eq(&x, 0.));
        assert!(y.basis_approx_eq(&y, 0.));
        assert!(!y.basis_approx_eq(&z, 0.));
        assert!(!y.basis_approx_eq(&x2, 0.));
        assert!(y.basis_approx_eq(&y2, 0.));

        assert!(!z.basis_approx_eq(&x, 0.));
        assert!(!z.basis_approx_eq(&y, 0.));
        assert!(z.basis_approx_eq(&z, 0.));
        assert!(!z.basis_approx_eq(&x2, 0.));
        assert!(!z.basis_approx_eq(&y2, 0.));

        assert!(x2.basis_approx_eq(&x, 0.));
        assert!(!x2.basis_approx_eq(&y, 0.));
        assert!(!x2.basis_approx_eq(&z, 0.));
        assert!(x2.basis_approx_eq(&x2, 0.));
        assert!(!x2.basis_approx_eq(&y2, 0.));

        assert!(!y2.basis_approx_eq(&x, 0.));
        assert!(y2.basis_approx_eq(&y, 0.));
        assert!(!y2.basis_approx_eq(&z, 0.));
        assert!(!y2.basis_approx_eq(&x2, 0.));
        assert!(y2.basis_approx_eq(&y2, 0.));
    }

    #[test]
    fn add_controls() {
        let x: Matrix = UnboundUnitaryGate::X.into();
        assert!(x.add_controls(1).approx_eq(
            &matrix!(
                1., 0., 0., 0.;
                0., 1., 0., 0.;
                0., 0., 0., 1.;
                0., 0., 1., 0.;
            ),
            0.0001,
            false
        ));
    }

    #[test]
    fn strip_control() {
        let cnot_a = matrix!(
            1., 0., 0., 0.;
            0., 1., 0., 0.;
            0., 0., 0., 1.;
            0., 0., 1., 0.;
        );
        let cnot_b = matrix!(
            1., 0., 0., 0.;
            0., 0., 0., 1.;
            0., 0., 1., 0.;
            0., 1., 0., 0.;
        );

        let (map_a, matrix_a) = cnot_a.strip_control(0.0001, false);
        assert_eq!(map_a, HashSet::from_iter(vec![0]));
        let (map_b, matrix_b) = cnot_b.strip_control(0.0001, false);
        assert_eq!(map_b, HashSet::from_iter(vec![1]));
        assert!(matrix_a.approx_eq(&matrix_b, 0.0001, false));
        assert_ne!(map_a, map_b);

        let i = matrix!(
            1., 0., 0., 0., 0., 0., 0., 0.;
            0., 1., 0., 0., 0., 0., 0., 0.;
            0., 0., 1., 0., 0., 0., 0., 0.;
            0., 0., 0., 1., 0., 0., 0., 0.;
            0., 0., 0., 0., 1., 0., 0., 0.;
            0., 0., 0., 0., 0., 1., 0., 0.;
            0., 0., 0., 0., 0., 0., 1., 0.;
            0., 0., 0., 0., 0., 0., 0., 1.;
        );

        let (map_a, matrix_a) = i.strip_control(0.0001, false);
        assert!(matrix_a.approx_eq(&Matrix::new_identity(8), 0.0001, false));
        assert!(map_a.is_empty());

        let fredkin = matrix!(
            1., 0., 0., 0., 0., 0., 0., 0.;
            0., 1., 0., 0., 0., 0., 0., 0.;
            0., 0., 1., 0., 0., 0., 0., 0.;
            0., 0., 0., 1., 0., 0., 0., 0.;
            0., 0., 0., 0., 1., 0., 0., 0.;
            0., 0., 0., 0., 0., 0., 1., 0.;
            0., 0., 0., 0., 0., 1., 0., 0.;
            0., 0., 0., 0., 0., 0., 0., 1.;
        );

        let (map_a, matrix_a) = fredkin.strip_control(0.0001, false);
        assert_eq!(map_a, HashSet::from_iter(vec![0]));
        let x: Matrix = UnboundUnitaryGate::SWAP.into();
        assert!(matrix_a.approx_eq(&x, 0.0001, false));

        let toffoli = matrix!(
            1., 0., 0., 0., 0., 0., 0., 0.;
            0., 1., 0., 0., 0., 0., 0., 0.;
            0., 0., 1., 0., 0., 0., 0., 0.;
            0., 0., 0., 1., 0., 0., 0., 0.;
            0., 0., 0., 0., 1., 0., 0., 0.;
            0., 0., 0., 0., 0., 1., 0., 0.;
            0., 0., 0., 0., 0., 0., 0., 1.;
            0., 0., 0., 0., 0., 0., 1., 0.;
        );

        let (map_a, matrix_a) = toffoli.strip_control(0.0001, false);
        assert_eq!(map_a, HashSet::from_iter(vec![0, 1]));

        let x: Matrix = UnboundUnitaryGate::X.into();
        assert!(matrix_a.approx_eq(&x, 0.001, false));
    }
}
