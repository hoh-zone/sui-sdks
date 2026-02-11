pub mod keypair;
pub mod signature;

pub use crate::keypairs::ed25519;
pub use crate::keypairs::secp256k1;
pub use crate::keypairs::secp256r1;
pub use keypair::Keypair;
pub use signature::Signature;

pub type Ed25519Keypair = crate::keypairs::ed25519::Keypair;
pub type Secp256k1Keypair = crate::keypairs::secp256k1::Keypair;
pub type Secp256r1Keypair = crate::keypairs::secp256r1::Keypair;
