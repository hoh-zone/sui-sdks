import unittest

from pysdks.sui.keypairs import Ed25519Keypair, Secp256k1Keypair, Secp256r1Keypair, SignatureScheme
from pysdks.sui.verify import (
    parse_serialized_signature,
    to_serialized_signature,
    verify_personal_message,
    verify_raw_signature,
)


class TestVerify(unittest.TestCase):
    def test_verify_raw_and_personal_message(self):
        kp = Ed25519Keypair.generate()
        msg = b"abc"

        sig_raw = kp.sign(msg)
        self.assertTrue(verify_raw_signature(msg, sig_raw, kp.public_key_bytes(), SignatureScheme.ED25519))

        personal_payload = b"\x19Sui Signed Message:\n3\nabc"
        sig_personal = kp.sign(personal_payload)
        self.assertTrue(verify_personal_message(msg, sig_personal, kp.public_key_bytes(), SignatureScheme.ED25519))

    def test_serialized_signature_roundtrip_ed25519(self):
        kp = Ed25519Keypair.generate()
        sig = kp.sign(b"hello")
        ser = to_serialized_signature(SignatureScheme.ED25519, sig, kp.public_key_bytes())
        parsed = parse_serialized_signature(ser)
        self.assertEqual(SignatureScheme.ED25519, parsed.scheme)
        self.assertEqual(sig, parsed.signature)
        self.assertEqual(kp.public_key_bytes(), parsed.public_key)

    def test_serialized_signature_roundtrip_secp(self):
        k1 = Secp256k1Keypair.generate()
        sig1 = k1.sign(b"hello")
        ser1 = to_serialized_signature(SignatureScheme.SECP256K1, sig1, k1.public_key_bytes())
        parsed1 = parse_serialized_signature(ser1)
        self.assertEqual(SignatureScheme.SECP256K1, parsed1.scheme)
        self.assertEqual(sig1, parsed1.signature)
        self.assertEqual(k1.public_key_bytes(), parsed1.public_key)

        r1 = Secp256r1Keypair.generate()
        sig2 = r1.sign(b"hello")
        ser2 = to_serialized_signature(SignatureScheme.SECP256R1, sig2, r1.public_key_bytes())
        parsed2 = parse_serialized_signature(ser2)
        self.assertEqual(SignatureScheme.SECP256R1, parsed2.scheme)
        self.assertEqual(sig2, parsed2.signature)
        self.assertEqual(r1.public_key_bytes(), parsed2.public_key)

    def test_serialized_signature_length_validation(self):
        kp = Ed25519Keypair.generate()
        sig = kp.sign(b"m")

        with self.assertRaises(ValueError):
            to_serialized_signature(SignatureScheme.SECP256K1, sig, kp.public_key_bytes())


if __name__ == "__main__":
    unittest.main()
