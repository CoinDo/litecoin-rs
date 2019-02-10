use chain::IndexedBlock;
use primitives::hash::H32;
use serialization::{Deserializable, Error as ReaderError, Reader};
use std::io;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub magic: H32,
    pub block_size: u32,
    pub block: IndexedBlock,
}

impl Deserializable for Block {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, ReaderError>
    where
        T: io::Read,
    {
        let block = Block {
            magic: r#try!(reader.read()),
            block_size: r#try!(reader.read()),
            block: r#try!(reader.read()),
        };

        Ok(block)
    }
}
