//! Blockchain implementation for InTime

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: DateTime<Utc>,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub previous_hash: String,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub hash: String,
    pub miner: String,
    pub difficulty: u32,
}

impl Block {
    /// Create a new block
    pub fn new(
        index: u64,
        previous_hash: String,
        transactions: Vec<Transaction>,
        miner: String,
        difficulty: u32,
    ) -> Self {
        let timestamp = Utc::now();
        let mut block = Self {
            index,
            timestamp,
            previous_hash,
            transactions,
            nonce: 0,
            hash: String::new(),
            miner,
            difficulty,
        };
        
        block.hash = block.calculate_hash();
        block
    }
    
    /// Calculate the hash of the block
    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}{}{}",
            self.index,
            self.timestamp.timestamp(),
            self.previous_hash,
            serde_json::to_string(&self.transactions).unwrap_or_default(),
            self.nonce,
            self.miner,
            self.difficulty
        );
        
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }
    
    /// Mine the block by finding the golden nonce
    pub fn mine(&mut self) -> Result<()> {
        let target = "0".repeat(self.difficulty as usize);
        
        loop {
            self.hash = self.calculate_hash();
            
            if self.hash.starts_with(&target) {
                println!("Block mined: {}", self.hash);
                return Ok(());
            }
            
            self.nonce += 1;
            
            // Prevent infinite loops in case of very high difficulty
            if self.nonce > u64::MAX - 1 {
                return Err(Error::Mining("Failed to find valid nonce".to_string()));
            }
        }
    }
    
    /// Validate the block
    pub fn is_valid(&self, previous_block: Option<&Block>) -> bool {
        // Check if hash is correct
        if self.hash != self.calculate_hash() {
            return false;
        }
        
        // Check if hash meets difficulty requirement
        let target = "0".repeat(self.difficulty as usize);
        if !self.hash.starts_with(&target) {
            return false;
        }
        
        // Check if previous hash matches
        if let Some(prev_block) = previous_block {
            if self.previous_hash != prev_block.hash {
                return false;
            }
            
            if self.index != prev_block.index + 1 {
                return false;
            }
        }
        
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: u64, // 10 years in minutes
}

impl Blockchain {
    /// Create a new blockchain with genesis block
    pub fn new() -> Self {
        let mut blockchain = Self {
            blocks: Vec::new(),
            pending_transactions: Vec::new(),
            mining_reward: 60 * 24 * 365 * 10, // 10 years in minutes
        };
        
        blockchain.create_genesis_block();
        blockchain
    }
    
    /// Create the genesis block
    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(
            0,
            "0".to_string(),
            Vec::new(),
            "genesis".to_string(),
            2, // Easy difficulty for genesis
        );
        
        self.blocks.push(genesis_block);
    }
    
    /// Get the latest block
    pub fn get_latest_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }
    
    /// Add a transaction to pending transactions
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }
    
    /// Mine pending transactions
    pub fn mine_pending_transactions(&mut self, miner_address: String, difficulty: u32) -> Result<Block> {
        // Add mining reward transaction
        let reward_transaction = Transaction {
            id: uuid::Uuid::new_v4().to_string(),
            from: "system".to_string(),
            to: miner_address.clone(),
            amount: self.mining_reward,
            timestamp: Utc::now(),
            signature: "system".to_string(),
        };
        
        let mut transactions = vec![reward_transaction];
        transactions.extend(self.pending_transactions.drain(..));
        
        let mut new_block = Block::new(
            self.get_latest_block().index + 1,
            self.get_latest_block().hash.clone(),
            transactions,
            miner_address,
            difficulty,
        );
        
        new_block.mine()?;
        
        if !new_block.is_valid(Some(self.get_latest_block())) {
            return Err(Error::InvalidBlockHash(new_block.hash));
        }
        
        self.blocks.push(new_block.clone());
        Ok(new_block)
    }
    
    /// Validate the entire blockchain
    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let previous_block = &self.blocks[i - 1];
            
            if !current_block.is_valid(Some(previous_block)) {
                return false;
            }
        }
        
        true
    }
    
    /// Get balance for an address
    pub fn get_balance(&self, address: &str) -> u64 {
        let mut balance = 0i64;
        
        for block in &self.blocks {
            for transaction in &block.transactions {
                if transaction.from == address {
                    balance -= transaction.amount as i64;
                }
                if transaction.to == address {
                    balance += transaction.amount as i64;
                }
            }
        }
        
        balance.max(0) as u64
    }
    
    /// Create a verification signature for a block
    pub fn create_verification_signature(
        &self,
        wallet_address: &str,
        software_checksum: &str,
        block_hash: &str,
    ) -> String {
        let data = format!("{}{}{}", wallet_address, software_checksum, block_hash);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }
    
    /// Verify a block verification signature
    pub fn verify_block_signature(
        &self,
        wallet_address: &str,
        software_checksum: &str,
        block_hash: &str,
        signature: &str,
    ) -> bool {
        let expected_signature = self.create_verification_signature(
            wallet_address,
            software_checksum,
            block_hash,
        );
        expected_signature == signature
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.blocks.len(), 1);
        assert_eq!(blockchain.blocks[0].index, 0);
    }
    
    #[test]
    fn test_block_mining() {
        let mut block = Block::new(
            1,
            "previous_hash".to_string(),
            Vec::new(),
            "miner".to_string(),
            2,
        );
        
        assert!(block.mine().is_ok());
        assert!(block.hash.starts_with("00"));
    }
    
    #[test]
    fn test_blockchain_validation() {
        let blockchain = Blockchain::new();
        assert!(blockchain.is_valid());
    }
}
