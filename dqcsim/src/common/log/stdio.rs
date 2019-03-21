//! Utility function to spawn a log proxy implementation to forward standard i/o streams.

use crate::{
    common::log::{init, proxy::LogProxy, Loglevel, LoglevelFilter, Record},
    error, log, trace,
};
use crossbeam_channel::Sender;
use std::{
    io::Read,
    thread::{spawn, JoinHandle},
};

/// Forward standard i/o to log channel.
///
/// Spawns a thread which takes a readable stream and forwards lines as log
/// records to the log thread until it matches EOF. The log record level is
/// set to the level argument of the function.
///
/// Returns a thread::JoinHandle to the spawned thread.
pub fn proxy_stdio(
    name: impl Into<String>,
    mut stream: Box<Read + Send>,
    sender: Sender<Record>,
    level: Loglevel,
) -> JoinHandle<()> {
    let name = name.into();
    spawn(move || {
        init(LogProxy::boxed(name, LoglevelFilter::from(level), sender)).unwrap();
        let mut buf = Vec::new();
        let mut byte = [0u8];
        loop {
            match stream.read(&mut byte) {
                Ok(0) => {
                    trace!("EOF: closing stdio forwarding channel");
                    break;
                }
                Ok(_) => {
                    if byte[0] == 0x0A {
                        match String::from_utf8(buf.clone()) {
                            Ok(line) => log!(level, "{}", line),
                            Err(err) => error!("{}", err),
                        }
                        buf.clear()
                    } else {
                        buf.push(byte[0])
                    }
                }
                Err(error) => {
                    error!("{}", error);
                }
            }
        }
    })
}
