use crate::{
    configuration::PluginType,
    error::Result,
    ipc::{
        plugin::{connect_simulator, initialize},
        PluginChannel,
    },
    log::{init, proxy::LogProxy, LoglevelFilter},
    protocol::message::Response,
};
use ipc_channel::ipc::{IpcReceiverSet, IpcSelectionResult, IpcSender};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Incoming {
    Request,
}

/// Managed Simulator IPC connection wrapper.
pub struct Connection {
    pub map: HashMap<u64, Incoming>,
    pub incoming: IpcReceiverSet,
    pub response: IpcSender<Response>,
}

impl Connection {
    pub fn init(
        name: impl Into<String>,
        level: impl Into<LoglevelFilter>,
        simulator: impl Into<String>,
        plugin_type: PluginType,
    ) -> Result<Connection> {
        let mut incoming = IpcReceiverSet::new()?;
        let mut map = HashMap::with_capacity(3);

        let channel: PluginChannel = connect_simulator(simulator)?;

        init(LogProxy::boxed(name, level.into(), channel.log.clone()))?;

        initialize(&channel, plugin_type)?;

        map.insert(incoming.add(channel.request)?, Incoming::Request);

        Ok(Connection {
            map,
            incoming,
            response: channel.response,
        })
    }

    pub fn recv(&mut self, handler: impl FnMut(&IpcSelectionResult)) -> Result<()> {
        self.incoming.select()?.iter().for_each(handler);
        Ok(())
    }
}
