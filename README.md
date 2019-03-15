# DQCsim

[![Build](https://img.shields.io/travis/com/mbrobbel/dqcsim-rs.svg?style=flat-square)](https://travis-ci.com/mbrobbel/dqcsim-rs)
[![Docs](https://img.shields.io/badge/docs--brightgreen.svg?style=flat-square)](https://mbrobbel.github.io/dqcsim-rs/doc_/dqcsim/)
[![Book](https://img.shields.io/badge/book--brightgreen.svg?style=flat-square)](https://mbrobbel.github.io/dqcsim-rs/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg?style=flat-square)](https://github.com/mbrobbel/dqcsim-rs/blob/master/LICENSE)

DQCsim: Delft Quantum Classical Simulator

Build using `make` from the root directory to make everything work! It passes control to `cargo make` after making sure it exists, which then handles the build by calling `cargo` all over the place. `cargo make` also chains to some local `Makefile`s to perform installation and non-rust build steps.

> Work in progress
