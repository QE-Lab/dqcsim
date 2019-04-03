use dqcsim::{
    host::{accelerator::Accelerator, reproduction::HostCall, simulator::Simulator},
    info, note,
};

use failure::Error;

mod arg_parse;
use crate::arg_parse::*;

fn main() -> Result<(), Error> {
    let cfg = CommandLineConfiguration::parse().unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });

    let mut sim = Simulator::new(cfg.dqcsim).unwrap_or_else(|e| {
        eprintln!("Failed to construct simulator: {}", e);
        std::process::exit(1);
    });

    for host_call in cfg.host_calls.into_iter() {
        match host_call {
            HostCall::Start(d) => {
                info!("Executing 'start(...)' host call...");
                sim.simulation.start(d)?;
            }
            HostCall::Wait => {
                info!("Executing 'wait()' host call...");
                let ret = sim.simulation.wait()?;
                note!("'wait()' returned {}", &ret);
                if cfg.host_stdout {
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
                if cfg.host_stdout {
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
                if cfg.host_stdout {
                    println!("arb: {}", ret);
                }
            }
        }
    }

    Ok(())
}
