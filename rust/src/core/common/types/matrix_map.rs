use crate::core::common::types::Matrix;
use std::{cell::RefCell, collections::HashMap};

/// MatrixMap type to detect gates based on their matrices.
/// Users can add a key for every registered detector and link this to a type
/// T. A MatrixMap can be constructed using a MatrixMapBuilder.
pub struct MatrixMap<T, K> {
    detectors: Vec<(K, Box<Detector<T>>)>,
    map: RefCell<HashMap<Matrix, (K, T)>>,
}

pub type Detector<T> = dyn Fn(&Matrix) -> Option<T>;

impl<T: Clone, K: Clone + PartialEq> MatrixMap<T, K> {
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

    /// Check this MatrixMap for the provided Matrix.
    pub fn detect(&self, input: &Matrix) -> Option<(K, T)> {
        {
            let hit = self.map.borrow().get(input).cloned();
            if hit.is_some() {
                return hit;
            }
        }
        self.run_detectors(input)
    }

    /// Check this MatrixMap for the provided Matrix, using the detectors with
    /// provided keys. This never uses or updates the internal cache.
    pub fn detect_with(&self, input: &Matrix, keys: &[K]) -> Option<(K, T)> {
        self.detectors
            .iter()
            .filter(|(k, _)| keys.contains(k))
            .find_map(|(k, f)| f(input).map(|v| (k.clone(), v)))
    }

    /// Internal method to run all detectors for the given Matrix. This also
    /// updates the cache on hits.
    fn run_detectors(&self, input: &Matrix) -> Option<(K, T)> {
        self.detectors
            .iter()
            .find_map(|(k, f)| f(input).map(|v| (k.clone(), v)))
            .and_then(|(k, v)| {
                self.map
                    .borrow_mut()
                    .insert(input.clone(), (k.clone(), v.clone()));
                Some((k, v))
            })
    }
}

impl<T: Clone, K: Clone + PartialEq> Default for MatrixMap<T, K> {
    fn default() -> Self {
        MatrixMap::builder().with_defaults().finish()
    }
}

pub struct MatrixMapBuilder<T, K> {
    map: MatrixMap<T, K>,
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

    /// Adds default detectors to this MatrixMapBuilder.
    pub fn with_defaults(self) -> Self {
        unimplemented!()
    }

    /// Add a detector to this MatrixMapBuilder.
    pub fn with_detector<F: Fn(&Matrix) -> Option<T> + 'static>(
        mut self,
        key: K,
        callback: F,
    ) -> Self {
        self.map.detectors.push((key, Box::new(callback)));
        self
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matrix_map_builder() {
        // Construct a map which always returns x.
        let matrix_map = MatrixMap::builder()
            .with_detector("key", |_| Some("x"))
            .finish();
        let matrix = Matrix::new(vec![c!(1.)]);
        assert!(matrix_map.detect_with(&matrix, &["unknown"]).is_none());
        assert_eq!(
            matrix_map.detect_with(&matrix, &["key"]),
            Some(("key", "x"))
        );
        assert_eq!(matrix_map.detect(&matrix), Some(("key", "x")));

        let matrix = Matrix::new(vec![c!(1.)]);
        let matrix_map = MatrixMapBuilder::default()
            .with_detector("key", |input| {
                if input.approx_eq(&Matrix::new(vec![c!(1.)]), 0.) {
                    Some(1)
                } else {
                    None
                }
            })
            .finish();
        assert_eq!(matrix_map.detect(&matrix), Some(("key", 1)));
        // cache hit
        assert_eq!(matrix_map.detect(&matrix), Some(("key", 1)));
    }
}
