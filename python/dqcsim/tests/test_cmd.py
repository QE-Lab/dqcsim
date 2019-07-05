import unittest

from dqcsim.common.arb import ArbData
from dqcsim.common.cmd import ArbCmd
from dqcsim import raw

class Tests(unittest.TestCase):

    def test_constructor(self):
        self.assertEqual(repr(ArbCmd("a", "b")), "ArbCmd('a', 'b')")

        with self.assertRaises(TypeError):
            ArbCmd()

        with self.assertRaises(ValueError):
            ArbCmd("$#^", "dkbng")

        with self.assertRaises(ValueError):
            ArbCmd(" hello ", "dkbng")

        with self.assertRaises(ValueError):
            ArbCmd("a", ".dkbng")

        self.assertEqual(repr(ArbCmd("a", "b", b"c", d="e")), "ArbCmd('a', 'b', b'c', d='e')")

        self.assertEqual(repr(ArbCmd(ArbCmd("a", "b", b"c", d="e"))), "ArbCmd('a', 'b', b'c', d='e')")

    def test_getters(self):
        c = ArbCmd("a", "b")
        self.assertEqual(c.iface, "a")
        self.assertEqual(c.oper, "b")

    def test_eq(self):
        a = ArbCmd("a", "b")
        self.assertTrue(a == ArbCmd("a", "b"))
        self.assertFalse(a != ArbCmd("a", "b"))
        self.assertFalse(a == ArbCmd("a", "x"))
        self.assertFalse(a == ArbCmd("a", "x"))
        self.assertFalse(a == ArbData())
        self.assertFalse(a == ArbCmd("a", "x", b"a"))

    def test_handles(self):
        a = ArbCmd('x', 'y', b'a', b'b', b'c', b=3, c=4, d=5)
        a_handle = a._to_raw()
        self.maxDiff = None
        self.assertEqual(str(a_handle), """ArbCmd(
    ArbCmd {
        interface_identifier: "x",
        operation_identifier: "y",
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

        self.assertEqual(ArbCmd._from_raw(a_handle), a)

if __name__ == '__main__':
    unittest.main()
