use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn verify(&self) -> bool {
        // validations checing for:
        // index matches expected value
        // difficulty matches current difficulty
        // timestamp is greater than previous timestamp
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                println!("Index mismatch: {} != {}", &block.index, &i);
                return false;
            }
            else if !block::check_difficulty(&block.hash(), block.difficulty) {
                println!("Difficulty fail");
                return false;
            }
            else if i != 0 {
                // not genisis block
                let previous_block = &self.blocks[i - 1];
                if block.timestamp <= previous_block.timestamp {
                    println!("Time did not increase");
                    return false;
                } else if block.prev_block_hash != previous_block.hash {
                    println!("Hash mismatch");
                    return false;
                }
            }
            else {
                // genisis block
                if block.prev_block_hash != vec![0; 32] {
                    println!("Genisis block, prev_block_hash  invalid");
                    return false;
                }
            }
        }
        true
    }
}