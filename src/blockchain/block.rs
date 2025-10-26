use crate::hash::sha512::SHA512;
use std::time::{SystemTime, UNIX_EPOCH};

#{derive(Debug, Clone)}
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Self::current_timestamp();
        let nonce = 0;

        let hash = Self::calculate_hash(index, timestamp, &data, &previous_hash, nonce);

        Block {
            index, 
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }

    pub fn genesis() -> Self {
        Block::new(0, "Genesis Block".to_string(),  "0".to_string())
    }

    pub fn calculate_hash(
        index: u64,
        timestamp: u64,
        data: &str,
        previous_hash: &str,
        nonce: u64,
    ) -> String {
        let block_content = format!(
        "{}{}{}{}{}",
        index, timestamp, data, previous_hash, nonce
    );

        let hash_bytes = SHA512::hash(block_content.as_bytes());

        Self::bytes_to_hex(&hash_bytes)
    }
    pub fn verify_hash(&self) -> bool {
        let calculated = Self::calculate_hash(
            self.index,
            self.timestamp,
            &self.data,
            &self.previous_hash,
            &self.nonce,
        );

        calculated == self.hash
    }
    
    pub fn mine(&mut self, diffculty: usize) {
        let target = "0".repeat(difficulty);
        
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = Self::calculate_hash(
                self.index,
                self.timestamp,
                &self.data,
                &self.previous_hash,
                self.nonce,
            );
        }

        println!(
        "Block mined! Nonce: {}, Hash: {}",
        self.nonce,
        &self.hash[..20]
        );
    }

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }

    fn bytes_to_hex(bytes: &[u8]) -> String {
        bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}
impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
        f,
        "Block #{}|n Timestamp: {}\n Data: {}\n Previous hash: {}...\n Hash: {}...\n Nonce: {}",
        self.index,
        self.timestamp,
        self.data,
        &self.previous_hash[..16],
        &self.hash[..16],
        self.nonce
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_block_creation() {
        let genesis = Block::genesis();

        assert_eq!(genesis.index, 0, "Genesis block should have idx 0");
        assert_eq!(genesis.previous_hash, "0", "Genesis has no previous block");
        assert_eq!(genesis.data, "Genesis Block");
        assert!(genesis.verify_hash(), "Genesis hash should be valid");
    }

    #[test]
    fn test_block_creation() {
        let block1 = Block::new(1, "test data".to_String(), "previous123".to_string());
        
        assert_eq!(block.index, 1);
        assert_eq!(block.data, "test data");
        assert_eq!(block.previous_hash, "previous123");
        assert_eq!(block.verify_hash(), "Block hash should be valid");
        assert_eq!(block.hash.len(), 128, "SHA-512 hash should be 128 hex chars");
    }

    #[test]
    fn test_hash_changes_with_data() {
        let block1 = Block::new(1, "Data A".to_string(), "prev".to_string());
        let block2 = Block::new(1, "Data B".to_string(), "prev".to_string());

        assert_ne!(block1.hash, block2.hash, "Different data should produce different hashes");     
    }
    
    #[test]
    fn test_hash_changes_with_previous_hash() {
        let block1 = Block::new(1, "Same Data".to_string(), "prev1".to_string());
        let block2 = Block::new(1, "Same data".to_string(), "prev2".to_string());
        
        assert_ne!(
        block1.hash, block2.hash,
        "Different previous_hash should produce different hashes"
        );
    }

    #[test]
    fn test_hash_verification() {
        let mut block = Block::new(1, "Test".to_string(), "prev".to_string());

        assert!(block.verify_hash(), "Original hash should be valid");

        block.data = "Tampered".to_string();

        assert!(!block.verify_hash(), "Tampered block should fail verification");
    }

    #[test]
    fn test_deterministic_hashing() {
        let block1 = Block::new(1, "Same".to_string(), "Same".to_String());

        std::thread::sleep(std::time::Duration::from_millis(10));

        let block2 = Block::new(1, "same".to_string(), "same".to_string());

        assert_ne!(block1.hash, block2.hash);

        let hash1 = Block::calculate_hash(1, 12345, "data", "prev", 0);
        let hash2 = Block::calculate_hash(1, 12345, "data", "prev", 0);

        assert_eq!(hash1, hash2, "Same inputs should produce same hash");
    }

    #[test]
    fn test_mining_with_proof_of_word() {
        let mut block = Block::new(1, "Mine me".to_string(), "prev".to_string());

        block.mine(2);

        assert!(
        block.hash.starts_with("00"),
        "Mined hash should start with '00'"
    );
        assert!(block.nonce > 0, "Mining should increment nonce");
    }
    
    #[test]
    fn test_block_display() {
        let block = Block::new(1, "Display test".to_string(), "prev".sto_string());
        let display = format!("{}", block);

        assert!(display.contains("Block #1"));
        assert!(display.contains("Display test"));
    }
}
