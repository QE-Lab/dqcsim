import unittest, logging, os, sys, tempfile, re
from dqcsim.common import *
from dqcsim.host import *
from dqcsim.plugin import *

@plugin("Test frontend plugin", "Test", "0.1")
class TestFrontend(Frontend):
    def handle_run(self, *args, **kwargs):
        pass

    def handle_host_cmd_allocate(self, num=None, cmds=[]):
        qubits = self.allocate(num, *[ArbCmd(cmd[0], cmd[1], *cmd[2], **cmd[3]) for cmd in cmds])
        return ArbData(qubits=qubits)

    def handle_host_cmd_free(self, qubits=[]):
        self.free(qubits)

@plugin("Null operator plugin", "Test", "0.1")
class NullOperator(Operator):
    pass

@plugin("Test operator plugin", "Test", "0.1")
class TestOperator(Operator):
    def __init__(self):
        super().__init__()
        self.call_log = []

    def handle_allocate(self, qubits, cmds):
        self.call_log.append({
            'cmd': 'allocate',
            'qubits': {qubit: qubit for qubit in qubits},
            'cmds': [[cmd.iface, cmd.oper, list(cmd), dict(cmd.items())] for cmd in cmds],
        })
        self.allocate(len(qubits)*2, cmds)

    def handle_free(self, qubits):
        self.call_log.append({
            'cmd': 'free',
            'qubits': {qubit: qubit for qubit in qubits}
        })
        self.free([qubit*2-1 for qubit in qubits] + [qubit*2 for qubit in qubits])

    def handle_host_get_log(self):
        log = self.call_log
        self.call_log = []
        return ArbData(log=log)

@plugin("Test backend plugin", "Test", "0.1")
class TestBackend(Backend):
    def __init__(self):
        super().__init__()
        self.call_log = []

    def handle_allocate(self, qubits, cmds):
        self.call_log.append({
            'cmd': 'allocate',
            'qubits': {qubit: qubit for qubit in qubits},
            'cmds': [[cmd.iface, cmd.oper, list(cmd), dict(cmd.items())] for cmd in cmds],
        })

    def handle_free(self, qubits):
        self.call_log.append({
            'cmd': 'free',
            'qubits': {qubit: qubit for qubit in qubits}
        })

    def handle_host_get_log(self):
        log = self.call_log
        self.call_log = []
        return ArbData(log=log)

@plugin("Null backend plugin", "Test", "0.1")
class NullBackend(Backend):
    pass

class Tests(unittest.TestCase):
    def test_single_qubit(self):
        sim = Simulator(
            TestFrontend(), TestBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF
        )
        sim.simulate()
        qubit = sim.arb('front', 'cmd', 'allocate')['qubits']
        self.assertEqual(qubit, 1)
        sim.arb('front', 'cmd', 'free', qubits=qubit)
        self.assertEqual(sim.arb('back', 'get', 'log')['log'], [
            {'cmd': 'allocate', 'qubits': {1:1}, 'cmds': []},
            {'cmd': 'free', 'qubits': {1:1}},
        ])
        sim.stop()

    def test_multi_qubit(self):
        sim = Simulator(
            TestFrontend(), TestBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF
        )
        sim.simulate()
        qubits = sim.arb('front', 'cmd', 'allocate', num=5)['qubits']
        self.assertEqual(set(qubits), set([1, 2, 3, 4, 5]))
        sim.arb('front', 'cmd', 'free', qubits=qubits)
        self.assertEqual(sim.arb('back', 'get', 'log')['log'], [
            {'cmd': 'allocate', 'qubits': {1:1, 2:2, 3:3, 4:4, 5:5}, 'cmds': []},
            {'cmd': 'free', 'qubits': {1:1, 2:2, 3:3, 4:4, 5:5}},
        ])
        sim.stop()

    def test_operator(self):
        sim = Simulator(
            TestFrontend(), TestOperator(), TestBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF
        )
        sim.simulate()
        qubits = sim.arb('front', 'cmd', 'allocate', num=2)['qubits']
        self.assertEqual(set(qubits), set([1, 2]))
        sim.arb('front', 'cmd', 'free', qubits=qubits)
        self.assertEqual(sim.arb('op1', 'get', 'log')['log'], [
            {'cmd': 'allocate', 'qubits': {1:1, 2:2}, 'cmds': []},
            {'cmd': 'free', 'qubits': {1:1, 2:2}},
        ])
        self.assertEqual(sim.arb('back', 'get', 'log')['log'], [
            {'cmd': 'allocate', 'qubits': {1:1, 2:2, 3:3, 4:4}, 'cmds': []},
            {'cmd': 'free', 'qubits': {1:1, 2:2, 3:3, 4:4}},
        ])
        sim.stop()

    def test_null_operator(self):
        sim = Simulator(
            TestFrontend(), NullOperator(), TestBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF
        )
        sim.simulate()
        qubits = sim.arb('front', 'cmd', 'allocate', num=2)['qubits']
        self.assertEqual(set(qubits), set([1, 2]))
        sim.arb('front', 'cmd', 'free', qubits=qubits)
        self.assertEqual(sim.arb('back', 'get', 'log')['log'], [
            {'cmd': 'allocate', 'qubits': {1:1, 2:2}, 'cmds': []},
            {'cmd': 'free', 'qubits': {1:1, 2:2}},
        ])
        sim.stop()

    def test_null_backend(self):
        sim = Simulator(
            TestFrontend(), NullBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF
        )
        sim.simulate()
        qubits = sim.arb('front', 'cmd', 'allocate', num=2)['qubits']
        self.assertEqual(set(qubits), set([1, 2]))
        sim.arb('front', 'cmd', 'free', qubits=qubits)
        sim.stop()

    def test_single_cmd(self):
        sim = Simulator(
            TestFrontend(), TestOperator(), TestBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF
        )
        sim.simulate()
        qubits = sim.arb('front', 'cmd', 'allocate', num=2, cmds=[['a', 'b', [], {}]])['qubits']
        self.assertEqual(set(qubits), set([1, 2]))
        sim.arb('front', 'cmd', 'free', qubits=qubits)
        self.assertEqual(sim.arb('op1', 'get', 'log')['log'], [
            {'cmd': 'allocate', 'qubits': {1:1, 2:2}, 'cmds': [['a', 'b', [], {}]]},
            {'cmd': 'free', 'qubits': {1:1, 2:2}},
        ])
        self.assertEqual(sim.arb('back', 'get', 'log')['log'], [
            {'cmd': 'allocate', 'qubits': {1:1, 2:2, 3:3, 4:4}, 'cmds': [['a', 'b', [], {}]]},
            {'cmd': 'free', 'qubits': {1:1, 2:2, 3:3, 4:4}},
        ])
        sim.stop()

    def test_multi_cmd(self):
        sim = Simulator(
            TestFrontend(), TestOperator(), TestBackend(),
            repro=None, stderr_verbosity=Loglevel.OFF
        )
        sim.simulate()
        qubits = sim.arb('front', 'cmd', 'allocate', num=2, cmds=[['a', 'b', [], {}], ['c', 'd', [b'33'], {'e': 'f'}]])['qubits']
        self.assertEqual(set(qubits), set([1, 2]))
        sim.arb('front', 'cmd', 'free', qubits=qubits)
        self.assertEqual(sim.arb('op1', 'get', 'log')['log'], [
            {'cmd': 'allocate', 'qubits': {1:1, 2:2}, 'cmds': [['a', 'b', [], {}], ['c', 'd', [b'33'], {'e': 'f'}]]},
            {'cmd': 'free', 'qubits': {1:1, 2:2}},
        ])
        self.assertEqual(sim.arb('back', 'get', 'log')['log'], [
            {'cmd': 'allocate', 'qubits': {1:1, 2:2, 3:3, 4:4}, 'cmds': [['a', 'b', [], {}], ['c', 'd', [b'33'], {'e': 'f'}]]},
            {'cmd': 'free', 'qubits': {1:1, 2:2, 3:3, 4:4}},
        ])
        sim.stop()

if __name__ == '__main__':
    unittest.main()
