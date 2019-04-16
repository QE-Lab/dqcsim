import unittest
from dqcsim.common import *
from dqcsim.host import *
from dqcsim.plugin import *
import logging

class Tests(unittest.TestCase):

    def test_1(self):
        @plugin("My frontend plugin", "Me!", "3.14")
        class MyFrontend(Frontend):
            def handle_run(self, *args, **kwargs):
                self.info("Running with: {}, {}", args, kwargs)
                q = self.allocate()
                s, t = self.allocate(2)
                self.unitary(q, [0.0, 1.0, 1.0, 0.0], [s, t])
                self.measure(q)
                self.arb('a', 'b', a=True)
                self.info("Measurement: {}", self.get_measurement(q))
                return ArbData(*args, **kwargs)

        class MyBackend(Backend):
            def get_name(self):
                return "My backend plugin"

            def get_author(self):
                return "Me too!"

            def get_version(self):
                return "1.41"

            def handle_upstream_a_b(self, *args, **kwargs):
                self.info("hello! {}, {}", args, kwargs)

            def handle_unitary_gate(self, targets, matrix):
                self.info("Unitary gate: {}, {}", targets, matrix)

            def handle_measurement_gate(self, qubits):
                self.info("Measurement gate: {}", qubits)
                return [Measurement(qubit, 0) for qubit in qubits]

        l = logging.getLogger('dqcsim')
        l.setLevel(logging.NOTSET)
        ch = logging.StreamHandler()
        ch.setLevel(logging.NOTSET)
        formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
        ch.setFormatter(formatter)
        l.addHandler(ch)

        Simulator(MyFrontend(), MyBackend(), repro=None, stderr_verbosity=Loglevel.OFF, log_capture='dqcsim', log_capture_verbosity=Loglevel.INFO).run()


if __name__ == '__main__':
    unittest.main()
