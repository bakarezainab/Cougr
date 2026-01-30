# Add Arkanoid Example Game

## Description

This PR adds a comprehensive implementation of the Arkanoid game to the `examples/` directory to demonstrate the capabilities of the Cougr-Core ECS framework on Soroban. The implementation serves as a functional reference for building complex on-chain game logic with persistent state and component-based architecture.

## Changes

- **New Example**: Added `examples/arkanoid` with full project structure.
- **Game Logic**: Implemented core mechanics including:
    - `PaddleComponent`: Player controlled paddle with bounds checking.
    - `BallComponent`: Physics-based ball movement and collision.
    - `BricksComponent`: Grid of breakable bricks using optimized storage.
    - `ScoreComponent`: Tracking score, lives, and game state.
- **Systems**: Implemented ECS systems for Physics, Collision, and Scoring.
- **Testing**: Added comprehensive unit test suite covering initialization, movement, collisions, and win/loss conditions.
- **CI/CD**: Added GitHub Actions workflow (`arkanoid.yml`) for automated testing and building.
- **Documentation**: Provided detailed README with setup, deployment, and usage instructions.

## Technical Details

- Uses `cougr-core` ECS pattern for modularity.
- Implements custom `BricksComponent` using `soroban_sdk::Vec` for compatibility with Soroban storage (replacing unsupported large fixed arrays).
- Handles complex collision logic for walls, paddle, and bricks within a deterministic tick update function.
- Optimizes storage access by bundling state updates.

## Verification

- **Tests**: All 17 unit tests passed locally.
- **Build**: Compiles successfully for `wasm32-unknown-unknown` and `wasm32v1-none` targets.
- **Linting**: Code is formatted and passes `cargo clippy`.

## Checklist

- [x] Code compiles without warnings
- [x] All tests pass
- [x] Documentation updated
- [x] CI workflow added
