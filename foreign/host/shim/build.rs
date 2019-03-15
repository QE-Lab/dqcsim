extern crate cbindgen;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Generate C headers.
    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("c/gen/dqcshost.h");

    // Generate C++ minimal API headers.
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::Cxx)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("cpp/gen/dqcshost.hpp");
}
