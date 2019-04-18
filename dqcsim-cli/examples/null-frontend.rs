use dqcsim::{
    common::types::{ArbData, PluginMetadata, PluginType},
    debug, info,
    plugin::{definition::PluginDefinition, state::PluginState},
};
use failure::Error;
use std::env;

fn main() -> Result<(), Error> {
    let mut definition = PluginDefinition::new(
        PluginType::Frontend,
        PluginMetadata::new("example frontend".to_string(), "mb", "0.1.0"),
    );

    definition.initialize = Box::new(|_state, arb_cmds| {
        info!("running plugin init callback!");
        for arb_cmd in arb_cmds {
            debug!("{}", arb_cmd);
        }
        Ok(())
    });

    definition.run = Box::new(|state, _| {
        info!("running run callback!");
        state.send(ArbData::default()).expect("send failed");
        Ok(ArbData::default())
    });

    PluginState::run(&definition, env::args().nth(1).unwrap().as_ref())?;

    Ok(())
}
