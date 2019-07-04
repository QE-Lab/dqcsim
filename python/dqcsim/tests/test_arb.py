import unittest

from dqcsim.common.arb import ArbData
from dqcsim import raw

class Constructor(unittest.TestCase):

    def test_empty(self):
        self.assertEqual(repr(ArbData()), "ArbData()")

    def test_string_arg(self):
        with self.assertRaises(TypeError):
            ArbData("a")

    def test_bin_strs(self):
        self.assertEqual(repr(ArbData(b'a', b'b', b'\x00')), "ArbData(b'a', b'b', b'\\x00')")

    def test_json_good(self):
        self.assertEqual(repr(ArbData(a=3, b="hello", c={}, d=[1, 2.3, "four"])), "ArbData(a=3, b='hello', c={}, d=[1, 2.3, 'four'])")

    def test_json_bad(self):
        with self.assertRaises(TypeError):
            ArbData(a=ArbData())

    def test_copy(self):
        x = ArbData(b'a', b'b', a=3, b="hello", c={}, d=[1, 2.3, "four"])
        xr = repr(x)
        y = ArbData(x)
        self.assertEqual(repr(y), xr)
        x['d'][1] = 2.5
        self.assertEqual(repr(y), xr)
        x[0] = b'x'
        self.assertEqual(repr(y), xr)

class Operations(unittest.TestCase):

    def test_bool(self):
        self.assertEqual(bool(ArbData()), False)
        self.assertEqual(bool(ArbData(b'b')), True)
        self.assertEqual(bool(ArbData(a=3)), True)

    def test_len(self):
        self.assertEqual(len(ArbData()), 0)
        self.assertEqual(len(ArbData(b'b')), 1)
        self.assertEqual(len(ArbData(b'a', b'b', b'c')), 3)
        self.assertEqual(len(ArbData(a=3)), 0)

    def test_get(self):
        a = ArbData(b'a', b'b', b'c', a=3, b=4, c=5)
        self.assertEqual(a[0], b'a')
        self.assertEqual(a[1], b'b')
        self.assertEqual(a[-1], b'c')
        self.assertEqual(a['a'], 3)

        with self.assertRaises(KeyError):
            a['x']
        with self.assertRaises(IndexError):
            a[4]
        with self.assertRaises(TypeError):
            a[3.3]

    def test_set(self):
        a = ArbData()
        with self.assertRaises(IndexError):
            a[0] = b'a'
        a = ArbData(b'a', b'b', b'c')
        self.assertEqual(a[1], b'b')
        a[1] = b'x'
        self.assertEqual(a[1], b'x')
        with self.assertRaises(TypeError):
            a[1] = 'a'

        with self.assertRaises(TypeError):
            a[3.3] = b'c'

        a["test"] = 33
        self.assertEqual(a["test"], 33)
        with self.assertRaises(TypeError):
            a["test"] = ArbData()

    def test_del_eq(self):
        a = ArbData(b'a', b'b', b'c', a=3, b=4, c=5)
        del a[1]
        del a["b"]
        with self.assertRaises(TypeError):
            del a[3.3]
        self.assertEqual(a, ArbData(b'a', b'c', a=3, c=5))
        self.assertNotEqual(a, ArbData(b'a', b'd', a=3, c=5))
        self.assertNotEqual(a, ArbData(b'a', b'c', a=3, c=6))
        self.assertNotEqual(a, 3)

    def test_contains(self):
        a = ArbData(b'a', b'b', b'c', b=3, c=4, d=5)
        self.assertTrue(b'b' in a)
        self.assertTrue('b' in a)
        self.assertFalse(b'd' in a)
        self.assertTrue('d' in a)
        self.assertFalse(b'x' in a)
        self.assertFalse('x' in a)

    def test_iter(self):
        a = ArbData(b'a', b'b', b'c', b=3, c=4, d=5)
        x = []
        for i in a:
            x.append(i)
        self.assertEqual(x, [b'a', b'b', b'c'])

    def test_append_etc(self):
        a = ArbData()

        a.append(b'b')
        with self.assertRaises(TypeError):
            a.append('b')

        a.insert(0, b'a')
        with self.assertRaises(TypeError):
            a.append(0, 'a')

        a.extend([b'c', b'd'])
        with self.assertRaises(TypeError):
            a.extend([b'e', 'f'])

        self.assertEqual(a, ArbData(b'a', b'b', b'c', b'd', b'e'))

    def test_json_iters(self):
        a = ArbData(b=3, c=5, d=4)
        self.assertEqual(sorted(a.keys()), ['b', 'c', 'd'])
        self.assertEqual(sorted(a.values()), [3, 4, 5])
        self.assertEqual(sorted(a.items()), [('b', 3), ('c', 5), ('d', 4)])

    def test_clear_etc(self):
        a = ArbData(b'a', b'b', b'c', b=3, c=4, d=5)
        a.clear_args()
        self.assertEqual(a, ArbData(b=3, c=4, d=5))

        a = ArbData(b'a', b'b', b'c', b=3, c=4, d=5)
        a.clear_json()
        self.assertEqual(a, ArbData(b'a', b'b', b'c'))

        a = ArbData(b'a', b'b', b'c', b=3, c=4, d=5)
        a.clear()
        self.assertEqual(a, ArbData())

    def test_handles(self):
        a = ArbData(b'a', b'b', b'c', b=3, c=4, d=5)._to_raw()
        self.maxDiff = None
        self.assertEqual(str(a), """ArbData(
    ArbData {
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
)""")

        ArbData(b'c', b'd', b'e', b=6, c=7, d=8)._to_raw(a)
        self.assertEqual(str(a), """ArbData(
    ArbData {
        json: Map(
            {
                Text(
                    "b",
                ): Integer(
                    6,
                ),
                Text(
                    "c",
                ): Integer(
                    7,
                ),
                Text(
                    "d",
                ): Integer(
                    8,
                ),
            },
        ),
        args: [
            [
                99,
            ],
            [
                100,
            ],
            [
                101,
            ],
        ],
    },
)""")

        self.assertEqual(ArbData._from_raw(a), ArbData(b'c', b'd', b'e', b=6, c=7, d=8))

    def test_long_data(self):
        data = list(range(256))
        bdata = bytes(data) * 4
        a = ArbData(bdata, b=data)
        self.assertEqual(ArbData._from_raw(a._to_raw()), a)

if __name__ == '__main__':
    unittest.main()
