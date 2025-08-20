//! InTime Blockchain Miner
//! 
//! A Rust implementation of the InTime blockchain miner with "Proof of Being Poor" consensus.
//! Inspired by the movie "In Time", this blockchain uses a burning token protocol where
//! tokens are automatically burned over time.

use clap::Parser;
use intime_miner::{
    cli::{Cli, CliHandler},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Create CLI handler
    let mut handler = CliHandler::new();
    
    // Handle the command
    if let Err(e) = handler.handle_command(cli).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}
