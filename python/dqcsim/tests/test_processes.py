import unittest
from dqcsim.common import *
from dqcsim.host import *
from dqcsim.plugin import *
import logging
import os, sys

os.environ['PYTHONPATH'] = ':'.join([os.getcwd() + '/python'] + os.environ['PYTHONPATH'].split(':'))

p = os.path.dirname(__file__) + '/'

class Tests(unittest.TestCase):

    def test_null_processes(self):
        Simulator(
            p+'null_frontend.py',
            p+'null_operator.py',
            ('/usr/bin/python3', p+'null_backend.py'),
            repro=None, stderr_verbosity=Loglevel.OFF
        ).run()

    def test_env(self):
        os.environ['x'] = 'x'
        sim = Simulator(
            (p+'null_frontend.py', {'env': {'x': 'y'}}),
            ('/usr/bin/python3', p+'null_backend.py', {'env': {'x': None}, 'work': os.getcwd() + '/..'}),
            repro=None, stderr_verbosity=Loglevel.OFF
        )
        sim.simulate()
        front = sim.arb('front', 'work', 'env')
        self.assertEqual(front['env']['x'], 'y')
        self.assertEqual(front['work'], os.getcwd())
        back = sim.arb('back', 'work', 'env')
        self.assertTrue('x' not in back['env'])
        self.assertEqual(back['work'], os.path.dirname(os.getcwd()))
        sim.stop()

if __name__ == '__main__':
    unittest.main()
