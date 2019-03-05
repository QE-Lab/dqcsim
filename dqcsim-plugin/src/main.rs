// use dqcsim::plugin::config::PluginConfig;
use std::{env, error::Error};
// use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguments
    // let opt = PluginConfig::from_args();
    // panic!("Asdf");
    let args: Vec<String> = env::args().collect();
    let server = args[1].as_ref();

    dqcsim::util::log::connect(server, None).unwrap();
    // opt.server.expect("Missing server name"), opt.loglevel)?;

    log::info!("child process: {}", std::process::id());
    log::error!("Error from child process");

    // std::thread::sleep(std::time::Duration::from_secs(1));

    // log::error!("Awake.");

    Ok(())
}
