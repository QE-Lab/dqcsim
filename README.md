# DQCsim

[![License](https://badgen.net/badge/license/Apache-2.0/blue)](https://github.com/mbrobbel/dqcsim-rs/blob/master/LICENSE)
[![Docs](https://badgen.net/badge/docs/%20?color=green)](https://mbrobbel.github.io/dqcsim-rs/)
[![PyPi](https://img.shields.io/pypi/pyversions/dqcsim.svg)](https://pypi.org/project/dqcsim/)
[![Crates](https://badgen.net/crates/v/dqcsim)](https://crates.io/crates/dqcsim)
[![Linux build](https://img.shields.io/drone/build/mbrobbel/dqcsim-rs/master.svg?logo=linux&logoColor=white)](https://cloud.drone.io/mbrobbel/dqcsim-rs)
[![macOS build](https://img.shields.io/travis/com/mbrobbel/dqcsim-rs/master.svg?label=build&logo=apple&logoColor=white)](https://travis-ci.com/mbrobbel/dqcsim-rs)
[![Codecov](https://badgen.net/codecov/c/github/mbrobbel/dqcsim-rs)](https://codecov.io/gh/mbrobbel/dqcsim-rs)
[![Dependencies](https://deps.rs/repo/github/mbrobbel/dqcsim-rs/status.svg)](https://deps.rs/repo/github/mbrobbel/dqcsim-rs)



DQCsim, short for Delft Quantum & Classical simulator, is a *framework* that
can be used to tie *components* of quantum computer simulators together in a
*standardized* yet *flexible*, *developer-friendly*, and *reproducible* way.
Click [here](https://mbrobbel.github.io/dqcsim-rs/) for more information!

## Installation

If you're a user or a plugin developer (Python, C, or C++), the recommended
way to install DQCsim is through Python's package manager:

    sudo pip3 install dqcsim

This will install just the DQCsim core files and so-called "null" plugins for
testing. So you'll also want to install plugins in addition. This is currently
TODO, because there are no supported plugins yet. However, the current idea is
that these will also be distributed through pip, with a dependency on
`dqcsim`. For instance, you should be able to install `dqcsim-qx` through pip
to get the QX simulator with appropriate DQCsim bindings.

You can find more information
[here](https://mbrobbel.github.io/dqcsim-rs/install/).

## Getting started

Read the [documentation](https://mbrobbel.github.io/dqcsim-rs/)!

## Reporting bugs

We use github's issue tracker. Click
[here](https://github.com/mbrobbel/dqcsim-rs/issues/new) to open a new issue.

## Contributing to DQCsim

TODO: until we (jvanstraten and mbrobbel) get a first release going,
contributing is probably more trouble than it's worth.

### What to contribute?

Check github's [issue tracker](https://github.com/mbrobbel/dqcsim-rs/issues) to
see what we're working on and what needs to be done.

### Code style

For Rust code this is simple: always apply `cargo format` and
`cargo clippy -Dwarnings` before committing. The CI will fail if your code does
not comply. For C, C++, and Python, there isn't really a specific code style
defined right now; please just try to mimic the existing code.

Any tab character that isn't required by the language (looking at you, Make)
will be shot on sight.

### Building

Within the root directory of the repository resides a Cargo workspace, a Python `setup.py` (using `setuptools`), and a CMake buildsystem. Since we're using pip for distribution, `setup.py` is the master: running `python3 setup.py build` will chain to Cargo to build the Rust modules before building the Python-specific things. The C/C++ API is currently header-only, so it doesn't need to do anything with CMake. Running `python3 setup.py bdist_wheel` after the build will produce a wheel file in `target/python/dist`, which you can then install into a Python venv using pip; just replace `dqcsim` in the install notes above with the wheel file.

### Testing

Testing is done by the buildsystem associated with the language:

 - `cargo test` will run the core test suite for DQCsim and its command-line interface.
 - TODO will run the C/C++ API tests.
 - `python3 setup.py test` will run the Python API test suite.

### Code coverage

TODO (read the drone/travis build instructions)

### Version management & distribution

TODO
