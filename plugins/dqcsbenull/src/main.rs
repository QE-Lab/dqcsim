//! Basic backend implementation that doesn't do anything, to be used for
//! testing and perhaps as an example. Measurements return random values with
//! a 50/50 chance.

use dqcsim::{
    common::types::{
        ArbData, PluginMetadata, PluginType, QubitMeasurementResult, QubitMeasurementValue,
    },
    debug, info,
    plugin::{definition::PluginDefinition, state::PluginState},
};
use std::env;

fn main() {
    let mut definition = PluginDefinition::new(
        PluginType::Backend,
        PluginMetadata::new("Null backend", "TU Delft QCE", "0.1.0"),
    );

    definition.initialize = Box::new(|_state, arb_cmds| {
        info!("Running null backend initialization callback");
        for arb_cmd in arb_cmds {
            debug!("{}", arb_cmd);
        }
        Ok(())
    });

    definition.gate = Box::new(|state, gate| {
        let measured_qubits = gate.get_measures();
        if measured_qubits.is_empty() {
            debug!("Received a gate that doesn't measure anything");
            Ok(vec![])
        } else {
            debug!(
                "Received a gate that measures the following qubits: {:?}",
                measured_qubits
            );
            Ok(measured_qubits
                .iter()
                .map(|q| {
                    let value = if state.random_f64() < 0.5 {
                        QubitMeasurementValue::Zero
                    } else {
                        QubitMeasurementValue::One
                    };
                    QubitMeasurementResult::new(*q, value, ArbData::default())
                })
                .collect())
        }
    });

    PluginState::run(&definition, env::args().nth(1).unwrap().as_ref()).unwrap();
}
