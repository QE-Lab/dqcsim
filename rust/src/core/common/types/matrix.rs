use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    iter::FromIterator,
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

/// This mod provides ser/de for Vec<Complex64>
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

    pub fn serialize<S>(value: &[Complex64], serializer: S) -> Result<S::Ok, S::Error>
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
            let (a, b) = complex64_as_bytes(c);
            a.hash(state);
            b.hash(state);
        });
    }
}

// Byte-wise PartialEq for Matrix.
impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        // This results in an byte-wise comparison to determine Eq.
        self.len() == other.len()
            && self
                .data
                .iter()
                .zip(other.data.iter())
                .all(|(x, y)| complex64_as_bytes(x) == complex64_as_bytes(y))
    }
}

// Byte-wise Eq for Matrix.
impl Eq for Matrix {}

impl Index<(usize, usize)> for Matrix {
    type Output = Complex64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1 * self.dimension + index.0]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.1 * self.dimension + index.0]
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

impl From<Vec<Complex64>> for Matrix {
    fn from(elements: Vec<Complex64>) -> Self {
        Matrix::new(elements)
    }
}

impl FromIterator<Complex64> for Matrix {
    fn from_iter<I: IntoIterator<Item = Complex64>>(iter: I) -> Self {
        Matrix::new(iter)
    }
}

impl IntoIterator for Matrix {
    type Item = Complex64;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Matrix {
    /// Returns a new Matrix with provided elements.
    pub fn new(elements: impl IntoIterator<Item = Complex64>) -> Self {
        let elements = elements.into_iter().collect::<Vec<Complex64>>();
        let dimension = (elements.len() as f64).sqrt() as usize;
        Matrix {
            data: elements,
            dimension,
        }
    }

    /// Approximately compares this Matrix with another Matrix.
    /// TODO(mb): add details about this comparison
    pub fn approx_eq(&self, other: &Matrix, epsilon: f64) -> bool {
        // Sizes must match
        if self.len() != other.len() {
            return false;
        }
        let phase_delta =
            self.data
                .iter()
                .zip(other.data.iter())
                .fold(c!(0.), |mut phase_delta, (a, b)| {
                    phase_delta += a * b.conj();
                    phase_delta
                });
        let phase_delta = phase_delta / phase_delta.norm();
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

    /// Returns new Matrix with control behavior removed from the Matrix, and
    /// the control indices corresponding to the target qubits acting as
    /// control in the original Matrix.
    pub fn strip_control(&self) -> (Self, HashSet<usize>) {
        unimplemented!()
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
    /// Returns the element at given row and colum index.
    /// Returns `None` for out of bound indices.
    pub fn get(&self, row: usize, column: usize) -> Option<&Complex64> {
        self.data.get(row * self.dimension + column)
    }
}

// This looks likes nothing to me.
fn complex64_as_bytes(c: &Complex64) -> (u64, u64) {
    unsafe { (std::mem::transmute(c.re), std::mem::transmute(c.im)) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_1_SQRT_2;

    #[test]
    fn matrix() {
        let mut a = Matrix::new(vec![c!(1.), c!(1.), c!(1.), c!(-1.)]);
        let b = Matrix::new(vec![c!(0., 1.), c!(0.), c!(1.), c!(-1., -1.)]);
        let c = Matrix::new(vec![c!(1.), c!(-1., -1.)]);
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
        let x1 = Matrix::new(vec![c!(0.), c!(1.), c!(1.), c!(0.)]);
        let x2 = Matrix::new(vec![c!(0.), c!(0., -1.), c!(0., -1.), c!(0.)]);
        assert!(x1.approx_eq(&x2, 0.));
        assert!(x2.approx_eq(&x1, 0.));

        let h1 = Matrix::new(vec![
            c!(FRAC_1_SQRT_2),
            c!(FRAC_1_SQRT_2),
            c!(FRAC_1_SQRT_2),
            c!(-FRAC_1_SQRT_2),
        ]);
        let h2 = Matrix::new(vec![
            c!(-FRAC_1_SQRT_2),
            c!(-FRAC_1_SQRT_2),
            c!(-FRAC_1_SQRT_2),
            c!(FRAC_1_SQRT_2),
        ]);
        let h3 = Matrix::new(vec![
            c!(0., FRAC_1_SQRT_2),
            c!(0., FRAC_1_SQRT_2),
            c!(0., FRAC_1_SQRT_2),
            c!(0., -FRAC_1_SQRT_2),
        ]);
        let h4 = Matrix::new(vec![
            c!(0., -FRAC_1_SQRT_2),
            c!(0., -FRAC_1_SQRT_2),
            c!(0., -FRAC_1_SQRT_2),
            c!(0., FRAC_1_SQRT_2),
        ]);
        assert!(h1.approx_eq(&h2, 0.));
        assert!(h1.approx_eq(&h3, 0.));
        assert!(h1.approx_eq(&h4, 0.));
        assert!(h2.approx_eq(&h1, 0.));
        assert!(h2.approx_eq(&h3, 0.));
        assert!(h2.approx_eq(&h4, 0.));
        assert!(h3.approx_eq(&h1, 0.));
        assert!(h3.approx_eq(&h2, 0.));
        assert!(h3.approx_eq(&h4, 0.));
        assert!(h4.approx_eq(&h1, 0.));
        assert!(h3.approx_eq(&h2, 0.));
        assert!(h3.approx_eq(&h3, 0.));
    }
}
