use dqcsim::{
    common::{
        error::{inv_arg, inv_op, oe_inv_arg, ErrorKind, Result},
        gates::*,
        types::{
            ArbData, Gate, Matrix, PluginMetadata, PluginType, QubitMeasurementValue, QubitRef,
        },
    },
    plugin::{definition::PluginDefinition, state::PluginState},
    trace,
};
use meval::Context;
use num_complex::Complex64;
use qasm::{Argument, AstNode};
use serde_json::json;
use std::{collections::HashMap, env, fs::File, io::Read, iter::FromIterator, path::PathBuf};
use structopt::StructOpt;

/// Custom gate definition structure.
struct GateDefinition {
    name: String,
    params: Vec<String>,
    qubits: Vec<String>,
    body: QasmProgram,
    opaque: bool,
}

impl GateDefinition {
    /// Returns a new `GateDefinition` for the given inputs.
    fn new(
        identifier: String,
        qubits: Vec<String>,
        params: Vec<String>,
        body: QasmProgram,
        opaque: bool,
    ) -> Self {
        GateDefinition {
            name: identifier,
            qubits,
            params,
            body,
            opaque,
        }
    }
    /// Returns the body of this gate.
    fn get_body(&self) -> QasmProgram {
        self.body.clone()
    }
}

/// Parses a list of string parameters into a list of floats.
fn parse_params(input: Vec<String>) -> Result<Vec<f64>> {
    input
        .iter()
        .map(|p| meval::eval_str(p).or_else(|e| inv_op(format!("Bad expression: {}", e))))
        .collect()
}

/// Validates the length of an input slice is equal to the expected slice.
/// Returns an Error when the sizes differ, including the provided error
/// message.
fn validate_length<T>(input: &[T], expected: usize, msg: &str) -> Result<()> {
    let x = input.len();
    if x != expected {
        inv_arg(format!(
            "Bad size for {}. Expected: {} Got: {}",
            msg, expected, x
        ))?
    }
    Ok(())
}

/// A QasmProgram is the sequence of statements in the source file.
type QasmProgram = Vec<AstNode>;

/// A BitStore is a map of classical bit registers.
struct BitStore(HashMap<String, Vec<bool>>);

impl BitStore {
    /// Constructs a new empty BitStore.
    fn new() -> Self {
        Self(HashMap::new())
    }

    /// Allocates a new register in the BitStore with provided identifier and
    /// size.
    fn alloc(&mut self, identifier: String, count: i32) {
        self.0.insert(identifier, vec![false; count as usize]);
    }

    /// Remove all registers from this BitStore.
    fn clear(&mut self) {
        self.0.clear()
    }

    /// Returns a mutable slice to bit represented as `bool`s if the requested
    /// register exists.
    fn get<'a>(&'a mut self, arg: &Argument) -> Result<&'a mut [bool]> {
        let get_register = |register: &'a mut HashMap<String, Vec<bool>>,
                            identifier: &str|
         -> Result<&'a mut [bool]> {
            register
                .get_mut(identifier)
                .map(|bits| bits.as_mut_slice())
                .ok_or_else(oe_inv_arg(format!(
                    "Undefined bit register: {}",
                    identifier
                )))
        };

        match arg {
            Argument::Register(ref identifier) => get_register(&mut self.0, identifier),
            Argument::Qubit(ref identifier, index) => get_register(&mut self.0, identifier)?
                .get_mut(*index as usize)
                .map(|cref| std::slice::from_mut(cref))
                .ok_or_else(oe_inv_arg(format!(
                    "Index ({}) out of bounds on bit register: {}",
                    index, identifier,
                ))),
        }
    }

    /// Returns the numeric value of a bit register if the requested register
    /// exists.
    fn value(&self, identifier: &str) -> Result<i32> {
        self.0
            .get(identifier)
            .map(|bits| {
                bits.iter()
                    .enumerate()
                    .filter(|(_, x)| **x)
                    .map(|(i, _)| 1 << i)
                    .sum::<i32>()
            })
            .ok_or_else(oe_inv_arg(format!(
                "Undefined bit register: {}",
                identifier
            )))
    }
}

/// This structure captures the state of the plugin.
struct Program<'state, 'def> {
    /// Reference to the `PluginState`.
    state: &'state mut PluginState<'def>,

    /// The mappings of `qreg` identifiers to `QubitRef`.
    qubit: HashMap<String, Vec<QubitRef>>,
    /// The storage of `creg` identifiers and their values in a `BitStore`.
    bit: BitStore,
    /// The mapping of gate identifiers to `GateDefinition`.
    gate: HashMap<String, GateDefinition>,
}

impl<'state, 'def> Program<'state, 'def> {
    /// Constructs an empty `Program` using the provided `PluginState`.
    fn new(state: &'state mut PluginState<'def>) -> Self {
        Program {
            state,
            qubit: HashMap::new(),
            bit: BitStore::new(),
            gate: HashMap::new(),
        }
    }

    // Run this `Program` with the provided `QasmProgram`.
    fn run(&mut self, ast: QasmProgram) -> Result<()> {
        for node in ast {
            match node {
                // Declare a named register of qubits.
                AstNode::QReg(identifier, count) => {
                    self.qubit
                        .insert(identifier, self.state.allocate(count as usize, Vec::new())?);
                }

                // Declare a named register of bits.
                AstNode::CReg(identifier, count) => {
                    self.bit.alloc(identifier, count);
                }

                // Prevent transformations across this source line.
                AstNode::Barrier(ref args) => self.barrier(args)?,

                // Prepare qubit(s) in |0>.
                AstNode::Reset(ref arg) => self.reset(arg)?,

                // Make measurement(s) in Z basis.
                AstNode::Measure(ref qreg, ref creg) => self.measure(qreg, creg)?,

                // Apply a built-in or defined unitary gate.
                AstNode::ApplyGate(identifier, qubits, params) => {
                    self.apply_gate(&identifier, qubits, params)?
                }

                // Declare an opaque gate.
                AstNode::Opaque(identifier, qubits, params) => {
                    self.gate.insert(
                        identifier.clone(),
                        GateDefinition::new(
                            identifier,
                            qubits
                                .into_iter()
                                .map(|q| {
                                    if let Argument::Register(x) = q {
                                        Ok(x)
                                    } else {
                                        inv_arg("not supported in opaque gate definition")
                                    }
                                })
                                .collect::<Result<Vec<_>>>()?,
                            params,
                            vec![],
                            true,
                        ),
                    );
                }

                // Declare a unitary gate.
                AstNode::Gate(identifier, qubits, params, nodes) => {
                    self.gate.insert(
                        identifier.clone(),
                        GateDefinition::new(identifier, qubits, params, nodes, false),
                    );
                }

                // Conditionally apply quantum operation.
                AstNode::If(condition, value, node) => {
                    self.condition(&condition, value, *node)?;
                }
            }
        }

        // Free resources.
        self.state
            .free(self.qubit.drain().map(|(_, v)| v).flatten().collect())?;
        self.bit.clear();
        self.gate.clear();

        Ok(())
    }

    /// Sends the barrier instructions as a custom gate.
    fn barrier(&mut self, args: &Argument) -> Result<()> {
        self.state.gate(Gate::new_custom(
            "qasm.barrier",
            self.get_qubits(args)?,
            vec![],
            vec![],
            None as Option<Vec<Complex64>>,
            ArbData::default(),
        )?)
    }

    /// Resets given qubits to the |0> state. This is done by measuring the
    /// qubits and flipping the ones that are in the |1> state.
    fn reset(&mut self, arg: &Argument) -> Result<()> {
        let qubits = self.get_qubits(arg)?;
        self.state.gate(Gate::new_measurement(qubits.clone())?)?;
        qubits
            .into_iter()
            .map(|q| self.state.get_measurement(q).map(|m| (q, m.value)))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .filter(|(_, m)| *m == QubitMeasurementValue::One)
            .map(|(q, _)| self.state.gate(BoundGate::X(q).into()))
            .collect()
    }

    /// Measures given qubits and store the results in the corresponding
    /// classical registers.
    fn measure(&mut self, qreg: &Argument, creg: &Argument) -> Result<()> {
        let qubits = self.get_qubits(qreg)?;
        let bits = self.bit.get(creg)?;
        validate_length(
            &qubits,
            bits.len(),
            "qubit source and classical destination target",
        )?;

        self.state.gate(Gate::new_measurement(qubits.clone())?)?;
        for (qbit, bit) in qubits.into_iter().zip(self.bit.get(creg)?.iter_mut()) {
            match self.state.get_measurement(qbit)?.value {
                QubitMeasurementValue::Zero => *bit = false,
                QubitMeasurementValue::One => *bit = true,
                QubitMeasurementValue::Undefined => inv_op("Measurement failed")?,
            }
        }

        Ok(())
    }

    /// Applies the provided `AstNode` if the classical register's value
    /// matches the provided value.
    fn condition(&mut self, identifier: &str, value: i32, node: AstNode) -> Result<()> {
        if self.bit.value(identifier)? == value {
            match node {
                AstNode::Reset(ref arg) => self.reset(arg)?,
                AstNode::Measure(ref qreg, ref creg) => self.measure(qreg, creg)?,
                AstNode::ApplyGate(identifier, qubits, params) => {
                    self.apply_gate(&identifier, qubits, params)?
                }
                _ => inv_arg("only qops are allowed in if body")?,
            }
        }
        Ok(())
    }

    /// Applies the gate. This recursively reduces the provided gates to
    /// built-in gates which are then applied.
    fn apply_gate(
        &mut self,
        identifier: &str,
        qubits: Vec<Argument>,
        params: Vec<String>,
    ) -> Result<()> {
        self.get_gates(identifier, qubits, params)?
            .into_iter()
            .try_for_each(|gate| self.state.gate(gate))
    }

    /// Returns the `GateDefinition` for the given identifier if it exists.
    fn get_gate(&self, identifier: &str) -> Result<&GateDefinition> {
        self.gate
            .get(identifier)
            .ok_or_else(oe_inv_arg(format!("Undefined gate: {}", identifier)))
    }

    /// Recursively unrolls gate identifiers into a `Vec<Gate>` which can be
    /// forwarded to downstream plugins.
    fn get_gates(
        &mut self,
        identifier: &str,
        qubits: Vec<Argument>,
        params: Vec<String>,
    ) -> Result<Vec<Gate>> {
        let mut gates = vec![];
        match identifier {
            "U" => {
                // Built-in gate
                validate_length(&params, 3, "params")?;
                validate_length(&qubits, 1, "qubits")?;
                let params = parse_params(params)?;
                gates.append(
                    self.get_qubits(&qubits[0])?
                        .into_iter()
                        .map(|q| BoundGate::R(params[0], params[1], params[2], q).into())
                        .collect::<Vec<_>>()
                        .as_mut(),
                )
            }
            "CX" => {
                // Built-in gate.
                validate_length(&qubits, 2, "qubits")?;
                gates.append(
                    vec![Gate::new_unitary(
                        self.get_qubits(&qubits[1])?,
                        self.get_qubits(&qubits[0])?,
                        Matrix::from(UnboundGate::X),
                    )?]
                    .as_mut(),
                )
            }
            _ => {
                // Gate definitions.
                let gate = self.get_gate(identifier)?;
                validate_length(&params, gate.params.len(), "params")?;
                validate_length(&qubits, gate.qubits.len(), "qubits")?;
                let params = parse_params(params)?;

                if gate.opaque {
                    // Opaque gates are encoded as custom gates.
                    gates.append(
                        vec![Gate::new_custom(
                            gate.name.clone(),
                            qubits
                                .iter()
                                .map(|q| self.get_qubits(q))
                                .collect::<Result<Vec<_>>>()?
                                .into_iter()
                                .flatten(),
                            vec![],
                            vec![],
                            None as Option<Vec<Complex64>>,
                            ArbData::from_json(json!({ "params": params }).to_string(), vec![])?,
                        )?]
                        .as_mut(),
                    );
                } else {
                    // Build qubit map to resolve qubit arguments.
                    let qubit_map: HashMap<String, Argument> = HashMap::from_iter(
                        gate.qubits
                            .iter()
                            .cloned()
                            .enumerate()
                            .map(|(i, qubit)| (qubit, qubits[i].clone())),
                    );
                    // Build parameter context to resolve gate parameters.
                    let mut param_context = Context::new();
                    for (i, param) in gate.params.iter().enumerate() {
                        param_context.var(param, params[i]);
                    }
                    // Process gate body.
                    for node in gate.get_body() {
                        match node {
                            AstNode::ApplyGate(identifier, qubits, params) => {
                                let qubits = qubits
                                        .iter()
                                        .map(|q| match q {
                                            Argument::Register(ref identifier) => {
                                                Ok(qubit_map.get(identifier).cloned().unwrap())
                                            }
                                            _ => {
                                                inv_arg("qargs cannot be indexed within the body of the gate definition")
                                            },
                                        })
                                        .collect::<Result<Vec<_>>>()?;
                                gates.append(
                                    self.get_gates(
                                        &identifier,
                                        qubits,
                                        params
                                            .iter()
                                            .filter_map(|p| {
                                                meval::eval_str_with_context(p, &param_context)
                                                    .ok()
                                                    .map(|p| p.to_string())
                                            })
                                            .collect(),
                                    )?
                                    .as_mut(),
                                );
                            }
                            AstNode::Barrier(ref args) => self.barrier(args)?,
                            _ => {
                                inv_arg("only built-in gate statements, calls to previously defined gates, and barrier statements can appear in body")?;
                            }
                        }
                    }
                }
            }
        };
        Ok(gates)
    }

    /// Returns all qubit references in the register given by the identifier if
    /// it exists.
    fn get_qubits_register(&self, identifier: &str) -> Result<Vec<QubitRef>> {
        self.qubit
            .get(identifier)
            .cloned()
            .ok_or_else(oe_inv_arg(format!(
                "Undefined qubit register: {}",
                identifier
            )))
    }

    /// Returns qubit references for the given argument if they exist.
    fn get_qubits(&self, arg: &Argument) -> Result<Vec<QubitRef>> {
        match arg {
            Argument::Register(ref identifier) => self.get_qubits_register(identifier),
            Argument::Qubit(ref identifier, index) => self
                .get_qubits_register(identifier)?
                .get(*index as usize)
                .cloned()
                .map(|qref| vec![qref])
                .ok_or_else(oe_inv_arg(format!(
                    "Index ({}) out of bounds on qubit register: {}",
                    index, identifier,
                ))),
        }
    }
}

/// Returns the `PluginDefinition`.
fn plugin(input: PathBuf) -> PluginDefinition {
    let mut definition = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("OpenQASM 2.0 frontend", "Matthijs Brobbel", "0.1.0"),
    );

    // Run handle. This opens and parses the input file, and constructs and
    // runs the Program.
    definition.run = Box::new(move |state, _args| {
        trace!("Opening {}", &input.display());
        let mut file = File::open(&input)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        trace!("Parsing {}", &input.display());
        let source = qasm::process(&contents, &env::current_dir()?);
        let mut tokens = qasm::lex(&source);
        let ast = qasm::parse(&mut tokens)
            .map_err(|e| ErrorKind::Other(format!("Parsing failed: {}", e)))?;

        trace!("Running program");
        Program::new(state).run(ast)?;

        Ok(ArbData::default())
    });

    definition
}

#[derive(StructOpt)]
/// The plugin arguments.
struct Opt {
    /// Input file.
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    /// Simulator endpoint address.
    simulator: String,
}

fn main() -> Result<()> {
    // Parse arguments.
    let opt = Opt::from_args();
    // Get plugin definition.
    let plugin = plugin(opt.input);
    // Connect to simulator instance and run plugin.
    PluginState::run(&plugin, opt.simulator)
}
