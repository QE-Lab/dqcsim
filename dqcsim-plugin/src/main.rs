use dqcsim::util::log::{init, set_thread_logger, LogProxy};
use failure::Error;
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let server = args[1].as_ref();

    // Some delay. Remove later for performance improvement.
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Connect to simulator. Get PluginChannel.
    let mut channel = dqcsim::protocol::channel::connect(server)?;

    // Initialize thread local logger.
    let level = Some(log::LevelFilter::Trace);
    init(level).expect("Unable to set thread local logger");
    // Setup log proxy.
    set_thread_logger(LogProxy::boxed(
        channel.log().expect("Unable to get log channel"),
        level,
    ));

    log::info!("Connected.");

    std::thread::sleep(std::time::Duration::from_secs(2));
    log::info!("Done.");

    std::process::exit(1234);

    // Ok(())
}
