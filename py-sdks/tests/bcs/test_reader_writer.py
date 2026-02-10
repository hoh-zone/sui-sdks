import unittest

from pysdks.bcs import BCSReader, BCSWriter


class TestReaderWriter(unittest.TestCase):
    def test_read_write_primitives(self):
        w = BCSWriter()
        w.write_u8(1)
        w.write_u16(0x2233)
        w.write_u32(0x44556677)
        w.write_u64(0x1122334455667788)
        w.write_bool(True)
        w.write_uleb128(300)

        r = BCSReader(w.to_bytes())
        self.assertEqual(1, r.read_u8())
        self.assertEqual(0x2233, r.read_u16())
        self.assertEqual(0x44556677, r.read_u32())
        self.assertEqual(0x1122334455667788, r.read_u64())
        self.assertTrue(r.read_bool())
        self.assertEqual(300, r.read_uleb128())
        self.assertEqual(0, r.remaining())

    def test_bool_non_canonical(self):
        r = BCSReader(bytes([2]))
        with self.assertRaises(ValueError):
            r.read_bool()


if __name__ == "__main__":
    unittest.main()
