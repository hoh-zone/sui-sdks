import unittest
from unittest.mock import patch

from pysdks.sui.keypairs import Ed25519Keypair, SignatureScheme
from pysdks.sui.multisig import (
    MAX_SIGNER_IN_MULTISIG,
    MultisigPublicKey,
    MultisigSignature,
    MultisigSigner,
)


class TestMultisig(unittest.TestCase):
    def test_threshold_verification(self):
        k1 = Ed25519Keypair.generate()
        k2 = Ed25519Keypair.generate()
        k3 = Ed25519Keypair.generate()

        message = b"m"
        pub = MultisigPublicKey(
            public_keys=[k1.public_key_bytes(), k2.public_key_bytes(), k3.public_key_bytes()],
            weights=[1, 1, 1],
            threshold=2,
        )
        signer = MultisigSigner(pub, scheme=SignatureScheme.ED25519)

        ms = MultisigSignature(signatures=[k1.sign(message), k2.sign(message)], bitmap=[0, 1])
        self.assertTrue(signer.verify(message, ms))

        ms_low = MultisigSignature(signatures=[k1.sign(message)], bitmap=[0])
        self.assertFalse(signer.verify(message, ms_low))

    def test_duplicate_index_rejected(self):
        k1 = Ed25519Keypair.generate()
        message = b"m"
        pub = MultisigPublicKey(public_keys=[k1.public_key_bytes()], weights=[2], threshold=2)
        signer = MultisigSigner(pub, scheme=SignatureScheme.ED25519)
        ms = MultisigSignature(signatures=[k1.sign(message), k1.sign(message)], bitmap=[0, 0])
        self.assertFalse(signer.verify(message, ms))

    def test_unreachable_threshold_rejected(self):
        k1 = Ed25519Keypair.generate()
        with self.assertRaises(ValueError):
            MultisigPublicKey(public_keys=[k1.public_key_bytes()], weights=[1], threshold=2)

    def test_invalid_weight_rejected(self):
        k1 = Ed25519Keypair.generate()
        with self.assertRaises(ValueError):
            MultisigPublicKey(public_keys=[k1.public_key_bytes()], weights=[0], threshold=1)

    def test_max_signers_limit(self):
        signers = [Ed25519Keypair.generate() for _ in range(MAX_SIGNER_IN_MULTISIG + 1)]
        with self.assertRaises(ValueError):
            MultisigPublicKey(
                public_keys=[s.public_key_bytes() for s in signers],
                weights=[1 for _ in signers],
                threshold=1,
            )

    def test_signature_roundtrip_serialization(self):
        ms = MultisigSignature(signatures=[b"sig1", b"sig2"], bitmap=[0, 2])
        encoded = ms.to_base64()
        decoded = MultisigSignature.from_base64(encoded)
        self.assertEqual(ms.signatures, decoded.signatures)
        self.assertEqual(ms.bitmap, decoded.bitmap)

    def test_build_and_sign_helpers(self):
        pub = MultisigPublicKey(public_keys=[b"pk1", b"pk2", b"pk3"], weights=[1, 1, 1], threshold=2)
        signer = MultisigSigner(pub, scheme=SignatureScheme.ED25519)

        with patch("pysdks.sui.multisig.verify_raw_signature", return_value=True):
            built = signer.build(b"m", [(0, b"s1"), (1, b"s2")])
            self.assertEqual([0, 1], built.bitmap)
            self.assertTrue(signer.is_threshold_met(built))

        class _FakeSigner:
            def __init__(self, value: bytes):
                self.value = value

            def sign(self, _message: bytes) -> bytes:
                return self.value

        with patch("pysdks.sui.multisig.verify_raw_signature", return_value=True):
            signed = signer.sign(b"m", [(1, _FakeSigner(b"sx")), (2, _FakeSigner(b"sy"))])
            self.assertEqual([1, 2], signed.bitmap)

    def test_build_validation_errors(self):
        pub = MultisigPublicKey(public_keys=[b"pk1", b"pk2"], weights=[1, 1], threshold=2)
        signer = MultisigSigner(pub, scheme=SignatureScheme.ED25519)

        with self.assertRaises(ValueError):
            signer.build(b"m", [])

        with self.assertRaises(ValueError):
            signer.build(b"m", [(0, b"s1"), (0, b"s2")])

        with patch("pysdks.sui.multisig.verify_raw_signature", return_value=False):
            with self.assertRaises(ValueError):
                signer.build(b"m", [(0, b"s1"), (1, b"s2")])


if __name__ == "__main__":
    unittest.main()
