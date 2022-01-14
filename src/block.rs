use std::fmt::Debug;

use crate::BlockHash;

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub parent_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
}

impl Block {
    pub fn init(
        index: u32,
        timestamp: u128,
        parent_hash: BlockHash,
        nonce: u64,
        payload: String,
    ) -> Self {
        Block {
            index: index,
            timestamp: timestamp,
            hash: vec![0; 32],
            parent_hash: parent_hash,
            nonce: nonce,
            payload: payload,
        }
    }
}
