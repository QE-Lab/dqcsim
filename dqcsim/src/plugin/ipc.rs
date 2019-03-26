//! IPC channel setup functionality.

use crate::{
    common::{
        error::{ErrorKind, Result},
        log::Record,
        protocol::{
            GatestreamDown, GatestreamUp, PluginInitializeResponse, PluginToSimulator,
            SimulatorToPlugin,
        },
    },
    host::{configuration::PluginType, ipc::SimulatorChannel},
};
use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};
use serde::{Deserialize, Serialize};

/// The Plugin side of a Simulator and Plugin channel.
/// Constructed and owned by a [`Connection`] instance.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginChannel {
    pub log: Option<IpcSender<Record>>,
    pub request: IpcReceiver<SimulatorToPlugin>,
    pub response: IpcSender<PluginToSimulator>,
}

impl PluginChannel {
    /// Returns a PluginChannel wrapper.
    pub fn new(
        log: IpcSender<Record>,
        request: IpcReceiver<SimulatorToPlugin>,
        response: IpcSender<PluginToSimulator>,
    ) -> PluginChannel {
        PluginChannel {
            log: Some(log),
            request,
            response,
        }
    }
    /// Take log channel out the channel wrapper.
    pub fn log(&mut self) -> Option<IpcSender<Record>> {
        self.log.take()
    }
}

/// Channel between plugins. This side sends GateStream messages to a downstream plugin.
/// Constructed and owned by a [`Connection`] instance.
#[derive(Debug, Serialize, Deserialize)]
pub struct DownstreamChannel {
    pub tx: Option<IpcSender<GatestreamDown>>,
    pub rx: Option<IpcReceiver<GatestreamUp>>,
}

impl DownstreamChannel {
    /// Construct a new DownstreamChannel.
    pub fn new(tx: IpcSender<GatestreamDown>, rx: IpcReceiver<GatestreamUp>) -> DownstreamChannel {
        DownstreamChannel {
            tx: Some(tx),
            rx: Some(rx),
        }
    }
    pub fn rx_ref(&self) -> Result<&IpcReceiver<GatestreamUp>> {
        match &self.rx {
            Some(rx) => Ok(rx),
            None => Err(ErrorKind::IPCError(
                "Downstream receiver side not available".to_string(),
            ))?,
        }
    }
    pub fn tx_ref(&self) -> Result<&IpcSender<GatestreamDown>> {
        match &self.tx {
            Some(tx) => Ok(tx),
            None => Err(ErrorKind::IPCError(
                "Downstream sender side not available".to_string(),
            ))?,
        }
    }

    /// Take receiver out the channel wrapper.
    pub fn rx(&mut self) -> Option<IpcReceiver<GatestreamUp>> {
        self.rx.take()
    }
    /// Take sender out the channel wrapper.
    pub fn tx(&mut self) -> Option<IpcSender<GatestreamDown>> {
        self.tx.take()
    }
}

/// Channel between plugins. This side receives GateStream messages from an upstream plugin.
/// Constructed and owned by a [`Connection`] instance.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpstreamChannel {
    tx: Option<IpcSender<GatestreamUp>>,
    rx: Option<IpcReceiver<GatestreamDown>>,
}

impl UpstreamChannel {
    /// Construct a new UpstreamChannel.
    pub fn new(tx: IpcSender<GatestreamUp>, rx: IpcReceiver<GatestreamDown>) -> UpstreamChannel {
        UpstreamChannel {
            tx: Some(tx),
            rx: Some(rx),
        }
    }
    /// Take receiver out the channel wrapper.
    pub fn rx(&mut self) -> Option<IpcReceiver<GatestreamDown>> {
        self.rx.take()
    }
    /// Take sender out the channel wrapper.
    pub fn tx(&mut self) -> Option<IpcSender<GatestreamUp>> {
        self.tx.take()
    }
}

/// Connect a [`Plugin`] to a [`Simulation`] instance.
///
/// Attempts to connect to the provided server address. Then continues to
/// setup a request-response channel pair between the [`Simulation`] instance
/// and the [`Plugin`].
///
/// [`Plugin`]: ../plugin/struct.Plugin.html
/// [`Simulation`]: ../simulator/struct.Simulation.html
pub fn connect_simulator(server: impl Into<String>) -> Result<PluginChannel> {
    // Attempt to connect to the server
    let connect = IpcSender::connect(server.into())?;

    // Construct the Simulator and Plugin channel pair
    let (log, log_rx) = ipc_channel::ipc::channel()?;
    let (request_tx, request) = ipc_channel::ipc::channel()?;
    let (response, response_rx) = ipc_channel::ipc::channel()?;
    let simulator = SimulatorChannel::new(log_rx, request_tx, response_rx);
    let plugin = PluginChannel::new(log, request, response);

    // Send channel to the simulator.
    connect.send(simulator)?;
    Ok(plugin)

    // Wait for simulator to send configuration to complete the handshake.
    // match plugin.request.recv() {
    //     Ok(Request::Configuration(configuration)) => {
    //         plugin.response.send(Response::Success)?;
    //         Ok((plugin, *configuration))
    //     }
    //     _ => Err(ErrorKind::Other("Handshake problem".to_string()))?,
    // }
}

/// Connect an upstream [`Plugin`] instance to a downstream [`Plugin`] instance.
///
/// Attempts to connect to the provided server address. Then continues to
/// setup a downstream-upstream channel pair between the [`Plugin`] instances.
///
/// [`Plugin`]: ../plugin/struct.Plugin.html
/// [`Simulation`]: ../simulator/struct.Simulation.html
fn connect_downstream(server: impl Into<String>) -> Result<DownstreamChannel> {
    // Attempt to connect to the server
    let connect = IpcSender::connect(server.into())?;
    let (down_tx, down_rx) = ipc_channel::ipc::channel()?;
    let (up_tx, up_rx) = ipc_channel::ipc::channel()?;
    connect.send(UpstreamChannel::new(up_tx, down_rx))?;
    Ok(DownstreamChannel::new(down_tx, up_rx))
}

pub fn initialize(
    channel: &PluginChannel,
    plugin_type: PluginType,
) -> Result<(Option<DownstreamChannel>, Option<UpstreamChannel>)> {
    let init = channel.request.recv()?;
    let mut downstream_channel = None;
    let mut upstream_channel = None;
    match init {
        SimulatorToPlugin::Initialize(request) => {
            // Frontend and operator connect to downstream plugin first.
            match plugin_type {
                PluginType::Frontend | PluginType::Operator => {
                    downstream_channel = Some(connect_downstream(request.downstream.unwrap())?);
                }
                PluginType::Backend => {}
            }

            // Backend and operator start upstream channel.
            match plugin_type {
                PluginType::Backend | PluginType::Operator => {
                    let (upstream_server, upstream) = IpcOneShotServer::new().unwrap();

                    channel.response.send(PluginToSimulator::Initialized(
                        PluginInitializeResponse {
                            upstream: Some(upstream),
                        },
                    ))?;

                    let (_, sender): (_, UpstreamChannel) = upstream_server.accept().unwrap();
                    upstream_channel = Some(sender);
                }
                PluginType::Frontend => {
                    channel.response.send(PluginToSimulator::Initialized(
                        PluginInitializeResponse { upstream: None },
                    ))?;
                }
            }

            Ok((downstream_channel, upstream_channel))
        }
        _ => Err(ErrorKind::Other("Handshake problem".to_string()))?,
    }
}
