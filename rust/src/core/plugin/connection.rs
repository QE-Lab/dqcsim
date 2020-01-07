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

// TODO(mb): add example usage or link to plugin implementation instructions

use crate::{
    common::{
        channel::{PluginChannel, UpstreamChannel},
        error::{inv_op, ErrorKind, Result},
        protocol::{GatestreamDown, GatestreamUp, PluginToSimulator, SimulatorToPlugin},
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
    /// Variant used to label incoming downstream responses ([`GatestreamUp`]).
    Downstream,
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

    /// Pending upstream connection.
    pending_upstream: Option<IpcOneShotServer<UpstreamChannel>>,

    /// Simulator response sender.
    response: IpcSender<PluginToSimulator>,

    /// Optional Upstream sender. Is None for Frontend plugins.
    upstream: Option<IpcSender<GatestreamUp>>,

    /// Optional Downstream sender. Is None for Backend plugins.
    downstream: Option<IpcSender<GatestreamDown>>,
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
        connect.send((request_tx, response_rx))?;

        // Return the PluginChannel.
        Ok((response, request))
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
        incoming_map.insert(incoming.add(channel.1)?, Incoming::Simulator);

        Ok(Connection {
            incoming,
            incoming_map,
            incoming_buffer: VecDeque::new(),
            response: channel.0,
            downstream: None,
            pending_upstream: None,
            upstream: None,
        })
    }

    /// Connects to a downstream plugin.
    pub fn connect_downstream(&mut self, downstream: impl Into<String>) -> Result<()> {
        if self.downstream.is_some() {
            inv_op("already connected to a downstream plugin")?;
        }

        // Attempt to connect to the downstream plugin.
        let downstream = IpcSender::connect(downstream.into())?;

        // Create channel pair.
        let (down_tx, down_rx) = ipc_channel::ipc::channel()?;
        let (up_tx, up_rx) = ipc_channel::ipc::channel()?;

        // Send upstream channel to downstream plugin.
        downstream.send((up_tx, down_rx) as UpstreamChannel)?;

        // Store downstream channel incoming and outgoing in connection
        // wrapper.
        self.incoming_map
            .insert(self.incoming.add(up_rx)?, Incoming::Downstream);
        self.downstream.replace(down_tx);

        Ok(())
    }

    /// Creates a one-shot server for an upstream plugin to connect to,
    /// returning the address. Call `accept_upstream()` to finish connecting.
    pub fn serve_upstream(&mut self) -> Result<String> {
        if self.pending_upstream.is_some() {
            inv_op("already connecting to an upstream plugin")?;
        } else if self.upstream.is_some() {
            inv_op("already connected to an upstream plugin")?;
        }
        let (pending, address) = IpcOneShotServer::new()?;
        self.pending_upstream.replace(pending);
        Ok(address)
    }

    /// Waits for an upstream plugin to connect to our one-shot server and
    /// establishes the connection. Call `serve_upstream()` first.
    pub fn accept_upstream(&mut self) -> Result<()> {
        if self.pending_upstream.is_none() {
            inv_op("not yet connecting to an upstream plugin, call serve_upstream() first")?;
        } else if self.upstream.is_some() {
            inv_op("already connected to an upstream plugin")?;
        }

        // Wait for upstream plugin to connect.
        let (_, upstream): (_, UpstreamChannel) = self.pending_upstream.take().unwrap().accept()?;

        // Store upstream channel incoming and outgoing in connection
        // wrapper.
        self.incoming_map
            .insert(self.incoming.add(upstream.1)?, Incoming::Upstream);
        self.upstream.replace(upstream.0);

        Ok(())
    }

    /// Get downstream channel.
    ///
    /// Returns an error if the downstream sender side does not exist.
    fn downstream_ref(&self) -> Result<&IpcSender<GatestreamDown>> {
        Ok(self
            .downstream
            .as_ref()
            .ok_or_else(|| ErrorKind::IPCError("Downstream sender does not exist".to_string()))?)
    }

    /// Get sender of upstream channel.
    ///
    /// Returns an error if the upstream channel sender side does not exist.
    fn upstream_ref(&self) -> Result<&IpcSender<GatestreamUp>> {
        Ok(self
            .upstream
            .as_ref()
            .ok_or_else(|| ErrorKind::IPCError("Upstream sender does not exist".to_string()))?)
    }

    /// Send an OutgoingMessage.
    ///
    /// Send an OutgoingMessage using the corresponding sender. Returns an
    /// error when the channel is closed, does not exist or when sending
    /// failed.
    pub fn send(&self, message: OutgoingMessage) -> Result<()> {
        match message {
            OutgoingMessage::Simulator(response) => self.response.send(response)?,
            OutgoingMessage::Downstream(request) => self.downstream_ref()?.send(request)?,
            OutgoingMessage::Upstream(response) => self.upstream_ref()?.send(response)?,
        }
        Ok(())
    }

    /// Buffer incoming messages.
    /// If there are connected channels make sure at least one additional
    /// message is pending in the buffer.
    fn buffer_incoming(&mut self) -> Result<()> {
        let mut received_any = false;
        while !received_any && !self.incoming_map.is_empty() {
            // Store incoming message in the buffer.
            for event in self.incoming.select()? {
                match event {
                    IpcSelectionResult::MessageReceived(id, msg) => {
                        if let Some(incoming) = self.incoming_map.get(&id) {
                            self.incoming_buffer.push_back(match incoming {
                                Incoming::Simulator => IncomingMessage::Simulator(msg.to()?),
                                Incoming::Upstream => IncomingMessage::Upstream(msg.to()?),
                                Incoming::Downstream => IncomingMessage::Downstream(msg.to()?),
                            });
                            received_any = true;
                        }
                    }
                    IpcSelectionResult::ChannelClosed(id) => {
                        trace!("Channel closed: {:?}", self.incoming_map.get(&id));

                        // Remove channel from incoming map
                        self.incoming_map.remove(&id);
                    }
                }
            }
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
        if self.incoming_buffer.is_empty() {
            // Make sure at least one additional message is availale.
            self.buffer_incoming()?;
        }
        Ok(self.incoming_buffer.pop_front())
    }

    /// Fetch next downstream request.
    ///
    /// Fails if the downstream connection is closed. This method blocks until
    /// a new request is available.
    pub fn next_downstream_request(&mut self) -> Result<Option<IncomingMessage>> {
        // Check if there are new downstream messages.
        if let Some(idx) = self.incoming_buffer.iter().position(|msg| match msg {
            IncomingMessage::Downstream(_) => true,
            _ => false,
        }) {
            Ok(Some(self.incoming_buffer.remove(idx).unwrap()))
        } else {
            // Buffer incoming messages.
            self.buffer_incoming()?;
            // If there are no connected channels return None.
            if self.incoming_map.is_empty() {
                Ok(None)
            } else {
                // Check if the new messages contain a downstream message.
                self.next_downstream_request()
            }
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        trace!("Dropping Connection");
    }
}

#[cfg(test)]
mod tests {
    use super::{Connection, IncomingMessage, OutgoingMessage};
    use crate::common::{
        channel::SimulatorChannel,
        protocol::{PluginToSimulator, SimulatorToPlugin},
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
            let req = channel.1.recv();
            assert!(req.is_ok());
            assert_eq!(req.unwrap(), SimulatorToPlugin::Abort);

            // Send a response.
            let res = channel.0.send(PluginToSimulator::Success);
            assert!(res.is_ok());
        });

        // Simulator gets the SimulatorChannel.
        let (_, channel): (_, SimulatorChannel) = server.accept().unwrap();

        // Send a request.
        let req = channel.0.send(SimulatorToPlugin::Abort);
        assert!(req.is_ok());

        // Get a response.
        let res = channel.1.recv();
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
        let req = channel.0.send(SimulatorToPlugin::Abort);
        assert!(req.is_ok());

        // Get a response.
        let res = channel.1.recv();
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
        #[cfg(target_os = "linux")]
        assert_eq!(
            connection.err().unwrap().to_string(),
            String::from("I/O error: No such file or directory (os error 2)")
        );
    }
}
