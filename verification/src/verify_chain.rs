use crate::error::Error;
use crate::verify_block::BlockVerifier;
use crate::verify_header::HeaderVerifier;
use crate::verify_transaction::TransactionVerifier;
use chain::IndexedBlock;
use log::*;
use network::Network;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

pub struct ChainVerifier<'a> {
    pub block: BlockVerifier<'a>,
    pub header: HeaderVerifier<'a>,
    pub transactions: Vec<TransactionVerifier<'a>>,
}

impl<'a> ChainVerifier<'a> {
    pub fn new(block: &'a IndexedBlock, network: Network, current_time: u32) -> Self {
        trace!(target: "verification", "Block pre-verification {}", block.hash().to_reversed_str());
        ChainVerifier {
            block: BlockVerifier::new(block),
            header: HeaderVerifier::new(&block.header, network, current_time),
            transactions: block
                .transactions
                .iter()
                .map(TransactionVerifier::new)
                .collect(),
        }
    }

    pub fn check(&self) -> Result<(), Error> {
        r#try!(self.block.check());
        r#try!(self.header.check());
        r#try!(self.check_transactions());
        Ok(())
    }

    fn check_transactions(&self) -> Result<(), Error> {
        self.transactions
            .par_iter()
            .enumerate()
            .fold(
                || Ok(()),
                |result, (index, tx)| {
                    result.and_then(|_| tx.check().map_err(|err| Error::Transaction(index, err)))
                },
            )
            .reduce(|| Ok(()), |acc, check| acc.and(check))
    }
}
