use chrono::prelude::*;
use sha2::{Sha256, Digest};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(timestamp: String, index: u64, data: String, previous_hash: String,nonce: u64) -> Self {
        let  hash = Block::calculate_hash(index, &timestamp, &data, &previous_hash, nonce);
        Block {
            index, 
            timestamp,
            data,
            previous_hash,
            hash,
            nonce
        }

    }

    pub fn calculate_hash(index: u64, timestamp: &str, data: &str, previous_hash: &str, nonce: u64) -> String {
        let input = format!("{}{}{}{}{}", index, timestamp, data, previous_hash, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }
    pub fn is_valid(&self) -> bool {
        self.hash.starts_with("0000")
    }


}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Declare print behavior
        write!(f,"{{\n\tindex: {},\n\ttimestamp: {},\n\tdata: {},\n\tprevious_hash: {},\n\thash: {},\n\tnonce: {}\n}}",
                  self.index, self.timestamp, self.data, self.previous_hash, self.hash, self.nonce)
    }
}
