//! Token operations for InTime blockchain

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBalance {
    pub address: String,
    pub balance: u64,
    pub negative_balance: u64,
    pub last_burn_time: DateTime<Utc>,
}

impl TokenBalance {
    pub fn new(address: String) -> Self {
        Self {
            address,
            balance: 0,
            negative_balance: 0,
            last_burn_time: Utc::now(),
        }
    }
    
    /// Get effective balance (considering negative balance)
    pub fn effective_balance(&self) -> i64 {
        self.balance as i64 - self.negative_balance as i64
    }
    
    /// Burn tokens based on time elapsed
    pub fn burn_tokens(&mut self) -> Result<u64> {
        let now = Utc::now();
        let minutes_elapsed = (now - self.last_burn_time).num_minutes() as u64;
        
        if minutes_elapsed > 0 {
            let burned = if self.balance >= minutes_elapsed {
                self.balance -= minutes_elapsed;
                minutes_elapsed
            } else {
                // Add to negative balance
                let deficit = minutes_elapsed - self.balance;
                let burned = self.balance;
                self.balance = 0;
                self.negative_balance += deficit;
                burned
            };
            
            self.last_burn_time = now;
            Ok(burned)
        } else {
            Ok(0)
        }
    }
    
    /// Add tokens to balance
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenManager {
    pub balances: HashMap<String, TokenBalance>,
    pub total_supply: u64,
    pub total_burned: u64,
    pub burn_rate_per_minute: u64, // How many tokens are burned per minute globally
}

impl TokenManager {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            total_supply: 0,
            total_burned: 0,
            burn_rate_per_minute: 0,
        }
    }
    
    /// Create or get a token balance for an address
    pub fn get_or_create_balance(&mut self, address: &str) -> &mut TokenBalance {
        self.balances
            .entry(address.to_string())
            .or_insert_with(|| TokenBalance::new(address.to_string()))
    }
    
    /// Get balance for an address
    pub fn get_balance(&self, address: &str) -> Option<&TokenBalance> {
        self.balances.get(address)
    }
    
    /// Mint new tokens (for mining rewards)
    pub fn mint_tokens(&mut self, address: &str, amount: u64) -> Result<()> {
        let balance = self.get_or_create_balance(address);
        balance.add_tokens(amount)?;
        self.total_supply += amount;
        Ok(())
    }
    
    /// Transfer tokens between addresses
    pub fn transfer_tokens(
        &mut self,
        from: &str,
        to: &str,
        amount: u64,
    ) -> Result<()> {
        // Burn tokens first for both addresses
        self.burn_tokens_for_address(from)?;
        self.burn_tokens_for_address(to)?;
        
        // Check if sender has enough balance
        let from_balance = self.get_balance(from)
            .ok_or_else(|| Error::Token("Sender address not found".to_string()))?;
        
        if from_balance.balance < amount {
            return Err(Error::InsufficientBalance {
                required: amount,
                available: from_balance.balance,
            });
        }
        
        // Check if recipient can receive (must not have negative balance unless transfer covers it)
        let to_balance = self.get_balance(to);
        if let Some(to_bal) = to_balance {
            if to_bal.negative_balance > 0 && amount < to_bal.negative_balance {
                return Err(Error::Token(
                    "Cannot transfer to address with negative balance unless amount covers the deficit".to_string()
                ));
            }
        }
        
        // Perform the transfer
        let from_balance = self.get_or_create_balance(from);
        from_balance.balance -= amount;
        
        let to_balance = self.get_or_create_balance(to);
        to_balance.add_tokens(amount)?;
        
        Ok(())
    }
    
    /// Burn tokens for a specific address
    pub fn burn_tokens_for_address(&mut self, address: &str) -> Result<u64> {
        if let Some(balance) = self.balances.get_mut(address) {
            let burned = balance.burn_tokens()?;
            self.total_burned += burned;
            Ok(burned)
        } else {
            Ok(0)
        }
    }
    
    /// Burn tokens for all addresses
    pub fn burn_all_tokens(&mut self) -> Result<u64> {
        let mut total_burned = 0;
        
        for balance in self.balances.values_mut() {
            let burned = balance.burn_tokens()?;
            total_burned += burned;
        }
        
        self.total_burned += total_burned;
        Ok(total_burned)
    }
    
    /// Get total active supply (excluding negative balances)
    pub fn get_active_supply(&self) -> u64 {
        self.balances
            .values()
            .map(|balance| balance.balance)
            .sum()
    }
    
    /// Get total negative balance across all addresses
    pub fn get_total_negative_balance(&self) -> u64 {
        self.balances
            .values()
            .map(|balance| balance.negative_balance)
            .sum()
    }
    
    /// Get addresses with negative balances
    pub fn get_negative_balance_addresses(&self) -> Vec<String> {
        self.balances
            .values()
            .filter(|balance| balance.negative_balance > 0)
            .map(|balance| balance.address.clone())
            .collect()
    }
    
    /// Get addresses with positive balances
    pub fn get_positive_balance_addresses(&self) -> Vec<String> {
        self.balances
            .values()
            .filter(|balance| balance.balance > 0)
            .map(|balance| balance.address.clone())
            .collect()
    }
    
    /// Calculate burn rate per minute based on active addresses
    pub fn calculate_burn_rate(&mut self) -> u64 {
        let active_addresses = self.balances
            .values()
            .filter(|balance| balance.balance > 0 || balance.negative_balance > 0)
            .count() as u64;
        
        self.burn_rate_per_minute = active_addresses;
        self.burn_rate_per_minute
    }
    
    /// Get token statistics
    pub fn get_statistics(&self) -> TokenStatistics {
        let active_supply = self.get_active_supply();
        let total_negative = self.get_total_negative_balance();
        let effective_supply = active_supply as i64 - total_negative as i64;
        
        TokenStatistics {
            total_supply: self.total_supply,
            active_supply,
            total_burned: self.total_burned,
            total_negative_balance: total_negative,
            effective_supply: effective_supply.max(0) as u64,
            active_addresses: self.balances.len(),
            positive_balance_addresses: self.get_positive_balance_addresses().len(),
            negative_balance_addresses: self.get_negative_balance_addresses().len(),
            burn_rate_per_minute: self.burn_rate_per_minute,
        }
    }
}

impl Default for TokenManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenStatistics {
    pub total_supply: u64,
    pub active_supply: u64,
    pub total_burned: u64,
    pub total_negative_balance: u64,
    pub effective_supply: u64,
    pub active_addresses: usize,
    pub positive_balance_addresses: usize,
    pub negative_balance_addresses: usize,
    pub burn_rate_per_minute: u64,
}

/// Time Bank - A stable version of tokens that don't burn
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeBank {
    pub balances: HashMap<String, u64>,
    pub total_deposited: u64,
}

impl TimeBank {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            total_deposited: 0,
        }
    }
    
    /// Deposit active time into stable time
    pub fn deposit(
        &mut self,
        token_manager: &mut TokenManager,
        address: &str,
        amount: u64,
    ) -> Result<()> {
        // Burn tokens first
        token_manager.burn_tokens_for_address(address)?;
        
        // Check if address has enough balance
        let balance = token_manager.get_balance(address)
            .ok_or_else(|| Error::Token("Address not found".to_string()))?;
        
        if balance.balance < amount {
            return Err(Error::InsufficientBalance {
                required: amount,
                available: balance.balance,
            });
        }
        
        // Transfer from active to stable
        let token_balance = token_manager.get_or_create_balance(address);
        token_balance.balance -= amount;
        
        let stable_balance = self.balances.entry(address.to_string()).or_insert(0);
        *stable_balance += amount;
        
        self.total_deposited += amount;
        
        Ok(())
    }
    
    /// Withdraw stable time to active time
    pub fn withdraw(
        &mut self,
        token_manager: &mut TokenManager,
        address: &str,
        amount: u64,
    ) -> Result<()> {
        // Check if address has enough stable balance
        let stable_balance = self.balances.get(address)
            .ok_or_else(|| Error::Token("No stable balance found".to_string()))?;
        
        if *stable_balance < amount {
            return Err(Error::InsufficientBalance {
                required: amount,
                available: *stable_balance,
            });
        }
        
        // Check if recipient address can receive (no negative balance unless covered)
        let token_balance = token_manager.get_balance(address);
        if let Some(balance) = token_balance {
            if balance.negative_balance > 0 && amount < balance.negative_balance {
                return Err(Error::Token(
                    "Cannot withdraw to address with negative balance unless amount covers the deficit".to_string()
                ));
            }
        }
        
        // Transfer from stable to active
        let stable_balance = self.balances.get_mut(address).unwrap();
        *stable_balance -= amount;
        
        let token_balance = token_manager.get_or_create_balance(address);
        token_balance.add_tokens(amount)?;
        
        self.total_deposited -= amount;
        
        Ok(())
    }
    
    /// Get stable balance for an address
    pub fn get_balance(&self, address: &str) -> u64 {
        self.balances.get(address).copied().unwrap_or(0)
    }
    
    /// Get total stable supply
    pub fn get_total_supply(&self) -> u64 {
        self.balances.values().sum()
    }
}

impl Default for TimeBank {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token_balance_creation() {
        let balance = TokenBalance::new("test_address".to_string());
        assert_eq!(balance.balance, 0);
        assert_eq!(balance.negative_balance, 0);
        assert_eq!(balance.effective_balance(), 0);
    }
    
    #[test]
    fn test_token_manager() {
        let mut manager = TokenManager::new();
        
        // Mint tokens
        manager.mint_tokens("address1", 1000).unwrap();
        assert_eq!(manager.total_supply, 1000);
        
        // Check balance
        let balance = manager.get_balance("address1").unwrap();
        assert_eq!(balance.balance, 1000);
    }
    
    #[test]
    fn test_time_bank() {
        let mut token_manager = TokenManager::new();
        let mut time_bank = TimeBank::new();
        
        // Mint some tokens first
        token_manager.mint_tokens("address1", 1000).unwrap();
        
        // Deposit to time bank
        time_bank.deposit(&mut token_manager, "address1", 500).unwrap();
        
        assert_eq!(time_bank.get_balance("address1"), 500);
        assert_eq!(token_manager.get_balance("address1").unwrap().balance, 500);
    }
}
