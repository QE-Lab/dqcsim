import unittest, logging, os, sys, tempfile, re, math, cmath, pickle
from dqcsim.common import *
from dqcsim.host import *
from dqcsim.plugin import *

def catch_errors(fn, *args, **kwargs):
    try:
        return fn(*args, **kwargs)
    except Exception as e:
        return str(e)

@plugin("Test frontend plugin", "Test", "0.1")
class TestFrontend(Frontend):
    def handle_init(self, _):
        self.allocate(5)

    def handle_host_cmd_measure(self, measures=[]):
        self.measure(measures)

    def handle_host_cmd_advance(self, cycles=0):
        self.advance(cycles)

    def handle_host_cmd_arb(self, iface='', op=''):
        return ArbData(pickle.dumps(catch_errors(self.arb, iface, op)))

    def handle_host_cmd_stats(self, qubit=1):
        return ArbData(pickle.dumps((
            catch_errors(self.get_measurement, qubit),
            catch_errors(self.get_cycles_since_measure, qubit),
            catch_errors(self.get_cycles_between_measures, qubit),
            self.get_cycle()
        )))

@plugin("Null operator plugin", "Test", "0.1")
class NullOperator(Operator):
    def handle_host_cmd_stats(self, qubit=1):
        return ArbData(pickle.dumps((
            catch_errors(self.get_measurement, qubit),
            catch_errors(self.get_cycles_since_measure, qubit),
            catch_errors(self.get_cycles_between_measures, qubit),
            self.get_cycle()
        )))

@plugin("Test operator 1", "Test", "0.1")
class TestOperator1(NullOperator):
    def handle_measurement_gate(self, measures):
        self.measure([q+1 for q in measures])

    def handle_measurement(self, measurement):
        measurement.qubit -= 1
        measurement.value = True
        return measurement

    def handle_advance(self, cycles):
        self.advance(cycles*2)

    def handle_upstream_b_b(self):
        return ArbData(b'oper')

@plugin("Test operator 2", "Test", "0.1")
class TestOperator2(NullOperator):
    def handle_measurement(self, measurement):
        measurement.value = True
        return [measurement]

    def handle_upstream_a_a(self):
        return ArbData(b'oper')

@plugin("Test operator 3", "Test", "0.1")
class TestOperator3(NullOperator):
    def handle_measurement(self, measurement):
        pass

    def handle_upstream_a_b(self):
        return ArbData(b'oper')

@plugin("Null backend plugin", "Test", "0.1")
class NullBackend(Backend):
    def handle_unitary_gate(self, targets, matrix):
        pass

    def handle_measurement_gate(self, measures):
        return [Measurement(qubit, False) for qubit in measures]

    def handle_upstream_a_b(self):
        return ArbData(b'back')

class Tests(unittest.TestCase):
    def test_null_operator(self):
        sim = Simulator(
            TestFrontend(), NullOperator(), NullBackend(),
            repro=None, stderr_verbosity=Loglevel.ERROR
        )
        sim.simulate()
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            'Invalid argument: qubit 1 has not been measured yet',
            'Invalid argument: qubit 1 has not been measured yet',
            'Invalid argument: qubit 1 has not been measured yet',
            0,
        ))
        sim.arb('front', 'cmd', 'measure', measures=[1])
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            Measurement(1, 0),
            0,
            'Invalid argument: qubit 1 has only been measured once',
            0,
        ))
        sim.arb('front', 'cmd', 'advance', cycles=10)
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            Measurement(1, 0),
            10,
            'Invalid argument: qubit 1 has only been measured once',
            10,
        ))
        sim.arb('front', 'cmd', 'measure', measures=[1])
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            Measurement(1, 0),
            0,
            10,
            10,
        ))

        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='a', op='b')[0]),
            ArbData(b'back'))
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='a', op='a')[0]),
            'Invalid operation ID a for interface ID a')
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='b', op='b')[0]),
            ArbData())
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='b', op='a')[0]),
            ArbData())

        sim.stop()

    def test_operator1(self):
        sim = Simulator(
            TestFrontend(), TestOperator1(), NullBackend(),
            repro=None, stderr_verbosity=Loglevel.ERROR
        )
        sim.simulate()
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            'Invalid argument: qubit 1 has not been measured yet',
            'Invalid argument: qubit 1 has not been measured yet',
            'Invalid argument: qubit 1 has not been measured yet',
            0,
        ))
        self.assertEqual(pickle.loads(sim.arb('op1', 'cmd', 'stats', qubit=2)[0]), (
            'Invalid argument: qubit 2 has not been measured yet',
            'Invalid argument: qubit 2 has not been measured yet',
            'Invalid argument: qubit 2 has not been measured yet',
            0,
        ))
        sim.arb('front', 'cmd', 'measure', measures=[1])
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            Measurement(1, 1),
            0,
            'Invalid argument: qubit 1 has only been measured once',
            0,
        ))
        self.assertEqual(pickle.loads(sim.arb('op1', 'cmd', 'stats', qubit=2)[0]), (
            Measurement(2, 0),
            0,
            'Invalid argument: qubit 2 has only been measured once',
            0,
        ))
        sim.arb('front', 'cmd', 'advance', cycles=10)
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            Measurement(1, 1),
            10,
            'Invalid argument: qubit 1 has only been measured once',
            10,
        ))
        self.assertEqual(pickle.loads(sim.arb('op1', 'cmd', 'stats', qubit=2)[0]), (
            Measurement(2, 0),
            20,
            'Invalid argument: qubit 2 has only been measured once',
            20,
        ))
        sim.arb('front', 'cmd', 'measure', measures=[1])
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            Measurement(1, 1),
            0,
            10,
            10,
        ))
        self.assertEqual(pickle.loads(sim.arb('op1', 'cmd', 'stats', qubit=2)[0]), (
            Measurement(2, 0),
            0,
            20,
            20,
        ))

        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='a', op='b')[0]),
            ArbData(b'back'))
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='a', op='a')[0]),
            'Invalid operation ID a for interface ID a')
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='b', op='b')[0]),
            ArbData(b'oper'))
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='b', op='a')[0]),
            'Invalid operation ID a for interface ID b')

        sim.stop()

    def test_operator2(self):
        sim = Simulator(
            TestFrontend(), TestOperator2(), NullBackend(),
            repro=None, stderr_verbosity=Loglevel.ERROR
        )
        sim.simulate()
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            'Invalid argument: qubit 1 has not been measured yet',
            'Invalid argument: qubit 1 has not been measured yet',
            'Invalid argument: qubit 1 has not been measured yet',
            0,
        ))
        sim.arb('front', 'cmd', 'measure', measures=[1])
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            Measurement(1, 1),
            0,
            'Invalid argument: qubit 1 has only been measured once',
            0,
        ))
        sim.arb('front', 'cmd', 'advance', cycles=10)
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            Measurement(1, 1),
            10,
            'Invalid argument: qubit 1 has only been measured once',
            10,
        ))
        sim.arb('front', 'cmd', 'measure', measures=[1])
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            Measurement(1, 1),
            0,
            10,
            10,
        ))

        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='a', op='b')[0]),
            'Invalid operation ID b for interface ID a')
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='a', op='a')[0]),
            ArbData(b'oper'))
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='b', op='b')[0]),
            ArbData())
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='b', op='a')[0]),
            ArbData())

        sim.stop()

    def test_operator3(self):
        sim = Simulator(
            TestFrontend(), TestOperator3(), NullBackend(),
            repro=None, stderr_verbosity=Loglevel.ERROR
        )
        sim.simulate()
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            'Invalid argument: qubit 1 has not been measured yet',
            'Invalid argument: qubit 1 has not been measured yet',
            'Invalid argument: qubit 1 has not been measured yet',
            0,
        ))
        sim.arb('front', 'cmd', 'measure', measures=[1])
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            Measurement(1, None),
            0,
            'Invalid argument: qubit 1 has only been measured once',
            0,
        ))
        sim.arb('front', 'cmd', 'advance', cycles=10)
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            Measurement(1, None),
            10,
            'Invalid argument: qubit 1 has only been measured once',
            10,
        ))
        sim.arb('front', 'cmd', 'measure', measures=[1])
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'stats', qubit=1)[0]), (
            Measurement(1, None),
            0,
            10,
            10,
        ))

        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='a', op='b')[0]),
            ArbData(b'oper'))
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='a', op='a')[0]),
            'Invalid operation ID a for interface ID a')
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='b', op='b')[0]),
            ArbData())
        self.assertEqual(pickle.loads(sim.arb('front', 'cmd', 'arb', iface='b', op='a')[0]),
            ArbData())

        sim.stop()


if __name__ == '__main__':
    unittest.main()
