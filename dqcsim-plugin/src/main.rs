use dqcsim::{
    common::{
        protocol::{FrontendRunResponse, PluginToSimulator, SimulatorToPlugin},
        types::{ArbCmd, ArbData, PluginMetadata},
    },
    debug,
    host::configuration::PluginType,
    info,
    plugin::{
        connection::{Connection, IncomingMessage, OutgoingMessage},
        context::PluginState,
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

    eprintln!("{:#?}", args);

    let plugin_type = if name.starts_with("front") {
        PluginType::Frontend
    } else if name.starts_with("back") {
        PluginType::Backend
    } else {
        PluginType::Operator
    };

    let metadata = PluginMetadata::new("example", "mb", "0.1.0");
    // Init fn
    let initialize: Box<dyn Fn(&mut PluginState, Vec<ArbCmd>)> = Box::new(|_ctx, arb_cmds| {
        trace!("Running plugin init function.");
        for arb_cmd in arb_cmds {
            debug!("{}", arb_cmd);
        }
    });

    let mut connection = Connection::new(server)?.init(plugin_type, metadata, initialize)?;

    eprintln!("stderr");
    println!("stdout");

    while let Ok(next) = connection.next_request() {
        match next {
            Some(IncomingMessage::Simulator(SimulatorToPlugin::Abort)) => {
                connection.send(OutgoingMessage::Simulator(PluginToSimulator::Success))?;
                break;
            }
            Some(IncomingMessage::Simulator(SimulatorToPlugin::RunRequest(_))) => {
                connection.send(OutgoingMessage::Simulator(PluginToSimulator::RunResponse(
                    FrontendRunResponse {
                        return_value: Some(ArbData::default()),
                        messages: vec![],
                    },
                )))?;
            }
            _ => {
                eprintln!("{:?}", next);
                connection.send(OutgoingMessage::Simulator(PluginToSimulator::Success))?;
            }
        }
    }

    info!("Plugin down.");
    Ok(())
}
