use sha2::{Digest, Sha256};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use std::io;
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    index: u64,
    timestamp: DateTime<Utc>,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
    difficulty: u64,
}

#[derive(Debug, Clone)]
struct MiningStats {
    attempts: u64,
    total_time: Duration,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum MiningMethod {
    Normal,
    Competition,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String, difficulty: u64) -> Self {
        let timestamp = Utc::now();
        let nonce = 0;
        let hash = String::new(); // Will be calculated during mining
        
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
            difficulty,
        }
    }
    
    fn genesis() -> Self {
        let mut block = Block::new(
            0, 
            "🌟 Genesis Block - The adventure begins!".to_string(), 
            "0".to_string(),
            2
        );
        
        // Genesis doesn't need mining, but we do it for fun
        println!("⛏️  Mining genesis block...");
        let stats = block.mine();
        println!("✅ Genesis mined in {} attempts ({:.2}s)", 
                 stats.attempts, stats.total_time.as_secs_f64());
        block
    }
    
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let content = format!(
            "{}{}{}{}{}{}",
            self.index,
            self.timestamp.timestamp(),
            self.data,
            self.previous_hash,
            self.nonce,
            self.difficulty
        );
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    // THE STAR FUNCTION! - Mining magic happens here
    fn mine(&mut self) -> MiningStats {
        let start = Instant::now();
        let target = "0".repeat(self.difficulty as usize);
        let mut attempts = 0u64;
        
        println!("🎯 Target: hash starting with '{}'", target);
        println!("⚡ Mining block {}...", self.index);
        
        loop {
            // Calculate hash with current nonce
            self.hash = self.calculate_hash();
            attempts += 1;
            
            // Show progress every 50,000 attempts
            if attempts % 50_000 == 0 {
                let elapsed_time = start.elapsed().as_secs_f64();
                let hps = attempts as f64 / elapsed_time;
                println!("   💭 Attempt {}: nonce={}, hash={}... ({:.0} H/s)", 
                         attempts, self.nonce, &self.hash[..8], hps);
            }
            
            // Did we find the solution?
            if self.hash.starts_with(&target) {
                let total_time = start.elapsed();
                let hps = attempts as f64 / total_time.as_secs_f64();
                
                println!("🎉 BLOCK MINED!");
                println!("   🔢 Winning nonce: {}", self.nonce);
                println!("   🔐 Final hash: {}", self.hash);
                println!("   ⏱️  Time: {:.2}s", total_time.as_secs_f64());
                println!("   ⚡ Speed: {:.0} hashes/second", hps);
                
                return MiningStats {
                    attempts,
                    total_time,
                };
            }
            
            // Increment nonce for next attempt
            self.nonce += 1;
        }
    }
    
    // Simulate mining competition among multiple miners
    fn mining_competition(mut self, num_miners: u32) -> (Self, u32) {
        let start = Instant::now();
        let target = "0".repeat(self.difficulty as usize);
        let mut attempts_per_miner = vec![0u64; num_miners as usize];
        
        println!("🏁 MINING COMPETITION!");
        println!("🏭 {} miners competing for block {}", num_miners, self.index);
        println!("🎯 Target: {}", target);
        
        let mut rng = rand::thread_rng();
        
        loop {
            // Each miner makes multiple attempts per round
            for miner_id in 0..num_miners {
                let attempts_this_round = rng.gen_range(1000..5000);
                
                for _ in 0..attempts_this_round {
                    self.nonce = rng.gen::<u64>();
                    self.hash = self.calculate_hash();
                    attempts_per_miner[miner_id as usize] += 1;
                    
                    if self.hash.starts_with(&target) {
                        let total_time = start.elapsed();
                        let total_attempts: u64 = attempts_per_miner.iter().sum();
                        
                        println!("🏆 WINNER: MINER {}!", miner_id + 1);
                        println!("   🔢 Winning nonce: {}", self.nonce);
                        println!("   🔐 Hash: {}", self.hash);
                        println!("   ⏱️  Total time: {:.2}s", total_time.as_secs_f64());
                        println!("   📊 Total attempts by all miners: {}", total_attempts);
                        
                        // Show stats per miner
                        for (i, attempts) in attempts_per_miner.iter().enumerate() {
                            let percentage = (*attempts as f64 / total_attempts as f64) * 100.0;
                            println!("      Miner {}: {} attempts ({:.1}%)", i + 1, attempts, percentage);
                        }
                        
                        return (self, miner_id + 1);
                    }
                }
            }
            
            // Show progress every so often
            let elapsed_secs = start.elapsed().as_secs();
            if elapsed_secs > 0 && elapsed_secs % 10 == 0 {
                let total_attempts: u64 = attempts_per_miner.iter().sum();
                println!("   📈 Progress: {} total attempts in {}s", total_attempts, elapsed_secs);
            }
        }
    }
    
    fn is_valid(&self) -> bool {
        let calculated_hash = self.calculate_hash();
        let target = "0".repeat(self.difficulty as usize);
        
        calculated_hash == self.hash && self.hash.starts_with(&target)
    }
    
    fn display_info(&self) {
        println!("┌─ BLOCK {} (Difficulty: {}) ────────────────────", self.index, self.difficulty);
        println!("│ ⏰ Time: {}", self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("│ 📝 Data: {}", self.data);
        let prev_hash_display = if self.previous_hash.len() <= 16 {
            self.previous_hash.clone()
        } else {
            format!("{}...", &self.previous_hash[..16])
        };
        println!("│ 🔗 Previous hash: {}", prev_hash_display);
        println!("│ 🔐 Hash: {}", self.hash);
        println!("│ 🔢 Nonce: {}", self.nonce);
        println!("│ ✅ Valid: {}", if self.is_valid() { "✓" } else { "✗" });
        println!("└─────────────────────────────────────────────────");
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: u64,
    target_time: u64, // target seconds per block
    #[serde(skip)]
    mining_stats: Vec<(MiningStats, MiningMethod)>, // Statistics for each block mined (except genesis)
    #[serde(skip)]
    manual_difficulty_changes: Vec<(u64, u64)>, // (from, to) pairs of difficulty changes
}

impl Blockchain {
    fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 2,
            target_time: 10, // 10 seconds target
            mining_stats: Vec::new(),
            manual_difficulty_changes: Vec::new(),
        };
        
        blockchain.chain.push(Block::genesis());
        blockchain
    }
    
    fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
    
    fn add_mined_block(&mut self, data: String) -> MiningStats {
        // Clone the necessary data from the last block to avoid borrow issues
        let last_index = self.last_block().index;
        let last_hash = self.last_block().hash.clone();
        let new_index = last_index + 1;
        
        // Adjust difficulty before mining
        self.adjust_difficulty();
        
        let mut new_block = Block::new(
            new_index, 
            data, 
            last_hash,
            self.difficulty
        );
        
        println!("\n🚀 STARTING TO MINE BLOCK {}", new_index);
        println!("📊 Current difficulty: {}", self.difficulty);
        
        let stats = new_block.mine();
        self.chain.push(new_block);
        self.mining_stats.push((stats.clone(), MiningMethod::Normal));
        
        stats
    }
    
    fn block_competition(&mut self, data: String, num_miners: u32) -> u32 {
        // Clone the necessary data from the last block to avoid borrow issues
        let last_index = self.last_block().index;
        let last_hash = self.last_block().hash.clone();
        let new_index = last_index + 1;
        
        self.adjust_difficulty();
        
        let new_block = Block::new(
            new_index,
            data,
            last_hash,
            self.difficulty
        );
        
        let (mined_block, winning_miner) = new_block.mining_competition(num_miners);
        // Create a simplified MiningStats for competition
        let comp_stats = MiningStats {
            attempts: 0, // We don't track exact attempts in competition
            total_time: Duration::from_secs(0), // Not tracked
        };
        self.chain.push(mined_block);
        self.mining_stats.push((comp_stats, MiningMethod::Competition));
        
        winning_miner
    }
    
    // Adjust difficulty based on mining time of recent blocks
    fn adjust_difficulty(&mut self) {
        if self.chain.len() < 2 {
            return;
        }
        
        // Take last 3 blocks to calculate average time
        let recent_blocks = std::cmp::min(3, self.chain.len() - 1);
        let mut total_time = 0i64;
        
        for i in (self.chain.len() - recent_blocks)..self.chain.len() {
            if i > 0 {
                let time_diff = self.chain[i].timestamp.timestamp() - 
                                self.chain[i-1].timestamp.timestamp();
                total_time += time_diff;
            }
        }
        
        let avg_time = total_time as f64 / recent_blocks as f64;
        println!("⏱️ Average block mining time: {:.2}s", avg_time);
        
        if avg_time < self.target_time as f64 / 2.0 {
            self.difficulty += 1;
            println!("⬆️ Increasing difficulty to {}", self.difficulty);
        } else if avg_time > self.target_time as f64 * 2.0 && self.difficulty > 1 {
            self.difficulty -= 1;
            println!("⬇️ Decreasing difficulty to {}", self.difficulty);
        } else {
            println!("↔️ Difficulty stays at {}", self.difficulty);
        }
    }
    
    fn display_chain(&self) {
        println!("\n=== BLOCKCHAIN CHAIN ===");
        for block in &self.chain {
            block.display_info();
        }
        println!("=======================\n");
    }
    
    fn set_difficulty(&mut self, new_difficulty: u64) -> bool {
        if new_difficulty == 0 {
            println!("❌ Error: Difficulty must be at least 1");
            return false;
        }
        
        if new_difficulty > 10 {
            println!("⚠️ Warning: Setting difficulty above 10 may make mining very slow");
        }
        
        let old_difficulty = self.difficulty;
        self.manual_difficulty_changes.push((old_difficulty, new_difficulty));
        self.difficulty = new_difficulty;
        println!("🔄 Difficulty manually changed: {} → {}", old_difficulty, new_difficulty);
        return true;
    }
    
    fn display_statistics(&self) {
        println!("\n📊 BLOCKCHAIN STATISTICS 📊");
        println!("───────────────────────────");
        
        // Basic blockchain info
        println!("📏 Total blocks: {}", self.chain.len());
        println!("🔶 Current difficulty: {}", self.difficulty);
        println!("⏱️  Target mining time: {}s", self.target_time);
        
        // Skip genesis block in calculations
        if self.chain.len() <= 1 {
            println!("Not enough blocks for detailed statistics.\n");
            return;
        }
        
        // Calculate average mining time
        let mut total_mining_time = 0.0;
        for i in 1..self.chain.len() {
            let time_diff = (self.chain[i].timestamp - self.chain[i-1].timestamp).num_seconds();
            total_mining_time += time_diff as f64;
        }
        let avg_mining_time = total_mining_time / (self.chain.len() - 1) as f64;
        println!("⏱️  Average mining time: {:.2}s", avg_mining_time);
        
        // Mining method distribution
        let normal_blocks = self.mining_stats.iter().filter(|(_, method)| *method == MiningMethod::Normal).count();
        let comp_blocks = self.mining_stats.iter().filter(|(_, method)| *method == MiningMethod::Competition).count();
        
        println!("👨‍💻 Mining methods:");
        println!("   - Normal mining: {} blocks ({:.1}%)", 
                 normal_blocks, 
                 (normal_blocks as f64 / self.mining_stats.len() as f64) * 100.0);
        println!("   - Competition mining: {} blocks ({:.1}%)", 
                 comp_blocks, 
                 (comp_blocks as f64 / self.mining_stats.len() as f64) * 100.0);
        
        // Hashing statistics for normal mining
        let normal_stats: Vec<&MiningStats> = self.mining_stats.iter()
            .filter(|(_, method)| *method == MiningMethod::Normal)
            .map(|(stats, _)| stats)
            .collect();
        
        if !normal_stats.is_empty() {
            let total_attempts: u64 = normal_stats.iter().map(|s| s.attempts).sum();
            let avg_attempts = total_attempts as f64 / normal_stats.len() as f64;
            
            // Calculate average hash rate from attempts and time
            let total_time_secs: f64 = normal_stats.iter().map(|s| s.total_time.as_secs_f64()).sum();
            let avg_hash_rate = if total_time_secs > 0.0 {
                total_attempts as f64 / total_time_secs
            } else {
                0.0
            };
            
            println!("⚙️ Normal mining performance:");
            println!("   - Total hash attempts: {}", total_attempts);
            println!("   - Average attempts per block: {:.0}", avg_attempts);
            println!("   - Average hash rate: {:.0} H/s", avg_hash_rate);
        }
        
        // Display difficulty changes
        if !self.manual_difficulty_changes.is_empty() {
            println!("🔧 Manual difficulty changes:");
            for (i, (from, to)) in self.manual_difficulty_changes.iter().enumerate() {
                println!("   {}. {} → {}", i+1, from, to);
            }
        }
        
        println!("───────────────────────────\n");
    }
}

fn main() {
    println!("🖥️  Welcome to Rust Blockchain Mining Simulator!");
    
    let mut blockchain = Blockchain::new();
    let mut input = String::new();
    
    loop {
        println!("Choose an option:");
        println!("1. Mine a new block");
        println!("2. Mining competition (multiple miners)");
        println!("3. Display blockchain");
        println!("4. Show statistics");
        println!("5. Change difficulty");
        println!("6. Exit");
        print!("> ");
        io::Write::flush(&mut io::stdout()).unwrap();
        
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim() {
            "1" => {
                println!("Enter data for the new block:");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let data = input.trim().to_string();
                let stats = blockchain.add_mined_block(data);
                let hash_rate = if stats.total_time.as_secs_f64() > 0.0 {
                    stats.attempts as f64 / stats.total_time.as_secs_f64()
                } else {
                    0.0
                };
                println!("Block mined in {} attempts ({:.2}s, {:.0} H/s)", 
                         stats.attempts, 
                         stats.total_time.as_secs_f64(),
                         hash_rate);
            }
            "2" => {
                println!("Enter data for the new block:");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let data = input.trim().to_string();
                
                println!("Enter number of miners competing:");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let num_miners: u32 = input.trim().parse().unwrap_or(3);
                
                let winner = blockchain.block_competition(data, num_miners);
                println!("Miner {} won the competition!", winner);
            }
            "3" => {
                blockchain.display_chain();
            }
            "4" => {
                blockchain.display_statistics();
            }
            "5" => {
                println!("Enter new difficulty (1-10 recommended):");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                
                match input.trim().parse::<u64>() {
                    Ok(new_difficulty) => {
                        blockchain.set_difficulty(new_difficulty);
                    },
                    Err(_) => {
                        println!("❌ Error: Invalid difficulty value");
                    }
                }
            }
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option, please choose again.");
            }
        }
    }
}
