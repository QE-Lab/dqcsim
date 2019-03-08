use crate::protocol::message::Signal;
use failure::Error;
use signal_hook::iterator::Signals;
use std::{os::raw::c_int, thread};

/// Setup a signal hook.
///
/// Takes a reference to an array of signals and registers a signal hook for
/// these signals.
/// Returns a Receiver channel which receives the registered signals.
pub fn notify(signals: &[c_int]) -> Result<crossbeam_channel::Receiver<Signal>, Error> {
    let (tx, rx) = crossbeam_channel::bounded(100);
    let signals = Signals::new(signals)?;
    thread::spawn(move || {
        for signal in signals.forever() {
            match signal {
                signal_hook::SIGTERM => tx.send(Signal::SIGTERM),
                signal_hook::SIGINT => tx.send(Signal::SIGINT),
                signal_hook::SIGQUIT => tx.send(Signal::SIGQUIT),
                signal_hook::SIGKILL => tx.send(Signal::SIGKILL),
                _ => tx.send(Signal::Other(signal)),
            }
            .expect("Unable to forward signal");
        }
    });
    log::trace!("Signal hook running");
    Ok(rx)
}
