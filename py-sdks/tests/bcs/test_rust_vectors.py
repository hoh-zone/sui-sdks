import json
import unittest
from pathlib import Path

from pysdks.bcs import decode_uleb128, encode_uleb128


class TestRustVectorFormatULEB(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        root = Path(__file__).resolve().parents[1]
        fixture = root / "vectors" / "rust_bcs_uleb_vectors.json"
        cls.payload = json.loads(fixture.read_text(encoding="utf-8"))

    def test_fixture_schema(self):
        self.assertIn("meta", self.payload)
        self.assertIn("vectors", self.payload)
        self.assertIsInstance(self.payload["vectors"], list)
        self.assertGreater(len(self.payload["vectors"]), 0)

    def test_decode_vectors(self):
        for case in self.payload["vectors"]:
            with self.subTest(case=case["name"]):
                data = bytes.fromhex(case["encoded_hex"])
                if "error_contains" in case:
                    with self.assertRaises(ValueError) as cm:
                        decode_uleb128(data)
                    self.assertIn(case["error_contains"], str(cm.exception))
                    continue

                expected = case["value"]
                value, consumed = decode_uleb128(data)
                self.assertEqual(expected, value)
                self.assertEqual(len(data), consumed)

    def test_encode_vectors(self):
        for case in self.payload["vectors"]:
            with self.subTest(case=case["name"]):
                if "value" not in case:
                    continue
                expected = bytes.fromhex(case["encoded_hex"])
                encoded = encode_uleb128(case["value"])
                self.assertEqual(expected, encoded)


if __name__ == "__main__":
    unittest.main()
