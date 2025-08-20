# InTime Blockchain Miner

A Rust implementation of the InTime blockchain miner with "Proof of Being Poor" consensus mechanism. Inspired by the movie "In Time", this blockchain uses a burning token protocol where tokens (MIN) are automatically burned over time.

## Features

- **Proof of Being Poor**: Mining difficulty increases with wallet balance, encouraging wealth distribution
- **Burning Token Protocol**: Tokens automatically burn every minute
- **Mining Wallets**: Special wallets that can mine blocks with a 7-day cooling period
- **Consumer Wallets**: Regular wallets for token consumption
- **Time Bank**: Store stable tokens that don't burn
- **Block Verification**: During cooling period, miners verify new blocks
- **Energy Saving**: Configurable maximum difficulty to save energy

## Installation

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

### Build from Source

```bash
git clone <repository-url>
cd intime-miner
cargo build --release
```

## Usage

### Create a Wallet

Create a new mining wallet:
```bash
cargo run -- create-wallet --wallet-type mining --output my-wallet.json
```

Create a consumer wallet:
```bash
cargo run -- create-wallet --wallet-type consumer --output consumer-wallet.json
```

### Check Wallet Status

```bash
cargo run -- status --wallet my-wallet.json
```

### Start Mining

```bash
cargo run -- start-mining --wallet my-wallet.json --max-difficulty 4 --interval 10
```

Options:
- `--max-difficulty`: Maximum difficulty for energy saving (optional)
- `--interval`: Mining check interval in seconds (default: 10)

### Transfer Tokens

```bash
cargo run -- transfer --wallet my-wallet.json --to <recipient-address> --amount 1000
```

### Time Bank Operations

Deposit active time to stable time:
```bash
cargo run -- time-bank deposit --wallet my-wallet.json --amount 500
```

Withdraw stable time to active time:
```bash
cargo run -- time-bank withdraw --wallet my-wallet.json --amount 300
```

Check time bank balance:
```bash
cargo run -- time-bank balance --wallet my-wallet.json
```

### Blockchain Information

```bash
cargo run -- blockchain --file blockchain.json
```

### Convert Mining Wallet to Consumer

```bash
cargo run -- convert-wallet --wallet my-wallet.json
```

### Add Tokens (Testing)

```bash
cargo run -- add-tokens --wallet my-wallet.json --amount 1000
```

## Architecture

### Core Components

1. **Wallet Management** (`src/wallet.rs`)
   - Mining and consumer wallet types
   - Token burning mechanism
   - Cooling period management
   - Proof of Being Poor difficulty calculation

2. **Blockchain** (`src/blockchain.rs`)
   - Block structure and validation
   - Transaction management
   - Mining rewards (10 years per block)
   - Block verification signatures

3. **Mining Engine** (`src/mining.rs`)
   - Asynchronous mining loop
   - Block verification during cooling period
   - Energy-saving mechanisms
   - Online compensation (1 MIN per minute)

4. **Token System** (`src/token.rs`)
   - Burning token protocol
   - Time Bank for stable storage
   - Token statistics and management

5. **CLI Interface** (`src/cli.rs`)
   - Command-line interface
   - File-based persistence
   - User-friendly commands

### Proof of Being Poor

Mining difficulty is determined by wallet balance:

| Balance (Years) | Difficulty (Zeros) |
|-----------------|-------------------|
| 0-9             | 2                 |
| 10-99           | 3                 |
| 100-499         | 4                 |
| 500-999         | 5                 |
| 1000-1499       | 6                 |
| 1500-1999       | 7                 |
| 2000+           | 8                 |

This encourages miners to distribute wealth to maintain competitive mining capability.

### Token Economics

- **MIN**: Base token unit (1 minute of time)
- **Burning Rate**: 1 MIN per minute per active wallet
- **Mining Reward**: ~5.26 million MIN (10 years) per block
- **Online Compensation**: 1 MIN per minute for active miners
- **Time Bank**: Stable storage that doesn't burn

### Wallet Types

#### Mining Wallet
- Can mine blocks after 7-day cooling period
- Cannot transfer while actively mining
- Automatically converts to consumer when first transfer is made
- Must verify all blocks during cooling period

#### Consumer Wallet
- Regular wallet for token consumption
- Can transfer freely
- Cannot mine blocks
- Subject to token burning

## File Structure

```
intime-miner/
├── src/
│   ├── lib.rs          # Library root
│   ├── main.rs         # CLI entry point
│   ├── error.rs        # Error types
│   ├── wallet.rs       # Wallet management
│   ├── blockchain.rs   # Blockchain implementation
│   ├── mining.rs       # Mining logic
│   ├── token.rs        # Token operations
│   └── cli.rs          # Command-line interface
├── Cargo.toml          # Dependencies
└── README.md           # This file
```

## Configuration

### Mining Configuration

```rust
MinerConfig {
    max_difficulty: Some(4),        // Energy saving threshold
    mining_interval: Duration::from_secs(10),  // Check interval
    verification_interval: Duration::from_secs(30), // Block verification
}
```

### Wallet Files

Wallets are stored as JSON files containing:
- Address and ID
- Balance and negative balance
- Wallet type and mining status
- Cooling period information
- Verified blocks list

### Blockchain Files

Blockchain data is persisted as JSON containing:
- All blocks with transactions
- Pending transactions
- Mining reward configuration

## Development

### Running Tests

```bash
cargo test
```

### Building Documentation

```bash
cargo doc --open
```

### Code Structure

The project follows Rust best practices:
- Async/await for concurrent operations
- Error handling with `Result<T, Error>`
- Comprehensive unit tests
- Modular architecture
- CLI with clap for argument parsing

## Use Cases

### Revolving Miner
1. Earn tokens from mining
2. Convert mining wallet to consumer
3. Create new mining wallet (wait 7 days)
4. Trade with consumer wallet
5. Send funds to new mining wallet
6. Repeat cycle

### Time Bank Collector
1. Set up mining wallet as time bank
2. Purchase tokens
3. Send to time bank for stable storage
4. Serve the network while preserving wealth

### Consumer
1. Purchase tokens
2. Spend tokens over time
3. Tokens burn automatically
4. Purchase more as needed

## Security Considerations

- Wallet files contain sensitive information
- Private keys are simplified for demonstration
- Block verification ensures network integrity
- Cooling period prevents rapid wallet cycling
- Energy saving prevents excessive computation

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

This project is licensed under the MIT License.

## Acknowledgments

- Inspired by the movie "In Time" (2011)
- Built with Rust and Tokio for async operations
- Uses SHA-256 for hashing and proof of work
