# Asteroids (Soroban Example)

This folder bootstraps a Soroban workspace that will host the on-chain Asteroids example contract.

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello-world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
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

- New Soroban contracts can be added under `contracts/`, each in its own directory.
- Contracts have their own `Cargo.toml` files and share dependencies from the workspace `Cargo.toml`.
