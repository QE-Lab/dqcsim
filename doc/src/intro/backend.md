# Backend use cases

The backend part of a DQCsim simulation is what people usually mean when they
talk about a quantum computer simulator: it simulates the behavior of qubits in
a quantum circuit.

There are various mathematical ways to go about doing this, with their own pros
and cons in terms of accuracy, simulation speed, and memory footprint. This is
primarily why being able to swap out the backend is very powerful.

Another big reason for wanting to swap out backends would be to get access to
different error models. However, it is strongly recommended to describe such
error models in the form of a operator plugins, if it's possible to describe
them by modifying or inserting additional gates.
