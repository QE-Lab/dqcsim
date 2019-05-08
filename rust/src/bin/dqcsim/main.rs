use dqcsim::{
    error, fatal,
    host::{accelerator::Accelerator, reproduction::HostCall, simulator::Simulator},
    info, note,
};
use failure::Error;
use std::ffi::OsString;

mod arg_parse;
use crate::arg_parse::*;

fn run(
    sim: &mut Simulator,
    host_stdout: bool,
    host_calls: impl IntoIterator<Item = HostCall>,
) -> Result<(), Error> {
    for host_call in host_calls {
        match host_call {
            HostCall::Start(d) => {
                info!("Executing 'start(...)' host call...");
                sim.simulation.start(d)?;
            }
            HostCall::Wait => {
                info!("Executing 'wait()' host call...");
                let ret = sim.simulation.wait()?;
                note!("'wait()' returned {}", &ret);
                if host_stdout {
                    println!("wait(): {}", ret);
                }
            }
            HostCall::Send(d) => {
                info!("Executing 'send(...)' host call...");
                sim.simulation.send(d)?;
            }
            HostCall::Recv => {
                info!("Executing 'recv()' host call...");
                let ret = sim.simulation.recv()?;
                note!("'recv()' returned {}", &ret);
                if host_stdout {
                    println!("recv: {}", ret);
                }
            }
            HostCall::Yield => {
                info!("Executing 'yield()' host call...");
                sim.simulation.yield_to_accelerator()?;
            }
            HostCall::Arb(n, d) => {
                info!("Executing 'arb(...)' host call...");
                let ret = sim.simulation.arb(n, d)?;
                note!("'arb()' returned {}", &ret);
                if host_stdout {
                    println!("arb: {}", ret);
                }
            }
        }
    }

    Ok(())
}

fn internal_main<I, T>(args: I) -> Result<(), Error>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let mut cfg = CommandLineConfiguration::parse_from(args).or_else(|e| {
        println!("{}", e);
        Err(e)
    })?;

    let mut sim = Simulator::new(cfg.dqcsim).or_else(|e| {
        eprintln!("Failed to construct simulator: {}", e);
        Err(e)
    })?;

    let sim_result = run(&mut sim, cfg.host_stdout, cfg.host_calls.drain(..));

    if let Some(filename) = cfg.reproduction_file {
        match sim.simulation.write_reproduction_file(&filename) {
            Ok(_) => info!("Reproduction file written to {:?}.", filename),
            Err(e) => error!("When trying to write reproduction file: {}", e.to_string()),
        }
    }

    match &sim_result {
        Ok(_) => info!("Simulation completed successfully."),
        Err(e) => fatal!("Simulation failed: {}", e.to_string()),
    }

    sim_result
}

fn main() {
    let result = internal_main(std::env::args());
    std::process::exit(match result {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
