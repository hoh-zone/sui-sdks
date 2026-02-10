import unittest

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


if __name__ == "__main__":
    unittest.main()
