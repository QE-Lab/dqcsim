use dqcsim::{
    common::{
        error::err,
        log::{thread::LogThread, LoglevelFilter},
        types::{
            ArbCmd, ArbData, Gate, PluginMetadata, PluginType, QubitMeasurementResult,
            QubitMeasurementValue, QubitRef,
        },
    },
    host::{
        accelerator::Accelerator,
        configuration::{
            PluginLogConfiguration, PluginThreadConfiguration, Seed, SimulatorConfiguration,
        },
        plugin::Plugin,
        simulation::Simulation,
        simulator::Simulator,
    },
    plugin::definition::PluginDefinition,
};
use num_complex::Complex64;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub fn fe_op_be() -> (PluginDefinition, PluginDefinition, PluginDefinition) {
    let frontend = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
    );

    let operator = PluginDefinition::new(
        PluginType::Operator,
        PluginMetadata::new("operator", "dqcsim", "0.1.0"),
    );

    let backend = PluginDefinition::new(
        PluginType::Backend,
        PluginMetadata::new("backend", "dqcsim", "0.1.0"),
    );

    (frontend, operator, backend)
}

// TODO: matthijs
// #[test]
// // This tests bad initialization by plugin type mismatch.
// fn bad_plugin_type() {
//     let frontend = PluginDefinition::new(
//         PluginType::Frontend,
//         PluginMetadata::new("frontend", "dqcsim", "0.1.0"),
//     );

//     let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//     d.push("../target/debug/examples/plugin");
//     let not_operator = PluginProcessConfiguration::new(
//         "frontend",
//         PluginProcessSpecification::from_sugar(d, PluginType::Operator).unwrap(),
//     );
//     let backend = PluginDefinition::new(
//         PluginType::Backend,
//         PluginMetadata::new("backend", "dqcsim", "0.1.0"),
//     );

//     let ptc = |definition| {
//         PluginThreadConfiguration::new(
//             definition,
//             PluginLogConfiguration::new("", LoglevelFilter::Off),
//         )
//     };

//     let configuration = SimulatorConfiguration::default()
//         .without_reproduction()
//         .without_logging()
//         .with_plugin(ptc(frontend))
//         .with_plugin(not_operator)
//         .with_plugin(ptc(backend));

//     let simulator = Simulator::new(configuration);
//     assert!(simulator.is_err());
//     assert_eq!(simulator.unwrap_err().to_string(), "Failed to initialize plugin(s): Invalid operation: host is expecting a plugin of type Operator, but we\'re a plugin of type Frontend");
// }

#[test]
// This tests basic plugin arb command propagation.
fn plugin_to_plugin_arb() {
    let (mut frontend, mut operator, mut backend) = fe_op_be();

    frontend.initialize = Box::new(|state, _| {
        let res = state.arb(ArbCmd::new("id", "op_id", ArbData::default()));
        assert!(res.is_ok());
        let mut msg = ArbData::default();
        msg.set_json("{ \"a\": \"b\" }").unwrap();
        assert_eq!(res.unwrap(), msg);
        Ok(())
    });

    operator.upstream_arb = Box::new(|state, cmd| {
        assert_eq!(cmd, ArbCmd::new("id", "op_id", ArbData::default()));
        state.arb(cmd)
    });

    backend.upstream_arb = Box::new(|_, cmd| {
        assert_eq!(cmd, ArbCmd::new("id", "op_id", ArbData::default()));
        let mut msg = ArbData::default();
        msg.set_json("{ \"a\": \"b\" }").unwrap();
        Ok(msg)
    });

    let ptc = |definition| {
        PluginThreadConfiguration::new(
            definition,
            PluginLogConfiguration::new("", LoglevelFilter::Off),
        )
    };

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(ptc(frontend))
        .with_plugin(ptc(operator))
        .with_plugin(ptc(backend));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
// This tests basic simulator to plugin arb command sending.
fn simulator_to_plugin_arb() {
    let (mut frontend, mut operator, mut backend) = fe_op_be();

    frontend.host_arb = Box::new(|_, cmd| {
        assert_eq!(cmd, ArbCmd::new("front", "1", ArbData::default()));
        let mut msg = ArbData::default();
        msg.set_json("{ \"front\": 1 }").unwrap();
        Ok(msg)
    });

    operator.host_arb = Box::new(|_, cmd| {
        assert_eq!(cmd, ArbCmd::new("operator", "2", ArbData::default()));
        let mut msg = ArbData::default();
        msg.set_json("{ \"operator\": 2 }").unwrap();
        Ok(msg)
    });

    backend.host_arb = Box::new(|_, cmd| {
        assert_eq!(cmd, ArbCmd::new("backend", "3", ArbData::default()));
        let mut msg = ArbData::default();
        msg.set_json("{ \"backend\": 3 }").unwrap();
        Ok(msg)
    });

    let ptc = |definition| {
        PluginThreadConfiguration::new(
            definition,
            PluginLogConfiguration::new("", LoglevelFilter::Off),
        )
    };

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(ptc(frontend))
        .with_plugin(ptc(operator))
        .with_plugin(ptc(backend));

    let mut simulator = Simulator::new(configuration).unwrap();
    let simulation = &mut simulator.simulation;

    let res = simulation.arb_idx(0, ArbCmd::new("front", "1", ArbData::default()));
    let mut msg = ArbData::default();
    msg.set_json("{ \"front\": 1 }").unwrap();
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), msg);

    let res = simulation.arb("front", ArbCmd::new("front", "1", ArbData::default()));
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), msg);

    let res = simulation.arb("asdf", ArbCmd::new("front", "1", ArbData::default()));
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "Invalid argument: plugin asdf not found"
    );

    let res = simulation.arb_idx(3, ArbCmd::new("front", "1", ArbData::default()));
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "Invalid argument: index 3 out of range"
    );

    let res = simulation.arb_idx(-5, ArbCmd::new("front", "1", ArbData::default()));
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "Invalid argument: index -5 out of range"
    );

    let res = simulation.arb_idx(1, ArbCmd::new("operator", "2", ArbData::default()));
    let mut msg = ArbData::default();
    msg.set_json("{ \"operator\": 2 }").unwrap();
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), msg);

    let res = simulation.arb_idx(2, ArbCmd::new("backend", "3", ArbData::default()));
    let mut msg = ArbData::default();
    msg.set_json("{ \"backend\": 3 }").unwrap();
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), msg);

    let res = simulation.arb_idx(-1, ArbCmd::new("backend", "3", ArbData::default()));
    let mut msg = ArbData::default();
    msg.set_json("{ \"backend\": 3 }").unwrap();
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), msg);
}

#[test]
// This tests send/recv to accelerator via simulation api.
fn send_recv_accelerator() {
    let (mut frontend, _, backend) = fe_op_be();

    frontend.run = Box::new(|state, _| {
        assert!(state.send(ArbData::default()).is_ok());
        let res = state.recv();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), ArbData::default());
        Ok(ArbData::default())
    });

    let ptc = |definition| {
        PluginThreadConfiguration::new(
            definition,
            PluginLogConfiguration::new("", LoglevelFilter::Off),
        )
    };

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(ptc(frontend))
        .with_plugin(ptc(backend));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());

    let mut simulator = simulator.unwrap();
    assert!(simulator.simulation.send(ArbData::default()).is_ok());

    // test deadlock
    let res = simulator.simulation.recv();
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "Deadlock: recv() called while queue is empty and accelerator is idle"
    );

    // start a program
    assert!(simulator.simulation.start(ArbData::default()).is_ok());
    let res = simulator.simulation.recv();
    assert!(res.is_ok());
}

#[test]
// This tests basic plugin arb command propagation.
fn plugin_user_init_fail() {
    let (mut frontend, _, backend) = fe_op_be();

    frontend.initialize = Box::new(|_, _| err("please help"));

    let ptc = |definition| {
        PluginThreadConfiguration::new(
            definition,
            PluginLogConfiguration::new("", LoglevelFilter::Off),
        )
    };

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(ptc(frontend))
        .with_plugin(ptc(backend));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_err());
    assert_eq!(simulator.unwrap_err().to_string(), "please help");
}

#[test]
// This should not get stuck.
fn debug_deadlock() {
    let (mut frontend, _, backend) = fe_op_be();

    frontend.run = Box::new(|state, _| {
        // This recv should unblock if the simulation ends.
        let res = state.recv();
        assert_eq!(res.unwrap_err().to_string(), "Simulation aborted");
        Ok(ArbData::default())
    });

    let ptc = |definition| {
        PluginThreadConfiguration::new(
            definition,
            PluginLogConfiguration::new("", LoglevelFilter::Off),
        )
    };

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(ptc(frontend))
        .with_plugin(ptc(backend));

    let mut simulator = Simulator::new(configuration).unwrap();
    simulator.simulation.start(ArbData::default()).unwrap();
}

fn thread_config_type(plugin_type: PluginType) -> PluginThreadConfiguration {
    PluginThreadConfiguration::new(
        PluginDefinition::new(plugin_type, PluginMetadata::new("", "", "")),
        PluginLogConfiguration::new("", LoglevelFilter::Off),
    )
}

fn minimal_simulator() -> Simulator {
    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(thread_config_type(PluginType::Backend))
        .with_plugin(thread_config_type(PluginType::Frontend));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
    simulator.unwrap()
}

#[allow(unused)]
fn verbose_simulator_configuration() -> SimulatorConfiguration {
    let mut configuration = SimulatorConfiguration::default().without_reproduction();
    configuration.stderr_level = LoglevelFilter::Trace;
    configuration.dqcsim_level = LoglevelFilter::Trace;
    configuration
}

#[test]
fn simulation_default_config() {
    let configuration = SimulatorConfiguration::default().without_reproduction();
    let simulator = Simulator::new(configuration);
    assert!(simulator.is_err());
    assert_eq!(
        simulator.unwrap_err().to_string(),
        "Invalid argument: missing frontend"
    );
}

#[test]
fn simulation_missing_backend() {
    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .with_plugin(thread_config_type(PluginType::Frontend));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_err());
    assert_eq!(
        simulator.unwrap_err().to_string(),
        "Invalid argument: missing backend"
    );
}

#[test]
fn simulation_missing_frontend() {
    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .with_plugin(thread_config_type(PluginType::Backend));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_err());
    assert_eq!(
        simulator.unwrap_err().to_string(),
        "Invalid argument: missing frontend"
    );
}

#[test]
fn simulation_minimal_setup() {
    minimal_simulator();
}

#[test]
fn simulation_wait_without_start() {
    let simulation = &mut minimal_simulator().simulation;
    let wait = simulation.wait();
    assert!(wait.is_err());
    assert_eq!(
        wait.unwrap_err().to_string(),
        "Invalid operation: accelerator is not running; call start() first"
    );
}

#[test]
fn simulation_metadata() {
    let plugin_metadata = PluginMetadata::new("name", "author", "version");
    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(thread_config_type(PluginType::Backend))
        .with_plugin(PluginThreadConfiguration::new(
            PluginDefinition::new(PluginType::Frontend, plugin_metadata.clone()),
            PluginLogConfiguration::new("test", LoglevelFilter::Off),
        ));
    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
    let simulation = &mut simulator.unwrap().simulation;

    let metadata = simulation.get_metadata("test");
    assert!(metadata.is_ok());
    assert_eq!(&plugin_metadata, metadata.unwrap());

    let metadata = simulation.get_metadata("asdf");
    assert!(metadata.is_err());
    assert_eq!(
        metadata.unwrap_err().to_string(),
        "Invalid argument: plugin asdf not found"
    );

    let metadata = simulation.get_metadata_idx(0);
    assert!(metadata.is_ok());
    assert_eq!(&plugin_metadata, metadata.unwrap());

    let metadata = simulation.get_metadata_idx(2);
    assert!(metadata.is_err());
}

#[test]
fn simulation_initial_state() {
    let simulation = &mut minimal_simulator().simulation;
    assert!(simulation.yield_to_accelerator().is_ok());
}

#[test]
// Tests if initilize commands from a Plugin arrive in the intialize callback
// of a PluginDefinition.
fn simulation_init_cmds() {
    let mut definition =
        PluginDefinition::new(PluginType::Frontend, PluginMetadata::new("", "", ""));

    definition.initialize = Box::new(|_state, init_cmds| {
        assert_eq!(init_cmds.len(), 1);
        assert_eq!(ArbCmd::new("a", "b", ArbData::default()), init_cmds[0]);
        Ok(())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(thread_config_type(PluginType::Backend))
        .with_plugin(
            PluginThreadConfiguration::new(
                definition,
                PluginLogConfiguration::new("", LoglevelFilter::Off),
            )
            .with_init_cmd(ArbCmd::new("a", "b", ArbData::default())),
        );

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
// Attempt recv outside of run callbacks.
fn simulation_bad_recv() {
    let mut definition =
        PluginDefinition::new(PluginType::Frontend, PluginMetadata::new("", "", ""));

    definition.initialize = Box::new(|state, _| {
        let recv = state.recv();
        assert!(recv.is_err());
        assert_eq!(
            recv.unwrap_err().to_string(),
            "Invalid operation: recv() can only be called from inside the run() callback"
        );
        Ok(())
    });
    definition.drop = Box::new(|state| {
        let recv = state.recv();
        assert!(recv.is_err());
        assert_eq!(
            recv.unwrap_err().to_string(),
            "Invalid operation: recv() can only be called from inside the run() callback"
        );
        Ok(())
    });

    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(thread_config_type(PluginType::Backend))
        .with_plugin(PluginThreadConfiguration::new(
            definition,
            PluginLogConfiguration::new("", LoglevelFilter::Off),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
#[allow(clippy::redundant_closure)]
fn bad_simulation_pipeline_too_short() {
    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(thread_config_type(PluginType::Backend));

    let log_thread = LogThread::spawn(
        "dqcsim",
        configuration.dqcsim_level,
        configuration.stderr_level,
        configuration.log_callback,
        configuration.tee_files,
    )
    .unwrap();

    let pipeline: Vec<Box<dyn Plugin>> = configuration
        .plugins
        .into_iter()
        .map(|plugin| plugin.instantiate())
        .collect();

    let simulation = Simulation::new(pipeline, Seed::default(), None, &log_thread);
    assert!(simulation.is_err());
    assert_eq!(
        simulation.unwrap_err().to_string(),
        "Invalid argument: Simulation must consist of at least a frontend and backend"
    );
}

#[test]
#[allow(clippy::redundant_closure)]
fn bad_simulation_pipeline_fe_op_only() {
    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(thread_config_type(PluginType::Frontend))
        .with_plugin(thread_config_type(PluginType::Operator));

    let log_thread = LogThread::spawn(
        "dqcsim",
        configuration.dqcsim_level,
        configuration.stderr_level,
        configuration.log_callback,
        configuration.tee_files,
    )
    .unwrap();

    let pipeline: Vec<Box<dyn Plugin>> = configuration
        .plugins
        .into_iter()
        .map(|plugin| plugin.instantiate())
        .collect();

    let simulation = Simulation::new(pipeline, Seed::default(), None, &log_thread);
    assert!(simulation.is_err());
    #[cfg(target_os = "macos")]
    assert_eq!(
        simulation.unwrap_err().to_string(),
        "Interprocess communication error: io error: No senders exist for this port."
    );
    #[cfg(target_os = "linux")]
    assert_eq!(
        simulation.unwrap_err().to_string(),
        "Interprocess communication error: io error: All senders for this socket closed"
    );
}

#[test]
#[allow(clippy::redundant_closure)]
fn bad_simulation_pipeline_be_op_only() {
    let configuration = SimulatorConfiguration::default()
        .without_reproduction()
        .without_logging()
        .with_plugin(thread_config_type(PluginType::Backend))
        .with_plugin(thread_config_type(PluginType::Operator));

    let log_thread = LogThread::spawn(
        "dqcsim",
        configuration.dqcsim_level,
        configuration.stderr_level,
        configuration.log_callback,
        configuration.tee_files,
    )
    .unwrap();

    let pipeline: Vec<Box<dyn Plugin>> = configuration
        .plugins
        .into_iter()
        .map(|plugin| plugin.instantiate())
        .collect();

    let simulation = Simulation::new(pipeline, Seed::default(), None, &log_thread);
    assert!(simulation.is_err());
    #[cfg(target_os = "macos")]
    assert_eq!(
        simulation.unwrap_err().to_string(),
        "Interprocess communication error: io error: No senders exist for this port."
    );
    #[cfg(target_os = "linux")]
    assert_eq!(
        simulation.unwrap_err().to_string(),
        "Interprocess communication error: io error: All senders for this socket closed"
    );
}

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
