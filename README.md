# DQCsim

[![PyPi](https://badgen.net/pypi/v/dqcsim)](https://pypi.org/project/dqcsim/)
[![Crates.io](https://badgen.net/crates/v/dqcsim)](https://crates.io/crates/dqcsim/)
[![Rust workflow](https://github.com/qe-lab/dqcsim/workflows/Rust/badge.svg)](https://github.com/qe-lab/dqcsim/actions?query=workflow%3ARust)
[![Python workflow](https://github.com/qe-lab/dqcsim/workflows/Python/badge.svg)](https://github.com/qe-lab/dqcsim/actions?query=workflow%3APython)
[![C++ workflow](https://github.com/qe-lab/dqcsim/workflows/C++/badge.svg)](https://github.com/qe-lab/dqcsim/actions?query=workflow%3AC++)
[![Documentation workflow](https://github.com/qe-lab/dqcsim/workflows/Documentation/badge.svg)](https://qe-lab.github.io/dqcsim/)
[![Codecov.io](https://codecov.io/gh/qe-lab/dqcsim/branch/master/graph/badge.svg)](https://codecov.io/gh/qe-lab/dqcsim)

DQCsim, short for Delft Quantum & Classical simulator, is a *framework* that
can be used to tie *components* of quantum computer simulators together in a
*standardized* yet *flexible*, *developer-friendly*, and *reproducible* way.
Click [here](https://qe-lab.github.io/dqcsim/) for more information!

## Install

The recommended method to install DQCsim is through Python's package manager:

```bash
sudo pip3 install dqcsim
```

This installs DQCsim's core files and plugins. More information is available in
the [Installation](https://qe-lab.github.io/dqcsim/install/) section of the
[documentation](https://qe-lab.github.io/dqcsim/).

## Getting started

### Users

New users are encouraged to check out the
[documentation](https://qe-lab.github.io/dqcsim/).

### Plugin developers

Plugin developers can check out the [examples](./examples), existing [plugins](#plugins)
and refer to the API documentation:

- [Python](https://qe-lab.github.io/dqcsim/python-api/) ([Reference](https://qe-lab.github.io/dqcsim/py_/dqcsim/))
- [C++](https://qe-lab.github.io/dqcsim/cpp-api/) ([Reference](https://qe-lab.github.io/dqcsim/cpp_/))
- [C](https://qe-lab.github.io/dqcsim/c-api/) ([Reference](https://qe-lab.github.io/dqcsim/c-api/reference.apigen.html))
- [Rust](https://qe-lab.github.io/dqcsim/rust-api/) ([Reference](https://qe-lab.github.io/dqcsim/rust_/dqcsim/))

## Plugins

### Frontend

| Plugin | Description | Download | License | Platforms | Language |
|:-------|:------------|:---------|:--------|:----------|:---------|
| [openqasm](https://github.com/mbrobbel/dqcsim-openqasm) | OpenQASM 2.0 frontend | [![Crates.io](https://badgen.net/crates/v/dqcsim-openqasm)](https://crates.io/crates/dqcsim-openqasm/) | Apache-2.0 | Linux, macOS | Rust |
| [cqasm](https://github.com/jvanstraten/dqcsim-cqasm) | cQASM 1.0 frontend | [![PyPi](https://badgen.net/pypi/v/dqcsim-cqasm)](https://pypi.org/project/dqcsim-cqasm/) | Apache-2.0 | Linux, macOS | C++ |
| [null](rust/src/bin/null/) | No-op frontend | [![PyPi](https://badgen.net/pypi/v/dqcsim)](https://pypi.org/project/dqcsim/) | Apache-2.0 | Linux, macOS | Rust |

### Operator

| Plugin | Description | Download | License | Platforms | Language |
|:-------|:------------|:---------|:--------|:----------|:---------|
| [openql-mapper](https://github.com/QE-LAB/dqcsim-openql-mapper) | OpenQL mapper operator | [![PyPi](https://badgen.net/pypi/v/dqcsim-openql-mapper)](https://pypi.org/project/dqcsim-openql-mapper/) | Apache-2.0 | Linux | C++ |
| [null](rust/src/bin/null/) | No-op operator | [![PyPi](https://badgen.net/pypi/v/dqcsim)](https://pypi.org/project/dqcsim/) | Apache-2.0 | Linux, macOS | Rust |

### Backend

| Plugin | Description | Download | License | Platforms | Language |
|:-------|:------------|:---------|:--------|:----------|:---------|
| [quantumsim](https://github.com/QE-LAB/dqcsim-quantumsim) | Quantumsim backend | [![PyPi](https://badgen.net/pypi/v/dqcsim-quantumsim)](https://pypi.org/project/dqcsim-quantumsim/) | GPL-3.0 | Linux, macOS | Python |
| [qx](https://github.com/QE-LAB/dqcsim-qx) | QX backend | [![PyPi](https://badgen.net/pypi/v/dqcsim-qx)](https://pypi.org/project/dqcsim-qx/) | Apache-2.0 | Linux, macOS | C++ |
| [null](rust/src/bin/null/) | No-op backend | [![PyPi](https://badgen.net/pypi/v/dqcsim)](https://pypi.org/project/dqcsim/) | Apache-2.0 | Linux, macOS | Rust |
| [iqs](https://github.com/UB-Quantic/dqcsim-iqs) | Intel QS backend | [![GitHub](https://badgen.net/github/release/UB-Quantic/dqcsim-iqs)](https://github.com/UB-Quantic/dqcsim-iqs/releases/latest) | Apache-2.0 | Linux, macOS | C++ |

Please open a PR to have your plugin added to this list.

## Build and test from source

**Setup**

The core of DQCsim is written in Rust. The crate defines a set of C-bindings to
support plugin development in other languages. DQCsim is distributed as a
*batteries included* Python package that includes the shared library and
headers for C and C++ plugin development.

**Requirements**

- [Rust](https://rustup.rs/) (stable)

Python support:

- [Python](https://www.python.org/downloads/) (3.5+)
- [Swig](https://github.com/swig/swig/)

C/C++ tests:

- [CMake](https://github.com/Kitware/CMake) (3.14+)

Documentation:

- [mdbook](https://github.com/rust-lang/mdBook)
- [pdoc](https://pypi.org/project/pdoc/)
- [Doxygen](https://github.com/doxygen/doxygen)

### Python

To build the `dqcsim` Python package:

```bash
python3 setup.py bdist_wheel
```

This builds a release wheel to `target/python/dist/`. For debug builds set the
`DQCSIM_DEBUG` environment variable.

### C/C++

To build the C and C++ headers build the `dqcsim` Rust crate with the
`bindings` feature enabled:

```bash
cargo build --manifest-path=rust/Cargo.toml --features=bindings
```

The generated headers are stored in `target/include`.

### Rust

The `dqcsim` crate can be built with the following (non-default) features:

- `cli`:  the command-line interface binary
- `null-plugins`: the null (no-op) plugin binaries
- `bindings`: genertion of headers required for C, C++ and Python plugin
  development

To build all targets and features:

```bash
cargo build --all-targets --all-features
```

Add `--release` for release builds.

### Documentation

To build the documentation use the [Makefile](./doc/Makefile) in the
[doc](./doc) directory directly from the root of the repository:

```bash
make -C doc
```

Documentation output is stored in `target/book`.

### Test

#### Rust

To test all targets and features:

```bash
cargo test --all-targets --all-features
```

#### C/C++

To test the C-bindings and C++ wrapper:

```bash
mkdir build
cd build
cmake .. -DBUILD_TESTS=ON
make
CTEST_OUTPUT_ON_FAILURE=1 make test
```

Add `-DCMAKE_BUILD_TYPE=DEBUG` to CMake for debug builds.

#### Python

To test the Python package:

```bash
python3 setup.py build test
```
