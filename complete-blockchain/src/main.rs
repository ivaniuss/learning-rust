use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
enum BlockchainError {
    #[error("Invalid index: expected {expected}, found {found}")]
    InvalidIndex { expected: u64, found: u64 },

    #[error("Invalid previous hash: expected {expected}, found {found}")]
    InvalidPreviousHash { expected: String, found: String },

    #[error("Invalid hash: last_hash {last_hash}, block_hash {block_hash}")]
    InvalidHash { last_hash: String, block_hash: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid block: {index}: {message}")]
    InvalidBlock { index: u64, message: String },
}

type Result<T> = std::result::Result<T, BlockchainError>;

#[derive(Serialize, Deserialize, Clone, Default)]
struct Block {
    index: u64,
    timestamp: DateTime<Utc>,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now();
        let nonce = 0;
        let hash = Self::calculate_hash(index, &timestamp, &data, &previous_hash, nonce);
        Self {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }

    fn genesis() -> Self {
        Self::new(0, "Genesis Block".to_string(), "0".to_string())
    }

    fn calculate_hash(
        index: u64,
        timestamp: &DateTime<Utc>,
        data: &String,
        previous_hash: &String,
        nonce: u64,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher
            .update(format!("{}{}{}{}{}", index, timestamp, data, previous_hash, nonce).as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn is_valid(&self) -> bool {
        let hash = Self::calculate_hash(
            self.index,
            &self.timestamp,
            &self.data,
            &self.previous_hash,
            self.nonce,
        );
        hash == self.hash
    }

    fn show_info(&self) {
        println!(
            "┌─ BLOQUE {} ─────────────────────────────────────",
            self.index
        );
        println!(
            "| Timestamp: {}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        );
        println!("| Data: {}", self.data);
        println!("| Previous Hash: {}", &self.previous_hash[..self.previous_hash.len().min(16)]);
        println!("| Hash: {}", &self.hash[..self.hash.len().min(16)]);
        println!("| Valid: {}", if self.is_valid() { "✓" } else { "✗" });
        println!("└─────────────────────────────────────────────────");
    }
}

#[derive(Serialize, Deserialize)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: u64,
}

impl Blockchain {
    fn new() -> Self {
        let mut blockchain = Self {
            chain: Vec::new(),
            difficulty: 2,
        };
        blockchain.chain.push(Block::genesis());
        blockchain
    }

    fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    fn add_block(&mut self, mut block: Block) -> Result<()> {
        let last_block = self.last_block().ok_or(BlockchainError::InvalidBlock {
            index: 0,
            message: "No blocks in chain".to_string(),
        })?;
        block.index = last_block.index + 1;
        block.previous_hash = last_block.hash.clone();
        block.timestamp = Utc::now();
        block.nonce = 0;
        block.hash = Block::calculate_hash(
            block.index,
            &block.timestamp,
            &block.data,
            &block.previous_hash,
            block.nonce,
        );

        self.validate_new_block(&block)?;
        self.chain.push(block);
        Ok(())
    }

    fn validate_new_block(&self, block: &Block) -> Result<()> {
        let last_block = self.last_block().ok_or(BlockchainError::InvalidBlock {
            index: 0,
            message: "No blocks in chain".to_string(),
        })?;
        
        if block.index != last_block.index + 1 {
            return Err(BlockchainError::InvalidIndex {
                expected: last_block.index + 1,
                found: block.index,
            });
        }

        // Verify the previous hash
        if block.previous_hash != last_block.hash {
            return Err(BlockchainError::InvalidPreviousHash {
                expected: last_block.hash.clone(),
                found: block.previous_hash.clone(),
            });
        }

        // Verify the hash
        if !block.is_valid() {
            return Err(BlockchainError::InvalidHash {
                last_hash: last_block.hash.clone(),
                block_hash: block.hash.clone(),
            });
        }

        Ok(())
    }

    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if !current_block.is_valid() {
                println!("Invalid block at index {}", current_block.index);
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                println!("Previous hash mismatch at block {}", current_block.index);
                return false;
            }
        }
        true
    }

    fn search_blocks(&self, text: &str) -> Vec<&Block> {
        self.chain
            .iter()
            .filter(|block| block.data.to_lowercase().contains(&text.to_lowercase()))
            .collect()
    }

    fn statistics(&self) -> Result<(usize, String, String)> {
        if self.chain.is_empty() {
            return Err(BlockchainError::InvalidBlock {
                index: 0,
                message: "Chain is empty".to_string(),
            });
        }
        
        Ok((
            self.chain.len(),
            self.chain
                .first()
                .unwrap()
                .timestamp
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string(),
            self.chain
                .last()
                .unwrap()
                .timestamp
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string(),
        ))
    }

    fn show_chain(&self) {
        println!("===Complete Blockchain===\n");
        for block in &self.chain {
            block.show_info();
        }
    }

    fn save_file(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        println!("Blockchain saved to {}", path);
        Ok(())
    }

    fn load_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let blockchain: Self = serde_json::from_str(&content)?;
        
        if !blockchain.is_chain_valid() {
            return Err(BlockchainError::InvalidBlock {
                index: 0,
                message: "Invalid blockchain".to_string(),
            });
        }

        println!("Blockchain loaded from {}", path);
        Ok(blockchain)
    }
}

fn main() -> Result<()> {
    println!("===Program to simulate a blockchain===\n");

    let mut blockchain = Blockchain::new();

    println!("\n===Interactive Menu===\n");
    println!("1. Add block\n");
    println!("2. Show blockchain\n");
    println!("3. Validate blockchain\n");
    println!("4. Search blocks\n");
    println!("5. Statistics\n");
    println!("6. Save blockchain\n");
    println!("7. Load blockchain\n");
    println!("8. Exit\n");

    loop {
        println!("Enter your choice: ");
        let mut choice = String::new();
        if let Err(e) = io::stdin().read_line(&mut choice) {
            println!("Error reading input: {}", e);
            continue;
        }

        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Enter block data: ");
                let mut data = String::new();
                if let Err(e) = io::stdin().read_line(&mut data) {
                    println!("Error reading input: {}", e);
                    continue;
                }
                let data = data.trim().to_string();
                
                match blockchain.add_block(Block { data, ..Block::default() }) {
                    Ok(_) => println!("Block added successfully!"),
                    Err(e) => println!("Error adding block: {}", e),
                }
            }
            "2" => blockchain.show_chain(),
            "3" => if blockchain.is_chain_valid() {
                println!("Blockchain is valid!");
            } else {
                println!("Blockchain is invalid!");
            },
            "4" => {
                println!("Enter search text: ");
                let mut text = String::new();
                if let Err(e) = io::stdin().read_line(&mut text) {
                    println!("Error reading input: {}", e);
                    continue;
                }
                let text = text.trim();
                let blocks = blockchain.search_blocks(text);
                if blocks.is_empty() {
                    println!("No blocks found with this text");
                } else {
                    for block in blocks {
                        block.show_info();
                    }
                }
            }
            "5" => {
                match blockchain.statistics() {
                    Ok((length, start_time, end_time)) => {
                        println!("Blockchain length: {}", length);
                        println!("Start time: {}", start_time);
                        println!("End time: {}", end_time);
                    }
                    Err(e) => println!("Error getting statistics: {}", e),
                }
            }
            "6" => {
                if let Err(e) = blockchain.save_file("blockchain.json") {
                    println!("Error saving blockchain: {}", e);
                }
            }
            "7" => {
                match Blockchain::load_file("blockchain.json") {
                    Ok(loaded_blockchain) => {
                        blockchain = loaded_blockchain;
                        println!("Blockchain loaded successfully!");
                    }
                    Err(e) => println!("Error loading blockchain: {}", e),
                }
            }
            "8" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid choice!\n"),
        }
    }

    Ok(())
}
