use crate::{ipc::message, plugin::config::PluginConfig};
use crossbeam_channel::{Receiver, Sender};
use dqcsim_log::Record;
use dqcsim_log::{LogProxy, LogThread};
use ipc_channel::{
    ipc::{IpcOneShotServer, IpcReceiver},
    router::ROUTER,
};
use log::{error, info, trace};
use std::{
    error::Error,
    process::{Child, Command, Stdio},
    sync::{Arc, Condvar, Mutex},
    thread::{Builder, JoinHandle},
    time::Duration,
};

/// The Plugin configuration.
pub mod config;

/// The Plugin structure used in a Simulator
pub struct Plugin {
    /// The configuration.
    config: PluginConfig,
    /// The thread handler.
    handler: Option<JoinHandle<()>>,
    /// The sender part of the control channel.
    controller: crossbeam_channel::Sender<message::DQCsimToPlugin>,
    /// IPC connect timeout
    ipc_connect_timeout: Option<Duration>,
}

const IPC_CONNECT_TIMEOUT_SECS: u64 = 5;

/// The plugin thread control function.
impl Plugin {
    pub fn new(
        config: PluginConfig,
        logger: &LogThread,
        ipc_connect_timeout: Option<Duration>,
    ) -> Plugin {
        // Create a channel to control the plugin thread.
        let (controller, rx): (
            Sender<message::DQCsimToPlugin>,
            Receiver<message::DQCsimToPlugin>,
        ) = crossbeam_channel::unbounded();

        // Spawn thread for the plugin.
        let name = config.name.clone();
        let loglevel = config.loglevel;
        let sender = logger
            .get_sender()
            .expect("Unable to get sender side of log channel.");

        let handler = Builder::new()
            .name(config.name.to_owned())
            .spawn(move || {
                dqcsim_log::set_thread_logger(Box::new(LogProxy::new(sender.clone(), loglevel)));
                info!(
                    "[{}] Plugin running in thread: {:?}",
                    &name,
                    std::thread::current().id()
                );

                // Setup child container
                let mut child: Option<Child> = None;

                loop {
                    match rx.recv() {
                        Ok(msg) => match msg.command {
                            message::D2Punion::D2Pinit(ref _init) => {
                                info!("start");

                                // Setup control channel
                                let (server, server_name): (
                                    IpcOneShotServer<IpcReceiver<Record>>,
                                    String,
                                ) = IpcOneShotServer::new().unwrap();

                                trace!("Server for {}: {}", &name, &server_name);

                                child = Some(
                                    Command::new("target/debug/dqcsim-plugin")
                                        .stderr(Stdio::piped())
                                        .stdout(Stdio::piped())
                                        .arg(&server_name)
                                        .spawn()
                                        .expect("Failed to start echo process"),
                                );

                                let pair = Arc::new((Mutex::new(false), Condvar::new()));
                                let pair2 = pair.clone();

                                // Setup connection in separate thread as
                                // ipc-channel setup has no support for timeouts.
                                let handle: JoinHandle<IpcReceiver<Record>> =
                                    std::thread::spawn(move || {
                                        {
                                            let (ref lock, _) = *pair2;
                                            let mut started = lock.lock().unwrap();
                                            *started = true;
                                            log::debug!("connect thread started");
                                        }
                                        // Wait for child process to connect and get the receiver.
                                        let (_, receiver): (_, IpcReceiver<Record>) =
                                            server.accept().expect("Unable to connect.");
                                        {
                                            let (_, ref cvar) = *pair2;
                                            log::debug!(
                                                "Connect thread finished; notifying waiting thread"
                                            );
                                            cvar.notify_one();
                                        }
                                        receiver
                                    });
                                let (ref lock, ref cvar) = *pair;
                                let timeout = ipc_connect_timeout
                                    .unwrap_or(Duration::from_secs(IPC_CONNECT_TIMEOUT_SECS));
                                log::debug!(
                                    "Waiting on connect thread for {} secs",
                                    timeout.as_secs()
                                );
                                let (started, wait_result) = cvar
                                    .wait_timeout(
                                        lock.lock().expect("IPC connection startup lock poisoned"),
                                        timeout,
                                    )
                                    .expect("IPC connection startup lock poisoned");

                                if *started && !wait_result.timed_out() {
                                    let receiver = handle.join().unwrap();
                                    // Forward log messages from child to log thread.
                                    ROUTER.route_ipc_receiver_to_crossbeam_sender(
                                        receiver,
                                        sender.clone(),
                                    );
                                } else {
                                    log::error!(
                                        "Timeout exceeded waiting for IPC connection (started: {})",
                                        *started
                                    );
                                }
                            }
                            message::D2Punion::D2Pfini(ref fini) => {
                                if fini.graceful {
                                    // Wait for child to finish
                                    trace!(
                                        "child stopped: {}",
                                        child.unwrap().wait().expect("child failed.")
                                    );
                                } else {
                                    trace!("Killing child process");
                                    child.unwrap().kill().expect("kill failed.");
                                }

                                // TODO: pipestream reader which dumps lines as they come in.
                                // https://gist.github.com/ArtemGr/db40ae04b431a95f2b78

                                // Dump stdout
                                // let mut stdout = String::new();
                                // child
                                //     .unwrap()
                                //     .stdout
                                //     .unwrap()
                                //     .read_to_string(&mut stdout)
                                //     .expect("stdout read failed.");
                                // info!("{}", stdout);
                                // let mut stderr = String::new();
                                // child
                                //     .unwrap()
                                //     .stderr
                                //     .unwrap()
                                //     .read_to_string(&mut stderr)
                                //     .expect("stderr read failed.");
                                // error!("{}", stderr);
                                break;
                            }
                            _ => break,
                        },
                        Err(x) => {
                            error!("{:?}", x.description());
                            break;
                        }
                    }
                }
                info!("Plugin thread stopping.");
            })
            .ok();

        Plugin {
            config,
            handler,
            controller,
            ipc_connect_timeout,
        }
    }
    /// Initialize the plugin.
    /// This starts the plugin thread, and initializes the control channel.
    pub fn init(&self) -> Result<(), ()> {
        trace!("Init plugin {}", self.config.name);
        self.controller
            .send(message::DQCsimToPlugin {
                command: message::D2Punion::D2Pinit(message::D2Pinit {
                    down_push_uri: "downPush".to_owned(),
                    down_pull_uri: "downPull".to_owned(),
                    arb_cmds: Vec::new(),
                    logger_prefix: self.config.name.to_owned(),
                    log_level: message::LogLevel::Critical,
                }),
            })
            .unwrap();
        Ok(())
    }
}

impl Drop for Plugin {
    fn drop(&mut self) {
        self.controller
            .send(message::DQCsimToPlugin {
                command: message::D2Punion::D2Pfini(message::D2Pfini { graceful: true }),
            })
            .unwrap();
        self.handler
            .take()
            .expect("Plugin failed.")
            .join()
            .expect("Plugin failed.");
    }
}
