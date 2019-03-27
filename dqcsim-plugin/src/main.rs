use dqcsim::{
    common::protocol::{ArbCmd, PluginMetadata, PluginToSimulator, SimulatorToPlugin},
    host::configuration::PluginType,
    info,
    plugin::{
        connection::{Connection, IncomingMessage, OutgoingMessage},
        context::PluginContext,
    },
    trace,
};
use failure::Error;
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    // Keeping this to defer type
    let name: &str = args[1].as_ref();
    let server = args[2].as_ref();

    let plugin_type = if name.starts_with("front") {
        PluginType::Frontend
    } else if name.starts_with("back") {
        PluginType::Backend
    } else {
        PluginType::Operator
    };

    let metadata = PluginMetadata::new("example", "0.1.0", "mb");
    // Init fn
    let initialize: Box<dyn Fn(&mut PluginContext, Vec<ArbCmd>)> = Box::new(|_, _| {
        trace!("Running plugin init function.");
    });

    let mut connection = Connection::new(server)?.init(plugin_type, metadata, initialize)?;

    eprintln!("stderr");
    println!("stdout");

    if let Ok(Some(IncomingMessage::Simulator(SimulatorToPlugin::Abort))) =
        connection.next_request()
    {
        connection.send(OutgoingMessage::Simulator(PluginToSimulator::Success))?;
    } else {
        std::process::exit(1);
    }

    info!("Plugin down.");
    Ok(())
}
