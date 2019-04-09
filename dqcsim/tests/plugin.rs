use dqcsim::{
    common::{
        log::LoglevelFilter,
        types::{ArbCmd, ArbData, PluginMetadata, PluginType},
    },
    host::{
        configuration::{
            PluginLogConfiguration, PluginThreadConfiguration, SimulatorConfiguration,
        },
        simulator::Simulator,
    },
    plugin::definition::PluginDefinition,
};

fn fe_op_be() -> (PluginDefinition, PluginDefinition, PluginDefinition) {
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
}
