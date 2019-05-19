import unittest
from dqcsim.common import *
from dqcsim.host import *
from dqcsim.plugin import *
import logging
import os, sys

os.environ['PYTHONPATH'] = ':'.join([os.getcwd() + '/python'] + os.environ['PYTHONPATH'].split(':'))

#sys.path.insert(0, os.getcwd())

class Tests(unittest.TestCase):

    def test_null_threads(self):
        @plugin("Null frontend plugin", "Test", "0.1")
        class NullFrontend(Frontend):
            def handle_run(self, *args, **kwargs):
                pass

        @plugin("Null operator plugin", "Test", "0.1")
        class NullOperator(Operator):
            pass

        @plugin("Null backend plugin", "Test", "0.1")
        class NullBackend(Backend):
            def handle_unitary_gate(self, targets, matrix):
                pass

            def handle_measurement_gate(self, qubits):
                return [Measurement(qubit, 0) for qubit in qubits]

        Simulator(
            NullFrontend(), NullOperator(), NullBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF
        ).run()

    def test_null_processes(self):
        print(os.environ['PATH'], file=sys.stderr)
        p = os.path.dirname(__file__) + '/'
        Simulator(
            p+'null_frontend.py', p+'null_operator.py', p+'null_backend.py',
            repro=None, stderr_verbosity=Loglevel.OFF
        ).run()

if __name__ == '__main__':
    unittest.main()
