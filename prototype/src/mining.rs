//! Mining logic for InTime blockchain

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;
use chrono::Utc;
use sha2::{Digest, Sha256};

use crate::{
    blockchain::{Blockchain, Transaction},
    wallet::Wallet,
    Error, Result,
};

#[derive(Debug, Clone)]
pub struct MinerConfig {
    pub max_difficulty: Option<u32>, // Energy saving: stop mining if difficulty exceeds this
    pub mining_interval: Duration,   // How often to check for new blocks to mine
    pub verification_interval: Duration, // How often to verify new blocks during cooling
}

impl Default for MinerConfig {
    fn default() -> Self {
        Self {
            max_difficulty: None,
            mining_interval: Duration::from_secs(10),
            verification_interval: Duration::from_secs(30),
        }
    }
}

pub struct Miner {
    pub wallet: Arc<Mutex<Wallet>>,
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub config: MinerConfig,
    pub software_checksum: String,
    pub is_running: Arc<Mutex<bool>>,
}

impl Miner {
    /// Create a new miner
    pub fn new(
        wallet: Wallet,
        blockchain: Blockchain,
        config: MinerConfig,
    ) -> Self {
        let software_checksum = Self::calculate_software_checksum();
        
        Self {
            wallet: Arc::new(Mutex::new(wallet)),
            blockchain: Arc::new(Mutex::new(blockchain)),
            config,
            software_checksum,
            is_running: Arc::new(Mutex::new(false)),
        }
    }
    
    /// Calculate checksum of the software source code
    fn calculate_software_checksum() -> String {
        // In a real implementation, this would hash the actual source code
        // For now, we'll use a placeholder
        let mut hasher = Sha256::new();
        hasher.update(b"intime-miner-v0.1.0");
        hex::encode(hasher.finalize())[..16].to_string()
    }
    
    /// Start the mining process
    pub async fn start(&self) -> Result<()> {
        let mut is_running = self.is_running.lock().await;
        if *is_running {
            return Err(Error::Mining("Miner is already running".to_string()));
        }
        *is_running = true;
        drop(is_running);
        
        let wallet = self.wallet.clone();
        let blockchain = self.blockchain.clone();
        let config = self.config.clone();
        let software_checksum = self.software_checksum.clone();
        let is_running = self.is_running.clone();
        
        tokio::spawn(async move {
            Self::mining_loop(wallet, blockchain, config, software_checksum, is_running).await;
        });
        
        Ok(())
    }
    
    /// Stop the mining process
    pub async fn stop(&self) -> Result<()> {
        let mut is_running = self.is_running.lock().await;
        *is_running = false;
        
        let mut wallet = self.wallet.lock().await;
        wallet.is_mining = false;
        
        println!("Mining stopped");
        Ok(())
    }
    
    /// Main mining loop
    async fn mining_loop(
        wallet: Arc<Mutex<Wallet>>,
        blockchain: Arc<Mutex<Blockchain>>,
        config: MinerConfig,
        software_checksum: String,
        is_running: Arc<Mutex<bool>>,
    ) {
        let mut interval = time::interval(config.mining_interval);
        
        loop {
            interval.tick().await;
            
            let running = *is_running.lock().await;
            if !running {
                break;
            }
            
            let mut wallet_guard = wallet.lock().await;
            let mut blockchain_guard = blockchain.lock().await;
            
            // Burn tokens first
            if let Err(e) = wallet_guard.burn_tokens() {
                eprintln!("Error burning tokens: {}", e);
                continue;
            }
            
            // Check if wallet is in cooling period
            if wallet_guard.is_in_cooling_period() {
                // During cooling period, verify new blocks
                if let Err(e) = Self::verify_new_blocks(
                    &mut wallet_guard,
                    &blockchain_guard,
                    &software_checksum,
                ).await {
                    eprintln!("Error verifying blocks: {}", e);
                }
                continue;
            }
            
            // Check if wallet can start mining
            if let Err(e) = wallet_guard.can_start_mining() {
                eprintln!("Cannot start mining: {}", e);
                continue;
            }
            
            // Get mining difficulty based on wallet balance (Proof of Being Poor)
            let difficulty = wallet_guard.get_mining_difficulty();
            
            // Check if difficulty exceeds maximum (energy saving)
            if let Some(max_diff) = config.max_difficulty {
                if difficulty > max_diff {
                    println!("Difficulty {} exceeds maximum {}, acting as time bank", difficulty, max_diff);
                    wallet_guard.is_mining = false;
                    continue;
                }
            }
            
            // Start mining if there are pending transactions
            if !blockchain_guard.pending_transactions.is_empty() {
                wallet_guard.is_mining = true;
                
                println!("Mining with difficulty {} (balance: {} years)", 
                    difficulty, 
                    wallet_guard.balance / (60 * 24 * 365)
                );
                
                match blockchain_guard.mine_pending_transactions(
                    wallet_guard.address.clone(),
                    difficulty,
                ) {
                    Ok(block) => {
                        println!("Successfully mined block {}: {}", block.index, block.hash);
                        
                        // Add mining reward to wallet
                        if let Err(e) = wallet_guard.add_tokens(blockchain_guard.mining_reward) {
                            eprintln!("Error adding mining reward: {}", e);
                        }
                        
                        // Compensate for being online (1 minute = 1 MIN)
                        if let Err(e) = wallet_guard.add_tokens(1) {
                            eprintln!("Error adding online compensation: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Mining failed: {}", e);
                    }
                }
                
                wallet_guard.is_mining = false;
            } else {
                // No pending transactions, just compensate for being online
                if let Err(e) = wallet_guard.add_tokens(1) {
                    eprintln!("Error adding online compensation: {}", e);
                }
            }
        }
        
        println!("Mining loop ended");
    }
    
    /// Verify new blocks during cooling period
    async fn verify_new_blocks(
        wallet: &mut Wallet,
        blockchain: &Blockchain,
        software_checksum: &str,
    ) -> Result<()> {
        // Get the latest block
        let latest_block = blockchain.get_latest_block();
        
        // Check if we've already verified this block
        if !wallet.verified_blocks.contains(&latest_block.hash) {
            // Verify the block
            if latest_block.is_valid(
                blockchain.blocks.get(blockchain.blocks.len().saturating_sub(2))
            ) {
                // Create verification signature
                let signature = blockchain.create_verification_signature(
                    &wallet.address,
                    software_checksum,
                    &latest_block.hash,
                );
                
                // Add to verified blocks
                wallet.add_verified_block(latest_block.hash.clone())?;
                
                println!("Verified block {}: {} (signature: {})", 
                    latest_block.index, 
                    latest_block.hash,
                    signature
                );
            } else {
                println!("Block {} failed validation", latest_block.hash);
            }
        }
        
        Ok(())
    }
    
    /// Create a transaction
    pub async fn create_transaction(
        &self,
        to: String,
        amount: u64,
    ) -> Result<()> {
        let mut wallet = self.wallet.lock().await;
        let mut blockchain = self.blockchain.lock().await;
        
        // Check if mining wallet can transfer
        if wallet.is_mining {
            return Err(Error::MiningWalletTransferRestriction);
        }
        
        // Burn tokens first
        wallet.burn_tokens()?;
        
        // Check balance
        if wallet.balance < amount {
            return Err(Error::InsufficientBalance {
                required: amount,
                available: wallet.balance,
            });
        }
        
        // Create transaction
        let transaction = Transaction {
            id: uuid::Uuid::new_v4().to_string(),
            from: wallet.address.clone(),
            to,
            amount,
            timestamp: Utc::now(),
            signature: "placeholder".to_string(), // In real implementation, this would be cryptographically signed
        };
        
        // Add to pending transactions
        blockchain.add_transaction(transaction);
        
        // Update wallet balance
        wallet.balance -= amount;
        
        println!("Transaction created for {} MIN", amount);
        Ok(())
    }
    
    /// Get wallet status
    pub async fn get_wallet_status(&self) -> Result<WalletStatus> {
        let wallet = self.wallet.lock().await;
        let blockchain = self.blockchain.lock().await;
        
        Ok(WalletStatus {
            address: wallet.address.clone(),
            wallet_type: wallet.wallet_type.clone(),
            balance: wallet.balance,
            negative_balance: wallet.negative_balance,
            effective_balance: wallet.effective_balance(),
            is_mining: wallet.is_mining,
            is_in_cooling_period: wallet.is_in_cooling_period(),
            cooling_period_end: wallet.cooling_period_end,
            verified_blocks_count: wallet.verified_blocks.len(),
            mining_difficulty: wallet.get_mining_difficulty(),
            blockchain_balance: blockchain.get_balance(&wallet.address),
        })
    }
    
    /// Get blockchain status
    pub async fn get_blockchain_status(&self) -> Result<BlockchainStatus> {
        let blockchain = self.blockchain.lock().await;
        
        Ok(BlockchainStatus {
            total_blocks: blockchain.blocks.len(),
            pending_transactions: blockchain.pending_transactions.len(),
            latest_block_hash: blockchain.get_latest_block().hash.clone(),
            is_valid: blockchain.is_valid(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct WalletStatus {
    pub address: String,
    pub wallet_type: crate::wallet::WalletType,
    pub balance: u64,
    pub negative_balance: u64,
    pub effective_balance: i64,
    pub is_mining: bool,
    pub is_in_cooling_period: bool,
    pub cooling_period_end: Option<chrono::DateTime<chrono::Utc>>,
    pub verified_blocks_count: usize,
    pub mining_difficulty: u32,
    pub blockchain_balance: u64,
}

#[derive(Debug, Clone)]
pub struct BlockchainStatus {
    pub total_blocks: usize,
    pub pending_transactions: usize,
    pub latest_block_hash: String,
    pub is_valid: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::Wallet;
    
    #[tokio::test]
    async fn test_miner_creation() {
        let wallet = Wallet::new_mining();
        let blockchain = Blockchain::new();
        let config = MinerConfig::default();
        
        let miner = Miner::new(wallet, blockchain, config);
        assert!(!miner.software_checksum.is_empty());
    }
    
    #[tokio::test]
    async fn test_wallet_status() {
        let wallet = Wallet::new_mining();
        let blockchain = Blockchain::new();
        let config = MinerConfig::default();
        
        let miner = Miner::new(wallet, blockchain, config);
        let status = miner.get_wallet_status().await.unwrap();
        
        assert_eq!(status.balance, 0);
        assert!(status.is_in_cooling_period);
    }
}
