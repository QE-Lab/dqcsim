use crate::{
    error::{err, Result},
    ipc::SimulatorChannel,
    trace,
};
use ipc_channel::ipc::IpcOneShotServer;
use std::{
    process::{Child, Command, Stdio},
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

/// IPC connect timeout in seconds.
///
/// Wait at most this Duration for the child process to connect to the channel.
const IPC_CONNECT_TIMEOUT_SECS: Duration = Duration::from_secs(5);

/// Start a [`Plugin`] child process and initialize the communication channel.
///
/// Returns a (`Child`, [`SimulatorChannel`]) pair if the plugin started and
/// the channel was setup succesfully.
///
/// The optional connect timeout argument can be used to override the default timeout
/// duration of 5 seconds.
///
/// [`Plugin`]: ../plugin/struct.Plugin.html
/// [`SimulatorChannel`]: ../struct.SimulatorChannel.html
pub fn start(
    command: &mut Command,
    connect_timeout: Option<Duration>,
) -> Result<(Child, SimulatorChannel)> {
    // Setup channel
    let (server, server_name) = IpcOneShotServer::new()?;

    // Spawn child process
    let child = command
        .arg(server_name)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Make sure child connects
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::mutex_atomic))]
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();
    let handle: thread::JoinHandle<Result<SimulatorChannel>> = thread::spawn(move || {
        {
            let &(ref lock, _) = &*pair2;
            let mut started = lock.lock().expect("Unable to aquire lock");
            *started = true;
        }
        // Wait for the child to connect and get the channel.
        let (_, channel): (_, SimulatorChannel) = server.accept()?;
        {
            let &(_, ref cvar) = &*pair2;
            cvar.notify_one();
        }
        Ok(channel)
    });
    let &(ref lock, ref cvar) = &*pair;
    trace!("Waiting for plugin to connect.");
    let (started, wait_result) = cvar
        .wait_timeout(
            lock.lock()
                .expect("Plugin IPC connection start lock poisoned"),
            connect_timeout.unwrap_or(IPC_CONNECT_TIMEOUT_SECS),
        )
        .expect("Plugin IPC connection start lock poisoned");
    if *started && !wait_result.timed_out() {
        match handle.join() {
            Ok(channel) => {
                let channel = channel?;
                trace!("Plugin started and connected.");
                Ok((child, channel))
            }
            Err(_) => err("plugin IPC connection start thread failed")?,
        }
    } else {
        err("plugin did not connect within specified timeout")?
    }
}

#[cfg(test)]
mod tests {
    use super::start;
    use std::{process::Command, time::Duration};

    #[test]
    fn timeout() {
        let command = "/bin/sh";
        let timeout = start(
            Command::new(command).arg("sleep").arg("1"),
            Some(Duration::from_nanos(1u64)),
        );
        assert!(timeout.is_err());
        let err = timeout.err().unwrap();
        assert_eq!(
            format!("{}", err),
            "Error: plugin did not connect within specified timeout"
        );
    }

}
