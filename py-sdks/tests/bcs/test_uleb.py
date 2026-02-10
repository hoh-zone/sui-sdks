import unittest

from pysdks.bcs import decode_uleb128, encode_uleb128


class TestULEB(unittest.TestCase):
    def test_roundtrip(self):
        for value in [0, 1, 127, 128, 16384, 2**32 - 1]:
            encoded = encode_uleb128(value)
            decoded, consumed = decode_uleb128(encoded)
            self.assertEqual(value, decoded)
            self.assertEqual(len(encoded), consumed)

    def test_non_canonical_rejected(self):
        with self.assertRaises(ValueError):
            decode_uleb128(bytes([0x80, 0x00]))

    def test_u32_overflow_rejected(self):
        with self.assertRaises(ValueError):
            decode_uleb128(encode_uleb128(2**32))


if __name__ == "__main__":
    unittest.main()
