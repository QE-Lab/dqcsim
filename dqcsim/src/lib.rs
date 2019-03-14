//! Delft Quantum Classical Simulator
//!
//! The `dqcsim` crate provides all the required components to build and run
//! quantum classical simulations.
//!
//! # Use
//!
//! ## Simulation
//!
//! The `dqcsim` library can be used to build and drive a simulation, however,
//! it may be more convenient to use the provided binary crate, `dqcsim-cli`,
//! which provides an advanced command-line interface wrapper for this crate.
//!
//! ## Rust crates
//!
//! Rust crates can directly use the `dqcsim` crate to implement simulator
//! plugins, i.e. by implementing the [`TODO`] trait.
//!
//! ## Other languages
//!
//! ...
//!
//! # Concepts
//!
//! A quantum classical [`Simulation`].
//!
//! ...
//!
//! A simulation [`Plugin`].
//!
//! ...
//!
//! [`Plugin`]: ./plugin/struct.Plugin.html
//! [`Simulation`]: ./simulator/struct.Simulation.html

/// Plugin control structure.
pub mod plugin;

/// Simulator instance.
pub mod simulator;

/// Configuration structures for the plugins and simulator.
pub mod configuration;

/// Simulation run reproduction functionality.
pub mod reproduction;

/// Defines the protocols for all forms of communication.
pub mod protocol;

/// Utility functions and modules.
pub mod util;

/// IPC functionality.
pub mod ipc;

/// Logging.
pub use dqcsim_log as log;
