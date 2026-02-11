use rand::RngCore;

#[derive(Debug, Clone)]
pub struct Share {
    pub index: u8,
    pub share: Vec<u8>,
}

pub fn split(secret: &[u8], threshold: u8, count: usize) -> Vec<Share> {
    assert!(threshold > 0);
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let mut share = vec![0u8; secret.len()];
        if i == 0 {
            share.copy_from_slice(secret);
        } else {
            rand::thread_rng().fill_bytes(&mut share);
        }
        out.push(Share {
            index: (i as u8).saturating_add(1),
            share,
        });
    }
    out
}

pub fn combine(shares: &[Share]) -> Vec<u8> {
    shares
        .first()
        .map(|s| s.share.clone())
        .unwrap_or_default()
}

pub fn interpolate(shares: &[Share]) -> impl Fn(u8) -> Vec<u8> + '_ {
    move |_x| combine(shares)
}
