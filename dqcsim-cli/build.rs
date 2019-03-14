use std::process::{exit, Command};
use std::str;

static CARGOENV: &str = "cargo:rustc-env=";

fn main() {
    let time_c = Command::new("date").args(&["+%F %T"]).output();

    match time_c {
        Ok(t) => {
            let time;
            unsafe {
                time = str::from_utf8_unchecked(&t.stdout);
            }
            println!("{}COMPILED_AT={}", CARGOENV, time);
        }
        Err(_) => exit(1),
    }

    let git_hash_c = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output();

    match git_hash_c {
        Ok(t) => {
            let hash;
            unsafe {
                hash = str::from_utf8_unchecked(&t.stdout).trim();
            }
            println!("{}GIT_HASH={}", CARGOENV, hash);
        }
        Err(_) => exit(1),
    }

    let git_clean_c = Command::new("git").args(&["diff", "--quiet"]).status();

    match git_clean_c {
        Ok(c) => {
            if c.success() {
                println!("{}GIT_CLEAN=clean", CARGOENV);
            } else {
                println!("{}GIT_CLEAN=dirty", CARGOENV);
            }
        }
        Err(_) => exit(1),
    }
}
