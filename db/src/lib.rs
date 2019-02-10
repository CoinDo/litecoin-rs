#[macro_use]
extern crate log;

mod block_chain_db;
pub mod kv;

pub use crate::block_chain_db::{BlockChainDatabase, ForkChainDatabase};
