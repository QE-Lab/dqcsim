use failure::Error;
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let server = args[1].as_ref();

    std::thread::sleep(std::time::Duration::from_secs(1));

    let channel = dqcsim::protocol::channel::connect(server, Some(log::LevelFilter::Trace))?;

    log::info!("Connected.");

    std::thread::sleep(std::time::Duration::from_secs(1));
    log::info!("Done.");

    Ok(())
}
