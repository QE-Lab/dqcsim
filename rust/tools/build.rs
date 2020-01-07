use std::{
    process::{exit, Command},
    str,
};

#[cfg(feature = "bindings")]
use std::{
    fs,
    io::{self, prelude::*},
    path,
};

static CARGOENV: &str = "cargo:rustc-env=";

#[cfg(feature = "bindings")]
fn prepreprocess(path: &path::Path) -> io::Result<String> {
    use lazy_static::lazy_static;
    use regex::{Captures, Regex, RegexBuilder, Replacer};

    println!("cargo:rerun-if-changed={}", path.to_str().unwrap());

    lazy_static! {
        static ref INCLUDE_RE: Regex = RegexBuilder::new("^( *)#include \"([^\"]+)\".*\n")
            .multi_line(true)
            .build()
            .unwrap();
        static ref SOL_RE: Regex = RegexBuilder::new("^").multi_line(true).build().unwrap();
        static ref TRIM_EOL: Regex = RegexBuilder::new(" +$").multi_line(true).build().unwrap();
    }

    struct PrePreprocessReplacer<'t> {
        base: &'t path::Path,
    }

    impl<'t> Replacer for PrePreprocessReplacer<'t> {
        fn replace_append(&mut self, caps: &Captures, dst: &mut String) {
            let indent = caps.get(1).unwrap().as_str();
            let fname = path::Path::new(caps.get(2).unwrap().as_str());

            let text = prepreprocess(&self.base.join(fname))
                .unwrap_or_else(|_| prepreprocess(fname).unwrap());
            let text = SOL_RE.replace_all(&text, indent);
            let text = TRIM_EOL.replace_all(&text, "");
            dst.push_str(&text);
        }
    }

    let mut contents = String::new();
    io::BufReader::new(fs::File::open(path)?).read_to_string(&mut contents)?;
    Ok(INCLUDE_RE
        .replace_all(
            &contents,
            PrePreprocessReplacer {
                base: path.parent().unwrap_or(path::Path::new(".")),
            },
        )
        .to_string())
}

#[cfg(feature = "bindings")]
fn cbindgen() {
    use std::env;

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = format!(
        "{}/../target/include",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );

    let trailer = include_str!("../src/bindings/trailer.inc");
    let header_c = include_str!("../src/bindings/header_c.inc");
    let header_cpp = include_str!("../src/bindings/header_cpp.inc");

    // Generate C headers.
    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::C)
        .with_no_includes()
        .with_header(header_c)
        .with_trailer(trailer)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(format!("{}/dqcsim.h", out_dir));

    // Generate C++ minimal API headers.
    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::Cxx)
        .with_no_includes()
        .with_header(header_cpp)
        .with_namespaces(&["dqcsim", "raw"])
        .with_trailer(trailer)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(format!("{}/cdqcsim", out_dir));

    // Generate C++ extended API headers.
    fs::write(
        format!("{}/dqcsim", out_dir),
        prepreprocess(path::Path::new("../cpp/include/dqcsim")).unwrap(),
    )
    .unwrap();

    // Generate SWIG header.
    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::C)
        .with_line_length(100000)
        .with_documentation(false)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(format!("{}/dqcsim-py.h", out_dir));
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
