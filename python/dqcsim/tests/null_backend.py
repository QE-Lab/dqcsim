from dqcsim.common import *
from dqcsim.plugin import *
import os

@plugin("Null backend plugin", "Test", "0.1")
class NullBackend(Backend):
    def handle_drop(self):
        self.trace('null backend dropped!')

    def handle_unitary_gate(self, targets, matrix, arb):
        pass

    def handle_measurement_gate(self, qubits, matrix, arb):
        return [Measurement(qubit, 0) for qubit in qubits]

    def handle_prepare_gate(self, targets, matrix, arb):
        pass

    def handle_host_work_env(self):
        return ArbData(work=os.getcwd(), env=dict(os.environ))

if __name__ == '__main__':
    NullBackend().run()
