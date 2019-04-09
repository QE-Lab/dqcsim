use dqcsim::{
    common::{
        log::{thread::LogThread, LoglevelFilter},
        types::{ArbCmd, ArbData, PluginMetadata, PluginType},
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
fn simulation_deadlock() {
    let simulation = &mut minimal_simulator().simulation;
    let wait = simulation.wait();
    assert!(wait.is_err());
    assert_eq!(
        wait.unwrap_err().to_string(),
        "Deadlock: accelerator is blocked on recv() while we are expecting it to return"
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
    #[cfg(os = "macos")]
    assert_eq!(
        simulation.unwrap_err().to_string(),
        "Failed to initialize plugin(s): Interprocess communication error: io error: No senders exist for this port.; Interprocess communication error: io error: No senders exist for this port."
    );
    #[cfg(os = "linux")]
    assert_eq!(
        simulation.unwrap_err().to_string(),
    "Failed to initialize plugin(s): Interprocess communication error: io error: All senders for this socket closed; Interprocess communication error: io error: All senders for this socket closed"
    );
}

#[test]
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
    #[cfg(os = "macos")]
    assert_eq!(
        simulation.unwrap_err().to_string(),
        "Failed to initialize plugin(s): Interprocess communication error: io error: No senders exist for this port."
    );
    #[cfg(os = "linux")]
    assert_eq!(
        simulation.unwrap_err().to_string(),
        "Failed to initialize plugin(s): Interprocess communication error: io error: All senders for this socket closed."
    );
}
