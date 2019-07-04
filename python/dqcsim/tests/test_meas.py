import unittest

from dqcsim.common.arb import ArbData
from dqcsim.common.meas import Measurement
from dqcsim import raw

class Tests(unittest.TestCase):

    def test_constructor(self):
        self.assertEqual(repr(Measurement(1, 0)), "Measurement(1, 0)")
        self.assertEqual(repr(Measurement(2, 1)), "Measurement(2, 1)")
        self.assertEqual(repr(Measurement(3, True)), "Measurement(3, 1)")
        self.assertEqual(repr(Measurement(4, False)), "Measurement(4, 0)")
        self.assertEqual(repr(Measurement(5, None)), "Measurement(5, None)")

        with self.assertRaises(TypeError):
            Measurement()

        with self.assertRaises(ValueError):
            Measurement(0, 0)

        with self.assertRaises(ValueError):
            Measurement(-1, 0)

        with self.assertRaises(ValueError):
            Measurement('hello', 0)

        self.assertEqual(repr(Measurement(7, 1, b"c", d="e")), "Measurement(7, 1, b'c', d='e')")

    def test_getters(self):
        c = Measurement(33, 1)
        self.assertEqual(c.qubit, 33)
        self.assertEqual(c.value, 1)

    def test_eq(self):
        a = Measurement(1, 1)
        self.assertTrue(a == Measurement(1, 1))
        self.assertFalse(a != Measurement(1, 1))
        self.assertFalse(a == Measurement(1, 0))
        self.assertFalse(a == Measurement(2, 1))
        self.assertFalse(a == ArbData())
        self.assertFalse(a == Measurement(1, 1, b"a"))

    def test_handles(self):
        a = Measurement(33, 1, b'a', b'b', b'c', b=3, c=4, d=5)
        a_handle = a._to_raw()
        self.maxDiff = None
        self.assertEqual(str(a_handle), """QubitMeasurementResult(
    QubitMeasurementResult {
        qubit: QubitRef(
            33,
        ),
        value: One,
        data: ArbData {
            json: Map(
                {
                    Text(
                        "b",
                    ): Integer(
                        3,
                    ),
                    Text(
                        "c",
                    ): Integer(
                        4,
                    ),
                    Text(
                        "d",
                    ): Integer(
                        5,
                    ),
                },
            ),
            args: [
                [
                    97,
                ],
                [
                    98,
                ],
                [
                    99,
                ],
            ],
        },
    },
)""")
        self.assertEqual(Measurement._from_raw(a_handle), a)

        a = Measurement(42, 0)
        a_handle = a._to_raw()
        self.maxDiff = None
        self.assertEqual(str(a_handle), """QubitMeasurementResult(
    QubitMeasurementResult {
        qubit: QubitRef(
            42,
        ),
        value: Zero,
        data: ArbData {
            json: Map(
                {},
            ),
            args: [],
        },
    },
)""")
        self.assertEqual(Measurement._from_raw(a_handle), a)

        a = Measurement(23, None)
        a_handle = a._to_raw()
        self.maxDiff = None
        self.assertEqual(str(a_handle), """QubitMeasurementResult(
    QubitMeasurementResult {
        qubit: QubitRef(
            23,
        ),
        value: Undefined,
        data: ArbData {
            json: Map(
                {},
            ),
            args: [],
        },
    },
)""")
        self.assertEqual(Measurement._from_raw(a_handle), a)

if __name__ == '__main__':
    unittest.main()
