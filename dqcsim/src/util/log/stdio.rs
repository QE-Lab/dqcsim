use crate::util::log::{set_thread_logger, LogProxy, Record};
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
pub fn stdio_to_log(
    mut stream: Box<Read + Send>,
    sender: Sender<Record>,
    level: log::Level,
) -> JoinHandle<()> {
    spawn(move || {
        set_thread_logger(LogProxy::boxed(sender, Some(log::LevelFilter::Trace)));
        let mut buf = Vec::new();
        let mut byte = [0u8];
        loop {
            match stream.read(&mut byte) {
                Ok(0) => {
                    log::trace!("EOF: closing stdio forwarding channel");
                    break;
                }
                Ok(_) => {
                    if byte[0] == 0x0A {
                        match String::from_utf8(buf.clone()) {
                            Ok(line) => log::log!(level, "{}", line),
                            Err(err) => log::error!("{}", err),
                        }
                        buf.clear()
                    } else {
                        buf.push(byte[0])
                    }
                }
                Err(error) => {
                    log::error!("{}", error);
                }
            }
        }
    })
}
