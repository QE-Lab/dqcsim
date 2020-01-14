//! Converter trait and ConverterMap collection.
//!
//! Defines the [`Converter`] trait and provides a [`ConverterMap`] collection
//! to store Converters and provide caching for these Converters.
//!
//! [`Converter`]: ./trait.Converter.html
//! [`ConverterMap`]: ./struct.ConverterMap.html

use crate::common::error::{oe_err, oe_inv_arg, Result};
use std::{cell::RefCell, collections::HashMap, hash::Hash};

/// A type that can be used as a Converter.
///
/// Types implementing Converter can be used to detect inputs and link them to
/// their outputs, and vice versa. The output is always a specific case of the
/// input, so detection can fail in the sense that a given input is not an
/// instance of the output type, while the opposite cannot fail this way.
///
/// A collection of types implementing Converter can be used in a ConverterMap
/// to convert common types to plugin-specific types and back. This is
/// primarily used for the C API, where the user cannot define their own
/// converter traits to do the equivalent more ergonomically.
pub trait Converter<I, O> {
    /// The detect function implements the detector function. The return values
    /// are as follows:
    ///  - `Ok(Some(O))`: successful match
    ///  - `Ok(None)`: the input is not an instance of the output type
    ///  - `Err(_)`: something went wrong during detection
    fn detect(&self, input: &I) -> Result<Option<O>>;
    /// The construct function implements the opposite of the detector
    /// function, converting the plugin-specific type to the more generic type.
    ///  - `Ok(O)`: successful construction
    ///  - `Err(_)`: something went wrong during construction
    fn construct(&self, input: &O) -> Result<I>;
}

/// A type that can be used as a cache key in a Converter.
///
/// The cache key must be constructable from a reference to the converter's
/// input. All types that are Clone can be used as a cache key for themselves.
pub trait ConverterCacheKey<I> {
    fn from_input(input: &I) -> Self;
}

/// Blanket ConverterCacheKey implementation for all types that are Clone.
impl<T> ConverterCacheKey<T> for T
where
    T: Clone,
{
    fn from_input(input: &T) -> Self {
        input.clone()
    }
}

/// K: user-defined key for identifying which converter to use
/// I: detector input = constructor output
/// O: detector output = constructor input
/// C: detector cache key. I is converted to C before being placed in the
///    detector cache.
#[derive(Default)]
pub struct ConverterMap<'c, K, I, O, C = I>
where
    K: Eq + Hash,
    C: Eq + Hash,
{
    /// The collection of `Converter`s are stored in this map as trait objects
    /// with a wrapping tuple including the corresponding key.
    converters: HashMap<K, Box<dyn Converter<I, O> + 'c>>,
    /// The order in which converters are called when
    order: Vec<K>,
    /// The cache is stored in a HashMap that maps from input type I to the
    /// output type (K, O).
    cache: RefCell<HashMap<C, Option<(K, O)>>>,
    /// Whether the detector cache should be used to short-circuit straight to
    /// the detection result (true), or only to the converter key (false).
    fully_cached: bool,
}

impl<'c, K, I, O, C> ConverterMap<'c, K, I, O, C>
where
    K: Eq + Hash + Clone,
    C: Eq + Hash,
{
    /// Constructs a new empty ConverterMap.
    pub fn new(fully_cached: bool) -> Self {
        ConverterMap {
            converters: HashMap::new(),
            order: vec![],
            cache: RefCell::new(HashMap::new()),
            fully_cached,
        }
    }

    /// Appends a Converter with the specified key to the back of the collection
    /// of Detectors in this map.
    pub fn push(&mut self, key: impl Into<K>, converter: impl Converter<I, O> + 'c) {
        let key: K = key.into();
        self.cache.borrow_mut().retain(|_, v| {
            if let Some((k, _)) = v {
                k != &key
            } else {
                false
            }
        });
        if self
            .converters
            .insert(key.clone(), Box::new(converter))
            .is_some()
        {
            self.order.retain(|k| k != &key);
        }
        self.order.push(key);
    }

    /// Inserts a Converter at position index within the collection of Detectors
    /// in this map, with the specified key associated to the inserted
    /// Converter.
    pub fn insert(
        &mut self,
        index: usize,
        key: impl Into<K>,
        converter: impl Converter<I, O> + 'c,
    ) {
        self.clear_cache();
        let key: K = key.into();
        if self
            .converters
            .insert(key.clone(), Box::new(converter))
            .is_some()
        {
            self.order.retain(|k| k != &key);
        }
        self.order.insert(index, key);
    }

    /// Appends the specified Converter with the corresponding specified key to
    /// the collection and returns the updated DetectorMap.
    pub fn with(mut self, key: impl Into<K>, converter: impl Converter<I, O> + 'c) -> Self {
        self.push(key, converter);
        self
    }

    /// Clears the cache.
    pub fn clear_cache(&self) {
        self.cache.borrow_mut().clear();
    }

    /// Returns the number of Detectors in the collection.
    pub fn len(&self) -> usize {
        self.converters.len()
    }

    /// Returns true if the collection contains no Detectors.
    pub fn is_empty(&self) -> bool {
        self.converters.is_empty()
    }
}

impl<'c, I, K, O, C> Converter<I, (K, O)> for ConverterMap<'c, K, I, O, C>
where
    I: Clone,
    K: Eq + Hash + Clone,
    C: Eq + Hash + ConverterCacheKey<I>,
    O: Clone,
{
    fn detect(&self, input: &I) -> Result<Option<(K, O)>> {
        // Get the cache key for this input.
        let cache_key = C::from_input(input);

        // Check the cache.
        if let Some(hit) = self.cache.borrow().get(&cache_key) {
            // Cache hit. If we're fully cached, we can return the output
            // immediately. If we're not fully cached, we need to call the
            // detector function that matched this input the previous time.
            // If there was no such match, we can return that there is no
            // match without calling anything.
            if self.fully_cached {
                Ok(hit.clone())
            } else if let Some((key, _)) = hit {
                Ok(Some((
                    key.clone(),
                    self.converters[key]
                        .detect(input)?
                        .ok_or_else(oe_err("unstable detector function"))?,
                )))
            } else {
                Ok(None)
            }
        } else {
            // Cache miss. Check all converters in order.
            self.order
                .iter()
                .find_map(|k| {
                    self.converters[k]
                        .detect(input)
                        .map(|res| res.map(|output| (k.clone(), output)))
                        .transpose()
                })
                .transpose()
                .and_then(|output| {
                    self.cache.borrow_mut().insert(cache_key, output.clone());
                    Ok(output)
                })
        }
    }

    fn construct(&self, input: &(K, O)) -> Result<I> {
        self.converters
            .get(&input.0)
            .ok_or_else(oe_inv_arg("key does not map to any converter"))?
            .construct(&input.1)
    }
}
