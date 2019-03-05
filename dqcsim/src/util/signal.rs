use crate::protocol::message::Signal;
use signal_hook::iterator::Signals;
use std::{error::Error, os::raw::c_int, thread};

/// Setup a signal hook for the provided signals.
/// Returns a Receiver channel which receives arrived signals.
pub fn notify(signals: &[c_int]) -> Result<crossbeam_channel::Receiver<Signal>, Box<dyn Error>> {
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
    log::trace!("Signal hook running.");
    Ok(rx)
}
