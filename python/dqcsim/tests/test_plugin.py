import unittest, logging, os, sys, tempfile, re
from dqcsim.common import *
from dqcsim.host import *
from dqcsim.plugin import *

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

class Tests(unittest.TestCase):
    def test_improper_init(self):
        @plugin("Broken operator plugin", "Test", "0.1")
        class BrokenOperator(Operator):
            def __init__(self):
                pass
        with self.assertRaisesRegex(RuntimeError, "overridden __init__"):
            BrokenOperator().run('x')

    def test_start(self):
        trace_fn = sys.gettrace()
        match = [False]
        after_wait = [False]
        def run_frontend(sim):
            if trace_fn is not None:
                sys.settrace(trace_fn)
            plugin = NullFrontend()
            join = plugin.start(sim)
            try:
                plugin.start(sim)
            except RuntimeError as e:
                match[0] = bool(re.match("Plugin has been started before. Make a new instance!", str(e)))
            join.wait()
            after_wait[0] = True
        sim = Simulator(
            run_frontend, NullBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF
        )
        sim.simulate()
        sim.stop()
        self.assertTrue(match[0])
        self.assertTrue(after_wait[0])

    def test_arb_routing(self):
        @plugin("Test frontend plugin", "Test", "0.1")
        class TestFrontend(Frontend):
            def handle_run(self, *args, **kwargs):
                pass
            def handle_host_x_y_z(self):
                pass
            def handle_host_x_z_y(self):
                return 3
            def handle_host_x_z_z(self):
                raise ValueError('test')
        with self.assertRaisesRegex(RuntimeError, "cannot auto-detect"):
            TestFrontend()
        fe = TestFrontend(host_arb_ifaces=['x'])

        @plugin("Test frontend plugin", "Test", "0.1")
        class TestOperator(Operator):
            def handle_upstream_x_y_z(self):
                pass
            def handle_upstream_x(self):
                pass
        with self.assertRaisesRegex(RuntimeError, "cannot auto-detect"):
            TestOperator()
        op = TestOperator(upstream_arb_ifaces=['x'])

        with tempfile.TemporaryDirectory() as base:
            sim = Simulator(
                (fe, {'tee': {base+'/sim.log': Loglevel.TRACE}}), op, NullBackend(),
                repro=None, stderr_verbosity=Loglevel.OFF
            )
            sim.simulate()
            with self.assertRaisesRegex(RuntimeError, "Invalid operation ID y_y for interface ID x"):
                sim.arb('front', 'x', 'y_y')
            sim.arb('front', 'x', 'y_z')
            with self.assertRaisesRegex(RuntimeError, "User implementation of host arb should return.*int"):
                sim.arb('front', 'x', 'z_y')
            with self.assertRaisesRegex(RuntimeError, "test"):
                sim.arb('front', 'x', 'z_z')
            sim.stop()

            with open(base+'/sim.log', 'r') as f:
                f = f.read()
                self.assertTrue('ValueError' in f)

    def test_plugin_state_and_random(self):
        @plugin("Test frontend plugin", "Test", "0.1")
        class TestFrontend(Frontend):
            def handle_run(self, *args, **kwargs):
                pass
            def handle_host_test_random(self):
                def check_u64(x):
                    return type(x) == int and x >= 0 and x <= 0xFFFFFFFFFFFFFFFF
                return ArbData(
                    f64=[self.random_float() for _ in range(100)],
                    u64=list(map(check_u64, (self.random_long() for _ in range(100)))))
        fe = TestFrontend()
        with self.assertRaisesRegex(RuntimeError, "Cannot call plugin operator outside of a callback"):
            fe.random_float()
        with self.assertRaisesRegex(RuntimeError, "Cannot call plugin operator outside of a callback"):
            fe.random_long()
        with self.assertRaisesRegex(RuntimeError, "Cannot call plugin operator outside of a callback"):
            fe.info('hello')
        sim = Simulator(
            fe, NullBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF)
        sim.simulate()
        x = sim.arb('front', 'test', 'random')
        sim.stop()
        for f64 in x['f64']:
            self.assertEqual(type(f64), float)
            self.assertTrue(f64 >= 0.0)
            self.assertTrue(f64 < 1.0)
        for u64 in x['u64']:
            self.assertTrue(u64)


if __name__ == '__main__':
    unittest.main()
