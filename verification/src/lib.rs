//! Bitcoin consensus verification
//!
//!	Full block verification consists of two phases:
//!	- pre-verification
//! - full-verification
//!
//! In this library, pre-verification is done by `VerifyXXX` structures
//! Full-verification is done by `AcceptXXX` structures
//!
//! Use cases:
//!
//! --> A. on_new_block:
//!
//! A.1 VerifyHeader
//! A.2 VerifyBlock,
//! A.3 VerifyTransaction for each tx
//!
//! A.4.a if it is block from canon chain
//! A.4.a.1 AcceptHeader
//! A.4.a.2 AcceptBlock
//! A.4.a.3 AcceptTransaction for each tx
//!
//! A.4.b if it is block from side chain becoming canon
//! decanonize old canon chain blocks
//! canonize new canon chain blocks (without currently processed block)
//! A.4.b.1 AcceptHeader for each header
//! A.4.b.2 AcceptBlock for each block
//! A.4.b.3 AcceptTransaction for each tx in each block
//! A.4.b.4 AcceptHeader
//! A.4.b.5 AcceptBlock
//! A.4.b.6 AcceptTransaction for each tx
//! if any step failed, revert chain back to old canon
//!
//! A.4.c if it is block from side chain do nothing
//!
//! --> B. on_memory_pool_transaction
//!
//! B.1 VerifyMemoryPoolTransaction
//! B.2 AcceptMemoryPoolTransaction
//!
//! --> C. on_block_header
//!
//! C.1 VerifyHeader
//! C.2 AcceptHeader (?)
//!
//! --> D. after successfull chain_reorganization
//!
//! D.1 AcceptMemoryPoolTransaction on each tx in memory pool
//!
//! --> E. D might be super inefficient when memory pool is large
//! so instead we might want to call AcceptMemoryPoolTransaction on each tx
//! that is inserted into assembled block

mod canon;
pub mod constants;
mod deployments;
mod error;
mod sigops;
mod timestamp;
mod work;
mod work_bch;

// pre-verification
mod verify_block;
mod verify_chain;
mod verify_header;
mod verify_transaction;

// full verification
mod accept_block;
mod accept_chain;
mod accept_header;
mod accept_transaction;

// backwards compatibility
mod chain_verifier;

pub use crate::accept_block::BlockAcceptor;
pub use crate::accept_chain::ChainAcceptor;
pub use crate::accept_header::HeaderAcceptor;
pub use crate::accept_transaction::{MemoryPoolTransactionAcceptor, TransactionAcceptor};
pub use crate::canon::{CanonBlock, CanonHeader, CanonTransaction};

pub use crate::verify_block::BlockVerifier;
pub use crate::verify_chain::ChainVerifier;
pub use crate::verify_header::HeaderVerifier;
pub use crate::verify_transaction::{MemoryPoolTransactionVerifier, TransactionVerifier};

pub use crate::chain_verifier::BackwardsCompatibleChainVerifier;
pub use crate::deployments::Deployments;
pub use crate::error::{Error, TransactionError};
pub use crate::sigops::transaction_sigops;
pub use crate::timestamp::{median_timestamp, median_timestamp_inclusive};
pub use crate::work::{
    block_reward_satoshi, is_valid_proof_of_work, is_valid_proof_of_work_hash, work_required,
};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Blocks verification level.
pub enum VerificationLevel {
    /// Full verification.
    Full,
    /// Transaction scripts are not checked.
    Header,
    /// No verification at all.
    NoVerification,
}

/// Interface for block verification
pub trait Verify: Send + Sync {
    fn verify(&self, level: VerificationLevel, block: &chain::IndexedBlock) -> Result<(), Error>;
}
