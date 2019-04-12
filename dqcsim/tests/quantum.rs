use dqcsim::{
    common::{
        log::LoglevelFilter,
        types::{
            ArbCmd, ArbData, Gate, PluginMetadata, PluginType, QubitMeasurementResult,
            QubitMeasurementValue, QubitRef,
        },
    },
    host::{
        accelerator::Accelerator,
        configuration::{
            PluginLogConfiguration, PluginThreadConfiguration, SimulatorConfiguration,
        },
        simulator::Simulator,
    },
    plugin::definition::PluginDefinition,
};
use num_complex::Complex64;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[test]
fn backend_allocate() {
    let mut backend = PluginDefinition::new(
        PluginType::Backend,
        PluginMetadata::new("backend", "dqcsim", "0.1.0"),
    );

    let frontend = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
    );

    backend.initialize = Box::new(|state, _| {
        let q = state.allocate(1, vec![]);
        assert!(q.is_err());
        assert_eq!(
            q.unwrap_err().to_string(),
            "Invalid operation: allocate() is not available for backends"
        );
        Ok(())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(PluginThreadConfiguration::new(
            frontend,
            PluginLogConfiguration::new("front", LoglevelFilter::Trace),
        ))
        .with_plugin(PluginThreadConfiguration::new(
            backend,
            PluginLogConfiguration::new("backend", LoglevelFilter::Trace),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
fn backend_free() {
    let mut backend = PluginDefinition::new(
        PluginType::Backend,
        PluginMetadata::new("backend", "dqcsim", "0.1.0"),
    );

    let frontend = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
    );

    backend.initialize = Box::new(|state, _| {
        let f = state.free(vec![]);
        assert!(f.is_err());
        assert_eq!(
            f.unwrap_err().to_string(),
            "Invalid operation: free() is not available for backends"
        );
        Ok(())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(PluginThreadConfiguration::new(
            frontend,
            PluginLogConfiguration::new("front", LoglevelFilter::Trace),
        ))
        .with_plugin(PluginThreadConfiguration::new(
            backend,
            PluginLogConfiguration::new("backend", LoglevelFilter::Trace),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
fn backend_gate() {
    let mut backend = PluginDefinition::new(
        PluginType::Backend,
        PluginMetadata::new("backend", "dqcsim", "0.1.0"),
    );

    let frontend = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
    );

    backend.initialize = Box::new(|state, _| {
        let g =
            state.gate(Gate::new_measurement(vec![QubitRef::from_foreign(1u64).unwrap()]).unwrap());
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid operation: gate() is not available for backends"
        );
        Ok(())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(PluginThreadConfiguration::new(
            frontend,
            PluginLogConfiguration::new("front", LoglevelFilter::Trace),
        ))
        .with_plugin(PluginThreadConfiguration::new(
            backend,
            PluginLogConfiguration::new("backend", LoglevelFilter::Trace),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
fn backend_get_measurement() {
    let mut backend = PluginDefinition::new(
        PluginType::Backend,
        PluginMetadata::new("backend", "dqcsim", "0.1.0"),
    );

    let frontend = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
    );

    backend.initialize = Box::new(|state, _| {
        let g = state.get_measurement(QubitRef::from_foreign(1).unwrap());
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid operation: get_measurement() is not available for backends"
        );
        Ok(())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(PluginThreadConfiguration::new(
            frontend,
            PluginLogConfiguration::new("front", LoglevelFilter::Trace),
        ))
        .with_plugin(PluginThreadConfiguration::new(
            backend,
            PluginLogConfiguration::new("backend", LoglevelFilter::Trace),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
fn backend_get_cycles_since_measure() {
    let mut backend = PluginDefinition::new(
        PluginType::Backend,
        PluginMetadata::new("backend", "dqcsim", "0.1.0"),
    );

    let frontend = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
    );

    backend.initialize = Box::new(|state, _| {
        let g = state.get_cycles_since_measure(QubitRef::from_foreign(1).unwrap());
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid operation: get_cycles_since_measure() is not available for backends"
        );
        Ok(())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(PluginThreadConfiguration::new(
            frontend,
            PluginLogConfiguration::new("front", LoglevelFilter::Trace),
        ))
        .with_plugin(PluginThreadConfiguration::new(
            backend,
            PluginLogConfiguration::new("backend", LoglevelFilter::Trace),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
fn backend_get_cycles_between_measures() {
    let mut backend = PluginDefinition::new(
        PluginType::Backend,
        PluginMetadata::new("backend", "dqcsim", "0.1.0"),
    );

    let frontend = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
    );

    backend.initialize = Box::new(|state, _| {
        let g = state.get_cycles_between_measures(QubitRef::from_foreign(1).unwrap());
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid operation: get_cycles_between_measures() is not available for backends"
        );
        Ok(())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(PluginThreadConfiguration::new(
            frontend,
            PluginLogConfiguration::new("front", LoglevelFilter::Trace),
        ))
        .with_plugin(PluginThreadConfiguration::new(
            backend,
            PluginLogConfiguration::new("backend", LoglevelFilter::Trace),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
fn backend_advance() {
    let mut backend = PluginDefinition::new(
        PluginType::Backend,
        PluginMetadata::new("backend", "dqcsim", "0.1.0"),
    );

    let frontend = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
    );

    backend.initialize = Box::new(|state, _| {
        let a = state.advance(3u64);
        assert!(a.is_err());
        assert_eq!(
            a.unwrap_err().to_string(),
            "Invalid operation: advance() is not available for backends"
        );
        Ok(())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(PluginThreadConfiguration::new(
            frontend,
            PluginLogConfiguration::new("front", LoglevelFilter::Trace),
        ))
        .with_plugin(PluginThreadConfiguration::new(
            backend,
            PluginLogConfiguration::new("backend", LoglevelFilter::Trace),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
fn backend_get_cycle() {
    let mut backend = PluginDefinition::new(
        PluginType::Backend,
        PluginMetadata::new("backend", "dqcsim", "0.1.0"),
    );

    let frontend = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
    );

    backend.initialize = Box::new(|state, _| {
        let a = state.get_cycle();
        assert!(a.is_err());
        assert_eq!(
            a.unwrap_err().to_string(),
            "Invalid operation: get_cycle() is not available for backends"
        );
        Ok(())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(PluginThreadConfiguration::new(
            frontend,
            PluginLogConfiguration::new("front", LoglevelFilter::Trace),
        ))
        .with_plugin(PluginThreadConfiguration::new(
            backend,
            PluginLogConfiguration::new("backend", LoglevelFilter::Trace),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
fn backend_arb() {
    let mut backend = PluginDefinition::new(
        PluginType::Backend,
        PluginMetadata::new("backend", "dqcsim", "0.1.0"),
    );

    let frontend = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
    );

    backend.initialize = Box::new(|state, _| {
        let a = state.arb(ArbCmd::new("a", "b", ArbData::default()));
        assert!(a.is_err());
        assert_eq!(
            a.unwrap_err().to_string(),
            "Invalid operation: arb() is not available for backends"
        );
        Ok(())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(PluginThreadConfiguration::new(
            frontend,
            PluginLogConfiguration::new("front", LoglevelFilter::Trace),
        ))
        .with_plugin(PluginThreadConfiguration::new(
            backend,
            PluginLogConfiguration::new("backend", LoglevelFilter::Trace),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
fn measurement_non_alloc() {
    let mut frontend = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
    );

    frontend.initialize = Box::new(|state, _| {
        let m = state.get_measurement(QubitRef::from_foreign(1).unwrap());
        assert!(m.is_err());
        assert_eq!(
            m.unwrap_err().to_string(),
            "Invalid argument: qubit 1 is not allocated"
        );
        Ok(())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(PluginThreadConfiguration::new(
            frontend,
            PluginLogConfiguration::new("front", LoglevelFilter::Trace),
        ))
        .with_plugin(PluginThreadConfiguration::new(
            PluginDefinition::new(
                PluginType::Backend,
                PluginMetadata::new("backend", "dqcsim", "0.1.0"),
            ),
            PluginLogConfiguration::new("backend", LoglevelFilter::Trace),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
fn measurement_not_measured() {
    let mut frontend = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
    );

    frontend.initialize = Box::new(|state, _| {
        let q = state.allocate(1, vec![]).expect("alloc fail");
        let m = state.get_measurement(q[0]);
        assert!(m.is_err());
        assert_eq!(
            m.unwrap_err().to_string(),
            "Invalid argument: qubit 1 has not been measured yet"
        );
        Ok(())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(PluginThreadConfiguration::new(
            frontend,
            PluginLogConfiguration::new("front", LoglevelFilter::Trace),
        ))
        .with_plugin(PluginThreadConfiguration::new(
            PluginDefinition::new(
                PluginType::Backend,
                PluginMetadata::new("backend", "dqcsim", "0.1.0"),
            ),
            PluginLogConfiguration::new("backend", LoglevelFilter::Trace),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
fn free_non_alloc() {
    let mut frontend = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
    );

    frontend.initialize = Box::new(|state, _| {
        let m = state.free(vec![QubitRef::from_foreign(1).unwrap()]);
        assert!(m.is_err());
        assert_eq!(
            m.unwrap_err().to_string(),
            "Invalid argument: qubit 1 is not allocated"
        );
        Ok(())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(PluginThreadConfiguration::new(
            frontend,
            PluginLogConfiguration::new("front", LoglevelFilter::Trace),
        ))
        .with_plugin(PluginThreadConfiguration::new(
            PluginDefinition::new(
                PluginType::Backend,
                PluginMetadata::new("backend", "dqcsim", "0.1.0"),
            ),
            PluginLogConfiguration::new("backend", LoglevelFilter::Trace),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
// This attempts to simulate the quantum specific methods.
fn quantum_minimal() {
    let mut backend = PluginDefinition::new(
        PluginType::Backend,
        PluginMetadata::new("backend", "dqcsim", "0.1.0"),
    );

    #[derive(Debug)]
    struct Backend {
        pub qubits: HashMap<QubitRef, Complex64>,
    }

    let backend_data = Arc::new(Mutex::new(Backend {
        qubits: HashMap::new(),
    }));

    let bd_allocate = Arc::clone(&backend_data);
    backend.allocate = Box::new(move |state, qubits, _| {
        dqcsim::debug!("Allocating {} qubits", qubits.len());
        let mut data = bd_allocate.lock().unwrap();
        for qubit in qubits {
            data.qubits.insert(
                qubit,
                Complex64::new(state.random_f64(), state.random_f64()),
            );
        }
        Ok(())
    });

    let bd_free = Arc::clone(&backend_data);
    backend.free = Box::new(move |_, qubits| {
        dqcsim::debug!("Freeing {} qubits", qubits.len());
        let mut data = bd_free.lock().unwrap();
        for qubit in qubits {
            data.qubits.remove(&qubit);
        }
        Ok(())
    });

    // let bd_gate = Arc::clone(&backend_data);
    backend.gate = Box::new(move |_, gate| {
        let mut measurement = Vec::with_capacity(gate.get_measures().len());
        for q in gate.get_measures() {
            measurement.push(QubitMeasurementResult::new(
                *q,
                QubitMeasurementValue::Zero,
                ArbData::default(),
            ));
        }
        Ok(measurement)
    });

    let mut frontend = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
    );

    let operator = PluginDefinition::new(
        PluginType::Operator,
        PluginMetadata::new("operator", "dqcsim", "0.1.0"),
    );

    frontend.run = Box::new(|state, _| {
        // First allocate some qubits
        let qubits = state.allocate(4, vec![]);
        assert!(qubits.is_ok());
        let qubits = qubits.unwrap();
        assert_eq!(qubits.len(), 4);
        assert_eq!(
            format!("{:?}", qubits),
            "[QubitRef(1), QubitRef(2), QubitRef(3), QubitRef(4)]"
        );

        let bad_measure = Gate::new_measurement(vec![
            QubitRef::from_foreign(2).unwrap(),
            QubitRef::from_foreign(2).unwrap(),
        ]);
        assert!(bad_measure.is_err());
        assert_eq!(
            bad_measure.unwrap_err().to_string(),
            "Invalid argument: qubit 2 is measured more than once"
        );

        state
            .gate(Gate::new_measurement(vec![QubitRef::from_foreign(3).unwrap()]).unwrap())
            .unwrap();

        let result = state
            .get_measurement(QubitRef::from_foreign(3).unwrap())
            .unwrap();
        assert_eq!(result.qubit, QubitRef::from_foreign(3).unwrap());

        Ok(ArbData::default())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        // .without_logging()
        .with_stderr_level(LoglevelFilter::Trace)
        .with_plugin(PluginThreadConfiguration::new(
            frontend,
            PluginLogConfiguration::new("front", LoglevelFilter::Trace),
        ))
        .with_plugin(PluginThreadConfiguration::new(
            operator,
            PluginLogConfiguration::new("operator", LoglevelFilter::Trace),
        ))
        .with_plugin(PluginThreadConfiguration::new(
            backend,
            PluginLogConfiguration::new("backend", LoglevelFilter::Trace),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
    let mut simulator = simulator.unwrap();
    assert!(simulator.simulation.start(ArbData::default()).is_ok());
    let res = simulator.simulation.wait();

    assert!(res.is_ok());
    backend_data.as_ref();
}
