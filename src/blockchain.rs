use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenisisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new()
        }
    }

    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        // validations checking for:
        let i = self.blocks.len();

        if block.index != i as u32 {
            // index matches expected value
            return Err(BlockValidationErr::MismatchedIndex);
        }
        else if !block::check_difficulty(&block.hash(), block.difficulty) {
            // difficulty matches current difficulty
            return Err(BlockValidationErr::InvalidHash);
        }
        else if i != 0 {
            // not genisis block
            let previous_block = &self.blocks[i - 1];
            if block.timestamp <= previous_block.timestamp {
                 // timestamp is greater than previous timestamp
                return Err(BlockValidationErr::AchronologicalTimestamp);
            } else if block.prev_block_hash != previous_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        }
        else {
            // genisis block
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenisisBlockFormat);
            }
        }
        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                if
                    !(&input_hashes - &self.unspent_outputs).is_empty() ||
                    !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;

                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if coinbase.output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);

            self.blocks.push(block);
        }
        Ok(())
    }
}