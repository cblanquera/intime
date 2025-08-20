//! Command line interface for InTime miner

use clap::{Parser, Subcommand};
use std::time::Duration;

use crate::{
    blockchain::Blockchain,
    mining::{Miner, MinerConfig},
    token::{TimeBank, TokenManager},
    wallet::{Wallet, WalletType},
    Result,
};

#[derive(Parser)]
#[command(name = "intime-miner")]
#[command(about = "InTime blockchain miner with Proof of Being Poor consensus")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new wallet
    CreateWallet {
        /// Type of wallet to create (consumer or mining)
        #[arg(short, long, default_value = "mining")]
        wallet_type: String,
        
        /// Output file for wallet data
        #[arg(short, long, default_value = "wallet.json")]
        output: String,
    },
    
    /// Start mining
    StartMining {
        /// Wallet file to use for mining
        #[arg(short, long, default_value = "wallet.json")]
        wallet: String,
        
        /// Blockchain file to load/save
        #[arg(short, long, default_value = "blockchain.json")]
        blockchain: String,
        
        /// Maximum difficulty for energy saving
        #[arg(short, long)]
        max_difficulty: Option<u32>,
        
        /// Mining interval in seconds
        #[arg(short, long, default_value = "10")]
        interval: u64,
    },
    
    /// Check wallet status
    Status {
        /// Wallet file to check
        #[arg(short, long, default_value = "wallet.json")]
        wallet: String,
        
        /// Blockchain file to check
        #[arg(short, long, default_value = "blockchain.json")]
        blockchain: String,
    },
    
    /// Transfer tokens
    Transfer {
        /// Wallet file to transfer from
        #[arg(short, long, default_value = "wallet.json")]
        wallet: String,
        
        /// Blockchain file
        #[arg(short, long, default_value = "blockchain.json")]
        blockchain: String,
        
        /// Recipient address
        #[arg(short, long)]
        to: String,
        
        /// Amount to transfer (in minutes)
        #[arg(short, long)]
        amount: u64,
    },
    
    /// Time Bank operations
    TimeBank {
        #[command(subcommand)]
        operation: TimeBankCommands,
    },
    
    /// Show blockchain information
    Blockchain {
        /// Blockchain file to examine
        #[arg(short, long, default_value = "blockchain.json")]
        file: String,
    },
    
    /// Convert mining wallet to consumer wallet
    ConvertWallet {
        /// Wallet file to convert
        #[arg(short, long, default_value = "wallet.json")]
        wallet: String,
    },
    
    /// Add tokens to wallet (for testing)
    AddTokens {
        /// Wallet file
        #[arg(short, long, default_value = "wallet.json")]
        wallet: String,
        
        /// Amount to add (in minutes)
        #[arg(short, long)]
        amount: u64,
    },
}

#[derive(Subcommand)]
pub enum TimeBankCommands {
    /// Deposit active time to stable time
    Deposit {
        /// Wallet file
        #[arg(short, long, default_value = "wallet.json")]
        wallet: String,
        
        /// Amount to deposit (in minutes)
        #[arg(short, long)]
        amount: u64,
    },
    
    /// Withdraw stable time to active time
    Withdraw {
        /// Wallet file
        #[arg(short, long, default_value = "wallet.json")]
        wallet: String,
        
        /// Amount to withdraw (in minutes)
        #[arg(short, long)]
        amount: u64,
    },
    
    /// Check time bank balance
    Balance {
        /// Wallet file
        #[arg(short, long, default_value = "wallet.json")]
        wallet: String,
    },
}

pub struct CliHandler {
    pub token_manager: TokenManager,
    pub time_bank: TimeBank,
}

impl CliHandler {
    pub fn new() -> Self {
        Self {
            token_manager: TokenManager::new(),
            time_bank: TimeBank::new(),
        }
    }
    
    pub async fn handle_command(&mut self, cli: Cli) -> Result<()> {
        match cli.command {
            Commands::CreateWallet { wallet_type, output } => {
                self.create_wallet(wallet_type, output).await
            }
            Commands::StartMining {
                wallet,
                blockchain,
                max_difficulty,
                interval,
            } => {
                self.start_mining(wallet, blockchain, max_difficulty, interval).await
            }
            Commands::Status { wallet, blockchain } => {
                self.show_status(wallet, blockchain).await
            }
            Commands::Transfer {
                wallet,
                blockchain,
                to,
                amount,
            } => {
                self.transfer_tokens(wallet, blockchain, to, amount).await
            }
            Commands::TimeBank { operation } => {
                self.handle_time_bank(operation).await
            }
            Commands::Blockchain { file } => {
                self.show_blockchain_info(file).await
            }
            Commands::ConvertWallet { wallet } => {
                self.convert_wallet(wallet).await
            }
            Commands::AddTokens { wallet, amount } => {
                self.add_tokens(wallet, amount).await
            }
        }
    }
    
    async fn create_wallet(&self, wallet_type: String, output: String) -> Result<()> {
        let wallet = match wallet_type.to_lowercase().as_str() {
            "consumer" => Wallet::new_consumer(),
            "mining" => Wallet::new_mining(),
            _ => {
                println!("Invalid wallet type. Use 'consumer' or 'mining'");
                return Ok(());
            }
        };
        
        // Save wallet to file
        let wallet_json = serde_json::to_string_pretty(&wallet)?;
        std::fs::write(&output, wallet_json)?;
        
        println!("Created {} wallet: {}", wallet_type, wallet.address);
        println!("Wallet saved to: {}", output);
        
        if matches!(wallet.wallet_type, WalletType::Mining) {
            println!("Mining wallet is in cooling period until: {:?}", wallet.cooling_period_end);
            println!("During this time, the wallet will verify new blocks.");
        }
        
        Ok(())
    }
    
    async fn start_mining(
        &self,
        wallet_file: String,
        blockchain_file: String,
        max_difficulty: Option<u32>,
        interval: u64,
    ) -> Result<()> {
        // Load wallet
        let wallet_data = std::fs::read_to_string(&wallet_file)?;
        let wallet: Wallet = serde_json::from_str(&wallet_data)?;
        
        // Load or create blockchain
        let blockchain = if std::path::Path::new(&blockchain_file).exists() {
            let blockchain_data = std::fs::read_to_string(&blockchain_file)?;
            serde_json::from_str(&blockchain_data)?
        } else {
            Blockchain::new()
        };
        
        // Create miner config
        let config = MinerConfig {
            max_difficulty,
            mining_interval: Duration::from_secs(interval),
            verification_interval: Duration::from_secs(30),
        };
        
        // Create and start miner
        let miner = Miner::new(wallet, blockchain, config);
        
        println!("Starting InTime miner...");
        println!("Press Ctrl+C to stop mining");
        
        miner.start().await?;
        
        // Keep the program running
        tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
        
        println!("Stopping miner...");
        miner.stop().await?;
        
        // Save blockchain
        let blockchain = miner.blockchain.lock().await;
        let blockchain_json = serde_json::to_string_pretty(&*blockchain)?;
        std::fs::write(&blockchain_file, blockchain_json)?;
        
        // Save wallet
        let wallet = miner.wallet.lock().await;
        let wallet_json = serde_json::to_string_pretty(&*wallet)?;
        std::fs::write(&wallet_file, wallet_json)?;
        
        println!("Miner stopped and data saved.");
        
        Ok(())
    }
    
    async fn show_status(&self, wallet_file: String, blockchain_file: String) -> Result<()> {
        // Load wallet
        let wallet_data = std::fs::read_to_string(&wallet_file)?;
        let mut wallet: Wallet = serde_json::from_str(&wallet_data)?;
        
        // Load blockchain
        let blockchain = if std::path::Path::new(&blockchain_file).exists() {
            let blockchain_data = std::fs::read_to_string(&blockchain_file)?;
            serde_json::from_str(&blockchain_data)?
        } else {
            Blockchain::new()
        };
        
        // Burn tokens to get current balance
        wallet.burn_tokens()?;
        
        println!("=== Wallet Status ===");
        println!("Address: {}", wallet.address);
        println!("Type: {:?}", wallet.wallet_type);
        println!("Balance: {} MIN ({:.2} years)", wallet.balance, wallet.balance as f64 / (60.0 * 24.0 * 365.0));
        println!("Negative Balance: {} MIN", wallet.negative_balance);
        println!("Effective Balance: {} MIN", wallet.effective_balance());
        println!("Is Mining: {}", wallet.is_mining);
        println!("In Cooling Period: {}", wallet.is_in_cooling_period());
        
        if let Some(cooling_end) = wallet.cooling_period_end {
            println!("Cooling Period Ends: {}", cooling_end);
        }
        
        println!("Verified Blocks: {}", wallet.verified_blocks.len());
        println!("Mining Difficulty: {} zeros", wallet.get_mining_difficulty());
        
        println!("\n=== Blockchain Status ===");
        println!("Total Blocks: {}", blockchain.blocks.len());
        println!("Pending Transactions: {}", blockchain.pending_transactions.len());
        println!("Latest Block: {}", blockchain.get_latest_block().hash);
        println!("Blockchain Valid: {}", blockchain.is_valid());
        println!("Blockchain Balance: {} MIN", blockchain.get_balance(&wallet.address));
        
        Ok(())
    }
    
    async fn transfer_tokens(
        &self,
        wallet_file: String,
        blockchain_file: String,
        to: String,
        amount: u64,
    ) -> Result<()> {
        // Load wallet
        let wallet_data = std::fs::read_to_string(&wallet_file)?;
        let wallet: Wallet = serde_json::from_str(&wallet_data)?;
        
        // Load blockchain
        let blockchain = if std::path::Path::new(&blockchain_file).exists() {
            let blockchain_data = std::fs::read_to_string(&blockchain_file)?;
            serde_json::from_str(&blockchain_data)?
        } else {
            Blockchain::new()
        };
        
        // Create miner to handle transaction
        let config = MinerConfig::default();
        let miner = Miner::new(wallet, blockchain, config);
        
        // Create transaction
        miner.create_transaction(to.clone(), amount).await?;
        
        println!("Transaction created: {} MIN to {}", amount, to);
        println!("Transaction added to pending transactions.");
        
        // Save blockchain
        let blockchain = miner.blockchain.lock().await;
        let blockchain_json = serde_json::to_string_pretty(&*blockchain)?;
        std::fs::write(&blockchain_file, blockchain_json)?;
        
        // Save wallet
        let wallet = miner.wallet.lock().await;
        let wallet_json = serde_json::to_string_pretty(&*wallet)?;
        std::fs::write(&wallet_file, wallet_json)?;
        
        Ok(())
    }
    
    async fn handle_time_bank(&mut self, operation: TimeBankCommands) -> Result<()> {
        match operation {
            TimeBankCommands::Deposit { wallet, amount } => {
                let wallet_data = std::fs::read_to_string(&wallet)?;
                let wallet_obj: Wallet = serde_json::from_str(&wallet_data)?;
                
                self.time_bank.deposit(&mut self.token_manager, &wallet_obj.address, amount)?;
                
                println!("Deposited {} MIN to Time Bank", amount);
                println!("Stable Balance: {} MIN", self.time_bank.get_balance(&wallet_obj.address));
            }
            TimeBankCommands::Withdraw { wallet, amount } => {
                let wallet_data = std::fs::read_to_string(&wallet)?;
                let wallet_obj: Wallet = serde_json::from_str(&wallet_data)?;
                
                self.time_bank.withdraw(&mut self.token_manager, &wallet_obj.address, amount)?;
                
                println!("Withdrew {} MIN from Time Bank", amount);
                println!("Stable Balance: {} MIN", self.time_bank.get_balance(&wallet_obj.address));
            }
            TimeBankCommands::Balance { wallet } => {
                let wallet_data = std::fs::read_to_string(&wallet)?;
                let wallet_obj: Wallet = serde_json::from_str(&wallet_data)?;
                
                let stable_balance = self.time_bank.get_balance(&wallet_obj.address);
                println!("Time Bank Balance: {} MIN ({:.2} years)", 
                    stable_balance, 
                    stable_balance as f64 / (60.0 * 24.0 * 365.0)
                );
            }
        }
        
        Ok(())
    }
    
    async fn show_blockchain_info(&self, file: String) -> Result<()> {
        let blockchain_data = std::fs::read_to_string(&file)?;
        let blockchain: Blockchain = serde_json::from_str(&blockchain_data)?;
        
        println!("=== Blockchain Information ===");
        println!("Total Blocks: {}", blockchain.blocks.len());
        println!("Pending Transactions: {}", blockchain.pending_transactions.len());
        println!("Mining Reward: {} MIN ({:.2} years)", 
            blockchain.mining_reward, 
            blockchain.mining_reward as f64 / (60.0 * 24.0 * 365.0)
        );
        println!("Is Valid: {}", blockchain.is_valid());
        
        println!("\n=== Recent Blocks ===");
        for block in blockchain.blocks.iter().rev().take(5) {
            println!("Block {}: {} (difficulty: {}, miner: {})", 
                block.index, 
                &block.hash[..16], 
                block.difficulty,
                &block.miner[..16]
            );
        }
        
        if !blockchain.pending_transactions.is_empty() {
            println!("\n=== Pending Transactions ===");
            for tx in &blockchain.pending_transactions {
                println!("{} -> {}: {} MIN", 
                    &tx.from[..16], 
                    &tx.to[..16], 
                    tx.amount
                );
            }
        }
        
        Ok(())
    }
    
    async fn convert_wallet(&self, wallet_file: String) -> Result<()> {
        let wallet_data = std::fs::read_to_string(&wallet_file)?;
        let mut wallet: Wallet = serde_json::from_str(&wallet_data)?;
        
        wallet.convert_to_consumer()?;
        
        let wallet_json = serde_json::to_string_pretty(&wallet)?;
        std::fs::write(&wallet_file, wallet_json)?;
        
        println!("Wallet converted to consumer type");
        println!("Address: {}", wallet.address);
        
        Ok(())
    }
    
    async fn add_tokens(&self, wallet_file: String, amount: u64) -> Result<()> {
        let wallet_data = std::fs::read_to_string(&wallet_file)?;
        let mut wallet: Wallet = serde_json::from_str(&wallet_data)?;
        
        wallet.add_tokens(amount)?;
        
        let wallet_json = serde_json::to_string_pretty(&wallet)?;
        std::fs::write(&wallet_file, wallet_json)?;
        
        println!("Added {} MIN to wallet", amount);
        println!("New balance: {} MIN ({:.2} years)", 
            wallet.balance, 
            wallet.balance as f64 / (60.0 * 24.0 * 365.0)
        );
        
        Ok(())
    }
}

impl Default for CliHandler {
    fn default() -> Self {
        Self::new()
    }
}
