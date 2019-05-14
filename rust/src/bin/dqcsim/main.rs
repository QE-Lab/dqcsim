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
        ( $x:ident ) => {{
            $x.unwrap_err().to_string()
        }};
    }

    #[test]
    fn no_arguments() {
        let x = cli!();
        assert!(x.is_err());
        assert_eq!(err!(x), "\u{1b}[1;31merror:\u{1b}[0m At least one plugin specification is required\n\nUSAGE:\n    dqcsim [DQCSIM OPTIONS] <PLUGIN SPEC> [PLUGIN OPTIONS] [<PLUGIN SPEC> [PLUGIN OPTIONS] [...]]\n\nFor more information try \u{1b}[32m--help\u{1b}[0m");
    }

    #[test]
    fn help() {
        let x = cli!("--help");
        assert!(x.is_err());
        assert!(err!(x).contains("\nDelft Quantum & Classical simulator\n\nUSAGE:\n    dqcsim [DQCSIM OPTIONS] <PLUGIN SPEC> [PLUGIN OPTIONS] [<PLUGIN SPEC> [PLUGIN OPTIONS] [...]]\n\nDQCSIM OPTIONS:\n    \u{1b}[32m-C\u{1b}[0m, \u{1b}[32m--call\u{1b}[0m \u{1b}[32m<call>\u{1b}[0m\u{1b}[32m...\u{1b}[0m\n            Used to specify the host API call sequence. Refer to the \"host call sequence\" section for more info.\n\n        \u{1b}[32m--host-stdout\u{1b}[0m\n            Specifies that the return values of host API calls should be printed to stdout, in addition to being logged\n            with loglevel note. Use this if you want to send these values to another program through a pipe.\n        \u{1b}[32m--repro-out\u{1b}[0m \u{1b}[32m<filename>\u{1b}[0m\n            Output a reproduction file to the specified filename. The default is to output a reproduction file to\n            \"<basename(frontend)>.repro\".\n        \u{1b}[32m--no-repro-out\u{1b}[0m\n            Disables outputting a reproduction file.\n\n        \u{1b}[32m--repro-paths\u{1b}[0m \u{1b}[32m<style>\u{1b}[0m\n            Configures the way paths are stored in the reproduction file. The default is to save the paths as they were\n            specified on the command line. The alternatives are to force usage of absolute paths or to force making them\n            relative to DQCsim\'s working directory. [default: \u{1b}[32mkeep\u{1b}[0m]\n        \u{1b}[32m--reproduce\u{1b}[0m \u{1b}[32m<filename>\u{1b}[0m\n            Reproduce the simulation run specified by the given reproduction file as if the modeled physical experiment\n            is rerun. That is, physically random samples return different values. It is illegal to combine --reproduce\n            with any functional configuration; if you want to change the functional configuration you must change the\n            reproduction file manually.\n        \u{1b}[32m--reproduce-exactly\u{1b}[0m \u{1b}[32m<filename>\u{1b}[0m\n            Reproduce the simulation run specified by the given reproduction file as exactly as the plugins allow. That\n            is, even physically random samples should have the same values. It is illegal to combine --reproduce-exactly\n            with any functional configuration; if you want to change the functional configuration you must change the\n            reproduction file manually.\n        \u{1b}[32m--seed\u{1b}[0m \u{1b}[32m<seed>\u{1b}[0m\n            Specifies a random seed for the simulation. If a 64-bit unsigned number is specified, it is used directly.\n            Otherwise, the specified string is hashed to such a 64-bit number. If not specified, the current timestamp\n            (with the lowest granularity available) is used as a seed.\n    \u{1b}[32m-l\u{1b}[0m, \u{1b}[32m--level\u{1b}[0m \u{1b}[32m<level>\u{1b}[0m\n            Sets the minimum importance for a message to be written to stderr. [default: \u{1b}[32minfo\u{1b}[0m]\n\n    \u{1b}[32m-T\u{1b}[0m, \u{1b}[32m--tee\u{1b}[0m \u{1b}[32m<level>:<filename>\u{1b}[0m\u{1b}[32m...\u{1b}[0m\n            Logs messages to the specified file in addition to stderr. level sets the minimum importance for a message\n            to be logged to this file.\n        \u{1b}[32m--dqcsim-level\u{1b}[0m \u{1b}[32m<level>\u{1b}[0m\n            Sets the logging verbosity for DQCsim itself (the driver and host API). [default: \u{1b}[32mtrace\u{1b}[0m]\n\n        \u{1b}[32m--plugin-level\u{1b}[0m \u{1b}[32m<level>\u{1b}[0m\n            Sets the default logging verbosity for the plugins. [default: \u{1b}[32mtrace\u{1b}[0m]\n\n        \u{1b}[32m--long-help\u{1b}[0m\n            Shows a more complete help message than --help.\n\n    \u{1b}[32m-h\u{1b}[0m, \u{1b}[32m--help\u{1b}[0m\n            Prints help information\n\n    \u{1b}[32m-V\u{1b}[0m, \u{1b}[32m--version\u{1b}[0m\n            Prints version information\n\nPLUGIN SPEC:\n    Plugins are specified from frontend to backend. If no backend is specified, QX is used. The following syntax is\n    allowed:\n\n     - a valid path to the plugin executable;\n     - the basename of the plugin executable with implicit \"dqcsfe\"/\"dqcsop\"/\"dqcsbe\" prefix, searched for in A)\n       DQCsim\'s working directory, B) the dqcsim binary directory, and C) the system $PATH;\n     - a valid path to a script file with a file extension for which an interpreter exists.\n\nPLUGIN OPTIONS:\n    \u{1b}[32m-n\u{1b}[0m, \u{1b}[32m--name\u{1b}[0m \u{1b}[32m<name>\u{1b}[0m\n            Provides a custom name for the plugin, used for log messages and referencing the plugin on the command line\n            later in conjunction with --reproduce. If not provided, plugins are named \"front\", \"op<i>\" (where i starts\n            at 1 and counts from frontend to backend), and \"back\".\n    \u{1b}[32m-i\u{1b}[0m, \u{1b}[32m--init\u{1b}[0m \u{1b}[32m<<arb_cmd>>\u{1b}[0m\u{1b}[32m...\u{1b}[0m\n            Appends an ArbCmd object to the plugin\'s initialization method.\n\n        \u{1b}[32m--env\u{1b}[0m \u{1b}[32m<mod>\u{1b}[0m\u{1b}[32m...\u{1b}[0m\n            Sets, updates, overrides, or deletes an environment variable in the plugin scope. To set a variable, use the\n            syntax <key>=<value>. If you don\'t care about the value and just want to define the variable, just <key>\n            without the equals sign is sufficient to assign an empty string. To delete an environment variable, use\n            ~<key>.\n        \u{1b}[32m--work\u{1b}[0m \u{1b}[32m<filename>\u{1b}[0m\n            Overrides the working directory for the plugin.\n\n    \u{1b}[32m-l\u{1b}[0m, \u{1b}[32m--level\u{1b}[0m \u{1b}[32m<level>\u{1b}[0m\n            Sets the logging verbosity for the associated plugin, overriding \"--plugin-level\".\n\n    \u{1b}[32m-T\u{1b}[0m, \u{1b}[32m--tee\u{1b}[0m \u{1b}[32m<level>:<filename>\u{1b}[0m\u{1b}[32m...\u{1b}[0m\n            Logs messages to the specified file in addition to stderr. level sets the minimum importance for a message\n            to be logged to this file.\n        \u{1b}[32m--stdout\u{1b}[0m \u{1b}[32m<level>\u{1b}[0m\n            Specifies the loglevel that is to be used for logging the plugin\'s stdout stream (if any). In addition to\n            the available loglevels, you can also specify \"pass\" here, which prevents stdout from being captured by the\n            logging system, instead piping it to DQCsim\'s stdout. The default is \"info\".\n        \u{1b}[32m--stderr\u{1b}[0m \u{1b}[32m<level>\u{1b}[0m\n            Specifies the loglevel that is to be used for logging the plugin\'s stderr stream (if any). In addition to\n            the available loglevels, you can also specify \"pass\" here, which prevents stderr from being captured by the\n            logging system, instead piping it to DQCsim\'s stderr. The default is \"info\".\n        \u{1b}[32m--accept-timeout\u{1b}[0m \u{1b}[32m<level>\u{1b}[0m\n            Sets the timeout for DQCsim to connect to the plugin after the process is launched. The default is 5\n            seconds, so you normally shouldn\'t have to touch this. The value accepts floating point numbers as seconds,\n            integers with time units (h, m, s, ms, us, ns), or \"infinity\" to disable the timeout.\n        \u{1b}[32m--shutdown-timeout\u{1b}[0m \u{1b}[32m<level>\u{1b}[0m\n            Sets the timeout for plugin shutdown. When this timeout expires, DQCsim sends SIGKILL to the process to\n            terminate it. The default timeout is 5 seconds, so you normally shouldn\'t have to touch this. The value\n            accepts floating point numbers as seconds, integers with time units (h, m, s, ms, us, ns), or \"infinity\" to\n            disable the timeout.\n    \u{1b}[32m-h\u{1b}[0m, \u{1b}[32m--help\u{1b}[0m\n            Prints help information\n\nMORE INFORMATION:\n    This help text is abbreviated. Use --long-help for more information.\n"));
    }

    #[test]
    fn long_help() {
        let x = cli!("--long-help");
        assert!(err!(x).contains(
            "Run the specified cQASM file using the cQASM frontend and (default) QX backend."
        ));
    }

    #[test]
    fn version() {
        let x = cli!("--version");
        // output is written to stdout
        assert_eq!(err!(x), "\n");
    }

    #[test]
    fn cli_host_call_no_value() {
        let x = cli!("--call");
        assert_eq!(err!(x), "\u{1b}[1;31merror:\u{1b}[0m The argument \'\u{1b}[33m--call <call>...\u{1b}[0m\' requires a value but none was supplied\n\nUSAGE:\n    dqcsim [DQCSIM OPTIONS] <PLUGIN SPEC> [PLUGIN OPTIONS] [<PLUGIN SPEC> [PLUGIN OPTIONS] [...]]\n\nFor more information try \u{1b}[32m--help\u{1b}[0m\n");
    }

    #[test]
    fn cli_host_call_bad_value() {
        let x = cli!("--call", "hello");
        assert_eq!(err!(x), "\u{1b}[1;31merror:\u{1b}[0m Invalid value for \'\u{1b}[33m--call <call>...\u{1b}[0m\': Invalid argument: hello is not a valid host call function, valid values are start, wait, send, recv, yield, or arb\n");
    }

    // #[test]
    // fn cli_host_call_ok() {
    //     cli!("--call", "start", "--call", "wait", *FRONTEND, *BACKEND)
    //         .success()
    //         .stderr(predicate::str::contains(
    //             "Executing 'start(...)' host call...",
    //         ))
    //         .stderr(predicate::str::contains("Executing 'wait()' host call..."));
    // }

    // #[test]
    // fn cli_host_call_send_no_data() {
    //     cli!("--call", "send", *FRONTEND, *BACKEND)
    //         .failure()
    //         .code(1)
    //         .stdout(predicate::str::contains(
    //             "Invalid value for '--call <call>...': Invalid argument: the send API call requires an ArbData argument",
    //         ));
    // }

    // #[test]
    // fn cli_host_call_arb_no_data() {
    //     cli!("--call", "arb", *FRONTEND, *BACKEND)
    //         .failure()
    //         .code(1)
    //         .stdout(predicate::str::contains(
    //             "the arb API call requires a plugin and an ArbCmd argument",
    //         ));
    // }

    // #[test]
    // fn cli_host_call_arb_plugin_not_found() {
    //     cli!(
    //         "--call",
    //         "arb:a:b.c:{\"answer\": 42},x,y,z",
    //         *FRONTEND,
    //         *BACKEND
    //     )
    //     .failure()
    //     .stderr(predicate::str::contains(
    //         "Invalid argument: plugin a not found",
    //     ));
    // }

    // #[test]
    // fn cli_host_call_arb() {
    //     cli!(
    //         "--call",
    //         "arb:front:b.c:{\"answer\": 42},x,y,z",
    //         *FRONTEND,
    //         *BACKEND
    //     )
    //     .success()
    //     .stderr(predicate::str::contains(
    //         "Executing 'arb(...)' host call...",
    //     ));

    //     cli!(
    //         "--host-stdout",
    //         "--call",
    //         "arb:front:b.c:{\"answer\": 42},x,y,z",
    //         *FRONTEND,
    //         *BACKEND
    //     )
    //     .success()
    //     .stdout(predicate::str::contains("arb:"))
    //     .stderr(predicate::str::contains(
    //         "Executing 'arb(...)' host call...",
    //     ));
    // }

    // #[test]
    // fn cli_host_call_wait_data() {
    //     cli!("--call", "wait:{},a.b", *FRONTEND, *BACKEND)
    //         .failure()
    //         .code(1)
    //         .stdout(predicate::str::contains(
    //             "the wait API call does not take an argument",
    //         ));
    // }

    // #[test]
    // fn cli_host_call_recv_data() {
    //     cli!("--call", "recv:{},a.b", *FRONTEND, *BACKEND)
    //         .failure()
    //         .code(1)
    //         .stdout(predicate::str::contains(
    //             "the recv API call does not take an argument",
    //         ));
    // }

    // #[test]
    // fn cli_host_call_yield_data() {
    //     cli!("--call", "yield:{},a.b", *FRONTEND, *BACKEND)
    //         .failure()
    //         .code(1)
    //         .stdout(predicate::str::contains(
    //             "the yield API call does not take an argument",
    //         ));
    // }

    // #[test]
    // fn cli_host_call_yield() {
    //     cli!("--call", "yield", *FRONTEND, *BACKEND)
    //         .success()
    //         .stderr(predicate::str::contains("Executing 'yield()' host call..."));
    // }

    // #[test]
    // fn cli_host_call_recv() {
    //     cli!(
    //         "--call",
    //         "send:{},a.b",
    //         "--call",
    //         "recv",
    //         *FRONTEND,
    //         *BACKEND
    //     )
    //     .success()
    //     .stderr(predicate::str::contains("Executing 'recv()' host call..."));
    // }

    // #[test]
    // fn cli_host_call_recv_deadlock() {
    //     cli!("--call", "recv", "--call", "recv", *FRONTEND, *BACKEND)
    //         .failure()
    //         .stderr(predicate::str::contains("Executing 'recv()' host call..."))
    //         .stderr(predicate::str::contains(
    //             "Deadlock: accelerator exited before sending data",
    //         ));
    // }

    // #[test]
    // fn cli_host_call_send() {
    //     cli!("--call", "send:{},a.b", *FRONTEND, *BACKEND)
    //         .success()
    //         .stderr(predicate::str::contains(
    //             "Executing 'send(...)' host call...",
    //         ));
    // }

    // #[test]
    // fn cli_bad_repro_paths() {
    //     cli!("--repro-paths", "hello", *FRONTEND, *BACKEND)
    //         .failure()
    //         .code(1)
    //         .stdout(predicate::str::contains(
    //             "Invalid value for '--repro-paths <style>': Invalid argument: hello is not a valid reproduction path style, valid values are keep, relative, or absolute",
    //         ));
    // }

    // #[test]
    // fn cli_host_call_with_reproduce() {
    //     cli!(
    //         "--reproduce",
    //         "/dev/zero",
    //         "--call",
    //         "start",
    //         *FRONTEND,
    //         *BACKEND
    //     )
    //     .failure()
    //     .code(1)
    //     .stdout(predicate::str::contains(
    //         "The argument '--reproduce <filename>' cannot be used with '--call <call>...",
    //     ));
    // }

    // #[test]
    // fn cli_host_call_with_reproduce_exactly() {
    //     cli!(
    //         "--reproduce-exactly",
    //         "/dev/zero",
    //         "-C",
    //         "start",
    //         *FRONTEND,
    //         *BACKEND
    //     )
    //     .failure()
    //     .code(1)
    //     .stdout(predicate::str::contains(
    //         "The argument '--reproduce-exactly <filename>' cannot be used with '--call <call>...",
    //     ));
    // }

    // #[test]
    // fn cli_host_stdout() {
    //     cli!("--host-stdout", *FRONTEND, *BACKEND)
    //         .success()
    //         .stdout(predicate::str::contains("wait(): {}"));
    // }

    // #[test]
    // fn cli_with_operator() {
    //     cli!(*FRONTEND, *OPERATOR, *BACKEND)
    //         .success()
    //         .stderr(predicate::str::contains("op1"));
    // }

    // #[test]
    // fn cli_plugin_config_name() {
    //     cli!(*FRONTEND, "--name", "frontend-test", *BACKEND)
    //         .success()
    //         .stderr(predicate::str::contains("frontend-test"));
    // }

    // #[test]
    // fn cli_plugin_env_mod() {
    //     cli!(*FRONTEND, "--env", "key:value", *BACKEND).success();
    //     cli!(*FRONTEND, "--env", "~key", *BACKEND).success();
    // }

    // #[test]
    // fn cli_double_start_insert_wait() {
    //     cli!("-C", "start", "-C", "start", *FRONTEND, *BACKEND)
    //         .success()
    //         .stderr(predicate::str::contains("Executing 'start(...)' host call...").count(2))
    //         .stderr(predicates::str::contains("Executing 'wait()' host call...").count(2));
    // }

    // #[test]
    // fn cli_bad_path() {
    //     let path = assert_cmd::cargo::cargo_bin("asdf");
    //     cli!(path.to_str().unwrap())
    //         .failure()
    //         .code(1)
    //         .stdout(predicate::str::contains(
    //             "While interpreting plugin specification: Invalid argument: the plugin specification",
    //         ))
    //         .stdout(predicate::str::contains(
    //             "/asdf' appears to be a path, but the referenced file does not exist",
    //         ));
    // }

    // #[test]
    // fn cli_loglevel() {
    //     cli!("-l", "fatal", *FRONTEND, *BACKEND).success();
    //     cli!("-l", "f", *FRONTEND, *BACKEND).success();
    //     cli!("-lf", *FRONTEND, *BACKEND).success();
    // }

    // #[test]
    // fn cli_bad_loglevel() {
    //     cli!("-l", "hello", *FRONTEND, *BACKEND)
    //         .failure()
    //         .code(1)
    //         .stdout(predicate::str::contains(
    //             "Invalid value for '--level <level>': Invalid argument: hello is not a valid loglevel filter, valid values are off, fatal, error, warn, note, info, debug, or trace",
    //         ));
    // }

    // #[test]
    // fn cli_no_backend() {
    //     cli!(*FRONTEND)
    //         .failure()
    //         .code(1)
    //         .stdout(predicate::str::contains(
    //             "While interpreting plugin specification: Invalid argument: could not find plugin executable 'dqcsbeqx', needed for plugin specification 'qx'",
    //         ));
    // }

    // #[test]
    // fn cli_no_repro_out() {
    //     cli!(*FRONTEND, "--no-repro-out")
    //         .failure()
    //         .code(1)
    //         .stderr(predicate::str::contains(
    //             "Found argument '--no-repro-out' which wasn't expected, or isn't valid in this context",
    //         ));

    //     cli!("--no-repro-out", *FRONTEND, *BACKEND)
    //         .success()
    //         .stderr(predicate::str::contains(
    //             "Simulation completed successfully.",
    //         ));
    // }

    // #[test]
    // fn cli_repro_out() {
    //     cli!("--repro-out", "/not_allowed", *FRONTEND, *BACKEND)
    //         .success()
    //         .stderr(predicate::str::contains(
    //             "When trying to write reproduction file:",
    //         ));

    //     cli!("--repro-out", "/tmp/repro-out.out", *FRONTEND, *BACKEND)
    //         .success()
    //         .stderr(predicate::str::contains(
    //             "Simulation completed successfully.",
    //         ));
    // }

    // #[test]
    // fn cli_no_repro_out_repro_out() {
    //     cli!(
    //         "--no-repro-out",
    //         "--repro-out",
    //         "/tmp/test.repro",
    //         *FRONTEND,
    //         *BACKEND
    //     )
    //     .failure()
    //     .code(1)
    //     .stdout(predicate::str::contains(
    //         "The argument '--no-repro-out' cannot be used with '--repro-out <filename>'",
    //     ));
    // }

    // #[test]
    // fn cli_reproduce_bad_path() {
    //     cli!("--reproduce", "./asdf")
    //         .failure()
    //         .stdout(predicates::str::contains("While reading reproduction file"));
    // }

    // #[test]
    // fn cli_reproduce() {
    //     cli!(
    //         "--repro-out",
    //         "./dqcsim-cli.test.repro",
    //         *FRONTEND,
    //         *BACKEND
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
    //     cli!("--reproduce", "./dqcsim-cli.test.repro", *FRONTEND)
    //         .failure()
    //         .stdout(predicates::str::contains("Cannot define new plugins while"));

    //     // mod with def
    //     cli!(*FRONTEND, *BACKEND, "@front", "-l", "trace")
    //         .failure()
    //         .stdout(predicates::str::contains("Cannot modify plugins unless"));
    // }

}
