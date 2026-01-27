#![no_std]

use cougr_core::component::ComponentTrait;
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Bytes, Env, Symbol, Vec};

/// Board component - stores the 3x3 game board state (0=Empty, 1=X, 2=O)
#[contracttype]
#[derive(Clone, Debug)]
pub struct BoardComponent {
    pub cells: Vec<u32>,
    pub entity_id: u32,
}

impl BoardComponent {
    pub fn new(env: &Env, entity_id: u32) -> Self {
        let mut cells = Vec::new(env);
        for _ in 0..9 {
            cells.push_back(0u32);
        }
        Self { cells, entity_id }
    }
}

impl ComponentTrait for BoardComponent {
    fn component_type() -> Symbol {
        symbol_short!("board")
    }

    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        bytes.append(&Bytes::from_array(env, &self.entity_id.to_be_bytes()));
        for i in 0..9 {
            let cell = self.cells.get(i).unwrap_or(0);
            bytes.append(&Bytes::from_array(env, &cell.to_be_bytes()));
        }
        bytes
    }

    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 40 {
            return None;
        }
        let entity_id = u32::from_be_bytes([
            data.get(0).unwrap(),
            data.get(1).unwrap(),
            data.get(2).unwrap(),
            data.get(3).unwrap(),
        ]);
        let mut cells = Vec::new(env);
        for i in 0..9 {
            let offset = 4 + (i * 4) as u32;
            let cell = u32::from_be_bytes([
                data.get(offset).unwrap(),
                data.get(offset + 1).unwrap(),
                data.get(offset + 2).unwrap(),
                data.get(offset + 3).unwrap(),
            ]);
            cells.push_back(cell);
        }
        Some(Self { cells, entity_id })
    }
}

/// Player component - stores both players' addresses
#[contracttype]
#[derive(Clone, Debug)]
pub struct PlayerComponent {
    pub player_x: Address,
    pub player_o: Address,
    pub entity_id: u32,
}

impl PlayerComponent {
    pub fn new(player_x: Address, player_o: Address, entity_id: u32) -> Self {
        Self { player_x, player_o, entity_id }
    }
}

/// Game state component (status: 0=InProgress, 1=XWins, 2=OWins, 3=Draw)
#[contracttype]
#[derive(Clone, Debug)]
pub struct GameStateComponent {
    pub is_x_turn: bool,
    pub move_count: u32,
    pub status: u32,
    pub entity_id: u32,
}

impl GameStateComponent {
    pub fn new(entity_id: u32) -> Self {
        Self {
            is_x_turn: true,
            move_count: 0,
            status: 0,
            entity_id,
        }
    }
}

impl ComponentTrait for GameStateComponent {
    fn component_type() -> Symbol {
        symbol_short!("gstate")
    }

    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        bytes.append(&Bytes::from_array(env, &self.entity_id.to_be_bytes()));
        bytes.append(&Bytes::from_array(env, &[if self.is_x_turn { 1 } else { 0 }]));
        bytes.append(&Bytes::from_array(env, &self.move_count.to_be_bytes()));
        bytes.append(&Bytes::from_array(env, &self.status.to_be_bytes()));
        bytes
    }

    fn deserialize(_env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 13 {
            return None;
        }
        let entity_id = u32::from_be_bytes([
            data.get(0).unwrap(),
            data.get(1).unwrap(),
            data.get(2).unwrap(),
            data.get(3).unwrap(),
        ]);
        let is_x_turn = data.get(4).unwrap() != 0;
        let move_count = u32::from_be_bytes([
            data.get(5).unwrap(),
            data.get(6).unwrap(),
            data.get(7).unwrap(),
            data.get(8).unwrap(),
        ]);
        let status = u32::from_be_bytes([
            data.get(9).unwrap(),
            data.get(10).unwrap(),
            data.get(11).unwrap(),
            data.get(12).unwrap(),
        ]);
        Some(Self { is_x_turn, move_count, status, entity_id })
    }
}

/// ECS World State - stores all game entities and components
#[contracttype]
#[derive(Clone, Debug)]
pub struct ECSWorldState {
    pub board: BoardComponent,
    pub players: PlayerComponent,
    pub game_state: GameStateComponent,
    pub next_entity_id: u32,
}

/// External game state for API consumers
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameState {
    pub cells: Vec<u32>,
    pub player_x: Address,
    pub player_o: Address,
    pub is_x_turn: bool,
    pub move_count: u32,
    pub status: u32,
}

/// Move result returned after each move
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MoveResult {
    pub success: bool,
    pub game_state: GameState,
    pub message: Symbol,
}

const WORLD_KEY: Symbol = symbol_short!("WORLD");

#[contract]
pub struct TicTacToeContract;

#[contractimpl]
impl TicTacToeContract {
    /// Initialize a new game with two players
    pub fn init_game(env: Env, player_x: Address, player_o: Address) -> GameState {
        let mut next_entity_id = 0u32;

        let board = BoardComponent::new(&env, next_entity_id);
        next_entity_id += 1;

        let players = PlayerComponent::new(player_x.clone(), player_o.clone(), next_entity_id);
        next_entity_id += 1;

        let game_state = GameStateComponent::new(next_entity_id);
        next_entity_id += 1;

        let world_state = ECSWorldState {
            board,
            players,
            game_state,
            next_entity_id,
        };

        env.storage().instance().set(&WORLD_KEY, &world_state);
        Self::to_game_state(&env, &world_state)
    }

    /// Make a move on the board (position 0-8)
    pub fn make_move(env: Env, player: Address, position: u32) -> MoveResult {
        let mut world_state: ECSWorldState = env
            .storage()
            .instance()
            .get(&WORLD_KEY)
            .unwrap_or_else(|| panic!("Game not initialized"));

        let validation = Self::validation_system(&world_state, &player, position);
        if !validation.0 {
            return MoveResult {
                success: false,
                game_state: Self::to_game_state(&env, &world_state),
                message: validation.1,
            };
        }

        Self::execution_system(&mut world_state, position);
        Self::win_detection_system(&mut world_state);
        Self::turn_system(&mut world_state);

        env.storage().instance().set(&WORLD_KEY, &world_state);

        MoveResult {
            success: true,
            game_state: Self::to_game_state(&env, &world_state),
            message: symbol_short!("ok"),
        }
    }

    /// Get the current game state
    pub fn get_state(env: Env) -> GameState {
        let world_state: ECSWorldState = env
            .storage()
            .instance()
            .get(&WORLD_KEY)
            .unwrap_or_else(|| panic!("Game not initialized"));

        Self::to_game_state(&env, &world_state)
    }

    /// Check if a move is valid
    pub fn is_valid_move(env: Env, position: u32) -> bool {
        if position >= 9 {
            return false;
        }

        let world_state: ECSWorldState = env
            .storage()
            .instance()
            .get(&WORLD_KEY)
            .unwrap_or_else(|| panic!("Game not initialized"));

        if world_state.game_state.status != 0 {
            return false;
        }

        world_state.board.cells.get(position).unwrap_or(1) == 0
    }

    /// Get the winner's address if game is over
    pub fn get_winner(env: Env) -> Option<Address> {
        let world_state: ECSWorldState = env
            .storage()
            .instance()
            .get(&WORLD_KEY)
            .unwrap_or_else(|| panic!("Game not initialized"));

        match world_state.game_state.status {
            1 => Some(world_state.players.player_x),
            2 => Some(world_state.players.player_o),
            _ => None,
        }
    }

    /// Reset the game with the same players
    pub fn reset_game(env: Env) -> GameState {
        let world_state: ECSWorldState = env
            .storage()
            .instance()
            .get(&WORLD_KEY)
            .unwrap_or_else(|| panic!("Game not initialized"));

        Self::init_game(env, world_state.players.player_x, world_state.players.player_o)
    }

    fn validation_system(world: &ECSWorldState, player: &Address, position: u32) -> (bool, Symbol) {
        if world.game_state.status != 0 {
            return (false, symbol_short!("gameover"));
        }

        if position >= 9 {
            return (false, symbol_short!("invalid"));
        }

        let is_player_x = *player == world.players.player_x;
        let is_player_o = *player == world.players.player_o;

        if !is_player_x && !is_player_o {
            return (false, symbol_short!("notplay"));
        }

        if world.game_state.is_x_turn && !is_player_x {
            return (false, symbol_short!("notturn"));
        }
        if !world.game_state.is_x_turn && !is_player_o {
            return (false, symbol_short!("notturn"));
        }

        let cell = world.board.cells.get(position).unwrap_or(0);
        if cell != 0 {
            return (false, symbol_short!("occupied"));
        }

        (true, symbol_short!("ok"))
    }

    fn execution_system(world: &mut ECSWorldState, position: u32) {
        let cell_value = if world.game_state.is_x_turn { 1u32 } else { 2u32 };
        world.board.cells.set(position, cell_value);
        world.game_state.move_count += 1;
    }

    fn win_detection_system(world: &mut ECSWorldState) {
        let cells = &world.board.cells;

        let patterns: [[u32; 3]; 8] = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8],
            [0, 3, 6], [1, 4, 7], [2, 5, 8],
            [0, 4, 8], [2, 4, 6],
        ];

        for pattern in patterns.iter() {
            let a = cells.get(pattern[0]).unwrap_or(0);
            let b = cells.get(pattern[1]).unwrap_or(0);
            let c = cells.get(pattern[2]).unwrap_or(0);

            if a != 0 && a == b && b == c {
                world.game_state.status = a;
                return;
            }
        }

        if world.game_state.move_count >= 9 {
            world.game_state.status = 3;
        }
    }

    fn turn_system(world: &mut ECSWorldState) {
        if world.game_state.status == 0 {
            world.game_state.is_x_turn = !world.game_state.is_x_turn;
        }
    }

    fn to_game_state(env: &Env, world: &ECSWorldState) -> GameState {
        let mut cells = Vec::new(env);
        for i in 0..9 {
            cells.push_back(world.board.cells.get(i).unwrap_or(0));
        }

        GameState {
            cells,
            player_x: world.players.player_x.clone(),
            player_o: world.players.player_o.clone(),
            is_x_turn: world.game_state.is_x_turn,
            move_count: world.game_state.move_count,
            status: world.game_state.status,
        }
    }
}

#[cfg(test)]
mod test;
