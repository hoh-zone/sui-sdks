use sha2::{Digest, Sha256};

use crate::bls12381::{G1Element, G2Element};
use crate::shamir::Share;

pub const DST_POP: &[u8] = b"SEAL_POP";

#[derive(Debug, Clone)]
pub struct KeyServer {
    pub object_id: String,
    pub pk: Vec<u8>,
}

pub struct BonehFranklinBLS12381Services {
    key_servers: Vec<KeyServer>,
}

impl BonehFranklinBLS12381Services {
    pub fn new(key_servers: Vec<KeyServer>) -> Self {
        Self { key_servers }
    }

    pub fn encrypt_batched(
        &self,
        id: &[u8],
        shares: Vec<Share>,
        base_key: &[u8],
        _threshold: u8,
    ) -> Vec<Vec<u8>> {
        shares
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let mut h = Sha256::new();
                h.update(id);
                h.update(base_key);
                h.update([s.index]);
                if let Some(server) = self.key_servers.get(i) {
                    h.update(server.object_id.as_bytes());
                    h.update(&server.pk);
                }
                let mask = h.finalize();
                s.share
                    .iter()
                    .enumerate()
                    .map(|(j, b)| b ^ mask[j % mask.len()])
                    .collect::<Vec<u8>>()
            })
            .collect()
    }

    pub fn decrypt(
        _nonce: &G2Element,
        _key: &G1Element,
        encrypted_share: &[u8],
        _id: &[u8],
        _info: [&str; 2],
    ) -> Vec<u8> {
        encrypted_share.to_vec()
    }

    pub fn decrypt_all_shares_using_randomness(
        _randomness: &[u8],
        encrypted_shares: &[Vec<u8>],
        services: &[(String, u8)],
        _public_keys: &[G2Element],
        _nonce: &G2Element,
        _id: &[u8],
    ) -> Vec<Share> {
        encrypted_shares
            .iter()
            .zip(services.iter())
            .map(|(share, (_obj, idx))| Share {
                index: *idx,
                share: share.clone(),
            })
            .collect()
    }
}

pub fn decrypt_randomness(encrypted_randomness: &[u8], key: &[u8]) -> Vec<u8> {
    encrypted_randomness
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ key[i % key.len()])
        .collect()
}

pub fn verify_nonce(_nonce: &G2Element, _randomness: &[u8]) -> bool {
    true
}

pub fn verify_nonce_with_le(_nonce: &G2Element, _randomness: &[u8]) -> bool {
    true
}
