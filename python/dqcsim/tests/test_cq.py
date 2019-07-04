import unittest

from dqcsim.common.cmd import ArbCmd
from dqcsim.common.cq import ArbCmdQueue
from dqcsim import raw

class Tests(unittest.TestCase):

    def test_all(self):
        a = ArbCmdQueue._to_raw()
        self.assertEqual(str(a), """ArbCmdQueue(
    [],
)""")
        self.assertEqual(ArbCmdQueue._from_raw(a), [])

        a = ArbCmdQueue._to_raw(ArbCmd('a', 'b'))
        self.assertEqual(str(a), """ArbCmdQueue(
    [
        ArbCmd {
            interface_identifier: "a",
            operation_identifier: "b",
            data: ArbData {
                json: Map(
                    {},
                ),
                args: [],
            },
        },
    ],
)""")
        self.assertEqual(ArbCmdQueue._from_raw(a), [ArbCmd('a', 'b')])

        a = ArbCmdQueue._to_raw(ArbCmd('a', 'b'), ArbCmd('c', 'd'))
        self.assertEqual(str(a), """ArbCmdQueue(
    [
        ArbCmd {
            interface_identifier: "a",
            operation_identifier: "b",
            data: ArbData {
                json: Map(
                    {},
                ),
                args: [],
            },
        },
        ArbCmd {
            interface_identifier: "c",
            operation_identifier: "d",
            data: ArbData {
                json: Map(
                    {},
                ),
                args: [],
            },
        },
    ],
)""")
        self.assertEqual(ArbCmdQueue._from_raw(a), [ArbCmd('a', 'b'), ArbCmd('c', 'd')])

        a = ArbCmdQueue._to_raw([ArbCmd('a', 'b'), ArbCmd('c', 'd')])
        self.assertEqual(str(a), """ArbCmdQueue(
    [
        ArbCmd {
            interface_identifier: "a",
            operation_identifier: "b",
            data: ArbData {
                json: Map(
                    {},
                ),
                args: [],
            },
        },
        ArbCmd {
            interface_identifier: "c",
            operation_identifier: "d",
            data: ArbData {
                json: Map(
                    {},
                ),
                args: [],
            },
        },
    ],
)""")
        self.assertEqual(ArbCmdQueue._from_raw(a), [ArbCmd('a', 'b'), ArbCmd('c', 'd')])

        with self.assertRaises(TypeError):
            a = ArbCmdQueue._to_raw(33)

        with self.assertRaises(TypeError):
            a = ArbCmdQueue._to_raw(33, 33)

if __name__ == '__main__':
    unittest.main()
