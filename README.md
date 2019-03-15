# DQCsim

[![Build Status](https://travis-ci.com/mbrobbel/dqcsim-rs.svg?token=kqmepcprqUJV8x3zhy5x&branch=master)](https://travis-ci.com/mbrobbel/dqcsim-rs)
[![Docs](https://img.shields.io/badge/docs--brightgreen.svg)](https://mbrobbel.github.io/dqcsim-rs/doc_/dqcsim/)
[![Book](https://img.shields.io/badge/book--brightgreen.svg)](https://mbrobbel.github.io/dqcsim-rs/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/mbrobbel/dqcsim-rs/blob/master/LICENSE)

DQCsim: Delft Quantum Classical Simulator

Build using `make` from the root directory to make everything work! It passes control to `cargo make` after making sure it exists, which then handles the build by calling `cargo` all over the place. `cargo make` also chains to some local `Makefile`s to perform installation and non-rust build steps.

> Work in progress
