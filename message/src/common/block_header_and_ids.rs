use crate::common::PrefilledTransaction;
use chain::{BlockHeader, ShortTransactionID};
use serialization_derive::{Deserializable, Serializable};

#[derive(Debug, PartialEq, Serializable, Deserializable)]
pub struct BlockHeaderAndIDs {
    pub header: BlockHeader,
    pub nonce: u64,
    pub short_ids: Vec<ShortTransactionID>,
    pub prefilled_transactions: Vec<PrefilledTransaction>,
}
