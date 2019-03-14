use dqcsim::simulator::{SimulationOpt, Simulator};
use failure::Error;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    Simulator::new(SimulationOpt::from_args())?.abort()?;
    Ok(())
}
