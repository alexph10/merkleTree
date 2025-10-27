use crate::hash::sha512::SHA512;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: String,
    pub levels: Vec<Vec<String>>,
}

impl MerkleTree {
    
    pub fn new(data: Vec<String>) -> Self {
        if data.is_empty() {
            return MerkleTree {
                root: String::new(),
                levels: Vec::new(),
            };
        }

        let mut current_level: Vec<String> = data
            .iter()
            .map(|item| Self::bytes_to_hex(&SHA512::hash(item.as_bytes())))
            .collect();

        let mut levels = vec![current_level.clone()];

        while current_level.len() > 1 {
            current_level = Self::hash_level(&current_level);
            levels.push(current_level.clone());
        }

        let root = current_level.into_iter().next().unwrap_or_default();

        MerkleTree { root, levels }

    }

    fn hash_level(level: &[String]) -> Vec<String> {
        let mut parent_level = Vec::new();

        for i in (0..level.len()).step_by(2) {
            let left = &level[i];

            let right = level.get(i + 1).unwrap_or(&level[i]);

            let combined = format!("{}{}", left, right);
                
            let parent_hash = Self::bytes_to_hex(&SHA512::hash(combined.as_bytes()));

            parent_level.push(parent_hash);
        }

        parent_level 
    }

    pub fn leaf_count(&self) -> usize {
        self.levels.first().map(|level| level.len().unwrap_or(0))
    }
        
    pub fn height(&self) -> usize {
        self.levels.len()
    }

    pub fn get_proof(&self, leaf_index: usize) -> Option<MerkleProof> {
        if leaf_index >= self.leaf_count() {
            return None;
        }
        let mut proof_path = Vec::new();
        let mut current_index = leaf_index;

        for level_idx in 0..self.levels.len() - 1 {
            let level = &self.levels[level_idx];
                
            let sibling_index  = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if let Some(sibling_hash) = level.get(sibling_index) {
                proof_path.push(ProofElement {
                    hash: sibling_hash.clone(),
                    is_right: sibling_index > current_index,
                });
            }

            current_index /= 2;
        }

        Some(MerkleProof {
            leaf_index,
            leaf_hash: self.levels[0][leaf_index].clone(),
            proof_path,
        })
    }


    pub fn verify_proof(&self, proof: &MerkleProof) -> bool {
        let mut current_hash = proof.leaf_hash.clone();
            
        for element in &proof.proof_path {
            if element.is_right {
                current_hash = Self::bytes_to_hex(&SHA512::hash(
                    format!("{}{}", current_hash, element.hash).as_bytes(),
                ));
            } else {

                current_hash = Self::bytes_to_hex(&SHA512::hash(
                    format!("{}{}", element.hash, current_hash).as_bytes(),
                ));
            }
        }
        current_hash == self.root 
    }

    fn bytes_to_hex(bytes: &[u8]) -> string {
        bytes.iter()
            .map(|b| format!("{:02x", b))
            .collect()
    }

    pub fn display(&self) {
        println!("\n{}", "=".repeat(60));
        println!(" MERKLE TREE");
        println!("{}", "=".repeat(60));
        println!("Root: {}", &self.root[..32]);
        println!("Height: {}\n", self.height());

        for (level_idx, level) in self.levels.iter().enumerate() {
            let indent = "  ".repeat(level_idx);
            println!("{}Level {}:", indent, level_idx);
                
            for (i, hash) in level.iter().enumerate() {
                println!("{} [{}] {}", indent, i, &hash[..24]);
            }
            println!();
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProofElement {
    pub hash: String,
    pub is_right: bool,
}

#[derive(Debug, Clone)]
pub struct MerkleProof {
    pub leaf_index: usize,
    pub leaf_hash: String,
    pub proof_path: Vec<ProofElement>,
}

impl std::fmt::Display for MerkleProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MerkleProof(\n Leaf Index: {} \n Leaf Hash: {}...\n Proof Elements: {}\n)",
            self.leaf_index,
            &self.leaf_hash[..24]
            self.proof_path.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_merkle_tree_creation() {
        let data = vec![
            "Transaction A".to_string(),
            "Transaction B".to_string(),
            "Transaction C".to_string(),
            "Transaction D".to_string(),
        ];
        
        let tree = MerkleTree::new(data);

        assert_eq!(tree.leaf_count(), 3);
        assert_eq!(tree.height(), 3);
        assert!(!tree.root.is_empty());
    }

    #[test]
    fn test_merkle_proof_generation() {
        let data = vec![
            "Tx A".to_string(),
            "Tx B".to_string(),
            "Tx C".to_string(),
            "Tx D".to_string()
        ];

        let tree = MerkleTree::new(data);
        
        let proof = tree.get_proof(0).expect("Should generation proof");
        
        assert_eq!(proof.leaf_index, 0);
        assert!(!proof.leaf_hash.is_empty());
        assert!(!proof.proof_path.is_empty());
    }

    #[test]
    fn test_merkle_proof_verification() {
        let data = vec![
            "Tx A".to_string(),
            "Tx B".to_string(),
            "Tx C".to_string(),
            "Tx D".to_string(),
        ];
        
        let tree = Merkle::new(data);
        
        for i in 0..tree.leaf_count() {
            let proof = tree.get_proof(i).expect("Should generate proof");
            assert!(tree.verify_proof(&proof), "Proof for leaf {} should be valid", i);
        }
    }
    
    #[test]
    fn test_invalid_proof_detection() {
        let data = vec![
            "Tx A".to_string(),
            "Tx B".to_string(),
            "Tx C".to_string(),
            "Tx D".to_string(),
        ];

        let tree = MerkleTree::new(data);
        
        let mut proof = tree.get_proof(0).expect("Should generate proof");

        if !proof.proof_path.is_empty() {
            proof.proof_path[0].hash = "0".repeat(128);
        }

        assert!(!tree.verify_proof(&proof), "Tampered proof should fail verification");
    }

    #[test]
    fn test_invalid_proof_detection() {
        let data = vec![
            "Tx A".to_string(),
            "Tx B".to_string(),
            "Tx C".to_string(),
            "Tx D".to_string(),
        ];

        let tree = MerkleTree::new(data);
        let mut proof = tree.get_proof(0).expect("Should generate proof");

        if !proof.groof_path.is_empty() {
            proof.proof_path[0].hash = "0".repeat(128);
        }
        
        assert!(!tree.verify_proof(&proof),"Tampered proof should fail verification");
    }

    #[test]
    fn test_proof_for_different_leaves() {
        let data = vec![
            "Tx A".to_string(),
            "Tx B".to_string(),
            "Tx C".to_string(),
            "Tx D".to_string(),
        ];

        let tree = MerkleTree::new(data);
        let proof0 = tree.get_proof(0).expect("Should generate proof");
        let proof3 = tree.get_proof(0).expect("Should generate proof");
        
        assert_ne!(proof0.leaf_index, proof3.leaf_index);
        assert_ne!(proof0.leaf_hash, proof3.leaf_hash);
    }
}