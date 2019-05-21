import unittest, logging, os, sys, tempfile, re, math, cmath, pickle
from dqcsim.common import *
from dqcsim.host import *
from dqcsim.plugin import *

@plugin("Test frontend plugin", "Test", "0.1")
class TestFrontend(Frontend):
    def handle_run(self, *args, **kwargs):
        self.allocate(3)
        self.unitary([1], [0.0, 1.0, 1.0, 0.0])
        self.unitary([1], [0.0, 1.0, 1.0, 0.0], [2])
        self.i_gate(1)
        self.rx_gate(1, math.pi)
        self.ry_gate(1, math.pi)
        self.rz_gate(1, math.pi)
        self.r_gate(1, math.pi, 0, 0)
        self.r_gate(1, 0, math.pi, 0)
        self.r_gate(1, 0, 0, math.pi)
        self.swap_gate(1, 2)
        self.sqswap_gate(1, 2)
        self.x_gate(1)
        self.x90_gate(1)
        self.mx90_gate(1)
        self.y_gate(1)
        self.y90_gate(1)
        self.my90_gate(1)
        self.z_gate(1)
        self.z90_gate(1)
        self.mz90_gate(1)
        self.s_gate(1)
        self.sdag_gate(1)
        self.t_gate(1)
        self.tdag_gate(1)
        self.h_gate(1)
        self.cnot_gate(1, 2)
        self.toffoli_gate(1, 2, 3)
        self.fredkin_gate(1, 2, 3)
        self.custom_gate('a', [1], [2], [3], None, b'33', a='b')
        self.custom_gate('b', [1], [2], [3], [1.0, 0.0, 0.0, 1.0], b'33', a='b')
        self.measure(1)
        self.measure_x(1, 2, 3)
        self.measure_x([1, 2, 3])
        self.measure_y([1, 2, 3])
        self.measure_z(1, 2)
        self.free(1, 2, 3)

@plugin("Null operator plugin", "Test", "0.1")
class NullOperator(Operator):
    pass

@plugin("Test backend plugin", "Test", "0.1")
class TestBackendUnitary(Backend):
    def __init__(self):
        super().__init__()
        self.call_log = []

    def handle_unitary_gate(self, targets, matrix):
        self.call_log.append({
            'cmd': 'unitary',
            'targets': targets,
            'matrix': pickle.dumps(matrix),
        })

    def handle_measurement_gate(self, measures):
        self.call_log.append({
            'cmd': 'measurement',
            'measures': measures,
        })
        return [Measurement(qubit, False) for qubit in measures]

    def handle_a_gate(self, targets, controls, measures, matrix, *args, **kwargs):
        self.call_log.append({
            'cmd': 'a',
            'targets': targets,
            'controls': controls,
            'measures': measures,
            'matrix': pickle.dumps(matrix),
            'args': args,
            'kwargs': kwargs,
        })
        return [Measurement(qubit, False) for qubit in measures]

    def handle_b_gate(self, targets, controls, measures, matrix, *args, **kwargs):
        self.call_log.append({
            'cmd': 'b',
            'targets': targets,
            'controls': controls,
            'measures': measures,
            'matrix': pickle.dumps(matrix),
            'args': args,
            'kwargs': kwargs,
        })
        return [Measurement(qubit, False) for qubit in measures]

    def handle_host_get_log(self):
        log = self.call_log
        self.call_log = []
        return ArbData(log=log)

@plugin("Null backend plugin", "Test", "0.1")
class NullBackend(Backend):
    pass

@plugin("Invalid backend plugin", "Test", "0.1")
class InvalidBackend(Backend):
    pass

class Tests(unittest.TestCase):
    def assertEqualMatrix(self, pickled, reference):
        received = pickle.loads(pickled)
        def normalize(matrix):
            ref_magnitude = 0.0
            ref_angle = 1.0
            for x in matrix:
                if abs(x) > ref_magnitude + 0.1:
                    ref_magnitude = abs(x)
                    ref_angle = x.conjugate() * (1.0 / abs(x))
            return [x * ref_angle for x in matrix]
        print('---', file=sys.stderr)
        print(received, file=sys.stderr)
        received = normalize(received)
        reference = normalize(reference)
        self.assertEqual(len(received), len(reference))
        print('---', file=sys.stderr)
        print(received, file=sys.stderr)
        print(reference, file=sys.stderr)
        for rec, ref in zip(received, reference):
            self.assertTrue(abs(rec - ref) < 0.01)

    def assert_unitary(self, data, targets, matrix):
        self.assertEqual(data['cmd'], 'unitary')
        self.assertEqual(data['targets'], targets)
        self.assertEqualMatrix(data['matrix'], matrix)

    def test_single_qubit(self):
        sim = Simulator(
            TestFrontend(), TestBackendUnitary(),
            repro=None, stderr_verbosity=Loglevel.TRACE
        )
        sim.simulate()
        sim.start()
        sim.wait()
        log = sim.arb('back', 'get', 'log')['log']
        sim.stop()

        # unitary
        self.assert_unitary(log.pop(0), [1], [
            0.000+0.000j, 1.000+0.000j,
            1.000+0.000j, 0.000+0.000j,
        ])
        self.assert_unitary(log.pop(0), [2, 1], [
            1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j,
        ])

        # i_gate
        self.assert_unitary(log.pop(0), [1], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 1.000+0.000j,
        ])

        # rx_gate(pi)
        self.assert_unitary(log.pop(0), [1], [
            0.000+0.000j, 1.000+0.000j,
            1.000+0.000j, 0.000+0.000j,
        ])

        # ry_gate(pi)
        self.assert_unitary(log.pop(0), [1], [
            0.000+0.000j, 0.000-1.000j,
            0.000+1.000j, 0.000+0.000j,
        ])

        # rz_gate(pi)
        self.assert_unitary(log.pop(0), [1], [
            1.000+0.000j,  0.000-0.000j,
            0.000+0.000j, -1.000+0.000j,
        ])

        # r_gate(pi, 0, 0)
        self.assert_unitary(log.pop(0), [1], [
            1.000+0.000j,  0.000-0.000j,
            0.000+0.000j, -1.000+0.000j,
        ])

        # r_gate(0, pi, 0)
        self.assert_unitary(log.pop(0), [1], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 1.000+0.000j,
        ])

        # r_gate(0, 0, pi)
        self.assert_unitary(log.pop(0), [1], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 1.000+0.000j,
        ])

        # swap_gate
        self.assert_unitary(log.pop(0), [1, 2], [
            1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j,
        ])

        # sqswap_gate
        self.assert_unitary(log.pop(0), [1, 2], [
            1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.500+0.500j, 0.500-0.500j, 0.000+0.000j,
            0.000+0.000j, 0.500-0.500j, 0.500+0.500j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j,
        ])

        # x_gate
        self.assert_unitary(log.pop(0), [1], [
            0.000+0.000j, 1.000+0.000j,
            1.000+0.000j, 0.000+0.000j,
        ])

        # x90_gate
        log.pop(0) # TODO

        # mx90_gate
        log.pop(0) # TODO

        # y_gate
        self.assert_unitary(log.pop(0), [1], [
            0.000+0.000j, 0.000-1.000j,
            0.000+1.000j, 0.000+0.000j,
        ])

        # y90_gate
        log.pop(0) # TODO

        # my90_gate
        log.pop(0) # TODO

        # z_gate
        self.assert_unitary(log.pop(0), [1], [
            1.000+0.000j,  0.000-0.000j,
            0.000+0.000j, -1.000+0.000j,
        ])

        # z90_gate
        log.pop(0) # TODO

        # mz90_gate
        log.pop(0) # TODO

        # s_gate
        self.assert_unitary(log.pop(0), [1], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+1.000j,
        ])

        # sdag_gate
        self.assert_unitary(log.pop(0), [1], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000-1.000j,
        ])

        # t_gate
        self.assert_unitary(log.pop(0), [1], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, cmath.exp(1j * math.pi / 4),
        ])

        # tdag_gate
        self.assert_unitary(log.pop(0), [1], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, cmath.exp(-1j * math.pi / 4),
        ])

        # h_gate
        self.assert_unitary(log.pop(0), [1], [
            0.707+0.000j,  0.707+0.000j,
            0.707+0.000j, -0.707+0.000j,
        ])

        # cnot_gate
        self.assert_unitary(log.pop(0), [1, 2], [
            1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j,
        ])

        # toffoli_gate
        self.assert_unitary(log.pop(0), [1, 2, 3], [
            1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j,
        ])

        # fredkin_gate
        self.assert_unitary(log.pop(0), [1, 2, 3], [
            1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j,
        ])

        # custom_gate('a', [1], [2], [3], None, b'33', a='b')
        log.pop(0) # TODO

        # custom_gate('b', [1], [2], [3], [1.0, 0.0, 0.0, 1.0], b'33', a='b')
        log.pop(0) # TODO

        # measure(1)
        log.pop(0) # TODO

        # measure_x(1, 2, 3)
        log.pop(0) # TODO

        # measure_x(1, 2, 3)
        log.pop(0) # TODO

        # measure_y(1, 2, 3)
        log.pop(0) # TODO

        # measure_z(1, 2)
        log.pop(0) # TODO

if __name__ == '__main__':
    unittest.main()
