//! InTime Blockchain Miner
//! 
//! A Rust implementation of the InTime blockchain miner with "Proof of Being Poor" consensus.

pub mod wallet;
pub mod mining;
pub mod blockchain;
pub mod token;
pub mod cli;
pub mod error;

pub use error::{Error, Result};
