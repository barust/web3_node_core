
Rust project with Web3 integration via Alchemy API

# Rust Project with Web3 Integration via Alchemy API

> A minimal Web3 Ethereum service in Rust using `ethers-rs`, `Axum`, and AlchemyAPI.

## Overview

This is a foundational Web3 project written in Rust that demonstrates integration with the Ethereum blockchain via Alchemy API. It includes a simple REST API endpoint to fetch wallet balances. The project is structured with modular architecture and is ready for extension.

---

## Features

- Ethereum interaction using `ethers-rs` and JSON-RPC.
- Fetch balance for any Ethereum wallet address.
- Alchemy mainnet integration.
- Axum-based RESTful HTTP server.
- Fully asynchronous using Tokio.
- Modular DDD-style structure: `handlers`, `domain`, `infra`.
- Environment configuration with `.env`.
- Ready for CI/CD pipelines (GitHub Actions).

---

##  Tech Stack

- **Rust**
- **ethers-rs** (`ethers` crate)
- **Axum**
- **Tokio**
- **serde / serde_json**
- **dotenv**
- **tracing**

---

## Installation & Run

### 1. Clone the repository

```bash
git clone git@github.com:YOUR_USERNAME/web3_node_core.git
cd web3_node_core

### 2. Build dependencies

```bash
cargo build

### 3. Create .env file

``` env
### .env
RPC_URL=https://eth-mainnet.alchemyapi.io/v2/YOUR_API_KEY
PORT=8080

### 4. Run the server

``` bash
cargo run

The server will start at http://localhost:8080.

#### Example Request

```bash

curl -X POST http://localhost:8080/balance \
  -H "Content-Type: application/json" \
  -d '{"address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"}'


### Run Tests

```bash
cargo test


### Project Structure

web3_node_core/
├── src/
│   ├── main.rs
│   ├── handlers/          # API controllers
│   │   └── wallet.rs
│   ├── domain/            # Domain logic and models
│   │   └── wallet.rs
│   ├── infra/             # Infrastructure layer
│   │   └── provider.rs
├── .env
├── .gitignore
├── Cargo.toml
└── README.md


### CI/CD
GitHub Actions pipeline:

build.yml: compiles, runs tests, and checks formatting.


### Security Notes
No private keys are stored in the repository.

Alchemy API key is stored securely via .env.

Use HTTPS with Alchemy RPC endpoint.

### License
MIT License

### Acknowledgments
ethers-rs

Axum

Alchemy


