use dqcsim::{
    common::types::{PluginMetadata, PluginType},
    debug, info,
    plugin::{definition::PluginDefinition, state::PluginState},
};
use failure::Error;
use std::env;

fn main() -> Result<(), Error> {
    let mut definition = PluginDefinition::new(
        PluginType::Operator,
        PluginMetadata::new(format!("example frontend"), "mb", "0.1.0"),
    );

    definition.initialize = Box::new(|_state, arb_cmds| {
        info!("running plugin init callback!");
        for arb_cmd in arb_cmds {
            debug!("{}", arb_cmd);
        }
        Ok(())
    });

    PluginState::run(&definition, env::args().nth(1).unwrap().as_ref())?;

    Ok(())
}
