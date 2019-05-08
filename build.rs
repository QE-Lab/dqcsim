use std::{
    process::{exit, Command},
    str,
};

static CARGOENV: &str = "cargo:rustc-env=";

#[cfg(feature = "bindings")]
fn cbindgen() {
    use std::env;

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let trailer = include_str!("src/bindings/trailer.inc");

    // Generate C headers.
    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::C)
        .with_trailer(trailer)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("target/dqcsim.h");

    // Generate C++ minimal API headers.
    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::Cxx)
        .with_include("unistd.h")
        .with_include("stdio.h")
        .with_namespace("dqcsim")
        .with_trailer(trailer)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("target/dqcsim_raw.hpp");

    // Generate SWIG header.
    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::C)
        .with_line_length(100000)
        .with_documentation(false)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("target/dqcsim-py.h");
}

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

    #[cfg(feature = "bindings")]
    cbindgen();
}
