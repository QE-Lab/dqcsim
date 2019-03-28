//! Plugin to simulator connection wrapper.
//!
//! The `connection` module provides a [`Connection`] wrapper, which is used by
//! [`Plugin`] instances to wrap connection structures required for
//! communication between [`Plugin`] and [`Simulator`], and between [`Plugin`]
//! and [`Plugin`].
//!
//! This module defines two wrapper enumerations, which wrap around
//! [`protocol`] messages for both incoming ([`IncomingMessage`]) and outgoing
//! ([`OutgoingMessage`]) channels.
//!
//! [`Connection`]: ./struct.Connection.html
//! [`IncomingMessage`]: ./enum.IncomingMessage.html
//! [`OutgoingMessage`]: ./enum.OutgoingMessage.html
//! [`Plugin`]: ../../host/plugin/struct.Plugin.html
//! [`Simulator`]: ../../host/simulator/struct.Simulator.html
//! [`protocol`]: ../../common/protocol/index.html

// TODO: add example usage or link to plugin implementation instructions

use crate::{
    common::{
        error::{ErrorKind, Result},
        protocol::{
            GatestreamDown, GatestreamUp, PluginInitializeResponse, PluginToSimulator,
            SimulatorToPlugin,
        },
        types::{ArbCmd, PluginMetadata},
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
use std::collections::{HashMap, VecDeque};

/// Incoming enum used to map incoming requests in the IpcReceiverSet used in
/// the Connection wrapper.
#[derive(Debug, Copy, Clone)]
enum Incoming {
    /// Variant used to label incoming request ([`SimulatorToPlugin`]).
    Simulator,
    /// Variant used to label incoming upstream requests ([`GatestreamDown`]).
    Upstream,
}

/// Incoming messages variants.
///
/// The different variants contain the actual message. This structure is used
/// by Plugins to determine the origin of an incoming message.
#[derive(Debug, PartialEq)]
pub enum IncomingMessage {
    Simulator(SimulatorToPlugin),
    Upstream(GatestreamDown),
    Downstream(GatestreamUp),
}

/// Outgoing messages variants.
///
/// The different variants contain the actual message. This structure is used
/// by Connection instances to make it easy for Plugins to target their
/// outgoing messages.
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
/// Constructing a Connection instance should be the first thing a [`Plugin`]
/// does. The [`Simulator`] server address string is passed as an argument to
/// all Plugins started by a [`Simulator`]. The server address string can be
/// used to construct a Connection instance.
///
/// After constructin of the Connection instance, the [`Plugin`] can respond to
/// the initialization request from the [`Simulator`].
///
/// [`Plugin`]: ../../host/plugin/struct.Plugin.html
/// [`Simulator`]: ../../host/simulator/struct.Simulator.html
pub struct Connection {
    /// Set of incoming request channels.
    /// Simulator request channel for all plugins. Incoming requests from
    /// upstream plugins for operator and backend plugins.
    incoming: IpcReceiverSet,
    /// Map to label incoming requests to their channel.
    incoming_map: HashMap<u64, Incoming>,
    /// Buffer for incoming requests.
    incoming_buffer: VecDeque<IncomingMessage>,

    /// Optional Downstream channel. Is None for Backend plugins.
    downstream: Option<DownstreamChannel>,

    /// Simulator response sender.
    response: IpcSender<PluginToSimulator>,
    /// Optional Upstream sender. Is None for Frontend plugins.
    upstream: Option<IpcSender<GatestreamUp>>,
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
        let (request_tx, request) = ipc_channel::ipc::channel()?;
        let (response, response_rx) = ipc_channel::ipc::channel()?;

        // Send channel to the simulator.
        connect.send(SimulatorChannel::new(request_tx, response_rx))?;

        // Return the PluginChannel.
        Ok(PluginChannel::new(request, response))
    }

    /// Construct a Connection wrapper instance.
    ///
    /// The Connection wrapper attempts to connect to the [`Simulator`] using
    /// the address provided as argument. The required communication channels
    /// are generated and exchanged with the [`Simulator`].
    ///
    /// At this point the Connection wrapper can receive requests and send
    /// responses from and to the [`Simulator`], however logging and upstream
    /// and downstream plugin connections are not yet available.
    ///
    /// The first request the [`Simulator`] sends is always an initialization
    /// request, which should be handled with the [`init`] method.
    ///
    /// [`Simulator`]: ../../host/simulator/struct.Simulator.html
    /// [`init`]: ./struct.Connection.html#method.initc
    pub fn new(simulator: impl Into<String>) -> Result<Connection> {
        // Attempt to connect to the simulator instance.
        let channel = Connection::connect(simulator)?;

        // Create incoming request collections.
        let mut incoming = IpcReceiverSet::new()?;
        let mut incoming_map = HashMap::with_capacity(2);

        // Put incoming (SimulatorToPlugin) channel in receiver set and map.
        incoming_map.insert(incoming.add(channel.request)?, Incoming::Simulator);

        Ok(Connection {
            incoming,
            incoming_map,
            incoming_buffer: VecDeque::new(),
            response: channel.response,
            downstream: None,
            upstream: None,
        })
    }

    /// Handle the initialization request from the Simulator.
    ///
    /// Respond accordingly to the initialization request
    /// ([`SimulatorToPlugin::Initialize`]) from the [`Simulator`].
    ///
    /// [`SimulatorToPlugin::Initialize`]: ../../common/protocol/enum.SimulatorToPlugin.html#variant.Initialize
    /// [`Simulator`]: ../../host/simulator/struct.Simulator.html
    pub fn init(
        mut self,
        typ: PluginType,
        metadata: impl Into<PluginMetadata>,
        initialize: Box<dyn Fn(&mut PluginContext, Vec<ArbCmd>)>,
    ) -> Result<Connection> {
        // Wait for the initialization request from the Simulator.
        match if let Ok(Some(IncomingMessage::Simulator(SimulatorToPlugin::Initialize(req)))) =
            self.next_request()
        {
            // Setup logging.
            setup_logging(&req.configuration, req.log)?;

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
                        metadata: metadata.into(),
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
                        metadata: metadata.into(),
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
    ///
    /// Fails if either connection closed or if any connection is unexpectedly
    /// already closed. Returns Ok(None) if both request channels are closed.
    /// This method blocks until a new request is available.
    pub fn next_request(&mut self) -> Result<Option<IncomingMessage>> {
        // Fetch new stuff if buffer is empty.
        if self.incoming_buffer.is_empty() {
            // Store incoming message in the buffer.
            for event in self.incoming.select()? {
                match event {
                    IpcSelectionResult::MessageReceived(id, msg) => {
                        if let Some(incoming) = self.incoming_map.get(&id) {
                            self.incoming_buffer.push_back(match incoming {
                                Incoming::Simulator => IncomingMessage::Simulator(msg.to()?),
                                Incoming::Upstream => IncomingMessage::Upstream(msg.to()?),
                            });
                        }
                    }
                    IpcSelectionResult::ChannelClosed(id) => {
                        trace!("Channel closed: {:?}", self.incoming_map.get(&id));
                    }
                }
            }
        }
        Ok(self.incoming_buffer.pop_front())
    }

    /// Fetch next response from downstream plugin.
    ///
    /// This method blocks until a new response is available, and returns the
    /// message.
    pub fn next_response(&self) -> Result<IncomingMessage> {
        Ok(IncomingMessage::Downstream(
            self.downstream_ref()?.rx_ref()?.recv()?,
        ))
    }

    /// Attempt to fetch next response from downstream plugin.
    ///
    /// This method checks and returns a response, if it is available. This
    /// method does not block, and returns if no responses are available.
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

            // Wait for a request.
            let req = channel.request.recv();
            assert!(req.is_ok());
            assert_eq!(req.unwrap(), SimulatorToPlugin::Abort);

            // Send a response.
            let res = channel.response.send(PluginToSimulator::Success);
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

    #[test]
    fn bad_address() {
        // Attempt to connect to an non-existing server
        let connection = Connection::new("asdf");
        assert!(connection.is_err());

        #[cfg(target_os = "macos")]
        assert_eq!(
            connection.err().unwrap().to_string(),
            String::from("I/O error: Unknown Mach error: 44e")
        );
    }
}
