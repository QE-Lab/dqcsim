//! Basic operator implementation that doesn't do anything, to be used for
//! testing and perhaps as an example.
use dqcsim::{
    common::types::{PluginMetadata, PluginType},
    debug, info,
    plugin::{definition::PluginDefinition, state::PluginState},
};
use std::env;

fn main() {
    let mut definition = PluginDefinition::new(
        PluginType::Operator,
        PluginMetadata::new("Null operator", "TU Delft QCE", "0.1.0"),
    );

    definition.initialize = Box::new(|_state, arb_cmds| {
        info!("Running null operator initialization callback");
        for arb_cmd in arb_cmds {
            debug!("{}", arb_cmd);
        }
        Ok(())
    });

    PluginState::run(&definition, env::args().nth(1).unwrap().as_ref()).unwrap();
}
