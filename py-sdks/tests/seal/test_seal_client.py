import base64
import unittest
from unittest.mock import patch

from pysdks.seal import (
    DecryptOptions,
    DemType,
    EncryptOptions,
    KeyServer,
    KeyServerConfig,
    SealClient,
    SessionKey,
)


class _MockSuiClient:
    def get_object(self, object_id, options=None):
        if object_id == "0xpackage":
            return {"object": {"version": "1"}}
        return {"name": f"server-{object_id}", "url": "https://ks.invalid"}


class TestSealClient(unittest.TestCase):
    def test_encrypt_decrypt_roundtrip_with_cached_keys(self):
        client = SealClient(
            sui_client=_MockSuiClient(),
            server_configs=[KeyServerConfig(object_id="0x1", weight=1), KeyServerConfig(object_id="0x2", weight=1)],
            verify_key_servers=False,
        )

        with patch("pysdks.seal.client.retrieve_key_servers") as retrieve:
            retrieve.return_value = [
                KeyServer(object_id="0x1", name="k1", url="https://k1", weight=1),
                KeyServer(object_id="0x2", name="k2", url="https://k2", weight=1),
            ]

            encrypted = client.encrypt(
                EncryptOptions(
                    threshold=2,
                    package_id="0xpackage",
                    id="0xobj",
                    data=b"secret-message",
                    aad=b"ctx",
                    dem_type=DemType.AesGcm256,
                )
            )

            # Populate cache from encrypted payload so decrypt can run without network.
            from pysdks.seal.bcs import EncryptedObject

            parsed = EncryptedObject.parse(encrypted["encrypted_object"])
            full = f"{parsed.package_id}:{parsed.id}"
            shares = parsed.encrypted_shares["BonehFranklinBLS12381"]["encryptedShares"]
            for i, (object_id, _idx) in enumerate(parsed.services):
                client._cached_keys[f"{full}:{object_id}"] = shares[i]

            session_key = SessionKey.create(
                address="0xabc",
                package_id="0xpackage",
                ttl_min=5,
                sui_client=_MockSuiClient(),
            )

            plain = client.decrypt(
                DecryptOptions(
                    data=encrypted["encrypted_object"],
                    session_key=session_key,
                    tx_bytes=b"\x00fake-tx",
                )
            )

            self.assertEqual(b"secret-message", plain)

    def test_fetch_keys_threshold(self):
        client = SealClient(
            sui_client=_MockSuiClient(),
            server_configs=[KeyServerConfig(object_id="0x1", weight=1)],
            verify_key_servers=False,
        )
        session_key = SessionKey.create(
            address="0xabc",
            package_id="0xpackage",
            ttl_min=5,
            sui_client=_MockSuiClient(),
        )

        with patch("pysdks.seal.client.retrieve_key_servers") as retrieve, patch(
            "pysdks.seal.client.fetch_keys_for_all_ids"
        ) as fetch_keys:
            retrieve.return_value = [KeyServer(object_id="0x1", name="k1", url="https://k1", weight=1)]
            fetch_keys.return_value = [{"full_id": "0x1", "key": base64.b64decode(base64.b64encode(b"x" * 32))}]
            client.fetch_keys(
                type("Fetch", (), {
                    "ids": ["0xobj"],
                    "tx_bytes": b"\x00tx",
                    "session_key": session_key,
                    "threshold": 1,
                })()
            )
            self.assertTrue(client._cached_keys)
