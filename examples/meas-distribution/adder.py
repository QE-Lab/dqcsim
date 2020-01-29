import unittest
from dqcsim.plugin import *
from dqcsim.host import *
import tempfile
import os

@plugin("Adder", "Tutorial", "0.1")
class Adder(Frontend):

    def decompose_toffoli(self, c1, c2, t):
        self.h_gate(t)
        self.advance(1)
        self.cnot_gate(c2, t)
        self.advance(1)
        self.tdag_gate(t)
        self.advance(1)
        self.cnot_gate(c1, t)
        self.advance(1)
        self.t_gate(t)
        self.advance(1)
        self.cnot_gate(c2, t)
        self.advance(1)
        self.tdag_gate(t)
        self.advance(1)
        self.cnot_gate(c1, t)
        self.advance(1)
        self.t_gate(t)
        self.t_gate(c2)
        self.advance(1)
        self.cnot_gate(c1, c2)
        self.h_gate(t)
        self.advance(1)
        self.t_gate(c1)
        self.tdag_gate(c2)
        self.advance(1)
        self.cnot_gate(c1, c2)

    def full_adder(self, a, b, cin_s, cout):
        self.decompose_toffoli(a, b, cout)
        self.advance(1)
        self.cnot_gate(a, b)
        self.advance(1)
        self.decompose_toffoli(b, cin_s, cout)
        self.advance(1)
        self.cnot_gate(b, cin_s)
        self.advance(1)
        self.cnot_gate(a, b)
        self.advance(1)

    def handle_run(self):
        bitcount = 3

        # Allocate and randomize input A.
        a = self.allocate(bitcount)
        for q in a:
            self.prepare(q)
            self.h_gate(q)

        # Allocate and randomize input B.
        b = self.allocate(bitcount)
        for q in b:
            self.prepare(q)
            self.h_gate(q)

        # Set carry in/initial output to zero.
        s = self.allocate(bitcount + 1)
        for q in s:
            self.prepare(q)

        # Run the addition.
        for i in range(bitcount):
            self.full_adder(a[i], b[i], s[i], s[i+1])

        # Measure the result.
        s_int = 0
        for i, q in reversed(list(enumerate(s))):
            self.measure(q)
            if self.get_measurement(q).value:
                s_int |= 1 << i

        # To double-check, measure A and B too.
        a_int = 0
        for i, q in reversed(list(enumerate(a))):
            self.measure(q)
            if self.get_measurement(q).value:
                a_int |= 1 << i
        b_int = 0
        for i, q in reversed(list(enumerate(b))):
            self.measure(q)
            if self.get_measurement(q).value:
                b_int |= 1 << i

        # Print the result.
        self.info('The observed addition was %d + %d = %d' % (a_int, b_int, s_int))
        if a_int + b_int != s_int:
            self.warn('... and that\'s incorrect!')

        # Free the qubits.
        self.free(a + b + s)

Adder().run()
