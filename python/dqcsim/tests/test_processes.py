import unittest, logging, os, sys, tempfile
from dqcsim.common import *
from dqcsim.host import *
from dqcsim.plugin import *

os.environ['PYTHONPATH'] = ':'.join([os.getcwd() + '/python'] + os.environ.get('PYTHONPATH', '').split(':'))
os.environ['PATH'] = ':'.join([os.getcwd() + '/python/bin'] + os.environ.get('PATH', '').split(':'))

p = os.path.dirname(__file__) + '/'

class Tests(unittest.TestCase):

    def test_null_processes(self):
        Simulator(
            p+'null_frontend.py',
            p+'null_operator.py',
            (sys.executable, p+'null_backend.py'),
            repro=None, stderr_verbosity=Loglevel.ERROR
        ).run()

    def test_env(self):
        os.environ['x'] = 'x'
        sim = Simulator(
            (p+'null_frontend.py', {'env': {'x': 'y'}}),
            (sys.executable, p+'null_backend.py', {'env': {'x': None}, 'work': os.getcwd() + '/..'}),
            repro=None, stderr_verbosity=Loglevel.ERROR
        )
        sim.simulate()
        front = sim.arb('front', 'work', 'env')
        self.assertEqual(front['env']['x'], 'y')
        self.assertEqual(front['work'], os.getcwd())
        back = sim.arb('back', 'work', 'env')
        self.assertTrue('x' not in back['env'])
        self.assertEqual(back['work'], os.path.dirname(os.getcwd()))
        sim.stop()

    def test_init_arbs(self):
        sim = Simulator(
            (p+'null_frontend.py', {'init': ArbCmd('x', 'y')}),
            p+'null_backend.py',
            repro=None, stderr_verbosity=Loglevel.ERROR
        )
        sim.simulate()
        self.assertEqual(sim.arb('front', 'get', 'arbs')['data'], [
            {'iface': 'x', 'oper': 'y', 'args': [], 'kwargs': {}},
        ])
        sim.stop()

        sim = Simulator(
            (p+'null_frontend.py', {'init': [
                ArbCmd('x', 'y', b'a'), ArbCmd('y', 'z', b'b')
            ]}),
            p+'null_backend.py',
            repro=None, stderr_verbosity=Loglevel.ERROR
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
                (p+'null_frontend.py', {'tee': {
                    base+'/front_trace.log': Loglevel.TRACE,
                    base+'/front_info.log': Loglevel.INFO,
                }}),
                (p+'null_backend.py', {'tee': {
                    base+'/back_trace.log': Loglevel.TRACE,
                }}),
                repro=None, stderr_verbosity=Loglevel.ERROR
            )
            sim.simulate()
            sim.stop()

            with open(base+'/front_trace.log', 'r') as f:
                f = f.read()
                self.assertTrue('Trace' in f)
                self.assertTrue('null frontend dropped!' in f)
                self.assertFalse('null backend dropped!' in f)

            with open(base+'/front_info.log', 'r') as f:
                f = f.read()
                self.assertFalse('Trace' in f)
                self.assertTrue('Info' in f)
                self.assertFalse('null frontend dropped!' in f)
                self.assertFalse('null backend dropped!' in f)

            with open(base+'/back_trace.log', 'r') as f:
                f = f.read()
                self.assertTrue('Trace' in f)
                self.assertFalse('null frontend dropped!' in f)
                self.assertTrue('null backend dropped!' in f)

    def test_stdout_stderr_passthrough(self):
        sim = Simulator(
            (p+'null_frontend.py', {'stdout': None}),
            (p+'null_backend.py', {'stderr': None}),
            repro=None, stderr_verbosity=Loglevel.ERROR
        )
        # This just checks that the following doesn't raise any errors. We
        # don't actually ensure that stdout/stderr is passed through.
        sim.simulate()
        sim.stop()

    def test_reproduction(self):
        with tempfile.TemporaryDirectory() as base:
            sim = Simulator(
                p+'null_frontend.py', p+'null_backend.py',
                stderr_verbosity=Loglevel.ERROR
            )
            # This just checks that a reproduction file is generated. It
            # doesn't check the contents.
            sim.simulate()
            sim.stop(base + '/test.repro')
            with open(base + '/test.repro', 'r') as f:
                self.assertTrue(len(f.read()) > 0)

    def test_default_backend(self):
        sim = Simulator(
            p+'null_frontend.py',
            repro=None, stderr_verbosity=Loglevel.ERROR
        )
        try:
            sim.simulate()
            sim.stop(base + '/test.repro')
        except RuntimeError as e:
            if "could not find plugin executable 'dqcsbeqx'" not in str(e):
                raise

if __name__ == '__main__':
    unittest.main()
