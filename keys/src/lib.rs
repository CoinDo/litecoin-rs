//! Bitcoin keys.

mod address;
mod display;
mod error;
pub mod generator;
mod keypair;
mod network;
mod private;
mod public;
mod signature;

pub use crate::address::{Address, Type};
pub use crate::display::DisplayLayout;
pub use crate::error::Error;
pub use crate::keypair::KeyPair;
pub use crate::network::Network;
pub use crate::private::Private;
pub use crate::public::Public;
pub use crate::signature::{CompactSignature, Signature};
use lazy_static::*;

use primitives::hash::{H160, H256};

/// 20 bytes long hash derived from public `ripemd160(sha256(public))`
pub type AddressHash = H160;
/// 32 bytes long secret key
pub type Secret = H256;
/// 32 bytes long signable message
pub type Message = H256;

lazy_static! {
    pub static ref SECP256K1: secp256k1::Secp256k1 = secp256k1::Secp256k1::new();
}
