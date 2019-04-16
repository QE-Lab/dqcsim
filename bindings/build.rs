extern crate cbindgen;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let trailer = include_str!("trailer.inc");

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
