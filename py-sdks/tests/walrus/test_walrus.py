import unittest
from unittest.mock import patch

from pysdks.walrus import WalrusClient, blob_id_from_int, blob_id_to_int


class _MockSui:
    pass


class TestWalrus(unittest.TestCase):
    def test_blob_id_roundtrip(self):
        value = 12345678901234567890
        encoded = blob_id_from_int(value)
        decoded = blob_id_to_int(encoded)
        self.assertEqual(value, decoded)

    def test_compute_blob_metadata(self):
        client = WalrusClient.from_network(_MockSui(), network="testnet")
        out = client.compute_blob_metadata(b"hello")
        self.assertIn("blob_id", out)
        self.assertEqual(5, out["metadata"].unencoded_length)

    def test_read_blob(self):
        client = WalrusClient.from_network(_MockSui(), network="testnet")

        with patch.object(client.storage_node_client, "get_blob_metadata") as metadata, patch.object(
            client.storage_node_client, "get_sliver"
        ) as sliver:
            metadata.return_value = type("M", (), {"metadata": type("V", (), {"unencoded_length": 5})()})()
            sliver.return_value = b"hello"
            data = client.read_blob("blob", "https://node.invalid")
            self.assertEqual(b"hello", data)
