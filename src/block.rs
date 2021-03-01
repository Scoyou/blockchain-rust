use std::fmt::{ self, Debug, Formatter };
use  super::*;

// defines a new block struct
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
    pub difficulty: u128,
}

// implements debug data for block
// in this case, it just prints the
// block data to the command line
impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {}, at: {}, with: {}, nonce: {}", 
        &self.index, 
        hex::encode(&self.hash), 
        &self.timestamp, 
        &self.payload,
        &self.nonce)
    }
}

// implements a new block and returns itself
impl Block {
    // constructor
    pub fn new (index: u32, timestamp: u128, prev_block_hash: BlockHash, 
        nonce: u64, payload: String, difficulty: u128) -> Self {
        Block { 
            index, 
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
            difficulty,
        }
    }

    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}


// generates the hash for each block
impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

pub fn check_difficulty(hash: &BlockHash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}