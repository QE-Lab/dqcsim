use dqcsim::{
    common::protocol::{PluginToSimulator, SimulatorToPlugin},
    host::configuration::PluginType,
    info,
    plugin::connection::{Connection, IncomingMessage, OutgoingMessage},
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

    let mut connection = Connection::new(server)?;

    eprintln!("stderr");
    println!("stdout");

    match connection.next_request() {
        Ok(msg) => {
            trace!("{:?}", msg);
            match msg {
                Some(IncomingMessage::Simulator(req)) => match req {
                    SimulatorToPlugin::Abort => {
                        connection.send(OutgoingMessage::Simulator(PluginToSimulator::Success))?;
                        std::process::exit(0);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Err(err) => panic!("{}", err),
    }

    info!("Plugin down.");

    Ok(())
}
