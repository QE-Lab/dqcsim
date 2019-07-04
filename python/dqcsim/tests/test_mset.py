import unittest

from dqcsim.common.meas import Measurement
from dqcsim.common.mset import MeasurementSet
from dqcsim import raw

class Tests(unittest.TestCase):

    def test_all(self):
        self.maxDiff = None

        a = MeasurementSet._to_raw()
        self.assertEqual(str(a), """QubitMeasurementResultSet(
    {},
)""")
        self.assertEqual(MeasurementSet._from_raw(a), [])

        a = MeasurementSet._to_raw(Measurement(1, 0))
        self.assertEqual(str(a), """QubitMeasurementResultSet(
    {
        QubitRef(
            1,
        ): QubitMeasurementResult {
            qubit: QubitRef(
                1,
            ),
            value: Zero,
            data: ArbData {
                json: Map(
                    {},
                ),
                args: [],
            },
        },
    },
)""")
        self.assertEqual(MeasurementSet._from_raw(a), [Measurement(1, 0)])

        a = MeasurementSet._to_raw(Measurement(1, 0), Measurement(2, 1), Measurement(3, None))
        self.assertEqual(MeasurementSet._from_raw(a), [Measurement(1, 0), Measurement(2, 1), Measurement(3, None)])

        a = MeasurementSet._to_raw([Measurement(1, 0), Measurement(3, None), Measurement(2, 1)])
        self.assertEqual(MeasurementSet._from_raw(a), [Measurement(1, 0), Measurement(2, 1), Measurement(3, None)])

        with self.assertRaises(ValueError):
            a = MeasurementSet._to_raw([Measurement(1, 0), Measurement(2, 1), Measurement(1, None)])

        with self.assertRaises(TypeError):
            a = MeasurementSet._to_raw([0])

if __name__ == '__main__':
    unittest.main()
