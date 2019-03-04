use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let server = args[1].as_ref();

    dqcsim_log::connect(server, Some(log::LevelFilter::Trace))?;

    log::warn!("Warning from child process: {}", std::process::id());

    Ok(())
}
