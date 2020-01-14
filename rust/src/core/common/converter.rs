//! Converter trait and ConverterMap collection.
//!
//! Defines the [`Converter`] trait and provides a [`ConverterMap`] collection
//! to store Converters and provide caching for these Converters.
//!
//! [`Converter`]: ./trait.Converter.html
//! [`ConverterMap`]: ./struct.ConverterMap.html

use crate::common::{
    error::{inv_arg, oe_err, oe_inv_arg, Result},
    gates::UnboundGate,
    types::{ArbData, Gate, Matrix, QubitRef},
};
use integer_sqrt::IntegerSquareRoot;
use num_complex::Complex64;
use std::{cell::RefCell, collections::HashMap, convert::TryInto, f64::consts::PI, hash::Hash};

/// A type that can be constructed from (part of) an ArbData object.
pub trait FromArb
where
    Self: std::marker::Sized,
{
    /// Construct from the given ArbData, taking the parameters used for
    /// construction out of the data object.
    fn from_arb(src: &mut ArbData) -> Result<Self>;
}

impl FromArb for () {
    fn from_arb(_: &mut ArbData) -> Result<Self> {
        Ok(())
    }
}

impl FromArb for u64 {
    fn from_arb(src: &mut ArbData) -> Result<Self> {
        let mut args = src.get_args_mut().drain(..1);
        let i = args
            .nth(0)
            .ok_or_else(oe_inv_arg("expected 64-bit integer argument in ArbData"))?;
        Ok(u64::from_le_bytes(i[..].try_into().ok().ok_or_else(
            oe_inv_arg("expected 64-bit integer argument in ArbData"),
        )?))
    }
}

impl FromArb for f64 {
    fn from_arb(src: &mut ArbData) -> Result<Self> {
        let mut args = src.get_args_mut().drain(..1);
        let f = args
            .nth(0)
            .ok_or_else(oe_inv_arg("expected double argument in ArbData"))?;
        Ok(f64::from_le_bytes(f[..].try_into().ok().ok_or_else(
            oe_inv_arg("expected double argument in ArbData"),
        )?))
    }
}

impl FromArb for (f64, f64, f64) {
    fn from_arb(src: &mut ArbData) -> Result<Self> {
        let a = f64::from_arb(src)?;
        let b = f64::from_arb(src)?;
        let c = f64::from_arb(src)?;
        Ok((a, b, c))
    }
}

impl FromArb for Matrix {
    fn from_arb(src: &mut ArbData) -> Result<Self> {
        let mut args = src.get_args_mut().drain(..1);
        let data = args
            .nth(0)
            .ok_or_else(oe_inv_arg("expected matrix argument in ArbData"))?;
        if data.len() % 16 != 0 {
            inv_arg("invalid matrix size")?;
        }
        let num_entries = data.len() / 16;
        if num_entries != num_entries.integer_sqrt().pow(2) {
            inv_arg("invalid matrix size")?;
        }
        let mut entries = Vec::with_capacity(num_entries);
        for i in 0..num_entries {
            let re = f64::from_le_bytes(data[i * 16..i * 16 + 8].try_into().unwrap());
            let im = f64::from_le_bytes(data[i * 16 + 8..i * 16 + 16].try_into().unwrap());
            entries.push(Complex64::new(re, im));
        }
        Ok(entries.into())
    }
}

impl FromArb for ArbData {
    fn from_arb(src: &mut ArbData) -> Result<Self> {
        let mut ret = ArbData::default();
        ret.copy_from(src);
        src.clear();
        Ok(ret)
    }
}

/// A type that can be converted into an ArbData object.
pub trait ToArb {
    /// Convert self to ArbData parameters and add them to the data object.
    fn to_arb(self, dest: &mut ArbData);
}

impl ToArb for () {
    fn to_arb(self, _: &mut ArbData) {}
}

impl ToArb for u64 {
    fn to_arb(self, dest: &mut ArbData) {
        dest.get_args_mut().insert(0, self.to_le_bytes().to_vec());
    }
}

impl ToArb for f64 {
    fn to_arb(self, dest: &mut ArbData) {
        dest.get_args_mut().insert(0, self.to_le_bytes().to_vec());
    }
}

impl ToArb for (f64, f64, f64) {
    fn to_arb(self, dest: &mut ArbData) {
        self.2.to_arb(dest);
        self.1.to_arb(dest);
        self.0.to_arb(dest);
    }
}

impl ToArb for Matrix {
    fn to_arb(self, dest: &mut ArbData) {
        let mut data = Vec::with_capacity(16 * self.len());
        for entry in self.into_iter() {
            data.extend(entry.re.to_le_bytes().iter());
            data.extend(entry.im.to_le_bytes().iter());
        }
        dest.get_args_mut().insert(0, data);
    }
}

impl ToArb for ArbData {
    fn to_arb(self, dest: &mut ArbData) {
        dest.copy_from(&self)
    }
}

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
pub trait Converter {
    /// The more generic detector input type = constructor output type.
    type Input;
    /// The more specific detector output type = constructor input type.
    type Output;
    /// The detect function implements the detector function. The return values
    /// are as follows:
    ///  - `Ok(Some(O))`: successful match
    ///  - `Ok(None)`: the input is not an instance of the output type
    ///  - `Err(_)`: something went wrong during detection
    fn detect(&self, input: &Self::Input) -> Result<Option<Self::Output>>;
    /// The construct function implements the opposite of the detector
    /// function, converting the plugin-specific type to the more generic type.
    ///  - `Ok(O)`: successful construction
    ///  - `Err(_)`: something went wrong during construction
    fn construct(&self, input: &Self::Output) -> Result<Self::Input>;
}

pub trait MatrixConverter {
    // For parameterized matrices, this is the type of the parameters needed
    // for construction/returned by detection.
    type Parameters;
    fn detect_matrix(
        &self,
        matrix: &Matrix,
        epsilon: f64,
        ignore_global_phase: bool,
    ) -> Result<Option<Self::Parameters>>;
    fn construct_matrix(&self, parameters: &Self::Parameters) -> Result<Matrix>;
}

impl<T> Converter for T
where
    T: MatrixConverter,
{
    type Input = (Matrix, f64, bool);
    type Output = T::Parameters;

    fn detect(&self, input: &Self::Input) -> Result<Option<Self::Output>> {
        let (matrix, epsilon, ignore_global_phase) = input;
        self.detect_matrix(matrix, *epsilon, *ignore_global_phase)
    }

    fn construct(&self, parameters: &Self::Output) -> Result<Self::Input> {
        self.construct_matrix(parameters)
            .map(|matrix| (matrix, 0., false))
    }
}

/// Matrix converter object for fixed matrices.
struct FixedMatrixConverter {
    matrix: Matrix,
}

impl From<Matrix> for FixedMatrixConverter {
    fn from(matrix: Matrix) -> FixedMatrixConverter {
        FixedMatrixConverter { matrix }
    }
}

impl MatrixConverter for FixedMatrixConverter {
    type Parameters = ();

    fn detect_matrix(
        &self,
        matrix: &Matrix,
        epsilon: f64,
        ignore_global_phase: bool,
    ) -> Result<Option<Self::Parameters>> {
        if self.matrix.approx_eq(matrix, epsilon, ignore_global_phase) {
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }

    fn construct_matrix(&self, _: &Self::Parameters) -> Result<Matrix> {
        Ok(self.matrix.clone())
    }
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

/// Matrix converter object for the RX matrix.
struct RxMatrixConverter {}

impl MatrixConverter for RxMatrixConverter {
    type Parameters = f64;

    fn detect_matrix(
        &self,
        matrix: &Matrix,
        epsilon: f64,
        ignore_global_phase: bool,
    ) -> Result<Option<Self::Parameters>> {
        let cc = matrix[(0, 0)].re;
        let cs = matrix[(0, 0)].im;
        let ss = matrix[(1, 0)].re;
        let sc = -matrix[(1, 0)].im;
        let theta = detect_angle(ss + cc, cs - sc, cc - ss, sc + cs);
        let expected: Matrix = self.construct_matrix(&theta)?;
        if matrix.approx_eq(&expected, epsilon, ignore_global_phase) {
            Ok(Some(theta))
        } else {
            Ok(None)
        }
    }

    fn construct_matrix(&self, theta: &Self::Parameters) -> Result<Matrix> {
        Ok(UnboundGate::RX(*theta).into())
    }
}

/// Matrix converter object for the RY matrix.
struct RyMatrixConverter {}

impl MatrixConverter for RyMatrixConverter {
    type Parameters = f64;

    fn detect_matrix(
        &self,
        matrix: &Matrix,
        epsilon: f64,
        ignore_global_phase: bool,
    ) -> Result<Option<Self::Parameters>> {
        let cc = matrix[(0, 0)].re;
        let cs = matrix[(0, 0)].im;
        let ss = -matrix[(1, 0)].im;
        let sc = -matrix[(1, 0)].re;
        let theta = -detect_angle(ss + cc, cs - sc, cc - ss, sc + cs);
        let expected: Matrix = self.construct_matrix(&theta)?;
        if matrix.approx_eq(&expected, epsilon, ignore_global_phase) {
            Ok(Some(theta))
        } else {
            Ok(None)
        }
    }

    fn construct_matrix(&self, theta: &Self::Parameters) -> Result<Matrix> {
        Ok(UnboundGate::RY(*theta).into())
    }
}

/// Matrix converter object for the RZ matrix.
struct RzMatrixConverter {}

impl MatrixConverter for RzMatrixConverter {
    type Parameters = f64;

    fn detect_matrix(
        &self,
        matrix: &Matrix,
        epsilon: f64,
        ignore_global_phase: bool,
    ) -> Result<Option<Self::Parameters>> {
        let theta = detect_angle(
            matrix[(0, 0)].re,
            matrix[(0, 0)].im,
            matrix[(1, 1)].re,
            matrix[(1, 1)].im,
        );
        let expected: Matrix = self.construct_matrix(&theta)?;
        if matrix.approx_eq(&expected, epsilon, ignore_global_phase) {
            Ok(Some(theta))
        } else {
            Ok(None)
        }
    }

    fn construct_matrix(&self, theta: &Self::Parameters) -> Result<Matrix> {
        Ok(UnboundGate::RZ(*theta).into())
    }
}

/// Matrix converter object for the phase submatrix.
struct PhaseMatrixConverter {}

impl MatrixConverter for PhaseMatrixConverter {
    type Parameters = f64;

    fn detect_matrix(
        &self,
        matrix: &Matrix,
        epsilon: f64,
        ignore_global_phase: bool,
    ) -> Result<Option<Self::Parameters>> {
        let theta = detect_angle(
            matrix[(0, 0)].re,
            matrix[(0, 0)].im,
            matrix[(1, 1)].re,
            matrix[(1, 1)].im,
        );
        let expected: Matrix = self.construct_matrix(&theta)?;
        if matrix.approx_eq(&expected, epsilon, ignore_global_phase) {
            Ok(Some(theta))
        } else {
            Ok(None)
        }
    }

    fn construct_matrix(&self, theta: &Self::Parameters) -> Result<Matrix> {
        Ok(UnboundGate::Phase(*theta).into())
    }
}

/// Matrix converter object for the phase submatrix using θ = π/2^k​.
struct PhaseKMatrixConverter {}

impl MatrixConverter for PhaseKMatrixConverter {
    type Parameters = u64;

    fn detect_matrix(
        &self,
        matrix: &Matrix,
        epsilon: f64,
        ignore_global_phase: bool,
    ) -> Result<Option<Self::Parameters>> {
        let theta = detect_angle(
            matrix[(0, 0)].re,
            matrix[(0, 0)].im,
            matrix[(1, 1)].re,
            matrix[(1, 1)].im,
        );
        let k = if theta <= 0.0 {
            0u64
        } else {
            (-(theta / PI).log(2.0).round()) as u64
        };
        let expected: Matrix = self.construct_matrix(&k)?;
        if matrix.approx_eq(&expected, epsilon, ignore_global_phase) {
            Ok(Some(k))
        } else {
            Ok(None)
        }
    }

    fn construct_matrix(&self, k: &Self::Parameters) -> Result<Matrix> {
        Ok(UnboundGate::PhaseK(*k).into())
    }
}

/// Matrix converter object for any matrix of a certain size - simply has the
/// matrix itself as its parameter type.
struct UMatrixConverter {
    /// The number of qubits that the matrix should have, or None if don't
    /// care.
    num_qubits: Option<usize>,
}

impl UMatrixConverter {
    pub fn new(num_qubits: Option<usize>) -> UMatrixConverter {
        UMatrixConverter { num_qubits }
    }
}

impl MatrixConverter for UMatrixConverter {
    type Parameters = Matrix;

    fn detect_matrix(
        &self,
        matrix: &Matrix,
        _epsilon: f64,
        _ignore_global_phase: bool,
    ) -> Result<Option<Self::Parameters>> {
        if let Some(expected) = self.num_qubits {
            if matrix.num_qubits().unwrap() != expected {
                // matrix has incorrect size
                return Ok(None);
            }
        }
        Ok(Some(matrix.clone()))
    }

    fn construct_matrix(&self, matrix: &Self::Parameters) -> Result<Matrix> {
        let num_qubits = matrix
            .num_qubits()
            .ok_or_else(oe_inv_arg("matrix has invalid size"))?;
        if let Some(expected) = self.num_qubits {
            if num_qubits != expected {
                inv_arg(format!(
                    "matrix has incorrect size; expected matrix for {} qubits",
                    expected
                ))?;
            }
        }
        Ok(matrix.clone())
    }
}

/// Converter implementation for controlled matrices.
pub struct ControlledMatrixConverter<T>
where
    T: MatrixConverter,
{
    /// The converter dealing with detection and construction of the matrix.
    converter: T,
    /// How many control qubits are expected. If None, no constraint is placed
    /// on this.
    num_controls: Option<usize>,
    /// The max RMS deviation between the given matrix and the input matrix
    /// during detection.
    epsilon: f64,
    /// Whether global phase should be ignored if the matrix has no control
    /// qubits.
    ignore_global_phase: bool,
}

impl<T> ControlledMatrixConverter<T>
where
    T: MatrixConverter,
{
    pub fn new(
        converter: T,
        num_controls: Option<usize>,
        epsilon: f64,
        ignore_global_phase: bool,
    ) -> Self {
        Self {
            converter,
            num_controls,
            epsilon,
            ignore_global_phase,
        }
    }
}

impl<T> Converter for ControlledMatrixConverter<T>
where
    T: MatrixConverter,
{
    type Input = (Matrix, Option<usize>);
    type Output = T::Parameters;

    fn detect(&self, input: &Self::Input) -> Result<Option<Self::Output>> {
        let (matrix, num_controls) = input;
        let num_controls = num_controls.unwrap_or(0);

        // Check the number of control qubits.
        if let Some(expected) = self.num_controls {
            if num_controls != expected {
                return Ok(None);
            }
        }

        // Check the matrix.
        self.converter.detect_matrix(
            matrix,
            self.epsilon,
            self.ignore_global_phase && num_controls == 0,
        )
    }

    fn construct(&self, parameters: &Self::Output) -> Result<Self::Input> {
        Ok((
            self.converter.construct_matrix(parameters)?,
            self.num_controls,
        ))
    }
}

/// Converter implementation for unitary gates based on a matrix converter.
///
/// The matrix converter takes a matrix and a number of control qubits as input
/// for detection. The number of control qubits is wrapped in an Option, but
/// in the detection direction this is always Some, so it can just be
/// unwrapped. In the other direction, None means that the number of control
/// qubits can be freely derived from the number of qubit arguments, while
/// Some places a constraint on the number of expected control qubits.
pub struct UnitaryGateConverter<M>
where
    M: Converter<Input = (Matrix, Option<usize>)>,
{
    /// The wrapped matrix converter.
    matrix_converter: M,
}

impl<M> From<M> for UnitaryGateConverter<M>
where
    M: Converter<Input = (Matrix, Option<usize>)>,
{
    fn from(matrix_converter: M) -> Self {
        Self { matrix_converter }
    }
}

impl<M> Converter for UnitaryGateConverter<M>
where
    M: Converter<Input = (Matrix, Option<usize>)>,
    M::Output: FromArb + ToArb,
{
    type Input = Gate;
    type Output = (Vec<QubitRef>, ArbData);

    fn detect(&self, gate: &Gate) -> Result<Option<Self::Output>> {
        if gate.get_name().is_some() || !gate.get_measures().is_empty() {
            // Custom gate, measurement gate or unitary + measure compound; not
            // (just) a unitary so no match.
            Ok(None)
        } else if let Some(matrix) = gate.get_matrix() {
            // Unitary gate. Check conditions.
            if let Some(params) = self
                .matrix_converter
                .detect(&(matrix, Some(gate.get_controls().len())))?
            {
                // Matrix match; construct qubit argument vector.
                let mut qubits = vec![];
                qubits.extend(gate.get_controls().iter());
                qubits.extend(gate.get_targets().iter());
                // Construct data.
                let mut data = gate.data.clone();
                params.to_arb(&mut data);
                Ok(Some((qubits, data)))
            } else {
                // Matrix didn't match.
                Ok(None)
            }
        } else {
            // A gate with no name, matrix, or measured qubits is illegal.
            unreachable!();
        }
    }

    fn construct(&self, output: &Self::Output) -> Result<Gate> {
        let (qubits, data) = output;

        // Construct the data.
        let mut data = data.clone();
        let params = M::Output::from_arb(&mut data)?;

        // Construct the matrix.
        let (matrix, expected_num_controls) = self.matrix_converter.construct(&params)?;

        // Parse qubit argument vector.
        let num_targets = matrix.num_qubits().unwrap();
        let num_controls = qubits
            .len()
            .checked_sub(num_targets)
            .ok_or_else(oe_inv_arg(format!("need at least {} qubits", num_targets)))?;
        if let Some(expected) = expected_num_controls {
            if num_controls != expected {
                inv_arg(format!(
                    "expected {} control and {} target qubits",
                    expected, num_targets
                ))?;
            }
        }
        let controls = &qubits[..num_controls];
        let targets = &qubits[num_controls..];

        // Construct the gate.
        let mut gate =
            Gate::new_unitary(targets.iter().cloned(), controls.iter().cloned(), matrix)?;
        gate.data.copy_from(&data);

        Ok(gate)
    }
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
    converters: HashMap<K, Box<dyn Converter<Input = I, Output = O> + 'c>>,
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
    pub fn push(
        &mut self,
        key: impl Into<K>,
        converter: impl Converter<Input = I, Output = O> + 'c,
    ) {
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
        converter: impl Converter<Input = I, Output = O> + 'c,
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
    pub fn with(
        mut self,
        key: impl Into<K>,
        converter: impl Converter<Input = I, Output = O> + 'c,
    ) -> Self {
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

impl<'c, I, K, O, C> Converter for ConverterMap<'c, K, I, O, C>
where
    I: Clone,
    K: Eq + Hash + Clone,
    C: Eq + Hash + ConverterCacheKey<I>,
    O: Clone,
{
    type Input = I;
    type Output = (K, O);

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
