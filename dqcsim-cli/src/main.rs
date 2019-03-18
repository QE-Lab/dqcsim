use dqcsim::simulator::Simulator;
use failure::Error;

mod arg_parse;
use crate::arg_parse::*;

fn main() -> Result<(), Error> {
    let cfg = CommandLineConfiguration::parse().unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });
    dbg!(&cfg);
    Simulator::try_from(cfg.dqcsim)?.as_mut().abort(true)?;
    Ok(())
}
