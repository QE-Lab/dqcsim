use assert_cmd::prelude::*;
use lazy_static::lazy_static;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{path::PathBuf, process::Command};

// https://github.com/glehmann/hld/blob/f40f3b51f84969cc13714f81451ad17f33fbf2dc/tests/common/mod.rs#L79
macro_rules! cli {
    ( $( $v:expr ),* ) => {{
        let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();

        if cfg!(all(feature = "kcov", target_os = "linux")) {
            let target_dir = CLI.parent().unwrap();
            let cov_dir = target_dir.parent().unwrap().parent().unwrap().join("cov");
            std::fs::create_dir_all(&cov_dir).unwrap();
            Command::new("kcov")
                .args(&[
                    "--include-pattern=/src",
                    "--exclude-pattern=/.cargo",
                    "--exclude-region='#[cfg(test)]'",
                    "--verify",
                    &format!(
                        "{}/{}-{}",
                        cov_dir.display().to_string(),
                        env!("CARGO_PKG_NAME"),
                        rand_string
                    ),
                    &CLI.display().to_string(),
                    $($v,)*
                ])
                .assert()
        } else {
            let args: &[&str] = &[$($v,)*];
            Command::new(CLI.display().to_string()).args(args).assert()
        }
    }}
}

lazy_static! {
    static ref CLI: PathBuf = escargot::CargoBuild::new()
        .bin("dqcsim")
        .current_release()
        .current_target()
        .features("cli")
        .run()
        .unwrap()
        .path()
        .to_path_buf();
    static ref FRONTEND_PATH: PathBuf = escargot::CargoBuild::new()
        .bin("dqcsfenull")
        .current_release()
        .features("null-plugins")
        .run()
        .unwrap()
        .path()
        .to_path_buf();
    static ref FRONTEND: &'static str = FRONTEND_PATH.to_str().unwrap();
    static ref OPERATOR_PATH: PathBuf = escargot::CargoBuild::new()
        .bin("dqcsopnull")
        .current_release()
        .features("null-plugins")
        .run()
        .unwrap()
        .path()
        .to_path_buf();
    static ref OPERATOR: &'static str = OPERATOR_PATH.to_str().unwrap();
    static ref BACKEND_PATH: PathBuf = escargot::CargoBuild::new()
        .bin("dqcsbenull")
        .current_release()
        .features("null-plugins")
        .run()
        .unwrap()
        .path()
        .to_path_buf();
    static ref BACKEND: &'static str = BACKEND_PATH.to_str().unwrap();
}

#[test]
fn cli_with_macro() {
    cli!(*FRONTEND, *BACKEND).success();
}

#[test]
fn cli_no_arguments() {
    cli!()
        .failure()
        .code(1)
        .stdout(predicate::str::contains(include_str!(
            "../src/bin/dqcsim/arg_parse/usage.txt"
        )));
}

#[test]
fn cli_help() {
    cli!("--help")
        .failure()
        .code(1)
        .stdout(predicate::str::contains("Used to specify the host API call sequence. Refer to the \"host call sequence\" section for more info."));

    cli!("plugin", "--help")
        // TODO: jeroen other helps exit with failure code 1
        .success()
        .stdout(predicates::str::contains("PLUGIN OPTIONS"));
}

#[test]
fn cli_long_help() {
    cli!("--long-help")
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "Run the specified cQASM file using the cQASM frontend and (default) QX backend.",
        ));
}

#[test]
fn cli_version() {
    cli!("--version")
        .failure()
        .code(1)
        .stdout(predicate::str::is_match("^DQCsim [0-9].[0-9].[0-9] (.*)").unwrap());
}

#[test]
fn cli_host_call_no_value() {
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
fn cli_host_call_ok() {
    cli!("--call", "start", "--call", "wait", *FRONTEND, *BACKEND)
        .success()
        .stderr(predicate::str::contains(
            "Executing 'start(...)' host call...",
        ))
        .stderr(predicate::str::contains("Executing 'wait()' host call..."));
}

#[test]
fn cli_host_call_send_no_data() {
    cli!("--call", "send", *FRONTEND, *BACKEND)
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "Invalid value for '--call <call>...': Invalid argument: the send API call requires an ArbData argument",
        ));
}

#[test]
fn cli_host_call_arb_no_data() {
    cli!("--call", "arb", *FRONTEND, *BACKEND)
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "the arb API call requires a plugin and an ArbCmd argument",
        ));
}

#[test]
fn cli_host_call_arb_plugin_not_found() {
    cli!(
        "--call",
        "arb:a:b.c:{\"answer\": 42},x,y,z",
        *FRONTEND,
        *BACKEND
    )
    .failure()
    .stderr(predicate::str::contains(
        "Invalid argument: plugin a not found",
    ));
}

#[test]
fn cli_host_call_arb() {
    cli!(
        "--call",
        "arb:front:b.c:{\"answer\": 42},x,y,z",
        *FRONTEND,
        *BACKEND
    )
    .success()
    .stderr(predicate::str::contains(
        "Executing 'arb(...)' host call...",
    ));

    cli!(
        "--host-stdout",
        "--call",
        "arb:front:b.c:{\"answer\": 42},x,y,z",
        *FRONTEND,
        *BACKEND
    )
    .success()
    .stdout(predicate::str::contains("arb:"))
    .stderr(predicate::str::contains(
        "Executing 'arb(...)' host call...",
    ));
}

#[test]
fn cli_host_call_wait_data() {
    cli!("--call", "wait:{},a.b", *FRONTEND, *BACKEND)
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "the wait API call does not take an argument",
        ));
}

#[test]
fn cli_host_call_recv_data() {
    cli!("--call", "recv:{},a.b", *FRONTEND, *BACKEND)
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "the recv API call does not take an argument",
        ));
}

#[test]
fn cli_host_call_yield_data() {
    cli!("--call", "yield:{},a.b", *FRONTEND, *BACKEND)
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "the yield API call does not take an argument",
        ));
}

#[test]
fn cli_host_call_yield() {
    cli!("--call", "yield", *FRONTEND, *BACKEND)
        .success()
        .stderr(predicate::str::contains("Executing 'yield()' host call..."));
}

#[test]
fn cli_host_call_recv() {
    cli!(
        "--call",
        "send:{},a.b",
        "--call",
        "recv",
        *FRONTEND,
        *BACKEND
    )
    .success()
    .stderr(predicate::str::contains("Executing 'recv()' host call..."));
}

#[test]
fn cli_host_call_recv_deadlock() {
    cli!("--call", "recv", "--call", "recv", *FRONTEND, *BACKEND)
        .failure()
        .stderr(predicate::str::contains("Executing 'recv()' host call..."))
        .stderr(predicate::str::contains(
            "Deadlock: accelerator exited before sending data",
        ));
}

#[test]
fn cli_host_call_send() {
    cli!("--call", "send:{},a.b", *FRONTEND, *BACKEND)
        .success()
        .stderr(predicate::str::contains(
            "Executing 'send(...)' host call...",
        ));
}

#[test]
fn cli_host_call_with_reproduce() {
    cli!(
        "--reproduce",
        "/dev/zero",
        "--call",
        "start",
        *FRONTEND,
        *BACKEND
    )
    .failure()
    .code(1)
    .stdout(predicate::str::contains(
        "The argument '--reproduce <filename>' cannot be used with '--call <call>...",
    ));
}

#[test]
fn cli_host_call_with_reproduce_exactly() {
    cli!(
        "--reproduce-exactly",
        "/dev/zero",
        "-C",
        "start",
        *FRONTEND,
        *BACKEND
    )
    .failure()
    .code(1)
    .stdout(predicate::str::contains(
        "The argument '--reproduce-exactly <filename>' cannot be used with '--call <call>...",
    ));
}

#[test]
fn cli_host_stdout() {
    cli!("--host-stdout", *FRONTEND, *BACKEND)
        .success()
        .stdout(predicate::str::contains("wait(): {}"));
}

#[test]
fn cli_with_operator() {
    cli!(*FRONTEND, *OPERATOR, *BACKEND)
        .success()
        .stderr(predicate::str::contains("op1"));
}

#[test]
fn cli_plugin_config_name() {
    cli!(*FRONTEND, "--name", "frontend-test", *BACKEND)
        .success()
        .stderr(predicate::str::contains("frontend-test"));
}

#[test]
fn cli_plugin_env_mod() {
    cli!(*FRONTEND, "--env", "key:value", *BACKEND).success();
    cli!(*FRONTEND, "--env", "~key", *BACKEND).success();
}

#[test]
fn cli_double_start_insert_wait() {
    cli!("-C", "start", "-C", "start", *FRONTEND, *BACKEND)
        .success()
        .stderr(predicate::str::contains("Executing 'start(...)' host call...").count(2))
        .stderr(predicates::str::contains("Executing 'wait()' host call...").count(2));
}

#[test]
fn cli_bad_path() {
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
fn cli_no_backend() {
    cli!(*FRONTEND)
        .failure()
        .code(1)
        .stdout(predicate::str::contains(
            "While interpreting plugin specification: Invalid argument: could not find plugin executable 'dqcsbeqx', needed for plugin specification 'qx'",
        ));
}

#[test]
fn cli_no_repro_out() {
    cli!(*FRONTEND, "--no-repro-out")
        .failure()
        .code(1)
        .stderr(predicate::str::contains(
            "Found argument '--no-repro-out' which wasn't expected, or isn't valid in this context",
        ));

    cli!("--no-repro-out", *FRONTEND, *BACKEND)
        .success()
        .stderr(predicate::str::contains(
            "Simulation completed successfully.",
        ));
}

#[test]
fn cli_repro_out() {
    cli!("--repro-out", "/not_allowed", *FRONTEND, *BACKEND)
        .success()
        .stderr(predicate::str::contains(
            "When trying to write reproduction file:",
        ));

    cli!("--repro-out", "/tmp/repro-out.out", *FRONTEND, *BACKEND)
        .success()
        .stderr(predicate::str::contains(
            "Simulation completed successfully.",
        ));
}

#[test]
fn cli_no_repro_out_repro_out() {
    cli!(
        "--no-repro-out",
        "--repro-out",
        "/tmp/test.repro",
        *FRONTEND,
        *BACKEND
    )
    .failure()
    .code(1)
    .stdout(predicate::str::contains(
        "The argument '--no-repro-out' cannot be used with '--repro-out <filename>'",
    ));
}

#[test]
fn cli_reproduce_bad_path() {
    cli!("--reproduce", "./asdf")
        .failure()
        .stdout(predicates::str::contains("While reading reproduction file"));
}

#[test]
fn cli_reproduce() {
    cli!(
        "--repro-out",
        "./dqcsim-cli.test.repro",
        *FRONTEND,
        *BACKEND
    )
    .success();
    cli!("--reproduce", "./dqcsim-cli.test.repro").success();

    // illegal name override
    cli!(
        "--reproduce",
        "./dqcsim-cli.test.repro",
        "@front",
        "-n",
        "override-name"
    )
    .failure()
    .stdout(predicates::str::contains(
        "cannot be used when referencing a previously defined plugin",
    ));

    // illegal work override
    cli!(
        "--reproduce",
        "./dqcsim-cli.test.repro",
        "@front",
        "--work",
        "work"
    )
    .failure()
    .stdout(predicates::str::contains(
        "cannot be used when referencing a previously defined plugin",
    ));

    // override verbosity
    cli!(
        "--reproduce",
        "./dqcsim-cli.test.repro",
        "@front",
        "-l",
        "fatal"
    )
    .success();

    // exact reproduce
    cli!(
        "--reproduce-exactly",
        "./dqcsim-cli.test.repro",
        "@front",
        "-l",
        "fatal"
    )
    .success();

    // def with reproduce
    cli!("--reproduce", "./dqcsim-cli.test.repro", *FRONTEND)
        .failure()
        .stdout(predicates::str::contains("Cannot define new plugins while"));

    // mod with def
    cli!(*FRONTEND, *BACKEND, "@front", "-l", "trace")
        .failure()
        .stdout(predicates::str::contains("Cannot modify plugins unless"));
}
