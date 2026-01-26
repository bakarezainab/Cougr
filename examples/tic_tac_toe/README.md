# Tic Tac Toe On-Chain Game

A fully functional Tic Tac Toe game implemented as a Soroban smart contract on the Stellar blockchain, demonstrating the use of the **Cougr-Core** ECS (Entity Component System) framework for on-chain gaming.

## Overview

This example showcases how to build on-chain game logic using Soroban smart contracts with cougr-core's ECS patterns:

- **ComponentTrait Integration**: Game components implement cougr-core's `ComponentTrait` for type-safe serialization
- **ECS Architecture**: Game state is organized into distinct components (Board, Players, GameState)
- **System Pattern**: Game logic is split into discrete systems (Validation, Execution, Win Detection, Turn Management)
- **Entity Management**: Components track entity IDs following cougr-core patterns

## Cougr-Core Integration

This implementation demonstrates how cougr-core simplifies on-chain game development:

### ComponentTrait

All game components implement `cougr_core::component::ComponentTrait`:

```rust
impl ComponentTrait for BoardComponent {
    fn component_type() -> Symbol {
        symbol_short!("board")
    }

    fn serialize(&self, env: &Env) -> Bytes { ... }
    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> { ... }
}
```

**Benefits over vanilla Soroban:**

- Type-safe serialization with compile-time checking
- Standardized component interface across all games
- Automatic handling of byte packing/unpacking

### ECS Systems

Game logic follows cougr-core's System pattern:

```rust
// Validation System - queries PlayerComponent and GameStateComponent
fn validation_system(world: &ECSWorldState, player: &Address, position: u32) -> (bool, Symbol)

// Execution System - updates BoardComponent
fn execution_system(world: &mut ECSWorldState, position: u32)

// Win Detection System - checks BoardComponent, updates GameStateComponent
fn win_detection_system(world: &mut ECSWorldState)

// Turn Management System - updates GameStateComponent
fn turn_system(world: &mut ECSWorldState)
```

**Benefits:**

- Separation of concerns makes code maintainable
- Each system is independently testable
- Easy to extend with new systems (e.g., AI opponent, undo)

### Entity Tracking

Components include entity IDs for proper ECS relationships:

```rust
pub struct BoardComponent {
    pub cells: Vec<u32>,
    pub entity_id: u32,  // Tracks which entity owns this component
}
```

## Features

- Two-player gameplay using Stellar addresses
- Turn-based mechanics (X always goes first)
- Win detection for all 8 patterns (3 rows, 3 columns, 2 diagonals)
- Draw detection when board is full
- Move validation (position bounds, occupied cells, turn order)
- Game reset functionality
- 33 comprehensive unit tests

## Prerequisites

- **Rust** (1.70.0 or newer): [Install Rust](https://www.rust-lang.org/tools/install)
- **WASM target**: `rustup target add wasm32-unknown-unknown`
- **Stellar CLI** (optional): `cargo install stellar-cli`

## Building

```bash
# Build for testing
cargo build

# Build WASM contract
cargo build --target wasm32-unknown-unknown --release
```

## Testing

```bash
cargo test
```

All 33 tests should pass, covering:

- Game initialization
- Valid/invalid moves
- All 8 winning patterns
- Draw detection
- Game over handling
- Reset functionality

## Contract API

### `init_game(player_x: Address, player_o: Address) -> GameState`

Initialize a new game with two players.

### `make_move(player: Address, position: u32) -> MoveResult`

Make a move. Position is 0-8 (row-major order):

```text
0 | 1 | 2
---------
3 | 4 | 5
---------
6 | 7 | 8
```

### `get_state() -> GameState`

Get current game state.

### `is_valid_move(position: u32) -> bool`

Check if a move is valid.

### `get_winner() -> Option<Address>`

Get winner's address if game is over.

### `reset_game() -> GameState`

Reset with same players.

## Data Structures

### GameState

```rust
pub struct GameState {
    pub cells: Vec<u32>,      // 0=Empty, 1=X, 2=O
    pub player_x: Address,
    pub player_o: Address,
    pub is_x_turn: bool,
    pub move_count: u32,
    pub status: u32,          // 0=InProgress, 1=XWins, 2=OWins, 3=Draw
}
```

### MoveResult

```rust
pub struct MoveResult {
    pub success: bool,
    pub game_state: GameState,
    pub message: Symbol,      // ok, invalid, occupied, notturn, notplay, gameover
}
```

## Architecture

```text
ECSWorldState
├── BoardComponent     (entity_id: 0)
│   └── cells: Vec<u32> [9 cells]
├── PlayerComponent    (entity_id: 1)
│   ├── player_x: Address
│   └── player_o: Address
├── GameStateComponent (entity_id: 2)
│   ├── is_x_turn: bool
│   ├── move_count: u32
│   └── status: u32
└── next_entity_id: u32

Systems:
├── ValidationSystem   → Checks game rules
├── ExecutionSystem    → Applies moves
├── WinDetectionSystem → Checks 8 patterns
└── TurnSystem         → Switches turns
```

## Deployment

```bash
# Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/tic_tac_toe.wasm \
  --source <secret-key> \
  --network testnet

# Initialize game
stellar contract invoke \
  --id <contract-id> \
  --network testnet \
  -- init_game \
  --player_x <address-x> \
  --player_o <address-o>
```

## License

MIT OR Apache-2.0

## Resources

- [Cougr Repository](https://github.com/salazarsebas/Cougr)
- [Soroban Documentation](https://developers.stellar.org/docs/build/smart-contracts)
