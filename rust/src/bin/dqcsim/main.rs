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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! cli {
        ( $( $v:expr ),* ) => {{
            let args: &[&str] = &["test", $($v,)*];
            internal_main(args)
        }}
    }

    macro_rules! err {
        ( $x:expr ) => {{
            $x.unwrap_err().to_string()
        }};
    }

    static FRONTEND: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug/dqcsfenull");
    static BACKEND: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug/dqcsbenull");
    static OPERATOR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug/dqcsopnull");

    #[test]
    fn no_arguments() {
        assert!(err!(cli!()).contains("At least one plugin specification is required\n\nUSAGE:\n"));
    }

    #[test]
    fn help() {
        assert!(
            err!(cli!("--help")).contains("\nDelft Quantum & Classical simulator\n\nUSAGE:\n    ")
        );
    }

    #[test]
    fn long_help() {
        assert!(err!(cli!("--long-help")).contains(
            "Run the specified cQASM file using the cQASM frontend and (default) QX backend."
        ));
    }

    #[test]
    fn version() {
        assert_eq!(err!(cli!("--version")), "\n");
    }

    #[test]
    fn host_call_no_value() {
        assert!(
            err!(cli!("--call")).contains("requires a value but none was supplied\n\nUSAGE:\n  ")
        );
    }

    #[test]
    fn host_call_bad_value() {
        assert!(err!(cli!("--call", "hello")).contains("Invalid argument: hello is not a valid host call function, valid values are start, wait, send, recv, yield, or arb\n"));
    }

    #[test]
    fn host_call_ok() {
        assert!(cli!("--call", "start", "--call", "wait", FRONTEND, BACKEND).is_ok());
    }

    #[test]
    fn host_call_send_no_data() {
        assert!(err!(cli!("--call", "send", FRONTEND, BACKEND))
            .contains("Invalid argument: the send API call requires an ArbData argument"));
    }

    #[test]
    fn host_call_arb_no_data() {
        assert!(err!(cli!("--call", "arb", FRONTEND, BACKEND))
            .contains("the arb API call requires a plugin and an ArbCmd argument"));
    }

    #[test]
    fn host_call_arb_plugin_not_found() {
        assert!(err!(cli!(
            "--call",
            "arb:a:b.c:{\"answer\": 42},x,y,z",
            FRONTEND,
            BACKEND
        ))
        .contains("Invalid argument: plugin a not found"));
    }

    #[test]
    fn host_call_arb() {
        assert!(cli!(
            "--call",
            "arb:front:b.c:{\"answer\": 42},x,y,z",
            FRONTEND,
            BACKEND
        )
        .is_ok());

        assert!(cli!(
            "--host-stdout",
            "--call",
            "arb:front:b.c:{\"answer\": 42},x,y,z",
            FRONTEND,
            BACKEND
        )
        .is_ok());
    }

    #[test]
    fn host_call_wait_data() {
        assert!(err!(cli!("--call", "wait:{},a.b", FRONTEND, BACKEND))
            .contains("the wait API call does not take an argument"));
    }

    #[test]
    fn host_call_recv_data() {
        assert!(err!(cli!("--call", "recv:{},a.b", FRONTEND, BACKEND))
            .contains("the recv API call does not take an argument"));
    }

    #[test]
    fn host_call_yield_data() {
        assert!(err!(cli!("--call", "yield:{},a.b", FRONTEND, BACKEND))
            .contains("the yield API call does not take an argument"));
    }

    #[test]
    fn host_call_yield() {
        assert!(cli!("--call", "yield", FRONTEND, BACKEND).is_ok());
    }

    #[test]
    fn host_call_recv() {
        assert!(cli!("--call", "send:{},a.b", "--call", "recv", FRONTEND, BACKEND).is_ok());
    }

    #[test]
    fn host_call_recv_deadlock() {
        assert!(
            err!(cli!("--call", "recv", "--call", "recv", FRONTEND, BACKEND))
                .contains("Deadlock: accelerator exited before sending data")
        );
    }

    #[test]
    fn host_call_send() {
        assert!(cli!("--call", "send:{},a.b", FRONTEND, BACKEND).is_ok());
    }

    #[test]
    fn bad_repro_paths() {
        assert!(
            err!(cli!("--repro-paths", "hello", FRONTEND, BACKEND))
                .contains("Invalid argument: hello is not a valid reproduction path style, valid values are keep, relative, or absolute")
        );
    }

    #[test]
    fn host_call_with_reproduce() {
        assert!(err!(cli!(
            "--reproduce",
            "/dev/zero",
            "--call",
            "start",
            FRONTEND,
            BACKEND
        ))
        .contains("--reproduce <filename>"));
    }

    #[test]
    fn host_call_with_reproduce_exactly() {
        assert!(err!(cli!(
            "--reproduce-exactly",
            "/dev/zero",
            "-C",
            "start",
            FRONTEND,
            BACKEND
        ))
        .contains("--reproduce-exactly <filename>"));
    }

    #[test]
    fn host_stdout() {
        assert!(cli!("--host-stdout", FRONTEND, BACKEND).is_ok());
    }

    #[test]
    fn with_operator() {
        assert!(cli!(FRONTEND, OPERATOR, BACKEND).is_ok());
    }

    #[test]
    fn plugin_config_name() {
        assert!(cli!(FRONTEND, "--name", "frontend-test", BACKEND).is_ok());
    }

    #[test]
    fn plugin_env_mod() {
        assert!(cli!(FRONTEND, "--env", "key:value", BACKEND).is_ok());
        assert!(cli!(FRONTEND, "--env", "~key", BACKEND).is_ok());
    }

    #[test]
    fn double_start_insert_wait() {
        assert!(cli!("-C", "start", "-C", "start", FRONTEND, BACKEND).is_ok());
    }

    #[test]
    fn bad_path() {
        assert!(err!(cli!("/asdf"))
            .contains("/asdf' appears to be a path, but the referenced file does not exist"));
    }

    #[test]
    fn loglevel() {
        assert!(cli!("-l", "fatal", FRONTEND, BACKEND).is_ok());
        assert!(cli!("-l", "f", FRONTEND, BACKEND).is_ok());
        assert!(cli!("-lf", FRONTEND, BACKEND).is_ok());
    }

    #[test]
    fn bad_loglevel() {
        assert!(err!(cli!("-l", "hello", FRONTEND, BACKEND)).contains(
                "Invalid argument: hello is not a valid loglevel filter, valid values are off, fatal, error, warn, note, info, debug, or trace"
            ));
    }

    #[test]
    fn no_backend() {
        assert!(err!(cli!(FRONTEND)).contains(
                "While interpreting plugin specification: Invalid argument: could not find plugin executable 'dqcsbeqx', needed for plugin specification 'qx'",
            ));
    }

    #[test]
    fn no_repro_out() {
        assert!(cli!("--no-repro-out", FRONTEND, BACKEND).is_ok());
    }

    #[test]
    fn repro_out() {
        assert!(cli!("--repro-out", "/not_allowed", FRONTEND, BACKEND).is_ok());
        assert!(cli!("--repro-out", "/tmp/repro-out.out", FRONTEND, BACKEND).is_ok());
    }

    // #[test]
    // fn no_repro_out_repro_out() {
    //     cli!(
    //         "--no-repro-out",
    //         "--repro-out",
    //         "/tmp/test.repro",
    //         FRONTEND,
    //         BACKEND
    //     )
    //     .failure()
    //     .code(1)
    //     .stdout(predicate::str::contains(
    //         "The argument '--no-repro-out' cannot be used with '--repro-out <filename>'",
    //     ));
    // }

    // #[test]
    // fn reproduce_bad_path() {
    //     cli!("--reproduce", "./asdf")
    //         .failure()
    //         .stdout(predicates::str::contains("While reading reproduction file"));
    // }

    // #[test]
    // fn reproduce() {
    //     cli!(
    //         "--repro-out",
    //         "./dqcsim-cli.test.repro",
    //         FRONTEND,
    //         BACKEND
    //     )
    //     .success();
    //     cli!("--reproduce", "./dqcsim-cli.test.repro").success();

    //     // illegal name override
    //     cli!(
    //         "--reproduce",
    //         "./dqcsim-cli.test.repro",
    //         "@front",
    //         "-n",
    //         "override-name"
    //     )
    //     .failure()
    //     .stdout(predicates::str::contains(
    //         "cannot be used when referencing a previously defined plugin",
    //     ));

    //     // illegal work override
    //     cli!(
    //         "--reproduce",
    //         "./dqcsim-cli.test.repro",
    //         "@front",
    //         "--work",
    //         "work"
    //     )
    //     .failure()
    //     .stdout(predicates::str::contains(
    //         "cannot be used when referencing a previously defined plugin",
    //     ));

    //     // override verbosity
    //     cli!(
    //         "--reproduce",
    //         "./dqcsim-cli.test.repro",
    //         "@front",
    //         "-l",
    //         "fatal"
    //     )
    //     .success();

    //     // exact reproduce
    //     cli!(
    //         "--reproduce-exactly",
    //         "./dqcsim-cli.test.repro",
    //         "@front",
    //         "-l",
    //         "fatal"
    //     )
    //     .success();

    //     // def with reproduce
    //     cli!("--reproduce", "./dqcsim-cli.test.repro", FRONTEND)
    //         .failure()
    //         .stdout(predicates::str::contains("Cannot define new plugins while"));

    //     // mod with def
    //     cli!(FRONTEND, BACKEND, "@front", "-l", "trace")
    //         .failure()
    //         .stdout(predicates::str::contains("Cannot modify plugins unless"));
    // }

}
