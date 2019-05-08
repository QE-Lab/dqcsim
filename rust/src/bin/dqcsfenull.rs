//! Basic front-end implementation that doesn't do anything, to be used for
//! testing and perhaps as an example.
use dqcsim::{
    common::types::{ArbData, PluginMetadata, PluginType},
    debug, info,
    plugin::{definition::PluginDefinition, state::PluginState},
};
use std::env;

fn main() {
    let mut definition = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("Null frontend", "TU Delft QCE", "0.1.0"),
    );

    definition.initialize = Box::new(|_state, arb_cmds| {
        info!("Running null frontend initialization callback");
        for arb_cmd in arb_cmds {
            debug!("{}", arb_cmd);
        }
        Ok(())
    });

    definition.run = Box::new(|state, _args| {
        info!("Running null frontend run callback");
        state.send(ArbData::default()).expect("send failed");
        Ok(ArbData::default())
    });

    PluginState::run(&definition, env::args().nth(1).unwrap().as_ref()).unwrap();
}
