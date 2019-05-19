import unittest
from dqcsim.common import *
from dqcsim.host import *
from dqcsim.plugin import *
import logging
import os, sys

@plugin("Null frontend plugin", "Test", "0.1")
class NullFrontend(Frontend):
    def handle_run(self, *args, **kwargs):
        self.send(self.recv())

    def handle_host_log_test(self):
        self.info('__start__')
        self.trace('trace')
        self.debug('debug')
        self.info('info')
        self.note('note')
        self.warn('warn')
        self.error('error')
        self.fatal('fatal')
        self.log(Loglevel.INFO, 'log')
        self.info('__end__')

@plugin("Null operator plugin", "Test", "0.1")
class NullOperator(Operator):
    pass

@plugin("Null backend plugin", "Test", "0.1")
class NullBackend(Backend):
    def handle_unitary_gate(self, targets, matrix):
        pass

    def handle_measurement_gate(self, qubits):
        return [Measurement(qubit, 0) for qubit in qubits]

class Tests(unittest.TestCase):

    def test_meta_and_state_errors(self):
        sim = Simulator(
            NullFrontend(), NullOperator(), (NullBackend(), {'name': 'test'}),
            repro=None, stderr_verbosity=Loglevel.OFF
        )

        self.assertEqual(len(sim), 3)
        with self.assertRaisesRegex(RuntimeError, "No simulation is currently running"):
            sim.stop()
        with self.assertRaisesRegex(RuntimeError, "No simulation is currently running"):
            sim.start()
        with self.assertRaisesRegex(RuntimeError, "No simulation is currently running"):
            sim.wait()
        with self.assertRaisesRegex(RuntimeError, "No simulation is currently running"):
            sim.send()
        with self.assertRaisesRegex(RuntimeError, "No simulation is currently running"):
            sim.recv()
        with self.assertRaisesRegex(RuntimeError, "No simulation is currently running"):
            sim.yeeld()
        with self.assertRaisesRegex(RuntimeError, "No simulation is currently running"):
            sim.arb(0, 'a', 'b')
        with self.assertRaisesRegex(RuntimeError, "No simulation is currently running"):
            sim.get_meta(0)

        sim.simulate()
        with self.assertRaisesRegex(RuntimeError, "multiple simulations"):
            sim.simulate()
        sim.send()
        sim.start()
        sim.yeeld()
        sim.wait()
        sim.recv()
        sim.arb(0, 'a', 'b')
        sim.arb('front', 'a', 'b')
        with self.assertRaisesRegex(RuntimeError, "not found"):
            sim.arb('banana', 'a', 'b')
        sim.start()
        with self.assertRaisesRegex(RuntimeError, "Deadlock"):
            sim.wait()
        sim.send()
        sim.wait()
        sim.recv()
        with self.assertRaisesRegex(RuntimeError, "Deadlock"):
            sim.recv()

        self.assertEqual(sim.get_meta(0), ("Null frontend plugin", "Test", "0.1"))
        self.assertEqual(sim.get_meta(1), ("Null operator plugin", "Test", "0.1"))
        self.assertEqual(sim.get_meta(2), ("Null backend plugin", "Test", "0.1"))
        self.assertEqual(sim.get_meta('front'), ("Null frontend plugin", "Test", "0.1"))
        self.assertEqual(sim.get_meta('op1'), ("Null operator plugin", "Test", "0.1"))
        self.assertEqual(sim.get_meta('test'), ("Null backend plugin", "Test", "0.1"))
        with self.assertRaisesRegex(RuntimeError, "not found"):
            sim.get_meta('banana')
        sim.stop()
        with self.assertRaisesRegex(RuntimeError, "No simulation is currently running"):
            sim.stop()

        self.assertEqual(repr(sim), 'Simulator()')
        self.assertEqual(str(sim), 'Simulator()')

    def test_log_capture_callback(self):
        msgs = []
        capture = [False]
        def log(msg, source, level, mod, fname, line, timestamp, pid, tid):
            if msg == '__start__':
                capture[0] = True
            elif msg == '__end__':
                capture[0] = False
            elif capture[0]:
                msgs.append((msg, source, level, mod, fname, line))
        sim = Simulator(
            NullFrontend(), NullBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF,
            log_capture=log,
        )
        sim.simulate()
        sim.arb('front', 'log', 'test')
        sim.stop()
        self.assertEqual(len(msgs), 8)
        self.assertEqual(msgs[0], (
            'trace', 'front', Loglevel.TRACE,
            'dqcsim.tests.test_simulator', __file__, 15))
        self.assertEqual(msgs[1], (
            'debug', 'front', Loglevel.DEBUG,
            'dqcsim.tests.test_simulator', __file__, 16))
        self.assertEqual(msgs[2], (
            'info', 'front', Loglevel.INFO,
            'dqcsim.tests.test_simulator', __file__, 17))
        self.assertEqual(msgs[3], (
            'note', 'front', Loglevel.NOTE,
            'dqcsim.tests.test_simulator', __file__, 18))
        self.assertEqual(msgs[4], (
            'warn', 'front', Loglevel.WARN,
            'dqcsim.tests.test_simulator', __file__, 19))
        self.assertEqual(msgs[5], (
            'error', 'front', Loglevel.ERROR,
            'dqcsim.tests.test_simulator', __file__, 20))
        self.assertEqual(msgs[6], (
            'fatal', 'front', Loglevel.FATAL,
            'dqcsim.tests.test_simulator', __file__, 21))
        self.assertEqual(msgs[7], (
            'log', 'front', Loglevel.INFO,
            'dqcsim.tests.test_simulator', __file__, 22))

    def test_log_capture_logging(self):
        class Handler(logging.Handler):
            def __init__(self, *args, **kwargs):
                super().__init__(*args, **kwargs)
                self.msgs = []
                self.capture = False

            def emit(self, record):
                if record.msg == '__start__':
                    self.capture = True
                elif record.msg == '__end__':
                    self.capture = False
                elif self.capture:
                    self.msgs.append((
                        record.msg, record.name,
                        record.levelno, record.levelname,
                        record.pathname, record.lineno))
        handler = Handler()
        logger = logging.getLogger('dqcsim')
        logger.addHandler(handler)
        sim = Simulator(
            NullFrontend(), NullBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF,
            log_capture='dqcsim',
        )
        sim.simulate()
        sim.arb('front', 'log', 'test')
        sim.stop()
        self.assertEqual(len(handler.msgs), 8)
        self.assertEqual(handler.msgs[0], ('trace', 'front',  5, 'TRACE',    __file__, 15))
        self.assertEqual(handler.msgs[1], ('debug', 'front', 10, 'DEBUG',    __file__, 16))
        self.assertEqual(handler.msgs[2], ('info',  'front', 20, 'INFO',     __file__, 17))
        self.assertEqual(handler.msgs[3], ('note',  'front', 25, 'NOTE',     __file__, 18))
        self.assertEqual(handler.msgs[4], ('warn',  'front', 30, 'WARNING',  __file__, 19))
        self.assertEqual(handler.msgs[5], ('error', 'front', 40, 'ERROR',    __file__, 20))
        self.assertEqual(handler.msgs[6], ('fatal', 'front', 50, 'CRITICAL', __file__, 21))
        self.assertEqual(handler.msgs[7], ('log',   'front', 20, 'INFO',     __file__, 22))

    def test_manual_spawn(self):
        sim = Simulator(
            lambda sim: NullFrontend().run(sim),
            lambda sim: NullBackend().run(sim),
            repro=None, stderr_verbosity=Loglevel.OFF
        )
        sim.simulate()
        sim.send()
        sim.start()
        sim.wait()
        sim.recv()
        sim.stop()

if __name__ == '__main__':
    unittest.main()
