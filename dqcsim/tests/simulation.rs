use dqcsim::{
    common::{
        log::LoglevelFilter,
        types::{PluginMetadata, PluginType},
    },
    host::{
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
    let configuration = SimulatorConfiguration::default()
        .push_plugin(thread_config_type(PluginType::Backend))
        .push_plugin(thread_config_type(PluginType::Frontend));

    let simulator = Simulator::new(configuration);
    assert!(simulator.is_ok());
}
