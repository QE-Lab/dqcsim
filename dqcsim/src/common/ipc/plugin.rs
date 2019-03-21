use crate::{
    common::{
        error::{ErrorKind, Result},
        ipc::{DownstreamChannel, PluginChannel, SimulatorChannel, UpstreamChannel},
        protocol::{InitializeResponse, Request, Response},
    },
    host::configuration::PluginType,
};

use ipc_channel::ipc::{IpcOneShotServer, IpcSender};

/// Construct a ([`SimulatorChannel`], [`PluginChannel`]) channel pair.
///
/// [`PluginChannel`]: ../struct.PluginChannel.html
/// [`SimulatorChannel`]: ../struct.SimulatorChannel.html
fn channel() -> Result<(SimulatorChannel, PluginChannel)> {
    let (log, log_rx) = ipc_channel::ipc::channel()?;
    let (request_tx, request) = ipc_channel::ipc::channel()?;
    let (response, response_rx) = ipc_channel::ipc::channel()?;
    Ok((
        SimulatorChannel::new(log_rx, request_tx, response_rx),
        PluginChannel {
            log,
            request,
            response,
        },
    ))
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
    let (simulator, plugin) = channel()?;
    connect.send(simulator)?;
    Ok(plugin)
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
    connect.send(UpstreamChannel {
        rx: down_rx,
        tx: up_tx,
    })?;
    Ok(DownstreamChannel {
        tx: down_tx,
        rx: up_rx,
    })
}

pub fn initialize(
    channel: &PluginChannel,
    plugin_type: PluginType,
) -> Result<(Option<DownstreamChannel>, Option<UpstreamChannel>)> {
    let init = channel.request.recv()?;
    let mut downstream_channel = None;
    let mut upstream_channel = None;
    match init {
        Request::Init(request) => {
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

                    channel.response.send(Response::Init(InitializeResponse {
                        upstream: Some(upstream),
                    }))?;

                    let (_, sender): (_, UpstreamChannel) = upstream_server.accept().unwrap();
                    upstream_channel = Some(sender);
                }
                PluginType::Frontend => {
                    channel
                        .response
                        .send(Response::Init(InitializeResponse { upstream: None }))?;
                }
            }

            Ok((downstream_channel, upstream_channel))
        }
        _ => Err(ErrorKind::Other("Handshake problem".to_string()))?,
    }
}

#[cfg(test)]
mod tests {
    use super::connect_simulator as connect_simulator_;
    use crate::common::ipc::SimulatorChannel;
    use ipc_channel::ipc::IpcOneShotServer;

    #[test]
    fn connect_simulator() {
        let (server, server_name): (IpcOneShotServer<SimulatorChannel>, _) =
            IpcOneShotServer::new().unwrap();

        let plugin = std::thread::spawn(move || {
            let connect = connect_simulator_(server_name);
            assert!(connect.is_ok());
        });

        assert!(server.accept().is_ok());
        assert!(plugin.join().is_ok());
    }

}
