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
    warn,
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
// This tests basic (host and plugin) arb command propagation.
fn plugin_to_plugin_channel() {
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

    backend.upstream_arb = Box::new(|_, _| {
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
        .with_stderr_level(LoglevelFilter::Trace)
        .with_dqcsim_level(LoglevelFilter::Trace)
        .with_plugin(ptc(frontend))
        .with_plugin(ptc(operator))
        .with_plugin(ptc(backend));

    let mut simulator = Simulator::new(configuration).unwrap();
    let simulation = &mut simulator.simulation;
    assert!(simulation
        .arb_idx(0, ArbCmd::new("a", "b", ArbData::default()))
        .is_ok());
}
