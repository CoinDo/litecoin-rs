use crate::accept_block::BlockAcceptor;
use crate::accept_header::HeaderAcceptor;
use crate::accept_transaction::TransactionAcceptor;
use crate::canon::CanonBlock;
use crate::deployments::BlockDeployments;
use crate::error::Error;
use crate::VerificationLevel;
use log::*;
use network::ConsensusParams;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use storage::{duplex_store::DuplexTransactionOutputProvider, Store};

pub struct ChainAcceptor<'a> {
    pub block: BlockAcceptor<'a>,
    pub header: HeaderAcceptor<'a>,
    pub transactions: Vec<TransactionAcceptor<'a>>,
}

impl<'a> ChainAcceptor<'a> {
    pub fn new(
        store: &'a Store,
        consensus: &'a ConsensusParams,
        verification_level: VerificationLevel,
        block: CanonBlock<'a>,
        height: u32,
        median_time_past: u32,
        deployments: &'a BlockDeployments,
    ) -> Self {
        trace!(target: "verification", "Block verification {}", block.hash().to_reversed_str());
        let output_store = DuplexTransactionOutputProvider::new(
            store.as_transaction_output_provider(),
            block.raw(),
        );
        let headers = store.as_block_header_provider();

        ChainAcceptor {
            block: BlockAcceptor::new(
                store.as_transaction_output_provider(),
                consensus,
                block,
                height,
                median_time_past,
                deployments,
                headers,
            ),
            header: HeaderAcceptor::new(headers, consensus, block.header(), height, deployments),
            transactions: block
                .transactions()
                .into_iter()
                .enumerate()
                .map(|(tx_index, tx)| {
                    TransactionAcceptor::new(
                        store.as_transaction_meta_provider(),
                        output_store,
                        consensus,
                        tx,
                        verification_level,
                        block.hash(),
                        height,
                        block.header.raw.time,
                        median_time_past,
                        tx_index,
                        deployments,
                    )
                })
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
