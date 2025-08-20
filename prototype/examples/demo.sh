#!/bin/bash

# InTime Miner Demo Script
# This script demonstrates the basic functionality of the InTime blockchain miner

echo "=== InTime Blockchain Miner Demo ==="
echo

# Build the project
echo "Building the project..."
cargo build --release
echo

# Create a mining wallet
echo "Creating a mining wallet..."
cargo run --release -- create-wallet --wallet-type mining --output demo-wallet.json
echo

# Check initial status
echo "Checking initial wallet status..."
cargo run --release -- status --wallet demo-wallet.json
echo

# Add some tokens for testing
echo "Adding tokens for testing (1 year = 525,600 MIN)..."
cargo run --release -- add-tokens --wallet demo-wallet.json --amount 525600
echo

# Check status after adding tokens
echo "Checking wallet status after adding tokens..."
cargo run --release -- status --wallet demo-wallet.json
echo

# Create a consumer wallet for transfers
echo "Creating a consumer wallet..."
cargo run --release -- create-wallet --wallet-type consumer --output consumer-wallet.json
echo

# Get the consumer wallet address
CONSUMER_ADDRESS=$(cat consumer-wallet.json | grep -o '"address":"[^"]*"' | cut -d'"' -f4)
echo "Consumer wallet address: $CONSUMER_ADDRESS"
echo

# Transfer some tokens
echo "Transferring 1000 MIN to consumer wallet..."
cargo run --release -- transfer --wallet demo-wallet.json --to $CONSUMER_ADDRESS --amount 1000
echo

# Check blockchain status
echo "Checking blockchain status..."
cargo run --release -- blockchain
echo

# Demonstrate Time Bank
echo "Depositing 10000 MIN to Time Bank..."
cargo run --release -- time-bank deposit --wallet demo-wallet.json --amount 10000
echo

echo "Checking Time Bank balance..."
cargo run --release -- time-bank balance --wallet demo-wallet.json
echo

echo "Withdrawing 5000 MIN from Time Bank..."
cargo run --release -- time-bank withdraw --wallet demo-wallet.json --amount 5000
echo

# Final status check
echo "Final wallet status..."
cargo run --release -- status --wallet demo-wallet.json
echo

echo "=== Demo Complete ==="
echo "Files created:"
echo "- demo-wallet.json (mining wallet)"
echo "- consumer-wallet.json (consumer wallet)"
echo "- blockchain.json (blockchain data)"
echo
echo "To start mining (after cooling period ends):"
echo "cargo run --release -- start-mining --wallet demo-wallet.json"
