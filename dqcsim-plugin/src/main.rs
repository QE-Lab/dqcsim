use dqcsim::util::{
    ipc, log,
    log::{init, proxy::LogProxy},
};
use failure::Error;
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let server = args[1].as_ref();

    // Connect to simulator. Get PluginChannel.
    let mut channel = ipc::connect(server)?;

    // Initialize thread local logger.
    let level = Some(log::LevelFilter::Trace);
    // Setup log proxy.
    init(
        LogProxy::boxed(channel.log().expect("Unable to get log channel"), level),
        level.unwrap(),
    )
    .expect("Unable to set thread local logger");

    log::info!("Connected");

    eprintln!("stderr");
    println!("stdout");

    log::info!("Done");

    std::process::exit(1234);

    // Ok(())
}
