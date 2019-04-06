use dqcsim::{
    common::{
        log::LoglevelFilter,
        types::{ArbCmd, ArbData, PluginMetadata, PluginType},
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

fn thread_config_type(plugin_type: PluginType) -> PluginThreadConfiguration {
    PluginThreadConfiguration::new(
        PluginDefinition::new(plugin_type, PluginMetadata::new("", "", "")),
        PluginLogConfiguration::new("", LoglevelFilter::Off),
    )
}

fn minimal_simulator() -> Simulator {
    let configuration = SimulatorConfiguration::default()
        .push_plugin(thread_config_type(PluginType::Backend))
        .push_plugin(thread_config_type(PluginType::Frontend));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
    simulator.unwrap()
}

#[allow(unused)]
fn verbose_simulator_configuration() -> SimulatorConfiguration {
    let mut configuration = SimulatorConfiguration::default();
    configuration.stderr_level = LoglevelFilter::Trace;
    configuration.dqcsim_level = LoglevelFilter::Trace;
    configuration
}

#[test]
fn simulation_default_config() {
    let configuration = SimulatorConfiguration::default();
    let simulator = Simulator::new(configuration);
    assert!(simulator.is_err());
    assert_eq!(
        simulator.unwrap_err().to_string(),
        "Invalid argument: missing frontend"
    );
}

#[test]
fn simulation_missing_backend() {
    let configuration =
        SimulatorConfiguration::default().push_plugin(thread_config_type(PluginType::Frontend));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_err());
    assert_eq!(
        simulator.unwrap_err().to_string(),
        "Invalid argument: missing backend"
    );
}

#[test]
fn simulation_missing_frontend() {
    let configuration =
        SimulatorConfiguration::default().push_plugin(thread_config_type(PluginType::Backend));

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
        .push_plugin(thread_config_type(PluginType::Backend))
        .push_plugin(PluginThreadConfiguration::new(
            PluginDefinition::new(PluginType::Frontend, plugin_metadata.clone()),
            PluginLogConfiguration::new("test", LoglevelFilter::Off),
        ));
    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
    let simulation = &mut simulator.unwrap().simulation;

    let metadata = simulation.get_metadata("test");
    assert!(metadata.is_ok());
    assert_eq!(&plugin_metadata, metadata.unwrap());

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
        .push_plugin(thread_config_type(PluginType::Backend))
        .push_plugin(PluginThreadConfiguration {
            definition,
            init_cmds: vec![ArbCmd::new("a", "b", ArbData::default())],
            log_configuration: PluginLogConfiguration::new("", LoglevelFilter::Off),
        });

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}

#[test]
fn simulation_recv_in_initialize() {
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

    let configuration = SimulatorConfiguration::default()
        .push_plugin(thread_config_type(PluginType::Backend))
        .push_plugin(PluginThreadConfiguration::new(
            definition,
            PluginLogConfiguration::new("", LoglevelFilter::Off),
        ));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}
