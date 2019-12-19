use crate::core::common::{
    error::{inv_arg, inv_op, Result},
    gates::GateType,
    types::Matrix,
};
use std::{
    cell::RefCell,
    collections::HashMap,
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::gates::UnboundGate;
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
}
