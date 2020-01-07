// use dqcsim::{
//     common::types::{MatrixMap, PluginMetadata, PluginType},
//     debug, info,
//     plugin::{definition::PluginDefinition, state::PluginState},
// };
// use std::env;

fn main() {
    //     let mut definition = PluginDefinition::new(
    //         PluginType::Operator,
    //         PluginMetadata::new("Metrics operator", "Matthijs Brobbel", "0.1.0"),
    //     );

    //     definition.gate = Box::new(|state, gate| {
    //         let mm = MatrixMap::default();

    //         debug!("targetted qubits: {:?}", &gate.get_targets());
    //         debug!("control qubits: {:?}", &gate.get_controls());
    //         debug!("measured qubits: {:?}", &gate.get_measures());

    //         if let Some(matrix) = &gate.get_matrix() {
    //             match mm.detect(matrix).unwrap().map(|(_, g)| g) {
    //                 Some(gate) => debug!("Gate detected: {:?}", gate),
    //                 None => debug!("Unable to detect gate"),
    //             }
    //         }

    //         if let Some(name) = &gate.get_name() {
    //             debug!("named gate: {}", name);
    //         }
    //         state.gate(gate).map(|_| vec![])
    //     });

    //     definition.modify_measurement = Box::new(|_, measurement| {
    //         info!("{:?}", measurement);
    //         Ok(vec![measurement])
    //     });

    //     PluginState::run(&definition, env::args().nth(1).unwrap()).unwrap();
}
