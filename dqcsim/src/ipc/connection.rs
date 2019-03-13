use crate::{
    ipc::{
        plugin::{connect_simulator, initialize},
        PluginChannel,
    },
    plugin::PluginType,
};
use dqcsim_log::{init, proxy::LogProxy, LevelFilter};
use failure::Error;
use ipc_channel::ipc::IpcSelectionResult;

use ipc_channel::ipc::IpcReceiverSet;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Incoming {
    Request,
}

/// Managed Simulator IPC connection wrapper.
pub struct Connection {
    pub map: HashMap<u64, Incoming>,
    pub incoming: IpcReceiverSet,
}

impl Connection {
    pub fn init(
        simulator: impl Into<String>,
        plugin_type: PluginType,
    ) -> Result<Connection, Error> {
        let mut incoming = IpcReceiverSet::new()?;
        let mut map = HashMap::with_capacity(3);

        let channel: PluginChannel = connect_simulator(simulator)?;

        init(LogProxy::boxed(channel.log.clone()), LevelFilter::Trace)?;

        initialize(&channel, plugin_type)?;

        map.insert(incoming.add(channel.request)?, Incoming::Request);

        Ok(Connection { map, incoming })
    }

    pub fn recv(&mut self, handler: impl FnMut(&IpcSelectionResult)) -> Result<(), Error> {
        self.incoming.select()?.iter().for_each(handler);
        Ok(())
    }
}
