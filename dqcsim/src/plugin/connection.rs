use crate::{
    common::{
        error::Result,
        log::{init, proxy::LogProxy, tee_file::TeeFile, Log},
        protocol::Response,
    },
    host::configuration::PluginType,
    plugin::ipc::{connect_simulator, initialize},
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
    pub fn init(simulator: impl Into<String>, plugin_type: PluginType) -> Result<Connection> {
        let mut incoming = IpcReceiverSet::new()?;
        let mut map = HashMap::with_capacity(3);

        let (channel, configuration) = connect_simulator(simulator)?;

        let mut loggers = Vec::with_capacity(1 + configuration.nonfunctional.tee_files.len());
        loggers.push(LogProxy::boxed(
            configuration.name,
            configuration.nonfunctional.verbosity,
            channel.log.clone(),
        ) as Box<dyn Log>);
        loggers.extend(
            configuration
                .nonfunctional
                .tee_files
                .into_iter()
                .map(TeeFile::create)
                .map(Box::new)
                .map(|l| l as Box<dyn Log>),
        );
        init(loggers)?;

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
