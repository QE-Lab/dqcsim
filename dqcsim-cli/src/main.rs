use failure::Error;

mod arg_parse;
use crate::arg_parse::*;

fn main() -> Result<(), Error> {
    let cfg = CommandLineConfiguration::parse().unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });
    dbg!(cfg);
    //Simulator::new(SimulationOpt::from_args())?.abort()?;
    Ok(())
}
