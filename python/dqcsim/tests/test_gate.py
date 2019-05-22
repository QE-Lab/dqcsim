import unittest, logging, os, sys, tempfile, re, math, cmath, pickle
from dqcsim.common import *
from dqcsim.host import *
from dqcsim.plugin import *

@plugin("Test frontend plugin", "Test", "0.1")
class TestFrontend(Frontend):
    def handle_run(self, *args, **kwargs):
        self.allocate(5)
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
        self.prepare(1, 2)
        self.free(1, 2, 3)

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
        return [Measurement(qubit, qubit % 2) for qubit in measures]

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

@plugin("Test backend plugin", "Test", "0.2")
class TestBackendControlled(TestBackendUnitary):
    def handle_controlled_gate(self, targets, controls, matrix):
        self.call_log.append({
            'cmd': 'controlled',
            'targets': targets,
            'controls': controls,
            'matrix': pickle.dumps(matrix),
        })

@plugin("Null operator plugin", "Test", "0.1")
class NullOperator(Operator):
    pass

@plugin("Test operator 1", "Test", "0.1")
class Operator1(Operator):
    def handle_unitary_gate(self, targets, matrix):
        self.trace('unitary: {} {}', targets, matrix)
        self.unitary([q+1 for q in targets], matrix)

    def handle_measurement_gate(self, measures):
        self.trace('measurement: {}', measures)
        self.measure([q+2 for q in measures])

@plugin("Test operator 2", "Test", "0.1")
class Operator2(Operator):
    def handle_a_gate(self, targets, controls, measures, matrix, *args, **kwargs):
        self.custom_gate('a',
            [q+1 for q in targets],
            [q+1 for q in controls],
            [q+1 for q in measures],
            matrix, *args, **kwargs)

    def handle_controlled_gate(self, targets, controls, matrix):
        self.unitary([q+2 for q in targets], matrix, controls=[q+2 for q in controls])

@plugin("Test operator 3", "Test", "0.1")
class Operator3(Operator):
    def handle_unitary_gate(self, targets, matrix):
        self.unitary([q+1 for q in targets], matrix)

    def handle_controlled_gate(self, targets, controls, matrix):
        self.unitary([q+2 for q in targets], matrix, controls=[q+2 for q in controls])


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
        received = normalize(received)
        reference = normalize(reference)
        self.assertEqual(len(received), len(reference))
        for rec, ref in zip(received, reference):
            self.assertTrue(abs(rec - ref) < 0.01)

    def assert_unitary(self, data, targets, matrix):
        self.assertEqual(data['cmd'], 'unitary')
        self.assertEqual(data['targets'], targets)
        self.assertEqualMatrix(data['matrix'], matrix)

    def assert_controlled(self, data, targets, controls, matrix):
        self.assertEqual(data['cmd'], 'controlled')
        self.assertEqual(data['targets'], targets)
        self.assertEqual(set(data['controls']), set(controls))
        self.assertEqualMatrix(data['matrix'], matrix)

    def assert_gates(self, log, controlled, u=0, c=0, m=0, a=0, b=0):
        self.maxDiff = None

        # unitary
        self.assert_unitary(log.pop(0), [1+u], [
            0.000+0.000j, 1.000+0.000j,
            1.000+0.000j, 0.000+0.000j,
        ])
        if controlled:
            self.assert_controlled(log.pop(0), [1+c], [2+c], [
                0.000+0.000j, 1.000+0.000j,
                1.000+0.000j, 0.000+0.000j,
            ])
        else:
            self.assert_unitary(log.pop(0), [2+c, 1+c], [
                1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
                0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j,
                0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j,
                0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j,
            ])

        # i_gate
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 1.000+0.000j,
        ])

        # rx_gate(pi)
        self.assert_unitary(log.pop(0), [1+u], [
            0.000+0.000j, 1.000+0.000j,
            1.000+0.000j, 0.000+0.000j,
        ])

        # ry_gate(pi)
        self.assert_unitary(log.pop(0), [1+u], [
            0.000+0.000j, 0.000-1.000j,
            0.000+1.000j, 0.000+0.000j,
        ])

        # rz_gate(pi)
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j,  0.000-0.000j,
            0.000+0.000j, -1.000+0.000j,
        ])

        # r_gate(pi, 0, 0)
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j,  0.000-0.000j,
            0.000+0.000j, -1.000+0.000j,
        ])

        # r_gate(0, pi, 0)
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 1.000+0.000j,
        ])

        # r_gate(0, 0, pi)
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 1.000+0.000j,
        ])

        # swap_gate
        self.assert_unitary(log.pop(0), [1+u, 2+u], [
            1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j,
        ])

        # sqswap_gate
        self.assert_unitary(log.pop(0), [1+u, 2+u], [
            1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.500+0.500j, 0.500-0.500j, 0.000+0.000j,
            0.000+0.000j, 0.500-0.500j, 0.500+0.500j, 0.000+0.000j,
            0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j,
        ])

        # x_gate
        self.assert_unitary(log.pop(0), [1+u], [
            0.000+0.000j, 1.000+0.000j,
            1.000+0.000j, 0.000+0.000j,
        ])

        # x90_gate
        self.assert_unitary(log.pop(0), [1+u], [
            0.707+0.000j, 0.000-0.707j,
            0.000-0.707j, 0.707+0.000j,
        ])

        # mx90_gate
        self.assert_unitary(log.pop(0), [1+u], [
            0.707+0.000j, 0.000+0.707j,
            0.000+0.707j, 0.707+0.000j,
        ])

        # y_gate
        self.assert_unitary(log.pop(0), [1+u], [
            0.000+0.000j, 0.000-1.000j,
            0.000+1.000j, 0.000+0.000j,
        ])

        # y90_gate
        self.assert_unitary(log.pop(0), [1+u], [
            0.707+0.000j, -0.707+0.000j,
            0.707+0.000j,  0.707+0.000j,
        ])

        # my90_gate
        self.assert_unitary(log.pop(0), [1+u], [
             0.707+0.000j, 0.707+0.000j,
            -0.707+0.000j, 0.707+0.000j,
        ])

        # z_gate
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j,  0.000-0.000j,
            0.000+0.000j, -1.000+0.000j,
        ])

        # z90_gate
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+1.000j,
        ])

        # mz90_gate
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000-1.000j,
        ])

        # s_gate
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+1.000j,
        ])

        # sdag_gate
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000-1.000j,
        ])

        # t_gate
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.707+0.707j,
        ])

        # tdag_gate
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.707-0.707j,
        ])

        # h_gate
        self.assert_unitary(log.pop(0), [1+u], [
            0.707+0.000j,  0.707+0.000j,
            0.707+0.000j, -0.707+0.000j,
        ])

        if controlled:
            # cnot_gate
            self.assert_controlled(log.pop(0), [2+c], [1+c], [
                0.000+0.000j, 1.000+0.000j,
                1.000+0.000j, 0.000+0.000j,
            ])

            # toffoli_gate
            self.assert_controlled(log.pop(0), [3+c], [1+c, 2+c], [
                0.000+0.000j, 1.000+0.000j,
                1.000+0.000j, 0.000+0.000j,
            ])

            # fredkin_gate
            self.assert_controlled(log.pop(0), [2+c, 3+c], [1+c], [
                1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
                0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j,
                0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j,
                0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j,
            ])

        else:
            # cnot_gate
            self.assert_unitary(log.pop(0), [1+c, 2+c], [
                1.000+0.000j, 0.000+0.000j, 0.000+0.000j, 0.000+0.000j,
                0.000+0.000j, 1.000+0.000j, 0.000+0.000j, 0.000+0.000j,
                0.000+0.000j, 0.000+0.000j, 0.000+0.000j, 1.000+0.000j,
                0.000+0.000j, 0.000+0.000j, 1.000+0.000j, 0.000+0.000j,
            ])

            # toffoli_gate
            self.assert_unitary(log.pop(0), [1+c, 2+c, 3+c], [
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
            self.assert_unitary(log.pop(0), [1+c, 2+c, 3+c], [
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
        x = log.pop(0)
        self.assertEqual(pickle.loads(x.pop('matrix')), None)
        self.assertEqual(x, {
            'cmd': 'a',
            'targets': [1+a],
            'controls': [2+a],
            'measures': [3+a],
            'args': [b'33'],
            'kwargs': {'a': 'b'},
        })

        # custom_gate('b', [1], [2], [3], [1.0, 0.0, 0.0, 1.0], b'33', a='b')
        x = log.pop(0)
        self.assertEqual(pickle.loads(x.pop('matrix')), [1.0+0.0j, 0.0+0.0j, 0.0+0.0j, 1.0+0.0j])
        self.assertEqual(x, {
            'cmd': 'b',
            'targets': [1+b],
            'controls': [2+b],
            'measures': [3+b],
            'args': [b'33'],
            'kwargs': {'a': 'b'},
        })

        # measure(1)
        self.assertEqual(log.pop(0), {
            'cmd': 'measurement',
            'measures': [1+m],
        })

        # measure_x(1, 2, 3)
        self.assert_unitary(log.pop(0), [1+u], [
            0.707+0.000j,  0.707+0.000j,
            0.707+0.000j, -0.707+0.000j,
        ])
        self.assert_unitary(log.pop(0), [2+u], [
            0.707+0.000j,  0.707+0.000j,
            0.707+0.000j, -0.707+0.000j,
        ])
        self.assert_unitary(log.pop(0), [3+u], [
            0.707+0.000j,  0.707+0.000j,
            0.707+0.000j, -0.707+0.000j,
        ])
        self.assertEqual(log.pop(0), {
            'cmd': 'measurement',
            'measures': [1+m, 2+m, 3+m],
        })
        self.assert_unitary(log.pop(0), [1+u], [
            0.707+0.000j,  0.707+0.000j,
            0.707+0.000j, -0.707+0.000j,
        ])
        self.assert_unitary(log.pop(0), [2+u], [
            0.707+0.000j,  0.707+0.000j,
            0.707+0.000j, -0.707+0.000j,
        ])
        self.assert_unitary(log.pop(0), [3+u], [
            0.707+0.000j,  0.707+0.000j,
            0.707+0.000j, -0.707+0.000j,
        ])

        # measure_x(1, 2, 3)
        # meas_y = H, meas_z, H
        self.assert_unitary(log.pop(0), [1+u], [
            0.707+0.000j,  0.707+0.000j,
            0.707+0.000j, -0.707+0.000j,
        ])
        self.assert_unitary(log.pop(0), [2+u], [
            0.707+0.000j,  0.707+0.000j,
            0.707+0.000j, -0.707+0.000j,
        ])
        self.assert_unitary(log.pop(0), [3+u], [
            0.707+0.000j,  0.707+0.000j,
            0.707+0.000j, -0.707+0.000j,
        ])
        self.assertEqual(log.pop(0), {
            'cmd': 'measurement',
            'measures': [1+m, 2+m, 3+m],
        })
        self.assert_unitary(log.pop(0), [1+u], [
            0.707+0.000j,  0.707+0.000j,
            0.707+0.000j, -0.707+0.000j,
        ])
        self.assert_unitary(log.pop(0), [2+u], [
            0.707+0.000j,  0.707+0.000j,
            0.707+0.000j, -0.707+0.000j,
        ])
        self.assert_unitary(log.pop(0), [3+u], [
            0.707+0.000j,  0.707+0.000j,
            0.707+0.000j, -0.707+0.000j,
        ])

        # measure_y(1, 2, 3)
        # meas_y = S, Z, meas_z, S
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+1.000j,
        ])
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j,  0.000-0.000j,
            0.000+0.000j, -1.000+0.000j,
        ])
        self.assert_unitary(log.pop(0), [2+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+1.000j,
        ])
        self.assert_unitary(log.pop(0), [2+u], [
            1.000+0.000j,  0.000-0.000j,
            0.000+0.000j, -1.000+0.000j,
        ])
        self.assert_unitary(log.pop(0), [3+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+1.000j,
        ])
        self.assert_unitary(log.pop(0), [3+u], [
            1.000+0.000j,  0.000-0.000j,
            0.000+0.000j, -1.000+0.000j,
        ])
        self.assertEqual(log.pop(0), {
            'cmd': 'measurement',
            'measures': [1+m, 2+m, 3+m],
        })
        self.assert_unitary(log.pop(0), [1+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+1.000j,
        ])
        self.assert_unitary(log.pop(0), [2+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+1.000j,
        ])
        self.assert_unitary(log.pop(0), [3+u], [
            1.000+0.000j, 0.000+0.000j,
            0.000+0.000j, 0.000+1.000j,
        ])

        # measure_z(1, 2)
        self.assertEqual(log.pop(0), {
            'cmd': 'measurement',
            'measures': [1+m, 2+m],
        })

        # prepare(1, 2)
        if not m:
            self.assertEqual(log.pop(0), {
                'cmd': 'measurement',
                'measures': [1+m, 2+m],
            })
            self.assert_unitary(log.pop(0), [1+u], [
                0.000+0.000j, 1.000+0.000j,
                1.000+0.000j, 0.000+0.000j,
            ])
            self.assertEqual(log, [])

    def check_with_operator(self, operator_cls, *args, **kwargs):
        sim = Simulator(
            TestFrontend(), operator_cls(), TestBackendUnitary(),
            repro=None, stderr_verbosity=Loglevel.ERROR
        )
        sim.simulate()
        sim.start()
        sim.wait()
        self.assert_gates(sim.arb('back', 'get', 'log')['log'], False, *args, **kwargs)
        sim.stop()

        sim = Simulator(
            TestFrontend(), operator_cls(), TestBackendControlled(),
            repro=None, stderr_verbosity=Loglevel.ERROR
        )
        sim.simulate()
        sim.start()
        sim.wait()
        self.assert_gates(sim.arb('back', 'get', 'log')['log'], True, *args, **kwargs)
        sim.stop()

    def test_gates_with_null_operator(self):
        self.check_with_operator(NullOperator)

    def test_gates_with_operator_1(self):
        self.check_with_operator(Operator1, u=1, m=2)

    def test_gates_with_operator_2(self):
        self.check_with_operator(Operator2, a=1, u=2, c=2)

    def test_gates_with_operator_3(self):
        self.check_with_operator(Operator3, u=1, c=2)

    def test_invalid_backend(self):
        sim = Simulator(
            TestFrontend(), InvalidBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF
        )
        #with self.assertRaisesRegex(RuntimeError, "Python plugin doesn't implement handle_unitary_gate"):
        # TODO: the above doesn't work because Ferris is flipping out
        # somewhere, despite the fact that errors *should* just
        # propagate upward.
        with self.assertRaises(RuntimeError):
            sim.run()

if __name__ == '__main__':
    unittest.main()
