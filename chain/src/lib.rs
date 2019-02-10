pub mod constants;

mod block;
mod block_header;
mod merkle_root;
mod transaction;

mod indexed_block;
mod indexed_header;
mod indexed_transaction;
/// `IndexedBlock` extension
mod read_and_hash;

pub trait RepresentH256 {
    fn h256(&self) -> hash::H256;
}

pub use crate::block::Block;
pub use crate::block_header::BlockHeader;
pub use crate::merkle_root::{merkle_node_hash, merkle_root};
pub use crate::transaction::{OutPoint, Transaction, TransactionInput, TransactionOutput};
pub use primitives::hash;

pub use crate::indexed_block::IndexedBlock;
pub use crate::indexed_header::IndexedBlockHeader;
pub use crate::indexed_transaction::IndexedTransaction;
pub use crate::read_and_hash::{HashedData, ReadAndHash};

pub type ShortTransactionID = hash::H48;
