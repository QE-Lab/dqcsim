import unittest, logging, os, sys, tempfile
from dqcsim.common import *
from dqcsim.host import *
from dqcsim.plugin import *

@plugin("Null frontend plugin", "Test", "0.1")
class NullFrontend(Frontend):
    def __init__(self, *args, **kwargs):
        super().__init__()
        self.arbs_received = []

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
        self.log(Loglevel.INFO, 'log {x} {0}', 'test', x='33')
        try:
            self.log(33, 'oops')
        except TypeError as e:
            self.log(Loglevel.ERROR, str(e))
        self.info('__end__')

    def handle_host_x_y(self, *args, **kwargs):
        self.arbs_received.append({'iface': 'x', 'oper': 'y', 'args': args, 'kwargs': kwargs})

    def handle_host_y_z(self, *args, **kwargs):
        self.arbs_received.append({'iface': 'y', 'oper': 'z', 'args': args, 'kwargs': kwargs})

    def handle_host_get_arbs(self):
        return ArbData(data=self.arbs_received)

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
            elif capture[0] and source == 'front':
                msgs.append((msg, level, mod, fname, line))
        sim = Simulator(
            NullFrontend(), NullBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF,
            log_capture=log,
        )
        sim.simulate()
        sim.arb('front', 'log', 'test')
        sim.stop()
        self.assertEqual(len(msgs), 9)
        self.assertEqual(msgs[0], (
            'trace', Loglevel.TRACE,
            'dqcsim.tests.test_simulator', __file__, 17))
        self.assertEqual(msgs[1], (
            'debug', Loglevel.DEBUG,
            'dqcsim.tests.test_simulator', __file__, 18))
        self.assertEqual(msgs[2], (
            'info', Loglevel.INFO,
            'dqcsim.tests.test_simulator', __file__, 19))
        self.assertEqual(msgs[3], (
            'note', Loglevel.NOTE,
            'dqcsim.tests.test_simulator', __file__, 20))
        self.assertEqual(msgs[4], (
            'warn', Loglevel.WARN,
            'dqcsim.tests.test_simulator', __file__, 21))
        self.assertEqual(msgs[5], (
            'error', Loglevel.ERROR,
            'dqcsim.tests.test_simulator', __file__, 22))
        self.assertEqual(msgs[6], (
            'fatal', Loglevel.FATAL,
            'dqcsim.tests.test_simulator', __file__, 23))
        self.assertEqual(msgs[7], (
            'log 33 test', Loglevel.INFO,
            'dqcsim.tests.test_simulator', __file__, 24))
        self.assertEqual(msgs[8], (
            'level must be a Loglevel', Loglevel.ERROR,
            'dqcsim.tests.test_simulator', __file__, 28))

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
                elif self.capture and record.name == 'front':
                    self.msgs.append((
                        record.msg,
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
        self.assertEqual(len(handler.msgs), 9)
        self.assertEqual(handler.msgs[0], ('trace',                     5, 'TRACE',    __file__, 17))
        self.assertEqual(handler.msgs[1], ('debug',                    10, 'DEBUG',    __file__, 18))
        self.assertEqual(handler.msgs[2], ('info',                     20, 'INFO',     __file__, 19))
        self.assertEqual(handler.msgs[3], ('note',                     25, 'NOTE',     __file__, 20))
        self.assertEqual(handler.msgs[4], ('warn',                     30, 'WARNING',  __file__, 21))
        self.assertEqual(handler.msgs[5], ('error',                    40, 'ERROR',    __file__, 22))
        self.assertEqual(handler.msgs[6], ('fatal',                    50, 'CRITICAL', __file__, 23))
        self.assertEqual(handler.msgs[7], ('log 33 test',              20, 'INFO',     __file__, 24))
        self.assertEqual(handler.msgs[8], ('level must be a Loglevel', 40, 'ERROR',    __file__, 28))

    def test_manual_spawn(self):
        trace_fn = sys.gettrace()
        def run_frontend(sim):
            if trace_fn is not None:
                sys.settrace(trace_fn)
            NullFrontend().run(sim)
        def start_backend(sim):
            if trace_fn is not None:
                sys.settrace(trace_fn)
            NullBackend().start(sim).wait()
        sim = Simulator(
            run_frontend, start_backend,
            repro=None, stderr_verbosity=Loglevel.OFF
        )
        sim.simulate()
        sim.send()
        sim.start()
        sim.wait()
        sim.recv()
        sim.stop()

    def test_late_specification(self):
        sim = Simulator(repro=None, stderr_verbosity=Loglevel.OFF)
        with self.assertRaisesRegex(RuntimeError, "Frontend plugin was never specified"):
            sim.simulate()

        sim.with_frontend(NullFrontend())
        sim.with_backend(NullBackend())
        sim.simulate()
        sim.stop()

        sim.with_operator(NullOperator())
        sim.simulate()

        with self.assertRaisesRegex(RuntimeError, 'Cannot reconfigure simulation while it is running'):
            sim.with_operator(NullOperator())
        sim.stop()

        with self.assertRaises(TypeError):
            sim.with_operator(NullFrontend())

    def test_with_syntax(self):
        with Simulator(NullFrontend(), NullBackend(), repro=None, stderr_verbosity=Loglevel.OFF) as sim:
            sim.send()
            sim.start()
            sim.wait()
            sim.recv()

            sim.send()
            sim.run()
            sim.recv()

    def test_init_arbs(self):
        sim = Simulator(
            (NullFrontend(), {'init': ArbCmd('x', 'y')}), NullOperator(), NullBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF
        )
        sim.simulate()
        self.assertEqual(sim.arb('front', 'get', 'arbs')['data'], [
            {'iface': 'x', 'oper': 'y', 'args': [], 'kwargs': {}},
        ])
        sim.stop()

        sim = Simulator(
            (NullFrontend(), {'init': [
                ArbCmd('x', 'y', b'a'), ArbCmd('y', 'z', b'b')
            ]}), NullOperator(), NullBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF
        )
        sim.simulate()
        self.assertEqual(sim.arb('front', 'get', 'arbs')['data'], [
            {'iface': 'x', 'oper': 'y', 'args': [b'a'], 'kwargs': {}},
            {'iface': 'y', 'oper': 'z', 'args': [b'b'], 'kwargs': {}},
        ])
        sim.stop()

    def test_tee(self):
        with tempfile.TemporaryDirectory() as base:
            sim = Simulator(
                (NullFrontend(), {'tee': {
                    base+'/front.log': Loglevel.TRACE,
                }}),
                (NullBackend(), {'tee': {
                    base+'/back.log': Loglevel.TRACE,
                }}),
                tee={base+'/sim.log': Loglevel.TRACE},
                repro=None, stderr_verbosity=Loglevel.OFF
            )
            sim.simulate()
            sim.stop()

            with open(base+'/front.log', 'r') as f:
                f = f.read()
                self.assertTrue('Trace' in f)

            with open(base+'/back.log', 'r') as f:
                f = f.read()
                self.assertTrue('Trace' in f)

            with open(base+'/sim.log', 'r') as f:
                f = f.read()
                self.assertTrue('Trace' in f)

    def test_seed(self):
        with tempfile.TemporaryDirectory() as base:
            sim = Simulator(
                NullFrontend(), NullBackend(),
                tee={base+'/sim.log': Loglevel.INFO},
                repro=None, stderr_verbosity=Loglevel.OFF
            )
            sim.simulate(33)
            sim.stop()
            with open(base+'/sim.log', 'r') as f:
                f = f.read()
                self.assertTrue('seed: 33\n' in f)

        with tempfile.TemporaryDirectory() as base:
            sim = Simulator(
                NullFrontend(), NullBackend(),
                tee={base+'/sim.log': Loglevel.INFO},
                repro=None, stderr_verbosity=Loglevel.OFF
            )
            sim.simulate(123456789012345678901234567890)
            sim.stop()
            with open(base+'/sim.log', 'r') as f:
                f = f.read()
                self.assertTrue('seed: 1594492456\n' in f)

        with tempfile.TemporaryDirectory() as base:
            sim = Simulator(
                NullFrontend(), NullBackend(),
                tee={base+'/sim.log': Loglevel.INFO},
                repro=None, stderr_verbosity=Loglevel.OFF
            )
            sim.simulate('test')
            sim.stop()
            with open(base+'/sim.log', 'r') as f:
                f = f.read()
                self.assertTrue('seed: 73204161\n' in f)

    def test_usage_errors(self):
        with self.assertRaisesRegex(TypeError, "repro must be 'keep', 'absolute', 'relative', or None"):
            Simulator(repro='invalid')
        with self.assertRaisesRegex(TypeError, "dqcsim_verbosity must be a Loglevel"):
            Simulator(dqcsim_verbosity='invalid')
        with self.assertRaisesRegex(TypeError, "stderr_verbosity must be a Loglevel"):
            Simulator(stderr_verbosity='invalid')
        with self.assertRaisesRegex(TypeError, "log_capture must be callable or a string identifying a logger from the logging library"):
            Simulator(log_capture=33)
        with self.assertRaisesRegex(TypeError, "log_capture_verbosity must be a Loglevel"):
            Simulator(log_capture_verbosity='invalid')
        with self.assertRaises(ValueError):
            Simulator(tee='invalid')
        with self.assertRaisesRegex(TypeError, "tee file key must be a string"):
            Simulator(tee={3: Loglevel.TRACE})
        with self.assertRaisesRegex(TypeError, "tee file value must be a Loglevel"):
            Simulator(tee={'string': 3})
        with self.assertRaisesRegex(TypeError, "unexpected keyword argument 'invalid'"):
            Simulator(invalid=3)
        with self.assertRaisesRegex(TypeError, "init must be a single ArbCmd or a list/tuple of ArbCmds"):
            Simulator((Frontend(), {'init': 'invalid'}))
        with self.assertRaisesRegex(TypeError, "verbosity must be a Loglevel"):
            Simulator((Frontend(), {'verbosity': 'invalid'}))
        with self.assertRaises(ValueError):
            Simulator((Frontend(), {'tee': 'invalid'}))
        with self.assertRaisesRegex(TypeError, "tee file key must be a string"):
            Simulator((Frontend(), {'tee': {3: Loglevel.TRACE}}))
        with self.assertRaisesRegex(TypeError, "tee file value must be a Loglevel"):
            Simulator((Frontend(), {'tee': {'string': 3}}))
        with self.assertRaises(ValueError):
            Simulator(('null', {'env': 'invalid'}))
        with self.assertRaisesRegex(TypeError, "environment variable key must be a string"):
            Simulator(('null', {'env': {3: None}}))
        with self.assertRaisesRegex(TypeError, "environment variable value must be a string or None"):
            Simulator(('null', {'env': {'test': 3}}))
        with self.assertRaisesRegex(TypeError, "stderr must be a Loglevel or None"):
            Simulator(('null', {'stderr': 3}))
        with self.assertRaisesRegex(TypeError, "stdout must be a Loglevel or None"):
            Simulator(('null', {'stdout': 3}))
        with self.assertRaisesRegex(TypeError, "unexpected keyword argument 'invalid'"):
            Simulator(('null', {'invalid': 3}))


if __name__ == '__main__':
    unittest.main()
