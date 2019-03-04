use std::{os::raw::c_int, thread};

pub struct Simulation {}

fn notify(signals: &[c_int]) -> Result<crossbeam_channel::Receiver<c_int>, ()> {
    let (s, r) = crossbeam_channel::bounded(100);
    let signals = signal_hook::iterator::Signals::new(signals).unwrap();
    thread::spawn(move || {
        for signal in signals.forever() {
            s.send(signal).expect("Unable to send signal");
        }
    });
    log::error!("Send a signal to continue.");
    Ok(r)
}

impl Simulation {
    pub fn new() -> Simulation {
        let receiver = notify(&[
            signal_hook::SIGHUP,
            signal_hook::SIGTERM,
            signal_hook::SIGINT,
            signal_hook::SIGQUIT,
        ])
        .unwrap()
        .recv()
        .unwrap();
        dbg!(receiver);
        Simulation {}
    }
}
