from dqcsim.plugin import *

@plugin("Null backend plugin", "Test", "0.1")
class NullBackend(Backend):
    def handle_unitary_gate(self, targets, matrix):
        pass

    def handle_measurement_gate(self, qubits):
        return [Measurement(qubit, 0) for qubit in qubits]

NullBackend().run()
