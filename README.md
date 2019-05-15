# DQCsim

<!--[![Platform](https://badgen.net/badge/platform/Linux,macOS?list=1)]()-->
<!--[![Docs](https://docs.rs/dqcsim/badge.svg)](https://docs.rs/dqcsim)-->
[![License](https://badgen.net/badge/license/Apache-2.0/blue)](https://github.com/mbrobbel/dqcsim-rs/blob/master/LICENSE)
[![Docs](https://badgen.net/badge/docs/%20?color=green)](https://mbrobbel.github.io/dqcsim-rs/)
[![PyPi](https://img.shields.io/pypi/pyversions/dqcsim.svg)](https://pypi.org/project/dqcsim/)
[![Crates](https://badgen.net/crates/v/dqcsim)](https://crates.io/dqcsim)
[![Linux build](https://img.shields.io/drone/build/mbrobbel/dqcsim-rs/master.svg?logo=linux&logoColor=white)](https://cloud.drone.io/mbrobbel/dqcsim-rs)
[![macOS build](https://img.shields.io/travis/com/mbrobbel/dqcsim-rs/master.svg?label=build&logo=apple&logoColor=white)](https://travis-ci.com/mbrobbel/dqcsim-rs)
[![Codecov](https://badgen.net/codecov/c/github/mbrobbel/dqcsim-rs)](https://codecov.io/gh/mbrobbel/dqcsim-rs)
[![Dependencies](https://deps.rs/repo/github/mbrobbel/dqcsim-rs/status.svg)](https://deps.rs/repo/github/mbrobbel/dqcsim-rs)

DQCsim, short for Delft Quantum & Classical simulator, is a *framework* that can be used to tie *components* of quantum computer simulators together in a *standardized* yet *flexible*, *developer-friendly*, and *reproducible* way. Click the keywords below for more information!

<details><summary><i>Framework</i></summary><table><tr><td>

DQCsim only provides interfaces to tie simulator components together. That is, it does not contain any simulation code on its own. DQCsim is all the boilerplate code that you don't want to write when you're developing a new way to simulate qubits, a microarchitecture simulator, an error model, etc.

</td></tr></table></details>
<details><summary><i>Components</i></summary><table><tr><td>

DQCsim abstracts a quantum computer simulation into four components: backends, frontends, operators, and hosts. These components are separate operating system processes that each fulfill a well-defined function within the simulation, thus splitting the simulation up into more manageable parts. Briefly:

 - Backends deal with the mathematics of simulating (perfect) qubits.
 - Frontends deal with simulating a particular quantum architecture. Note that this may also be a "dummy" architecture that runs quantum-computer-agnostic code like cQASM, or even just the quantum algorithm itself if it is expressed as a sequence of DQCsim API calls.
 - Operators sit between a frontend and backend to monitor or manipulate the gate and measurement streams, for instance to introduce errors, verify architecture constraints, perform runtime remapping from logical to physical qubits, and so on.
 - The host program, if any, is any computer program that makes use of DQCsim as a quantum accelerator. The ultimate goal is for DQCsim's interface to be generic enough that it can simply be swapped out with a real quantum accelerator without requiring any changes to the host.

Backends, frontends, and operators are collectively called plugins with respect to DQCsim. For a host it's the other way around; it uses DQCsim as a plugin.

</td></tr></table></details>
<details><summary><i>Standardized</i></summary><table><tr><td>

DQCsim fully specifies a set of core features that each component needs to support, as well as the interfaces used to drive them. Therefore, as long as the components don't rely on any user-defined extra features in other components, they can be swapped out without breaking anything.

</td></tr></table></details>
<details><summary><i>Flexible</i></summary><table><tr><td>

Besides standardizing the basic features of each component, DQCsim provides powerful ways for users to implement their own features, without needing to change anything in DQCsim's codebase. So don't panic about DQCsim being written in Rust: you shouldn't need to read or write a single line of code in here!

</td></tr></table></details>
<details><summary><i>Developer-friendly</i></summary><table><tr><td>

All the components can be written in Python, C, C++, or Rust. Just use whichever language you prefer. No need to deal with CPython and whatnot when the frontend is written in Python and the backend in C++! Also, since the components are fully separated from one another, you don't need to read a single line of code of the components you're not interested in, and someone else's bug can never cause a segfault in your code.

</td></tr></table></details>
<details><summary><i>Reproducible</i></summary><table><tr><td>

While quantum mechanics are inherently stochastic, simulating it needs not be. DQCsim provides a random generator to the components that should be more than random enough for simulation purposes, while being reproducible when this is desirable, such as while debugging. DQCsim's interprocess communication, while inherently multithreaded, is also written to be deterministic; OS scheduling should never influence a simulation. Finally, after running a simulation once with a (complicated) host program, any non-deterministic behavior in it can be taken out of the equation by replaying its interaction with the frontend through a reproduction file.

</td></tr></table></details>

## Installation

If you're a user or a plugin developer (Python, C, or C++), the recommended way to install DQCsim is through Python's package manager:

    $ sudo pip3 install dqcsim

This will install just the DQCsim core files and so-called "null" plugins for testing. So you'll also want to install plugins in addition. This is currently TODO, because there are no supported plugins yet. However, the current idea is that these will also be distributed through pip, with a dependency on `dqcsim`. For instance, you should be able to install `dqcsim-qx` through pip to get the QX simulator with appropriate DQCsim bindings.

<details><summary><i>What if I don't have superuser access, or don't want to pollute my root directory?</i></summary><table><tr><td>

There are two alternatives:

<details><summary>Install into your home directory.</summary><table><tr><td>

Run

    $ pip3 install dqcsim --user

This will normally install the package into `~/.local`. You should probably check if `~/.local/bin` is in your `$PATH` environment variable, otherwise the command-line interface and plugins may not work out of the box. If you're developing in C or C++, you'll also have to add the following to `CFLAGS`: `-I ~/.local/include -L ~/.local/lib`.

</td></tr></table></details>
<details><summary>Use a <a href="https://docs.python.org/3/library/venv.html#creating-virtual-environments">venv</a>.</summary><table><tr><td>

To do this, create an install directory wherever you want, and then run

    $ python3 -m venv <your-install-directory>
    $ source <your-install-directory>/bin/activate
    (venv) $ pip3 install dqcsim

To leave the `venv`, run

    (venv) $ deactivate

If you're developing in C or C++, you'll also have to add the following to `CFLAGS`: `-I <your-install-directory>/include -L <your-install-directory>/lib`.

</td></tr></table></details>
</td></tr></table></details>
<details><summary><i>What if I don't want to use Python?</i></summary><table><tr><td>

It's strongly recommended to use pip anyway - we chose pip because it works best for all languages involved and is distribution-independent, not because it's Python. If you're really adamant on avoiding Python, read the build notes for DQCsim developers below and use Cargo exclusively.

</td></tr></table></details>
<details><summary><i>What if I want the bleeding edge, or want a specific version?</i></summary><table><tr><td>

You'll probably want to build from source. Read the build notes in the contributing section below.

</td></tr></table></details>
<details><summary><i>What about developing plugins in Rust?</i></summary><table><tr><td>

While this is not a use case we're particularly expecting, it should be pretty easy since DQCsim is itself written in Rust. Just add `dqcsim` to your Cargo dependencies and read the [crate documentation](https://mbrobbel.github.io/dqcsim-rs/doc_/dqcsim/).

</td></tr></table></details>
<details><summary><i>What about Windows? And what about developing a plugins in other languages?</i></summary><table><tr><td>

DQCsim is currently restricted to Linux and macOS due to the Rust crate we're using for interprocess communication. Eventually, we want to make a secondary interprocess communication interface that uses TCP and a more user-friendly protocol than whatever Rust serializes to by default. This should make plugin development possible in any language that lets you talk to TCP sockets and allow you to use any operating system.

</td></tr></table></details>

## Getting started

TODO

## Contributing to DQCsim

TODO: until we (jvanstraten and mbrobbel) get a first release going, contributing is probably more trouble than it's worth.

<details><summary>Click to expand anyway...</summary><table><tr><td>

### Code style

For Rust code this is simple: always apply `cargo format` and `cargo clippy -Dwarnings` before committing. The CI will fail if your code does not comply. For C, C++, and Python, there isn't really a specific code style defined right now; please just try to mimic the existing code.

Any tab character that isn't required by the language (looking at you, Make) will be shot on sight.

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

</details></td></tr></table>
