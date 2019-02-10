use chain::Transaction;
use primitives::hash::H256;
use serialization_derive::{Deserializable, Serializable};

#[derive(Debug, PartialEq, Serializable, Deserializable)]
pub struct BlockTransactions {
    pub blockhash: H256,
    pub transactions: Vec<Transaction>,
}
