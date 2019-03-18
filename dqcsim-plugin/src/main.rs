use dqcsim::{
    configuration::PluginType, debug, info, ipc::connection::Connection,
    protocol::message::Response,
};
use failure::Error;
use ipc_channel::ipc::IpcSelectionResult;
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let name: &str = args[1].as_ref();
    let server = args[2].as_ref();

    let plugin_type = if name.starts_with("front") {
        PluginType::Frontend
    } else if name.starts_with("back") {
        PluginType::Backend
    } else {
        PluginType::Operator
    };

    let mut connection = Connection::init(server, plugin_type)?;

    let map = connection.map.clone();

    connection
        .incoming
        .select()?
        .iter()
        .for_each(|message| match message {
            IpcSelectionResult::MessageReceived(id, message) => {
                debug!("[{:?}] {:?}", &map[&id], message);
                connection.response.send(Response::Success).unwrap();
                // std::process::exit(0);
            }
            IpcSelectionResult::ChannelClosed(id) => {
                debug!("[{:?}] Closed", &map[&id]);
                // std::process::exit(1);
            }
        });

    std::thread::sleep(std::time::Duration::from_secs(4));
    info!("Plugin down.");

    Ok(())
}
