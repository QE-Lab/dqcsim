# Summary

- [Introduction to DQCsim](./index.md)
  - [The components of a simulation](./intro/components.md)
    - [Frontend use cases](./intro/frontend.md)
    - [Backend use cases](./intro/backend.md)
    - [Operator use cases](./intro/operator.md)
    - [Host use cases](./intro/host.md)
  - [Jigsaw puzzle analogy](./intro/puzzle.md)
  - [DQCsim's interfaces](./intro/interfaces.md)
    - [ArbData and ArbCmds](./intro/arbs.md)
    - [Gate- and measurement streams](./intro/gatestream.md)
    - [The host interface](./intro/host-iface.md)
    - [Miscellaneous interfaces](./intro/misc-iface.md)
  - [Reproducibility](./intro/reproducibility.md)
- [Installation](./install/index.md)
  - [Plugin distribution](./install/plugins.md)
- [The command-line interface](./cli/index.md)
- [Python API](./python-api/index.md)
  - [Hello, world!](./python-api/hello-world.md)
  - [Debugging](./python-api/debugging.md)
  - [Sending some gates](./python-api/sending-gates.md)
  - [Controlling simulations](./python-api/simulations.md)
  - [Inserting an operator](./python-api/operator.md)
  - [Reference](./python-api/reference.md)
- [C API](./c-api/index.md)
  - [Usage](./c-api/usage.md)
  - [Concepts](./c-api/concepts.apigen.md)
    - [Handles: dqcs_handle_*](./c-api/handle.apigen.md)
    - [Memory management](./c-api/memory-management.apigen.md)
    - [Error handling: dqcs_error_*](./c-api/error.apigen.md)
    - [Callbacks](./c-api/callbacks.apigen.md)
  - [Type definitions: dqcs_*_t](./c-api/type-definitions.apigen.md)
  - [ArbData and ArbCmd objects](./c-api/arb-cmd-cq.apigen.md)
    - [ArbData objects: dqcs_arb_*](./c-api/arb.apigen.md)
    - [ArbCmd objects: dqcs_cmd_*](./c-api/cmd.apigen.md)
    - [ArbCmd queues: dqcs_cq_*](./c-api/cq.apigen.md)
  - [Qubits: dqcs_qbset_*](./c-api/qbset.apigen.md)
  - [Matrices: dqcs_mat_*](./c-api/mat.apigen.md)
  - [Gates: dqcs_gate_*](./c-api/gate.apigen.md)
  - [Gate maps: dqcs_gm_*](./c-api/gm.apigen.md)
  - [Measurements](./c-api/measurements.apigen.md)
    - [Singular measurements: dqcs_meas_*](./c-api/meas.apigen.md)
    - [Measurement sets: dqcs_mset_*](./c-api/mset.apigen.md)
  - [Plugins](./c-api/plugins.apigen.md)
    - [Defining a plugin: dqcs_pdef_*](./c-api/pdef.apigen.md)
    - [Running a plugin: dqcs_plugin_*](./c-api/plugin-run.apigen.md)
    - [Interacting with DQCsim: dqcs_plugin_*](./c-api/plugin-interact.apigen.md)
    - [Logging: dqcs_log_*](./c-api/log.apigen.md)
  - [Simulations](./c-api/simulations.apigen.md)
    - [Configuring plugins: dqcs_pcfg_*](./c-api/pcfg.apigen.md)
    - [Running local plugins: dqcs_tcfg_*](./c-api/tcfg.apigen.md)
    - [Configuring a simulation: dqcs_scfg_*](./c-api/scfg.apigen.md)
    - [Running a simulation: dqcs_sim_*](./c-api/sim.apigen.md)
  - [Reference](./c-api/reference.apigen.md)
- [C++ API](./cpp-api/index.md)
  - [Usage](./cpp-api/usage.md)
  - [Comparison to the C API](./cpp-api/ccompare.md)
  - [Plugin anatomy](./cpp-api/plugin.md)
  - [Host/simulation anatomy](./cpp-api/sim.md)
  - [Reference](./cpp-api/reference.md)
- [Rust API](./rust-api/index.md)

[Release](./release.md)
