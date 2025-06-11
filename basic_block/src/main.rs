use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Clone)]
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

    //genesis block
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
        println!("Index: {}", self.index);
        println!("Timestamp: {}", self.timestamp);
        println!("Data: {}", self.data);
        println!("Previous Hash: {}", self.previous_hash);
        println!("Hash: {}", self.hash);
        println!("Nonce: {}", self.nonce);
    }
}

fn main() {
    println!("===Program to simulate a blockchain===\n");
    
    let genesis_block = Block::genesis();
    genesis_block.show_info();

    let block_1 = Block::new(1, "Alice -> Bob 1 Tokens".to_string(), genesis_block.hash.clone());
    block_1.show_info();

    let block_2 = Block::new(2, "Bob -> Alice 2 Tokens".to_string(), block_1.hash.clone());
    block_2.show_info();

    let block_3 = Block::new(3, "Charlie -> Diana 3 Tokens".to_string(), block_2.hash.clone());
    block_3.show_info();

    println!("\n===Serializing JSON block 3===\n");
    match serde_json::to_string_pretty(&block_3 ) {
        Ok(json) => println!("JSON: {}", json),
        Err(e) => println!("Error serializing block: {}", e),
    }

    println!("\n===Validation===\n");
    println!("Block 1 is valid: {}", block_1.is_valid());
    println!("Block 2 is valid: {}", block_2.is_valid());
    println!("Block 3 is valid: {}", block_3.is_valid());

    println!("\n===Inmutability===\n");

    let mut modified_block = block_2.clone();
    modified_block.data = "Alice -> Bob 1 Tokens".to_string();
    println!("Original block 2: {}", block_2.is_valid());
    println!("Modified block: {}", modified_block.is_valid());
}