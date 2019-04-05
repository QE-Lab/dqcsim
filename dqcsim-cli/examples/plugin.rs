use dqcsim::{
    common::types::{ArbData, PluginMetadata, PluginType},
    debug, info,
    plugin::{definition::PluginDefinition, state::PluginState},
    trace,
};
use failure::Error;
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    // Keeping this to defer type
    let name: &str = args[1].as_ref();
    let server = args[2].as_ref();

    eprintln!("{:#?}", args);

    let plugin_type = if name.starts_with("front") {
        PluginType::Frontend
    } else if name.starts_with("back") {
        PluginType::Backend
    } else {
        PluginType::Operator
    };

    let mut definition = PluginDefinition::new(
        plugin_type,
        PluginMetadata::new(format!("example: {}", name), "mb", "0.1.0"),
    );

    definition.initialize = Box::new(|_state, arb_cmds| {
        trace!("running plugin init callback!");
        for arb_cmd in arb_cmds {
            debug!("{}", arb_cmd);
        }
        Ok(())
    });

    definition.run = Box::new(|_state, _| {
        info!("running run callback!");
        Ok(ArbData::default())
    });

    PluginState::run(&definition, server)?;

    Ok(())
}
