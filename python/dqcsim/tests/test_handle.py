import unittest

from dqcsim.common.handle import Handle
from dqcsim import raw

class Tests(unittest.TestCase):

    def test_normal(self):
        xh = raw.dqcs_arb_new()
        h = Handle(xh)

        self.assertTrue(bool(h))
        self.assertEqual(int(h), xh)
        self.assertEqual(str(h), """ArbData(
    ArbData {
        json: Map(
            {},
        ),
        args: [],
    },
)""")
        self.assertEqual(repr(h), """Handle({})""".format(xh))
        self.assertEqual(h.get_type(), raw.DQCS_HTYPE_ARB_DATA)
        self.assertFalse(h.is_type(raw.DQCS_HTYPE_INVALID))
        self.assertTrue(h.is_type(raw.DQCS_HTYPE_ARB_DATA))
        with h as i:
            self.assertEqual(i, xh)

        self.assertEqual(raw.dqcs_handle_type(xh), raw.DQCS_HTYPE_ARB_DATA)

        del h

        self.assertEqual(raw.dqcs_handle_type(xh), raw.DQCS_HTYPE_INVALID)

    def test_taken(self):
        xh = raw.dqcs_arb_new()
        h = Handle(xh)

        self.assertTrue(bool(h))
        self.assertEqual(int(h), xh)
        self.assertEqual(str(h), """ArbData(
    ArbData {
        json: Map(
            {},
        ),
        args: [],
    },
)""")
        self.assertEqual(repr(h), """Handle({})""".format(xh))
        self.assertEqual(h.get_type(), raw.DQCS_HTYPE_ARB_DATA)
        self.assertFalse(h.is_type(raw.DQCS_HTYPE_INVALID))
        self.assertTrue(h.is_type(raw.DQCS_HTYPE_ARB_DATA))
        with h as i:
            self.assertEqual(i, xh)

        self.assertEqual(h.take(), xh)

        self.assertFalse(bool(h))
        with self.assertRaises(ValueError):
            int(h)
        self.assertEqual(str(h), """Handle(None)""")
        self.assertEqual(repr(h), """Handle(None)""")
        with self.assertRaises(ValueError):
            h.get_type(), raw.DQCS_HTYPE_INVALID
        with self.assertRaises(ValueError):
            h.is_type(raw.DQCS_HTYPE_INVALID)
        with self.assertRaises(ValueError):
            with h as i:
                pass
        with self.assertRaises(ValueError):
            h.take()

        del h

        self.assertEqual(raw.dqcs_handle_type(xh), raw.DQCS_HTYPE_ARB_DATA)
        raw.dqcs_handle_delete(xh)

    def test_deleted(self):
        xh = raw.dqcs_arb_new()
        h = Handle(xh)
        raw.dqcs_handle_delete(xh)

        self.assertTrue(bool(h))
        self.assertEqual(int(h), xh)
        self.assertEqual(str(h), """Handle({}?)""".format(xh))
        self.assertEqual(repr(h), """Handle({})""".format(xh))
        self.assertEqual(h.get_type(), raw.DQCS_HTYPE_INVALID)
        self.assertTrue(h.is_type(raw.DQCS_HTYPE_INVALID))
        self.assertFalse(h.is_type(raw.DQCS_HTYPE_ARB_DATA))
        with h as i:
            self.assertEqual(i, xh)

        del h

    def test_empty(self):
        h = Handle()

        self.assertFalse(bool(h))
        self.assertEqual(int(h), 0)
        self.assertEqual(str(h), """Handle(0)""")
        self.assertEqual(repr(h), """Handle(0)""")
        self.assertEqual(h.get_type(), raw.DQCS_HTYPE_INVALID)
        self.assertTrue(h.is_type(raw.DQCS_HTYPE_INVALID))
        self.assertFalse(h.is_type(raw.DQCS_HTYPE_ARB_DATA))
        with h as i:
            self.assertEqual(i, 0)

        self.assertEqual(h.take(), 0)

        self.assertFalse(bool(h))
        with self.assertRaises(ValueError):
            int(h)
        self.assertEqual(str(h), """Handle(None)""")
        self.assertEqual(repr(h), """Handle(None)""")
        with self.assertRaises(ValueError):
            h.get_type(), raw.DQCS_HTYPE_INVALID
        with self.assertRaises(ValueError):
            h.is_type(raw.DQCS_HTYPE_INVALID)
        with self.assertRaises(ValueError):
            with h as i:
                pass
        with self.assertRaises(ValueError):
            h.take()

        del h

    def test_none(self):
        with self.assertRaises(TypeError):
            Handle(None)


if __name__ == '__main__':
    unittest.main()
