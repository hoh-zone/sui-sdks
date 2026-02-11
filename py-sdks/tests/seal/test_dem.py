import unittest

from pysdks.seal.dem import AesGcm256, Hmac256Ctr


class TestDem(unittest.TestCase):
    def test_aes_gcm_roundtrip(self):
        msg = b"hello seal"
        aad = b"aad"
        dem = AesGcm256(msg, aad)
        key = dem.generate_key()
        ct = dem.encrypt(key)
        pt = AesGcm256.decrypt(key, ct)
        self.assertEqual(msg, pt)

    def test_hmac_ctr_roundtrip(self):
        msg = b"hello seal ctr"
        aad = b"aad-2"
        dem = Hmac256Ctr(msg, aad)
        key = dem.generate_key()
        ct = dem.encrypt(key)
        pt = Hmac256Ctr.decrypt(key, ct)
        self.assertEqual(msg, pt)
