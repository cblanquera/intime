//! Wallet management for InTime blockchain

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletType {
    Consumer,
    Mining,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub id: String,
    pub address: String,
    pub wallet_type: WalletType,
    pub balance: u64, // Balance in minutes (MIN)
    pub created_at: DateTime<Utc>,
    pub cooling_period_end: Option<DateTime<Utc>>,
    pub is_mining: bool,
    pub verified_blocks: Vec<String>, // Block hashes verified during cooling period
    pub negative_balance: u64, // Negative balance that increases by the minute
    pub last_burn_time: DateTime<Utc>,
}

impl Wallet {
    /// Create a new consumer wallet
    pub fn new_consumer() -> Self {
        let id = Uuid::new_v4().to_string();
        let address = Self::generate_address(&id);
        let now = Utc::now();
        
        Self {
            id,
            address,
            wallet_type: WalletType::Consumer,
            balance: 0,
            created_at: now,
            cooling_period_end: None,
            is_mining: false,
            verified_blocks: Vec::new(),
            negative_balance: 0,
            last_burn_time: now,
        }
    }
    
    /// Create a new mining wallet with cooling period
    pub fn new_mining() -> Self {
        let id = Uuid::new_v4().to_string();
        let address = Self::generate_address(&id);
        let now = Utc::now();
        let cooling_end = now + Duration::days(7);
        
        Self {
            id,
            address,
            wallet_type: WalletType::Mining,
            balance: 0,
            created_at: now,
            cooling_period_end: Some(cooling_end),
            is_mining: false,
            verified_blocks: Vec::new(),
            negative_balance: 0,
            last_burn_time: now,
        }
    }
    
    /// Generate a wallet address from ID
    fn generate_address(id: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(id.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)[..40].to_string() // Take first 40 chars for address
    }
    
    /// Check if wallet is in cooling period
    pub fn is_in_cooling_period(&self) -> bool {
        if let Some(cooling_end) = self.cooling_period_end {
            Utc::now() < cooling_end
        } else {
            false
        }
    }
    
    /// Check if wallet can start mining
    pub fn can_start_mining(&self) -> Result<()> {
        match self.wallet_type {
            WalletType::Consumer => Err(Error::Wallet("Consumer wallets cannot mine".to_string())),
            WalletType::Mining => {
                if self.is_in_cooling_period() {
                    Err(Error::CoolingPeriod { 
                        until: self.cooling_period_end.unwrap() 
                    })
                } else {
                    Ok(())
                }
            }
        }
    }
    
    /// Add verified block during cooling period
    pub fn add_verified_block(&mut self, block_hash: String) -> Result<()> {
        if !self.is_in_cooling_period() {
            return Err(Error::Wallet("Not in cooling period".to_string()));
        }
        
        self.verified_blocks.push(block_hash);
        Ok(())
    }
    
    /// Burn tokens based on time elapsed
    pub fn burn_tokens(&mut self) -> Result<()> {
        let now = Utc::now();
        let minutes_elapsed = (now - self.last_burn_time).num_minutes() as u64;
        
        if minutes_elapsed > 0 {
            if self.balance >= minutes_elapsed {
                self.balance -= minutes_elapsed;
            } else {
                // Add to negative balance
                let deficit = minutes_elapsed - self.balance;
                self.balance = 0;
                self.negative_balance += deficit;
            }
            self.last_burn_time = now;
        }
        
        Ok(())
    }
    
    /// Add tokens to wallet
    pub fn add_tokens(&mut self, amount: u64) -> Result<()> {
        // First cover negative balance
        if self.negative_balance > 0 {
            if amount >= self.negative_balance {
                let remaining = amount - self.negative_balance;
                self.negative_balance = 0;
                self.balance = remaining;
            } else {
                self.negative_balance -= amount;
            }
        } else {
            self.balance += amount;
        }
        Ok(())
    }
    
    /// Transfer tokens to another wallet
    pub fn transfer(&mut self, amount: u64, to: &mut Wallet) -> Result<()> {
        // Check if mining wallet can transfer
        if matches!(self.wallet_type, WalletType::Mining) && self.is_mining {
            return Err(Error::MiningWalletTransferRestriction);
        }
        
        // Burn tokens first
        self.burn_tokens()?;
        
        // Check if we have enough balance
        if self.balance < amount {
            return Err(Error::InsufficientBalance {
                required: amount,
                available: self.balance,
            });
        }
        
        // Check if recipient can receive (must not have negative balance unless transfer covers it)
        if to.negative_balance > 0 && amount < to.negative_balance {
            return Err(Error::Wallet(
                "Cannot transfer to wallet with negative balance unless amount covers the deficit".to_string()
            ));
        }
        
        self.balance -= amount;
        to.add_tokens(amount)?;
        
        Ok(())
    }
    
    /// Convert mining wallet to consumer wallet
    pub fn convert_to_consumer(&mut self) -> Result<()> {
        match self.wallet_type {
            WalletType::Consumer => Err(Error::Wallet("Already a consumer wallet".to_string())),
            WalletType::Mining => {
                if self.is_mining {
                    return Err(Error::Wallet("Cannot convert while mining".to_string()));
                }
                
                self.wallet_type = WalletType::Consumer;
                self.cooling_period_end = None;
                self.verified_blocks.clear();
                Ok(())
            }
        }
    }
    
    /// Get effective balance (considering negative balance)
    pub fn effective_balance(&self) -> i64 {
        self.balance as i64 - self.negative_balance as i64
    }
    
    /// Get mining difficulty based on balance (Proof of Being Poor)
    pub fn get_mining_difficulty(&self) -> u32 {
        let years = self.balance / (60 * 24 * 365); // Convert minutes to years
        
        match years {
            0..=9 => 2,      // 2 zeros
            10..=99 => 3,    // 3 zeros
            100..=499 => 4,  // 4 zeros
            500..=999 => 5,  // 5 zeros
            1000..=1499 => 6, // 6 zeros
            1500..=1999 => 7, // 7 zeros
            _ => 8,          // 8 zeros for very wealthy miners
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_consumer_wallet() {
        let wallet = Wallet::new_consumer();
        assert!(matches!(wallet.wallet_type, WalletType::Consumer));
        assert_eq!(wallet.balance, 0);
        assert!(!wallet.is_in_cooling_period());
    }
    
    #[test]
    fn test_new_mining_wallet() {
        let wallet = Wallet::new_mining();
        assert!(matches!(wallet.wallet_type, WalletType::Mining));
        assert_eq!(wallet.balance, 0);
        assert!(wallet.is_in_cooling_period());
    }
    
    #[test]
    fn test_mining_difficulty() {
        let mut wallet = Wallet::new_mining();
        
        // 0 years = 2 zeros
        assert_eq!(wallet.get_mining_difficulty(), 2);
        
        // 10 years = 3 zeros
        wallet.balance = 60 * 24 * 365 * 10;
        assert_eq!(wallet.get_mining_difficulty(), 3);
        
        // 100 years = 4 zeros
        wallet.balance = 60 * 24 * 365 * 100;
        assert_eq!(wallet.get_mining_difficulty(), 4);
    }
}
