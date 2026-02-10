import unittest

from pysdks.sui.keypairs import Ed25519Keypair, Secp256k1Keypair, Secp256r1Keypair


class TestKeypairs(unittest.TestCase):
    def test_ed25519_sign_verify(self):
        kp = Ed25519Keypair.generate()
        self.assertEqual(32, len(kp.public_key_bytes()))

        msg = b"hello"
        sig = kp.sign(msg)
        self.assertEqual(64, len(sig))

        self.assertTrue(kp.verify(msg, sig))
        self.assertFalse(kp.verify(b"other", sig))

    def test_secp256k1_sign_verify_with_public(self):
        kp = Secp256k1Keypair.generate()
        self.assertEqual(33, len(kp.public_key_bytes()))

        msg = b"hello"
        sig = kp.sign(msg)
        self.assertEqual(64, len(sig))

        self.assertTrue(Secp256k1Keypair.verify_with_public_key(kp.public_key_bytes(), msg, sig))

    def test_secp256r1_sign_verify_with_public(self):
        kp = Secp256r1Keypair.generate()
        self.assertEqual(33, len(kp.public_key_bytes()))

        msg = b"hello"
        sig = kp.sign(msg)
        self.assertEqual(64, len(sig))

        self.assertTrue(Secp256r1Keypair.verify_with_public_key(kp.public_key_bytes(), msg, sig))


if __name__ == "__main__":
    unittest.main()
