# Asteroids (Soroban Example)

This folder hosts the on-chain Asteroids example contract.

## Project Structure

This example uses the single-contract Soroban layout:

```text
.
├── src
│   ├── lib.rs
│   ├── test.rs
│   └── Makefile
├── Cargo.toml
└── README.md
```

## Setup (from the Soroban "Hello World" guide)

These steps mirror the official Soroban getting-started flow, but scoped to this example.

1) Install Rust + Cargo (via rustup):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2) Ensure your toolchain is up to date and add the WASM target:

```bash
rustup update
rustup target add wasm32-unknown-unknown
```

3) Install the Stellar CLI (Soroban tooling):

```bash
cargo install stellar-cli --locked
```

4) Initialize a Soroban project (already done here):

```bash
mkdir -p examples/asteroids
cd examples/asteroids
stellar contract init .
```

## Common Troubleshooting

- Rust version errors: run `rustup update` and retry.
- Missing WASM target: re-run `rustup target add wasm32-unknown-unknown`.
- `stellar` not found: ensure `~/.cargo/bin` is on your `PATH`, or re-open your shell after installing the CLI.

## Notes

- This example is a single Soroban contract crate.

## End-to-End Walkthrough (Setup → Deployment)

1) Install Rust + Cargo (via rustup):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2) Add the WASM target:

```bash
rustup target add wasm32-unknown-unknown
```

3) Install the Stellar CLI:

```bash
cargo install stellar-cli --locked
```

4) Build and test the contract locally:

```bash
cd examples/asteroids
cargo fmt
cargo build
cargo test
```

5) Build the Soroban WASM:

```bash
stellar contract build
```

6) Deploy to Testnet (example flow):

```bash
stellar network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

stellar keys generate testnet --network testnet
stellar keys fund testnet --network testnet

stellar contract deploy \
  --wasm target/wasm32v1-none/release/asteroids.wasm \
  --source testnet \
  --network testnet
```

7) Invoke contract methods:

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source testnet \
  --network testnet \
  -- \
  init_game

stellar contract invoke \
  --id <CONTRACT_ID> \
  --source testnet \
  --network testnet \
  -- \
  thrust_ship

stellar contract invoke \
  --id <CONTRACT_ID> \
  --source testnet \
  --network testnet \
  -- \
  update_tick
```

## Verification (Jan 28, 2026)

Build:

```bash
cd examples/asteroids
cargo build
```

Soroban WASM build:

```bash
stellar contract build
```

Resulting WASM:

```
examples/asteroids/target/wasm32v1-none/release/asteroids.wasm
```

Tests:

```bash
cargo test
```

All tests pass:

```
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
```

## Testnet Deployment (Jan 28, 2026)

Contract ID:

```
CDRJQF43SCNVCYA45EXO4KZAVNMNP3L6GWR6DXH7WURUBBW5DOONJPJJ
```

Invocation transactions:

```
init_game:   df0bc4715b64cfaa8b5cdcbc09ca1cd6638b9187cdeac1780b439e08aa930699
thrust_ship: ad420bea80858147db70fcc7ea2c9a7efe151b36f3911922f2900a6d56e96f5f
update_tick: 52ac413b5b51ba8331a9c7ee5729639430eb923893231e42fc73e859114cbae5
```
