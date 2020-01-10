//! Detector trait and DetectorMap collection.
//!
//! Defines the [`Detector`] trait and provides a [`DetectorMap`] collection to
//! store Detectors and provide caching for these Detectors.
//!
//! [`Detector`]: ./trait.Detector.html
//! [`DetectorMap`]: ./struct.DetectorMap.html

use crate::common::{error::Result, types::Matrix};
use std::{cell::RefCell, collections::HashMap, hash::Hash};
use std::{fmt, fmt::Debug};

/// A type that can be used as a Detector.
///
/// Types implementing Detector can be used to detect inputs and link them to
/// their outputs. A collection of types implementing Detector can be used in a
/// DetectorMap to convert common types to plugin-specific types.
pub trait Detector {
    /// The input type of the Detector function.
    type Input;
    /// The output type of the Detector function.
    type Output;
    /// The detect function implements the detector function. When the detector
    /// matches it returns a success result value with a some option value of
    /// the output type.
    fn detect(&self, input: &Self::Input) -> Result<Option<Self::Output>>;
}

/// A collection of Detector types can be stored in a DetectorMap. The
/// DetectorMap in turn implements the Detector trait to enable cached
/// detection using multiple Detectors.
///
/// The generic lifetime 'a indicates the lifetime bound of the Detectors
/// stored in the map. The type K is the type of the key used for the
/// Detectors, and is included in the return type of the Detector
/// implementation of the DetectorMap. The type I is the associated Input type
/// of the Detectors in this map, and the type O is the associated Output type
/// of the Detectors in this map. The optional type C, defaults to the Input
/// type I, can be used to preprocess the Input type into a common type used in
/// the Detectors.
#[derive(Default)]
pub struct DetectorMap<'a, K, I, O, C = I>
where
    I: Eq + Hash,
{
    /// The collection of Detectors are stored in this map as trait objects
    /// with a wrapping tuple including the corresponding key.
    detectors: Vec<(K, Box<dyn Detector<Input = C, Output = O> + 'a>)>,
    /// The cache is stored in a HashMap that maps from input type I to the
    /// output type (K, O).
    map: RefCell<HashMap<I, Option<(K, O)>>>,
}

impl<'a, K, I, O, C> DetectorMap<'a, K, I, O, C>
where
    I: Hash + Eq,
{
    /// Constructs a new empty DetectorMap.
    pub fn new() -> Self {
        DetectorMap {
            detectors: vec![],
            map: RefCell::new(HashMap::new()),
        }
    }

    /// Constructs a new DetectorMap with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        DetectorMap {
            detectors: Vec::with_capacity(capacity),
            map: RefCell::new(HashMap::new()),
        }
    }

    /// Appends a Detector with the specified key to the back of the collection
    /// of Detectors in this map.
    pub fn push(&mut self, key: impl Into<K>, detector: impl Detector<Input = C, Output = O> + 'a) {
        self.map.borrow_mut().retain(|_, v| v.is_some());
        self.detectors.push((key.into(), Box::new(detector)));
    }

    /// Inserts a Detector at position index within the collection of Detectors
    /// in this map, with the specified key associated to the inserted
    /// Detector.
    pub fn insert(
        &mut self,
        index: usize,
        key: impl Into<K>,
        detector: impl Detector<Input = C, Output = O> + 'a,
    ) {
        self.clear_cache();
        self.detectors
            .insert(index, (key.into(), Box::new(detector)));
    }

    /// Appends the specified Detector with the corresponding specified key to
    /// the collection and returns the updated DetectorMap.
    pub fn with(
        mut self,
        key: impl Into<K>,
        detector: impl Detector<Input = C, Output = O> + 'a,
    ) -> Self {
        self.push(key, detector);
        self
    }

    /// Clears the cache.
    pub fn clear_cache(&self) {
        self.map.borrow_mut().clear();
    }

    /// Returns the number of Detectors in the collection.
    pub fn len(&self) -> usize {
        self.detectors.len()
    }

    /// Returns true if the collection contains no Detectors.
    pub fn is_empty(&self) -> bool {
        self.detectors.is_empty()
    }
}

impl<K, I, O, C> Detector for DetectorMap<'_, K, I, O, C>
where
    K: Clone,
    I: Clone + Eq + Hash + Into<C>,
    O: Clone,
{
    type Input = I;
    type Output = (K, O);

    fn detect(&self, input: &I) -> Result<Option<(K, O)>> {
        // Check the cache
        let cache = || self.map.borrow().get(input).cloned();
        cache()
            // Return first result of successful detector
            .map_or_else(
                || {
                    let input = input.to_owned().into();
                    self.detectors
                        .iter()
                        .find_map(|(k, f)| {
                            f.detect(&input)
                                .map(|res| res.map(|opt| (k.clone(), opt)))
                                .transpose()
                        })
                        .transpose()
                },
                Result::Ok,
            )
            // Update the cache
            .and_then(|opt| {
                self.map.borrow_mut().insert(input.to_owned(), opt.clone());
                Ok(opt)
            })
    }
}

impl<K, I, O, C> Debug for DetectorMap<'_, K, I, O, C>
where
    I: Eq + Hash,
    K: Eq + Hash + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set()
            .entries(self.detectors.iter().map(|(k, _)| k))
            .finish()
    }
}

/// A MatrixDetector to detect Matrix instances.
#[derive(Clone, Debug)]
pub struct MatrixDetector<'matrix, T> {
    matrix: &'matrix Matrix,
    epsilon: f64,
    ignore_global_phase: bool,
    output: &'matrix T,
}

impl<'matrix, T> MatrixDetector<'matrix, T> {
    /// Constructs a new MatrixDetector
    pub fn new(
        matrix: &'matrix Matrix,
        epsilon: f64,
        ignore_global_phase: bool,
        output: &'matrix T,
    ) -> Self {
        Self {
            matrix,
            epsilon,
            ignore_global_phase,
            output,
        }
    }
}

impl<'matrix, T> Detector for MatrixDetector<'matrix, T> {
    type Input = Matrix;
    type Output = &'matrix T;

    fn detect(&self, input: &Self::Input) -> Result<Option<Self::Output>> {
        Ok(
            if self
                .matrix
                .approx_eq(input, self.epsilon, self.ignore_global_phase)
            {
                Some(self.output)
            } else {
                None
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::detector::Detector;
    use crate::common::gates::GateType;

    #[test]
    fn matrix_detector() {
        let matrix = Matrix::new(vec![c!(1.), c!(0.), c!(0.), c!(1.)]);
        let detector = MatrixDetector::new(&matrix, 0.001, true, &GateType::I);
        assert_eq!(detector.detect(&matrix).unwrap(), Some(&GateType::I));
    }

    #[test]
    fn detector_map() {
        let matrix = Matrix::new(vec![c!(1.23, 3.45)]);
        let detector = MatrixDetector::new(&matrix, 0.001, true, &GateType::I);
        assert_eq!(detector.detect(&matrix).unwrap(), Some(&GateType::I));

        let detector_map = DetectorMap::new().with("test", detector);
        assert_eq!(
            detector_map.detect(&matrix).unwrap(),
            Some(("test", &GateType::I))
        );
        assert_eq!(
            detector_map.detect(&matrix).unwrap(),
            Some(("test", &GateType::I))
        );

        assert_eq!(format!("{:?}", detector_map), "{\"test\"}");
    }
}
