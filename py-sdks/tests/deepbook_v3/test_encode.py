import unittest

from pysdks.deepbook_v3.transactions import encode_bool, encode_u64, encode_u128, encode_vec_u128


class TestDeepbookEncode(unittest.TestCase):
    def test_encode_bool(self):
        self.assertEqual(b"\x01", encode_bool(True))
        self.assertEqual(b"\x00", encode_bool(False))

    def test_encode_u64(self):
        self.assertEqual((123).to_bytes(8, "little"), encode_u64(123))

    def test_encode_u128(self):
        self.assertEqual((1).to_bytes(16, "little"), encode_u128(1))
        with self.assertRaises(ValueError):
            encode_u128(-1)

    def test_encode_vec_u128(self):
        out = encode_vec_u128([1, 2])
        self.assertEqual(1 + 16 + 16, len(out))
        self.assertEqual(2, out[0])


if __name__ == "__main__":
    unittest.main()
