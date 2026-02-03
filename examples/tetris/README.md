# Tetris On-Chain with Cougr ECS

This example demonstrates a fully functional Tetris game implemented as a Soroban smart contract on the Stellar blockchain. It leverages the **[cougr-core](https://github.com/salazarsebas/Cougr)** ECS (Entity Component System) framework to manage game state, logic, and entities.

## Overview

The contract implements standard Tetris rules including:

* 7 Tetromino shapes (I, J, L, O, S, T, Z)
* Piece rotation (SRS-lite)
* Line clearing, scoring, and level progression
* Game over detection

It serves as a reference implementation for building complex on-chain logic using `cougr-core`.

### Benefits of Cougr ECS

Using `cougr-core` provides structured game development on Stellar:

| Feature | Benefit |
|---------|---------|
| **ECS Architecture** | Decouples data (Entities/Components) from logic (Systems), making the codebase modular and easier to test. |
| **Separation of Concerns** | Movement, collision, and scoring are handled by distinct logical blocks, preventing spaghetti code. |
| **State Management** | Efficient handling of game state updates during contract invocations. |

## Game Scoring System

| Action | Points (Base) |
|--------|---------------|
| Single Line | 100 x Level |
| Double Line | 300 x Level |
| Triple Line | 500 x Level |
| Tetris (4 Lines) | 800 x Level |

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Stellar CLI](https://github.com/stellar/stellar-cli)

## Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/salazarsebas/Cougr.git
   cd Cougr/examples/tetris
   ```

2. **Install Dependencies:**
   ```bash
   cargo build
   ```

## Build

To build the optimized WASM contract:

```bash
stellar contract build
```

The output will be in `target/wasm32-unknown-unknown/release/tetris.wasm`.

## Testing

Run the comprehensive test suite:

```bash
cargo test
```

## Live Deployment (Testnet)

The contract is deployed on the Stellar Testnet. You can interact with it using the CLI or a block explorer.

| Contract ID | Network | Explorer Link |
|-------------|---------|---------------|
| `CBWENGWFZHPNJPIHQAHXE5K34BGV2G5MOQIQ24PE44M6P42YULMQZYSF` | Testnet | [View on Explorer](https://lab.stellar.org/r/testnet/contract/CBWENGWFZHPNJPIHQAHXE5K34BGV2G5MOQIQ24PE44M6P42YULMQZYSF) |


### Deploying Your Own

1. **Configure Identity:**
   ```bash
   stellar keys generate --global alice
   stellar keys address alice
   ```

2. **Deploy Contract:**
   ```bash
   stellar contract deploy \
     --wasm target/wasm32-unknown-unknown/release/tetris.wasm \
     --source alice \
     --network testnet
   ```

3. **Interact Example:**

   *Initialize Game:*
   ```bash
   stellar contract invoke \
     --id <CONTRACT_ID> \
     --source alice \
     --network testnet \
     -- \
     init_game
   ```

## Documentation

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Cougr Core](https://github.com/salazarsebas/Cougr)
