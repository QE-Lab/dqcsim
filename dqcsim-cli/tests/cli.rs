use assert_cmd::prelude::*;
use lazy_static::lazy_static;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{path::PathBuf, process::Command};

// https://github.com/glehmann/hld/blob/f40f3b51f84969cc13714f81451ad17f33fbf2dc/tests/common/mod.rs#L79
macro_rules! cli {
    ( $( $v:expr ),* ) => {{
        let cli_bin = escargot::CargoBuild::new()
                .current_release()
                .current_target()
                .run()
                .unwrap()
                .path()
                .to_path_buf();
        let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect();

        if cfg!(all(feature = "kcov", target_os = "linux")) {
            let target_dir = cli_bin.parent().unwrap();
            let cov_dir = target_dir.join("cov");
            std::fs::create_dir_all(&cov_dir).unwrap();
            Command::new("kcov").args(&[
                "--include-pattern=/src",
                "--exclude-pattern=/.cargo",
                "--exclude-region='#[cfg(test)]'",
                "--verify",
                &format!("{}/{}-{}", cov_dir.display().to_string(), env!("CARGO_PKG_NAME"), rand_string),
                &cli_bin.display().to_string(),
                $($v,)*
            ]).assert()
        } else {
            let args: &[&str] = &[$($v,)*];
            Command::new(cli_bin.display().to_string()).args(args).assert()
        }
    }}
}

lazy_static! {
    static ref PLUGIN_PATH: PathBuf = assert_cmd::cargo::cargo_bin("examples/plugin");
    static ref PLUGIN: &'static str = PLUGIN_PATH.to_str().unwrap();
}

#[test]
fn with_macro() {
    cli!(*PLUGIN, *PLUGIN).success();
}

#[test]
fn no_arguments() {
    cli!()
        .failure()
        .code(1)
        .stdout(predicate::str::contains(include_str!(
            "../src/arg_parse/usage.txt"
        )));
}

#[test]
fn help() {
    cli!("--help")
        .failure()
        .code(1)
        .stdout(predicate::str::contains("Used to specify the host API call sequence. Refer to the \"host call sequence\" section for more info."));
}

#[test]
fn long_help() {
    cli!("--long-help")
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "Run the specified cQASM file using the cQASM frontend and (default) QX backend.",
        ));
}

#[test]
fn version() {
    cli!("--version")
        .failure()
        .code(1)
        .stdout(predicate::str::is_match("^DQCsim [0-9].[0-9].[0-9] (.*)").unwrap());
}

#[test]
fn host_call_no_value() {
    cli!("--call")
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "error: The argument '--call <call>...' requires a value but none was supplied",
        ));
    cli!("-C")
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "error: The argument '--call <call>...' requires a value but none was supplied",
        ));
}

#[test]
fn host_call_ok() {
    cli!("--call", "start", "--call", "wait", *PLUGIN, *PLUGIN)
        .success()
        .stderr(predicate::str::contains(
            "Executing 'start(...)' host call...",
        ))
        .stderr(predicate::str::contains("Executing 'wait()' host call..."));
}

#[test]
fn host_call_send_no_data() {
    cli!("--call", "send", *PLUGIN, *PLUGIN)
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "Invalid value for '--call <call>...': Invalid argument: the send API call requires an ArbData argument",
        ));
}

#[test]
fn host_call_arb_no_data() {
    cli!("--call", "arb", *PLUGIN, *PLUGIN)
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "the arb API call requires a plugin and an ArbCmd argument",
        ));
}

#[test]
fn host_call_arb_plugin_not_found() {
    cli!(
        "--call",
        "arb:a:b.c:{\"answer\": 42},x,y,z",
        *PLUGIN,
        *PLUGIN
    )
    .failure()
    .stderr(predicate::str::contains(
        "Invalid argument: plugin a not found",
    ));
}

#[test]
fn host_call_arb() {
    cli!(
        "--call",
        "arb:front:b.c:{\"answer\": 42},x,y,z",
        *PLUGIN,
        *PLUGIN
    )
    .success()
    .stderr(predicate::str::contains(
        "Executing 'arb(...)' host call...",
    ));
}

#[test]
fn host_call_wait_data() {
    cli!("--call", "wait:{},a.b", *PLUGIN, *PLUGIN)
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "the wait API call does not take an argument",
        ));
}

#[test]
fn host_call_recv_data() {
    cli!("--call", "recv:{},a.b", *PLUGIN, *PLUGIN)
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "the recv API call does not take an argument",
        ));
}

#[test]
fn host_call_yield_data() {
    cli!("--call", "yield:{},a.b", *PLUGIN, *PLUGIN)
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "the yield API call does not take an argument",
        ));
}

#[test]
fn host_call_yield() {
    cli!("--call", "yield", *PLUGIN, *PLUGIN)
        .success()
        .stderr(predicate::str::contains("Executing 'yield()' host call..."));
}

#[test]
fn host_call_recv_deadlock() {
    cli!("--call", "recv", *PLUGIN, *PLUGIN)
        .failure()
        .stderr(predicate::str::contains("Executing 'recv()' host call..."))
        .stderr(predicate::str::contains(
            "Deadlock: accelerator exited before sending data",
        ));
}

#[test]
fn host_call_send() {
    cli!("--call", "send:{},a.b", *PLUGIN, *PLUGIN)
        .success()
        .stderr(predicate::str::contains(
            "Executing 'send(...)' host call...",
        ));
}

#[test]
fn host_call_with_reproduce() {
    cli!(
        "--reproduce",
        "/dev/zero",
        "--call",
        "start",
        *PLUGIN,
        *PLUGIN
    )
    .failure()
    .code(1)
    .stdout(predicate::str::contains(
        "The argument '--reproduce <filename>' cannot be used with '--call <call>...",
    ));
}

#[test]
fn host_call_with_reproduce_exactly() {
    cli!(
        "--reproduce-exactly",
        "/dev/zero",
        "-C",
        "start",
        *PLUGIN,
        *PLUGIN
    )
    .failure()
    .code(1)
    .stdout(predicate::str::contains(
        "The argument '--reproduce-exactly <filename>' cannot be used with '--call <call>...",
    ));
}

#[test]
fn host_stdout() {
    cli!("--host-stdout", *PLUGIN, *PLUGIN)
        .success()
        .stdout(predicate::str::contains("wait(): {}"));
}

#[test]
fn bad_path() {
    let path = assert_cmd::cargo::cargo_bin("asdf");
    cli!(path.to_str().unwrap())
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "While interpreting plugin specification: Invalid argument: the plugin specification",
        ))
        .stdout(predicate::str::contains(
            "/asdf' appears to be a path, but the referenced file does not exist",
        ));
}

#[test]
fn no_backend() {
    cli!(*PLUGIN)
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "While interpreting plugin specification: Invalid argument: could not find plugin executable 'dqcsbeqx', needed for plugin specification 'qx'",
        ));
}

#[test]
fn no_repro_out() {
    cli!(*PLUGIN, "--no-repro-out")
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Found argument '--no-repro-out' which wasn't expected, or isn't valid in this context",
        ));

    cli!("--no-repro-out", *PLUGIN, *PLUGIN)
        .success()
        .stderr(predicate::str::contains(
            "Simulation completed successfully.",
        ));
}

#[test]
fn repro_out() {
    cli!("--repro-out", "/not_allowed", *PLUGIN, *PLUGIN)
        .success()
        .stderr(predicate::str::contains(
            "When trying to write reproduction file:",
        ));

    cli!("--repro-out", "/tmp/repro-out.out", *PLUGIN, *PLUGIN)
        .success()
        .stderr(predicate::str::contains(
            "Simulation completed successfully.",
        ));
}

#[test]
fn no_repro_out_repro_out() {
    cli!(
        "--no-repro-out",
        "--repro-out",
        "/tmp/repro.out",
        *PLUGIN,
        *PLUGIN
    )
    .failure()
    .code(1)
    .stdout(predicate::str::contains(
        "The argument '--no-repro-out' cannot be used with '--repro-out <filename>'",
    ));
}
