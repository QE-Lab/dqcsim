use crate::protocol::ipc::{ChannelError, PluginChannel, SimulatorChannel};
use failure::Error;
use ipc_channel::{
    ipc,
    ipc::{IpcOneShotServer, IpcSender},
};
use std::{
    process::{Child, Command, Stdio},
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

/// This function initializes the channel between simulator and plugin.
///
/// Normally this function is used in the plugin wrapper.
///
/// The server argument is provided by the simulator instance.
pub fn connect(server: impl Into<String>) -> Result<PluginChannel, Error> {
    let connect = IpcSender::connect(server.into())?;
    let (simulator, plugin) = channel()?;
    connect.send(simulator)?;
    Ok(plugin)
}

/// IPC connect timeout in seconds.
///
/// The PluginThread waits at most this amount in seconds for the child process
/// to connect to the IPC channel.
const IPC_CONNECT_TIMEOUT_SECS: u64 = 5;

pub fn setup(
    mut command: Command,
    ipc_connect_timeout: Option<Duration>,
) -> Result<(Child, SimulatorChannel), Error> {
    // Setup channel
    let (server, server_name) = IpcOneShotServer::new()?;

    // Spawn child process
    let child = command
        .arg(server_name)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Make sure child connects within timeout
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::mutex_atomic))]
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();
    let handle: thread::JoinHandle<SimulatorChannel> = thread::spawn(move || {
        {
            let &(ref lock, _) = &*pair2;
            let mut started = lock.lock().expect("Unable to aquire lock");
            *started = true;
        }
        // Wait for the child to connect and get the channel.
        let (_, channel): (_, SimulatorChannel) = server.accept().expect("Connection failed");
        {
            let &(_, ref cvar) = &*pair2;
            cvar.notify_one();
        }
        channel
    });
    let &(ref lock, ref cvar) = &*pair;
    let timeout = ipc_connect_timeout.unwrap_or(Duration::from_secs(IPC_CONNECT_TIMEOUT_SECS));
    let (started, wait_result) = cvar
        .wait_timeout(
            lock.lock().expect("IPC connection startup lock poisoned"),
            timeout,
        )
        .expect("IPC connection startup lock poisoned");
    if *started && !wait_result.timed_out() {
        let channel = handle.join().expect("Connection thread failed");
        log::trace!("connected.");
        Ok((child, channel))
    } else {
        log::trace!("timeout.");
        Err(ChannelError::Timeout.into())
    }
}

/// This function returns a (simulator, plugin) channel pair.
fn channel() -> Result<(SimulatorChannel, PluginChannel), Error> {
    let (log_tx, log_rx) = ipc::channel()?;
    let (control_tx, control_rx) = ipc::channel()?;
    let (reply_tx, reply_rx) = ipc::channel()?;
    Ok((
        SimulatorChannel::new(log_rx, control_tx, reply_rx),
        PluginChannel::new(log_tx, control_rx, reply_tx),
    ))
}
