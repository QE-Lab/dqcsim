import unittest

from dqcsim.common.qbset import QubitSet
from dqcsim import raw

class Tests(unittest.TestCase):

    def test_all(self):
        a = QubitSet._to_raw()
        self.assertEqual(str(a), """QubitReferenceSet(
    [],
)""")
        self.assertEqual(QubitSet._from_raw(a), [])

        a = QubitSet._to_raw(1)
        self.assertEqual(str(a), """QubitReferenceSet(
    [
        QubitRef(
            1,
        ),
    ],
)""")
        self.assertEqual(QubitSet._from_raw(a), [1])

        a = QubitSet._to_raw(1, 2, 3)
        self.assertEqual(str(a), """QubitReferenceSet(
    [
        QubitRef(
            1,
        ),
        QubitRef(
            2,
        ),
        QubitRef(
            3,
        ),
    ],
)""")
        self.assertEqual(QubitSet._from_raw(a), [1, 2, 3])

        a = QubitSet._to_raw([1, 2, 3])
        self.assertEqual(str(a), """QubitReferenceSet(
    [
        QubitRef(
            1,
        ),
        QubitRef(
            2,
        ),
        QubitRef(
            3,
        ),
    ],
)""")
        self.assertEqual(QubitSet._from_raw(a), [1, 2, 3])

        with self.assertRaises(RuntimeError):
            a = QubitSet._to_raw([1, 2, 1])

        with self.assertRaises(RuntimeError):
            a = QubitSet._to_raw([0])

if __name__ == '__main__':
    unittest.main()
