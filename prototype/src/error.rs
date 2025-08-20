//! Error types for the InTime miner

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Wallet error: {0}")]
    Wallet(String),
    
    #[error("Mining error: {0}")]
    Mining(String),
    
    #[error("Blockchain error: {0}")]
    Blockchain(String),
    
    #[error("Token error: {0}")]
    Token(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Time error: {0}")]
    Time(#[from] chrono::ParseError),
    
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: u64, available: u64 },
    
    #[error("Wallet is in cooling period until {until}")]
    CoolingPeriod { until: chrono::DateTime<chrono::Utc> },
    
    #[error("Mining wallet cannot transfer while mining")]
    MiningWalletTransferRestriction,
    
    #[error("Invalid nonce: {0}")]
    InvalidNonce(u64),
    
    #[error("Invalid block hash: {0}")]
    InvalidBlockHash(String),
}
