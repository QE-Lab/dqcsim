use crate::{
    protocol::message::{Control, Log, Reply},
    util::log::{init, set_thread_logger, LogProxy, LogThread, Record},
};
use crossbeam_channel::Sender;
use failure::{Error, Fail};
use ipc_channel::{
    ipc,
    ipc::{IpcOneShotServer, IpcReceiver, IpcSender},
    router::ROUTER,
};
use serde::{Deserialize, Serialize};
use std::{
    io,
    process::{Child, Command},
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

#[derive(Debug, Fail)]
pub enum ChannelError {
    #[fail(display = "Ipc channel failed.")]
    IpcError(#[fail(cause)] ipc_channel::Error),
    #[fail(display = "I/O error.")]
    IoError(#[fail(cause)] io::Error),
    #[fail(display = "Ipc channel timeout")]
    Timeout,
}

#[derive(Serialize, Deserialize)]
pub struct SimulatorChannel {
    log: Option<IpcReceiver<Record>>,
    control: Option<IpcSender<Control>>,
    reply: Option<IpcReceiver<Reply>>,
}

#[derive(Serialize, Deserialize)]
pub struct PluginChannel {
    log: Option<IpcSender<Record>>,
    control: Option<IpcReceiver<Control>>,
    reply: Option<IpcSender<Reply>>,
}

impl SimulatorChannel {
    pub fn new(
        log: IpcReceiver<Record>,
        control: IpcSender<Control>,
        reply: IpcReceiver<Reply>,
    ) -> SimulatorChannel {
        SimulatorChannel {
            log: Some(log),
            control: Some(control),
            reply: Some(reply),
        }
    }
}

impl PluginChannel {
    pub fn new(
        log: IpcSender<Record>,
        control: IpcReceiver<Control>,
        reply: IpcSender<Reply>,
    ) -> PluginChannel {
        PluginChannel {
            log: Some(log),
            control: Some(control),
            reply: Some(reply),
        }
    }
}

/// This function initializes the channel between simulator and plugin.
///
/// Normally this function is used in the plugin wrapper.
///
/// The server argument is provided by the simulator instance.
pub fn connect(
    server: impl Into<String>,
    level: Option<log::LevelFilter>,
) -> Result<PluginChannel, Error> {
    let connect = IpcSender::connect(server.into())?;
    let (simulator, mut plugin) = channel()?;
    connect.send(simulator)?;
    // Initialize thread local logger.
    init(level).expect("Unable to set thread local logger.");

    // Setup log proxy.
    let log_sender = plugin.log.take().unwrap();
    set_thread_logger(LogProxy::boxed(log_sender, level));

    Ok(plugin)
}

/// IPC connect timeout in seconds.
///
/// The PluginThread waits at most this amount in seconds for the child process
/// to connect to the IPC channel.
const IPC_CONNECT_TIMEOUT_SECS: u64 = 5;

pub fn setup(
    command: Command,
    sender: Sender<Record>,
    ipc_connect_timeout: Option<Duration>,
) -> Result<(Child, SimulatorChannel), Error> {
    // Setup channel
    let (server, server_name) = IpcOneShotServer::new()?;

    // Spawn child process
    let mut child = Command::from(command).arg(server_name).spawn()?;

    // Make sure child connects within timeout
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
        let mut channel = handle.join().expect("Connection thread failed");
        let log_receiver = channel.log.take().unwrap();
        ROUTER.route_ipc_receiver_to_crossbeam_sender(log_receiver, sender);
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
