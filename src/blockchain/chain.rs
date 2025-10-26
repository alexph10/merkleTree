use crate::blockchain::block::Block;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> {
        let genesis = Block::genesis();
        Blockchain {
            blocks: vec![genesis],
            difficulty: 2,
        }
    }

    pub fn with_difficulty(difficulty: usize) -> Self {
        let genesis = Block::genesis();
        Blockchain {
            blocks: vec![genesis],
            difficulty,
        }
    }

    pub fn latest_block(&self) -> &Block {
        self.blocks.last().expect("Blockchain should never be empty")
    }

    pub fn  add_block(&mut self, data: String, mine: bool) {
        let previous_block = self.latest_block();
        let mut new_block = Block::new(
            self.blocks.len() as u64,
            data,
            previous_block.hash.clone(),
        );

        if mine {
            new_block.mine(self.difficulty);
        }
        self.blocks.push(new_block);
    }
    
    pub fn is_valid(&self) -> bool {
        if self.blocks.is_empty() {
            return false;
        }
        let genesis = &self.blocks[0];
        if genesis.index != 0 || genesis.previous_hash != "0" {
            println!("Invalid genesis block");
            return false;
        }
        if !genesis.verify_hash() {
            println!("Genesis block hash is invalid");
            return false;
        }
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            if !current.verify_hash() {
                println!("Block {} has invalid hash", i);
                return false;
            }

            if current.previous_hash != previous.hash {
                println!("Block {} does not link to previous block correctly\nExpected: {}\nGot: {}",
                i,previous.hash, current.previous_hash);
                return false;
            }
            if current.index != previous.index + 1 {
                println!("Block {} has incorrect index", i );
                return false;
            }
        }
        true
    }
}
pub fn tamper_block(&mut self, index: usize, new_data: String) -> bool {
    if index >= self.blocks.len() {
        return false;
    }
    println!("\n  TAMPERING WITH BLOCK {}...", index);
    self.blocks[index].data = new_data;
    println!("Blockchain is now: {}", if self.is_valid() { "VALID" } else { "INVALID "});
    true
}

pub fn stats(&self) -> BlockchainStats {
    BlockchainStats {
        total_blocks: self.blocks.len(),
        is_valid: self.is_valid(),
        difficulty: self.difficulty,
        genesis_hash: self.blocks[0].hash.clone(),
        latest_hash: self.latest_block().hash.clone(),
    }
}
    

pub fn display(&self) {
    println!("\n{}", "=".repeat(50));
    println!(" BLOCKCHAIN");
    println!("{}", "=".repeat(50));
    println!("Total Blocks: {}", self.blocks.len());
    println!("Difficulty: {}", self.difficulty);
    println!("Valid: {}\n", if self.is_valid() { "✓" } else { "✗" });

    for block in &self.blocks {
        println!("{}", block);
        println!("{}", "-".repeat(50));
    }

}


#[derive(Debug)]
pub struct BlockchainStats {
    pub total_blocks: usize,
    pub is_valid: bool,
    pub difficulty: usize,
    pub genesis_hash: String,
    pub latest_hash: String,
}

impl std::fmt::Display for BlockchainStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Blockchain Stats:\n  Blocks: {}\n  Valid: {}\n  Difficulty: {}\n  Genesis: {}...\n  Latest: {}...",
            self.total_blocks,
            self.difficulty,
            &self.genesis_hash[..16],
            &self.latest_hash[..16]
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new();
        
        assert_eq!(blockchain.blocks.len(), 1, "Should start with genesis block");
        assert!(blockchain.is_valid(), "New blockchain should be valid");
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new();
        
        blockchain.add_block("First block".to_string(), false);
        blockchain.add_block("Second block".to_string(), false);
        
        assert_eq!(blockchain.blocks.len(), 3, "Should have 3 blocks total");
        assert!(blockchain.is_valid(), "Blockchain should remain valid");
    }

    #[test]
    fn test_block_linking() {
        let mut blockchain = Blockchain::new();
        
        blockchain.add_block("Block 1".to_string(), false);
        blockchain.add_block("Block 2".to_string(), false);
        
        // Verify each block links to the previous
        for i in 1..blockchain.blocks.len() {
            let current = &blockchain.blocks[i];
            let previous = &blockchain.blocks[i - 1];
            
            assert_eq!(
                current.previous_hash, previous.hash,
                "Block {} should link to block {}", i, i - 1
            );
        }
    }

    #[test]
    fn test_tampering_detection() {
        let mut blockchain = Blockchain::new();
        
        blockchain.add_block("Block 1".to_string(), false);
        blockchain.add_block("Block 2".to_string(), false);
        
        assert!(blockchain.is_valid(), "Original chain should be valid");
        
        // Tamper with block 1's data
        blockchain.blocks[1].data = "TAMPERED DATA".to_string();
        
        assert!(!blockchain.is_valid(), "Tampered chain should be invalid");
    }

    #[test]
    fn test_hash_chain_immutability() {
        let mut blockchain = Blockchain::new();
        
        blockchain.add_block("Block 1".to_string(), false);
        blockchain.add_block("Block 2".to_string(), false);
        blockchain.add_block("Block 3".to_string(), false);
        
        let original_hash_2 = blockchain.blocks[2].hash.clone();
        let original_hash_3 = blockchain.blocks[3].hash.clone();
        

        blockchain.blocks[1].data = "MODIFIED".to_string();
        

        assert_eq!(blockchain.blocks[2].hash, original_hash_2);
        assert_eq!(blockchain.blocks[3].hash, original_hash_3);

        assert!(!blockchain.is_valid());
    }

    #[test]       pub fn tamper_block(&mut self, index: usize, new_data: String) -> bool {
        if index >= self.blocks.len() {
            return false;
        }
        
        println!("\n⚠️  TAMPERING WITH BLOCK {}...", index);
        self.blocks[index].data = new_data;
        
        println!("Blockchain is now: {}", if self.is_valid() { "VALID ✓" } else { "INVALID ✗" });
        
        true
    }
    
    /// Get blockchain statistics
    pub fn stats(&self) -> BlockchainStats {
        BlockchainStats {
            total_blocks: self.blocks.len(),
            is_valid: self.is_valid(),
            difficulty: self.difficulty,
            genesis_hash: self.blocks[0].hash.clone(),
            latest_hash: self.latest_block().hash.clone(),
        }
    }
    
    /// Display the entire blockchain
    pub fn display(&self) {
        println!("\n{}", "=".repeat(50));
        println!(" BLOCKCHAIN");
        println!("{}", "=".repeat(50));
        println!("Total Blocks: {}", self.blocks.len());
        println!("Difficulty: {}", self.difficulty);
        println!("Valid: {}\n", if self.is_valid() { "✓" } else { "✗" });
        
        for block in &self.blocks {
            println!("{}", block);
            println!("{}", "-".repeat(50));
        }
    }
}

/// Statistics about the blockchain
#[derive(Debug)]
pub struct BlockchainStats {
    pub total_blocks: usize,
    pub is_valid: bool,
    pub difficulty: usize,
    pub genesis_hash: String,
    pub latest_hash: String,
}

impl std::fmt::Display for BlockchainStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Blockchain Stats:\n  Blocks: {}\n  Valid: {}\n  Difficulty: {}\n  Genesis: {}...\n  Latest: {}...",
            self.total_blocks,
            if self.is_valid { "✓" } else { "✗" },
            self.difficulty,
            &self.genesis_hash[..16],
            &self.latest_hash[..16]
        )
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new();
        
        assert_eq!(blockchain.blocks.len(), 1, "Should start with genesis block");
        assert!(blockchain.is_valid(), "New blockchain should be valid");
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new();
        
        blockchain.add_block("First block".to_string(), false);
        blockchain.add_block("Second block".to_string(), false);
        
        assert_eq!(blockchain.blocks.len(), 3, "Should have 3 blocks total");
        assert!(blockchain.is_valid(), "Blockchain should remain valid");
    }

    #[test]
    fn test_block_linking() {
        let mut blockchain = Blockchain::new();
        
        blockchain.add_block("Block 1".to_string(), false);
        blockchain.add_block("Block 2".to_string(), false);
        
        // Verify each block links to the previous
        for i in 1..blockchain.blocks.len() {
            let current = &blockchain.blocks[i];
            let previous = &blockchain.blocks[i - 1];
            
            assert_eq!(
                current.previous_hash, previous.hash,
                "Block {} should link to block {}", i, i - 1
            );
        }
    }

    #[test]
    fn test_tampering_detection() {
        let mut blockchain = Blockchain::new();
        
        blockchain.add_block("Block 1".to_string(), false);
        blockchain.add_block("Block 2".to_string(), false);
        
        assert!(blockchain.is_valid(), "Original chain should be valid");
        
        // Tamper with block 1's data
        blockchain.blocks[1].data = "TAMPERED DATA".to_string();
        
        assert!(!blockchain.is_valid(), "Tampered chain should be invalid");
    }

    #[test]
    fn test_hash_chain_immutability() {
        let mut blockchain = Blockchain::new();
        
        blockchain.add_block("Block 1".to_string(), false);
        blockchain.add_block("Block 2".to_string(), false);
        blockchain.add_block("Block 3".to_string(), false);
        
        let original_hash_2 = blockchain.blocks[2].hash.clone();
        let original_hash_3 = blockchain.blocks[3].hash.clone();
        
        // Tamper with block 1
        blockchain.blocks[1].data = "MODIFIED".to_string();
        
        // Block 2 and 3's stored hashes don't change (they're fixed values)
        // But their previous_hash references are now wrong
        assert_eq!(blockchain.blocks[2].hash, original_hash_2);
        assert_eq!(blockchain.blocks[3].hash, original_hash_3);
        
        // The chain is invalid because block 1's hash changed
        // but block 2's previous_hash still points to the old hash
        assert!(!blockchain.is_valid());
    }

    #[test]
    fn test_genesis_validation() {
        let blockchain = Blockchain::new();
        
        assert_eq!(blockchain.blocks[0].index, 0);
        assert_eq!(blockchain.blocks[0].previous_hash, "0");
        assert!(blockchain.is_valid());
    }

    #[test]
    fn test_mining_with_difficulty() {
        let mut blockchain = Blockchain::with_difficulty(2);
        
        blockchain.add_block("Mined block".to_string(), true);
        
        let latest = blockchain.latest_block();
        assert!(
            latest.hash.starts_with("00"),
            "Mined block should have hash starting with '00'"
        );
        assert!(latest.nonce > 0, "Mining should find a nonce");
        assert!(blockchain.is_valid(), "Mined blockchain should be valid");
    }

    #[test]
    fn test_sequential_indices() {
        let mut blockchain = Blockchain::new();
        
        for i in 1..=5 {
            blockchain.add_block(format!("Block {}", i), false);
        }
        
        for (i, block) in blockchain.blocks.iter().enumerate() {
            assert_eq!(block.index, i as u64, "Block index should match position");
        }
    }

    #[test]
    fn test_latest_block() {
        let mut blockchain = Blockchain::new();
        
        blockchain.add_block("Block 1".to_string(), false);
        let latest = blockchain.latest_block();
        assert_eq!(latest.data, "Block 1");
        
        blockchain.add_block("Block 2".to_string(), false);
        let latest = blockchain.latest_block();
        assert_eq!(latest.data, "Block 2");
    }

    #[test]
    fn test_stats() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block("Test".to_string(), false);
        
        let stats = blockchain.stats();
        assert_eq!(stats.total_blocks, 2);
        assert!(stats.is_valid);
        assert_eq!(stats.difficulty, 2);
    }
} 
    fn test_genesis_validation() {
        let blockchain = Blockchain::new();
        
        assert_eq!(blockchain.blocks[0].index, 0);
        assert_eq!(blockchain.blocks[0].previous_hash, "0");
        assert!(blockchain.is_valid());
    }

    #[test]
    fn test_mining_with_difficulty() {
        let mut blockchain = Blockchain::with_difficulty(2);
        
        blockchain.add_block("Mined block".to_string(), true);
        
        let latest = blockchain.latest_block();
        assert!(
            latest.hash.starts_with("00"),
            "Mined block should have hash starting with '00'"
        );
        assert!(latest.nonce > 0, "Mining should find a nonce");
        assert!(blockchain.is_valid(), "Mined blockchain should be valid");
    }

    #[test]
    fn test_sequential_indices() {
        let mut blockchain = Blockchain::new();
        
        for i in 1..=5 {
            blockchain.add_block(format!("Block {}", i), false);
        }
        
        for (i, block) in blockchain.blocks.iter().enumerate() {
            assert_eq!(block.index, i as u64, "Block index should match position");
        }
    }

    #[test]
    fn test_latest_block() {
        let mut blockchain = Blockchain::new();
        
        blockchain.add_block("Block 1".to_string(), false);
        let latest = blockchain.latest_block();
        assert_eq!(latest.data, "Block 1");
        
        blockchain.add_block("Block 2".to_string(), false);
        let latest = blockchain.latest_block();
        assert_eq!(latest.data, "Block 2");
    }

    #[test]
    fn test_stats() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block("Test".to_string(), false);
        
        let stats = blockchain.stats();
        assert_eq!(stats.total_blocks, 2);
        assert!(stats.is_valid);
        assert_eq!(stats.difficulty, 2);
    }
}
