use dqcsim::{debug, info, ipc::connection::Connection, plugin::PluginType};
use failure::Error;
use ipc_channel::ipc::IpcSelectionResult;
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let name: &str = args[1].as_ref();
    let server = args[2].as_ref();

    let plugin_type = if name.starts_with("frontend") {
        PluginType::Frontend
    } else if name.starts_with("backend") {
        PluginType::Backend
    } else {
        PluginType::Operator
    };

    let mut connection = Connection::init(server, plugin_type)?;
    let map = connection.map.clone();

    connection.recv(|message| match message {
        IpcSelectionResult::MessageReceived(id, message) => {
            debug!("[{:?}] {:?}", &map[&id], message);
        }
        IpcSelectionResult::ChannelClosed(id) => {
            debug!("[{:?}] Closed", &map[&id]);
        }
    })?;

    info!("Plugin down.");

    Ok(())
}
