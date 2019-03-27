//! Plugin to simulator connection wrapper.

use crate::{
    common::{
        error::{ErrorKind, Result},
        log::Record,
        protocol::{
            ArbCmd, GatestreamDown, GatestreamUp, PluginInitializeResponse, PluginMetadata,
            PluginToSimulator, SimulatorToPlugin,
        },
    },
    host::{configuration::PluginType, ipc::SimulatorChannel},
    plugin::{
        context::PluginContext,
        ipc::{DownstreamChannel, PluginChannel, UpstreamChannel},
        log::setup_logging,
    },
    trace,
};
use ipc_channel::ipc::{IpcOneShotServer, IpcReceiverSet, IpcSelectionResult, IpcSender};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Incoming {
    Simulator,
    Upstream,
    // this is a response
    Downstream,
}

#[derive(Debug, PartialEq)]
pub enum IncomingMessage {
    Simulator(SimulatorToPlugin),
    Upstream(GatestreamDown),
    Downstream(GatestreamUp),
}

#[derive(Debug, PartialEq)]
pub enum OutgoingMessage {
    Simulator(PluginToSimulator),
    Upstream(GatestreamUp),
    Downstream(GatestreamDown),
}

/// Plugin to Simulator connection wrapper.
///
/// This provides a [`Plugin`] with the ability to communicate with both a
/// [`Simulator`] instance and other upstream and downstream plugins.
///
/// Constructing a [`Connection`] instance should be the first thing a
/// [`Plugin`] should do. The [`Simulator`] server address string is passed
/// as an argument to all Plugins started by a [`Simulator`]. The server
/// address string can be used to construct a [`Connection`] instance.
///
/// A [`Connection`] instance attempts to connect to the [`Simulator`] instance
/// to receives its [`PluginConfiguration`]. Based on the configuration, the
/// [`Connection`] instance will spawn the thread-local loggers and then wait
/// the [`Simulator`] to send a [`Request::Init`] request, to connect to the
/// upstream and downstream plugins.
pub struct Connection {
    /// Set of incoming request channels.
    /// Simulator request channel for all plugins. Incoming requests from
    /// upstream plugins for operator and backend plugins.
    incoming: IpcReceiverSet,
    /// Map to label incoming requests to their channel.
    incoming_map: HashMap<u64, Incoming>,
    /// Buffer for incoming requests.
    incoming_buffer: Vec<IncomingMessage>,

    /// Optional Downstream channel. Is None for Backend plugins.
    downstream: Option<DownstreamChannel>,

    /// Simulator response sender.
    response: IpcSender<PluginToSimulator>,
    /// Optional Upstream sender. Is None for Frontend plugins.
    upstream: Option<IpcSender<GatestreamUp>>,

    /// Log record sender. Consumed during log initialization.
    log: Option<IpcSender<Record>>,
}

impl Connection {
    /// Connect to a Simulator instance.
    /// Attempts to connect to the provided simulator server address. Then
    /// continues to setup a request-response channel pair between the
    /// Simulator and the Plugin. The method returns the PluginChannel and
    /// sends the SimulatorChannel to the Simulator.
    fn connect(simulator: impl Into<String>) -> Result<PluginChannel> {
        // Attempt to connect to the Simulator.
        let connect = IpcSender::connect(simulator.into())?;

        // Construct the Simulator and Plugin channel pair.
        let (log, log_rx) = ipc_channel::ipc::channel()?;
        let (request_tx, request) = ipc_channel::ipc::channel()?;
        let (response, response_rx) = ipc_channel::ipc::channel()?;

        // Send channel to the simulator.
        connect.send(SimulatorChannel::new(log_rx, request_tx, response_rx))?;

        // Return the PluginChannel.
        Ok(PluginChannel::new(log, request, response))
    }

    /// Construct a Connection wrapper instance.
    /// The Connection wrapper connects to the Simulator. It does however not
    /// initialize.
    pub fn new(simulator: impl Into<String>) -> Result<Connection> {
        // Attempt to connect to the simulator instance.
        let channel = Connection::connect(simulator)?;

        // Create incoming request collections.
        let mut incoming = IpcReceiverSet::new()?;
        let mut incoming_map = HashMap::with_capacity(2);
        incoming_map.insert(incoming.add(channel.request)?, Incoming::Simulator);

        Ok(Connection {
            incoming,
            incoming_map,
            incoming_buffer: Vec::new(),
            downstream: None,
            response: channel.response,
            upstream: None,
            log: channel.log,
        })
    }

    /// Handle the Initialization request from the Simulator.
    ///
    /// This is always the first request the Simulator sends and all Plugins
    /// should respond accordingly in order for the Simulation to initialize.
    /// TODO: add doc ref to SimulatorToPlugin::Initialize variant.
    pub fn init(
        mut self,
        typ: PluginType,
        metadata: PluginMetadata,
        initialize: Box<dyn Fn(&mut PluginContext, Vec<ArbCmd>)>,
    ) -> Result<Connection> {
        // Wait for the Initialize request from the Simulator.
        match if let Ok(Some(IncomingMessage::Simulator(SimulatorToPlugin::Initialize(req)))) =
            self.next_request()
        {
            // Setup logging.
            setup_logging(&req.configuration, self.log.take().unwrap())?;

            // Make sure type reported by Plugin corresponds to
            // PluginSpecification.
            if typ != req.configuration.specification.typ {
                Err(ErrorKind::InvalidOperation(
                    "PluginType reported by Plugin does not correspond with PluginSpecification"
                        .to_string(),
                ))?
            }

            // Connect to downstream plugin (not for backend).
            if req.downstream.is_some() {
                // Attempt to connect to the downstream plugin.
                let downstream = IpcSender::connect(req.downstream.unwrap())?;

                // Create channel pair.
                let (down_tx, down_rx) = ipc_channel::ipc::channel()?;
                let (up_tx, up_rx) = ipc_channel::ipc::channel()?;

                // Send upstream channel to downstream plugin.
                downstream.send(UpstreamChannel::new(up_tx, down_rx))?;

                // Store the DownstreamChannel.
                self.downstream = Some(DownstreamChannel::new(down_tx, up_rx));
            }

            // Run user init code.
            // TODO: replace this with an actual context
            let mut ctx = PluginContext {};
            initialize(&mut ctx, req.configuration.functional.init);

            // Init IPC endpoint for upstream plugin.
            if typ != PluginType::Frontend {
                let (upstream_server, upstream) = IpcOneShotServer::new()?;

                // Send reply to simulator.
                self.send(OutgoingMessage::Simulator(PluginToSimulator::Initialized(
                    PluginInitializeResponse {
                        upstream: Some(upstream),
                        metadata,
                    },
                )))?;

                // Wait for upstream plugin to connect.
                let (_, mut upstream): (_, UpstreamChannel) = upstream_server.accept()?;

                // Store upstream channel incoming and outgoing in connection
                // wrapper.
                self.incoming_map.insert(
                    self.incoming.add(upstream.rx().unwrap())?,
                    Incoming::Upstream,
                );
                self.upstream = upstream.tx();
            } else {
                // Send reply to simulator.
                self.send(OutgoingMessage::Simulator(PluginToSimulator::Initialized(
                    PluginInitializeResponse {
                        upstream: None,
                        metadata,
                    },
                )))?;
            }
            Ok(())
        } else {
            Err(ErrorKind::InvalidOperation(
                "Unexpected request, expected an Initialize request".to_string(),
            ))?
        } {
            Ok(()) => Ok(self),
            Err(err) => {
                let err: String = err;
                self.send(OutgoingMessage::Simulator(PluginToSimulator::Failure(
                    err.clone(),
                )))?;
                Err(ErrorKind::InvalidOperation(format!(
                    "Initialization failed: {}",
                    err
                )))?
            }
        }
    }

    /// Get downstream channel.
    ///
    /// Returns an error if the downstream channel does not exist.
    fn downstream_ref(&self) -> Result<&DownstreamChannel> {
        Ok(self
            .downstream
            .as_ref()
            .ok_or_else(|| ErrorKind::IPCError("Downstream channel does not exist".to_string()))?)
    }

    /// Get sender of upstream channel.
    ///
    /// Returns an error if the upstream channel sender side does not exist.
    fn upstream_ref(&self) -> Result<&IpcSender<GatestreamUp>> {
        Ok(self
            .upstream
            .as_ref()
            .ok_or_else(|| ErrorKind::IPCError("Upstream channel does not exist".to_string()))?)
    }

    /// Send an OutgoingMessage.
    ///
    /// Send an OutgoingMessage using the corresponding sender. Returns an
    /// error when the channel is closed, does not exist or when sending
    /// failed.
    pub fn send(&self, message: OutgoingMessage) -> Result<()> {
        match message {
            OutgoingMessage::Simulator(response) => self.response.send(response)?,
            OutgoingMessage::Downstream(request) => {
                self.downstream_ref()?.tx_ref()?.send(request)?
            }
            OutgoingMessage::Upstream(response) => self.upstream_ref()?.send(response)?,
        }
        Ok(())
    }

    /// Fetch next request from either the Simulator request channel or the
    /// upstream Plugin request channel.
    /// Fails if either connection closed or if any connection is unexpectedly
    /// already closed. Returns Ok(None) if both request channels are closed.
    /// This method blocks until a new request is available.
    pub fn next_request(&mut self) -> Result<Option<IncomingMessage>> {
        // First drain the buffer.
        if !self.incoming_buffer.is_empty() {
            Ok(Some(self.incoming_buffer.remove(0)))
        } else {
            // Check if all channels are closed.
            let selection = self.incoming.select()?;
            if selection.is_empty() {
                Ok(None)
            } else {
                // Store incoming message in the buffer.
                for event in selection {
                    match event {
                        IpcSelectionResult::MessageReceived(id, msg) => {
                            if let Some(incoming) = self.incoming_map.get(&id) {
                                self.incoming_buffer.push(match incoming {
                                    Incoming::Simulator => IncomingMessage::Simulator(msg.to()?),
                                    Incoming::Upstream => IncomingMessage::Upstream(msg.to()?),
                                    _ => unreachable!(),
                                });
                            }
                        }
                        IpcSelectionResult::ChannelClosed(id) => {
                            trace!("Channel closed: {:?}", self.incoming_map.get(&id));
                        }
                    }
                }
                // Recurse.
                self.next_request()
            }
        }
    }

    /// Fetch next response from downstream plugin.
    /// This method blocks until a new response is available. Please refer to
    /// `try_next_response` for a non-blocking method.
    pub fn next_response(&self) -> Result<IncomingMessage> {
        Ok(IncomingMessage::Downstream(
            self.downstream_ref()?.rx_ref()?.recv()?,
        ))
    }

    /// Attempt to fetch next response from downstream plugin.
    /// This method is non-blocking.
    pub fn try_next_response(&self) -> Result<IncomingMessage> {
        Ok(IncomingMessage::Downstream(
            self.downstream_ref()?.rx_ref()?.try_recv()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{Connection, IncomingMessage, OutgoingMessage};
    use crate::{
        common::protocol::{PluginToSimulator, SimulatorToPlugin},
        host::ipc::SimulatorChannel,
    };
    use ipc_channel::ipc::IpcOneShotServer;

    #[test]
    fn connect() {
        // Main thread runs the 'Simulator'.
        let (server, server_name) = IpcOneShotServer::new().unwrap();

        // 'Plugin' runs in a thread.
        let plugin = std::thread::spawn(move || {
            // Get the PluginChannel
            let channel = Connection::connect(server_name).unwrap();
            assert!(channel.log.is_some());

            // Wait for a request.
            let req = channel.request.recv();
            assert!(req.is_ok());
            assert_eq!(req.unwrap(), SimulatorToPlugin::Abort);

            // Send a response.
            let res = channel.response.send(PluginToSimulator::Success);
            assert!(res.is_ok());
        });

        // Simulator gets the SimulatorChannel.
        let (_, mut channel): (_, SimulatorChannel) = server.accept().unwrap();

        // Take out the log channel.
        assert!(channel.log().is_some());
        assert!(channel.log().is_none());

        // Send a request.
        let req = channel.request.send(SimulatorToPlugin::Abort);
        assert!(req.is_ok());

        // Get a response.
        let res = channel.response.recv();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), PluginToSimulator::Success);

        assert!(plugin.join().is_ok());
    }

    #[test]
    fn simulator_connection() {
        // Main thread runs the 'Simulator'.
        let (server, server_name) = IpcOneShotServer::new().unwrap();

        // The 'Plugin' runs in a thread.
        let plugin = std::thread::spawn(move || {
            // Construct the Connection wrapper.
            let mut connection = Connection::new(server_name).unwrap();

            // Wait for a request.
            let req = connection.next_request();
            assert!(req.is_ok());
            assert_eq!(
                req.unwrap().unwrap(),
                IncomingMessage::Simulator(SimulatorToPlugin::Abort)
            );

            // Send a response.
            let res = connection.send(OutgoingMessage::Simulator(PluginToSimulator::Success));
            assert!(res.is_ok());
        });

        // Simulator gets the SimulatorChannel.
        let (_, channel): (_, SimulatorChannel) = server.accept().unwrap();

        // Send a request.
        let req = channel.request.send(SimulatorToPlugin::Abort);
        assert!(req.is_ok());

        // Get a response.
        let res = channel.response.recv();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), PluginToSimulator::Success);

        assert!(plugin.join().is_ok());
    }
}
