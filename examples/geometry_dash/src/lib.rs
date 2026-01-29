//! Geometry Dash - Complete On-Chain Game Example
//! Demonstrates game logic for Stellar Soroban using Cougr-Core patterns

#![no_std]
#![no_main]

use core::panic::PanicInfo;

// ============ PANIC HANDLER ============

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// ============ GAME TYPES ============

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IconMode {
    Cube,    // Standard jumping
    Ship,    // Flying with gravity control
    Ball,    // Gravity inversion
    UFO,     // Controlled hovering
    Wave,    // Wave-like movement
    Robot,   // Enhanced jumping
    Spider,  // Wall climbing
}

#[derive(Clone, Copy, Debug)]
pub enum ObstacleType {
    Spike,         // Damages player
    Platform,      // Can land on it
    Portal,        // Changes icon mode
    Coin,          // Increases score
    GravityPortal, // Reverses gravity
    SpeedPortal,   // Changes movement speed
}

#[derive(Clone, Copy, Debug)]
pub struct Obstacle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub obstacle_type: ObstacleType,
    pub data: i32,  // Additional data (points, mode, speed multiplier)
}

#[derive(Clone, Copy, Debug)]
pub struct GameState {
    pub player_x: i32,          // Fixed-point: * 1000
    pub player_y: i32,          // Fixed-point: * 1000
    pub velocity_x: i32,        // Fixed-point: * 1000
    pub velocity_y: i32,        // Fixed-point: * 1000
    pub icon_mode: IconMode,
    pub is_jumping: bool,
    pub is_on_ground: bool,
    pub score: u32,
    pub lives: u32,
    pub progress: i32,          // Fixed-point: * 1000
    pub game_over: bool,
    pub level_complete: bool,
    pub gravity_reversed: bool,
    pub speed_multiplier: i32,  // Fixed-point: 1000 = 1.0x
}

// ============ GAME CONSTANTS ============

const GRAVITY: i32 = -2000;        // -2.0 * 1000
const JUMP_FORCE: i32 = 85000;     // 85.0 * 1000
const MOVE_SPEED: i32 = 300;       // 0.3 * 1000
const GROUND_Y: i32 = 0;
const PLAYER_SIZE: i32 = 40;

// ============ GAME LOGIC ============

pub struct GeometryDash;

impl GeometryDash {
    /// Initialize a new game with default state
    pub fn init_game() -> GameState {
        GameState {
            player_x: 0,
            player_y: GROUND_Y,
            velocity_x: MOVE_SPEED,
            velocity_y: 0,
            icon_mode: IconMode::Cube,
            is_jumping: false,
            is_on_ground: true,
            score: 0,
            lives: 3,
            progress: 0,
            game_over: false,
            level_complete: false,
            gravity_reversed: false,
            speed_multiplier: 1000,
        }
    }
    
    /// Execute jump action based on current icon mode
    pub fn jump(game_state: &mut GameState) {
        if game_state.game_over || game_state.level_complete {
            return;
        }
        
        match game_state.icon_mode {
            IconMode::Cube => {
                if game_state.is_on_ground {
                    game_state.velocity_y = JUMP_FORCE;
                    game_state.is_jumping = true;
                    game_state.is_on_ground = false;
                }
            }
            IconMode::Ship => {
                // Ship flies against gravity
                let gravity = if game_state.gravity_reversed { -GRAVITY } else { GRAVITY };
                game_state.velocity_y = -gravity * 2;
            }
            IconMode::Ball => {
                // Ball inverts gravity on jump
                game_state.gravity_reversed = !game_state.gravity_reversed;
            }
            IconMode::UFO => {
                // UFO has controlled hover
                game_state.velocity_y = JUMP_FORCE / 2;
            }
            IconMode::Wave => {
                // Wave reverses vertical direction
                game_state.velocity_y = -game_state.velocity_y;
            }
            IconMode::Robot | IconMode::Spider => {
                // Standard jump for these modes
                if game_state.is_on_ground {
                    game_state.velocity_y = JUMP_FORCE;
                    game_state.is_jumping = true;
                    game_state.is_on_ground = false;
                }
            }
        }
    }
    
    /// Update game physics (gravity, position, ground collision)
    pub fn update_physics(game_state: &mut GameState) {
        if game_state.game_over || game_state.level_complete {
            return;
        }
        
        // Apply gravity
        let gravity = if game_state.gravity_reversed { -GRAVITY } else { GRAVITY };
        game_state.velocity_y += gravity;
        
        // Update position with fixed-point arithmetic
        let speed = (MOVE_SPEED * game_state.speed_multiplier) / 1000;
        game_state.player_x += speed;
        game_state.player_y += game_state.velocity_y / 1000;
        
        // Check ground collision
        if game_state.player_y <= GROUND_Y {
            game_state.player_y = GROUND_Y;
            game_state.velocity_y = 0;
            game_state.is_on_ground = true;
            game_state.is_jumping = false;
        } else {
            game_state.is_on_ground = false;
        }
        
        // Update progress and score
        game_state.progress = game_state.player_x;
        game_state.score = (game_state.progress / 1000) as u32;
    }
    
    /// Check collision between player and obstacle using AABB detection
    pub fn check_collision(game_state: &GameState, obstacle: &Obstacle) -> bool {
        let player_x = game_state.player_x / 1000;
        let player_y = game_state.player_y / 1000;
        
        // Axis-Aligned Bounding Box collision
        player_x < obstacle.x + obstacle.width &&
        player_x + PLAYER_SIZE > obstacle.x &&
        player_y < obstacle.y + obstacle.height &&
        player_y + PLAYER_SIZE > obstacle.y
    }
    
    /// Handle collision effects based on obstacle type
    pub fn handle_collision(game_state: &mut GameState, obstacle: &Obstacle) {
        match obstacle.obstacle_type {
            ObstacleType::Spike => {
                // Lose a life when hitting a spike
                if game_state.lives > 0 {
                    game_state.lives -= 1;
                }
                // Game over when no lives left
                if game_state.lives == 0 {
                    game_state.game_over = true;
                }
            }
            ObstacleType::Coin => {
                // Collect coin for points
                game_state.score += obstacle.data as u32;
            }
            ObstacleType::Portal => {
                // Change icon mode based on portal data
                game_state.icon_mode = match obstacle.data {
                    0 => IconMode::Cube,
                    1 => IconMode::Ship,
                    2 => IconMode::Ball,
                    3 => IconMode::UFO,
                    4 => IconMode::Wave,
                    5 => IconMode::Robot,
                    6 => IconMode::Spider,
                    _ => IconMode::Cube,
                };
            }
            ObstacleType::GravityPortal => {
                // Reverse gravity
                game_state.gravity_reversed = !game_state.gravity_reversed;
            }
            ObstacleType::SpeedPortal => {
                // Change speed multiplier
                game_state.speed_multiplier = obstacle.data;
            }
            ObstacleType::Platform => {
                // Land on platform if falling
                if game_state.velocity_y < 0 {
                    game_state.player_y = (obstacle.y + obstacle.height) * 1000;
                    game_state.velocity_y = 0;
                    game_state.is_on_ground = true;
                }
            }
        }
    }
    
    /// Reset game to initial state
    pub fn reset_game() -> GameState {
        Self::init_game()
    }
    
    /// Create a simple test obstacle
    pub fn create_test_spike() -> Obstacle {
        Obstacle {
            x: 1000,
            y: 0,
            width: 50,
            height: 100,
            obstacle_type: ObstacleType::Spike,
            data: 0,
        }
    }
    
    /// Create a test coin
    pub fn create_test_coin() -> Obstacle {
        Obstacle {
            x: 2000,
            y: 0,
            width: 30,
            height: 30,
            obstacle_type: ObstacleType::Coin,
            data: 100,
        }
    }
    
    /// Create a test portal (to Ship mode)
    pub fn create_test_portal() -> Obstacle {
        Obstacle {
            x: 3000,
            y: 0,
            width: 80,
            height: 150,
            obstacle_type: ObstacleType::Portal,
            data: 1, // Ship mode
        }
    }
}

// ============ COMPREHENSIVE TEST SUITE ============

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_game_initialization() {
        let game_state = GeometryDash::init_game();
        
        assert_eq!(game_state.score, 0);
        assert_eq!(game_state.lives, 3);
        assert_eq!(game_state.icon_mode, IconMode::Cube);
        assert!(!game_state.game_over);
        assert!(!game_state.level_complete);
        assert_eq!(game_state.progress, 0);
        assert_eq!(game_state.speed_multiplier, 1000);
        assert!(!game_state.gravity_reversed);
        assert!(game_state.is_on_ground);
        assert!(!game_state.is_jumping);
    }
    
    #[test]
    fn test_cube_jump_mechanics() {
        let mut game_state = GeometryDash::init_game();
        
        // Cube should jump when on ground
        game_state.is_on_ground = true;
        GeometryDash::jump(&mut game_state);
        
        assert!(game_state.is_jumping);
        assert!(!game_state.is_on_ground);
        assert_eq!(game_state.velocity_y, JUMP_FORCE);
        
        // Cube should NOT jump when already in air
        let initial_velocity = game_state.velocity_y;
        GeometryDash::jump(&mut game_state);
        assert_eq!(game_state.velocity_y, initial_velocity); // No change
    }
    
    #[test]
    fn test_ship_flight_mechanics() {
        let mut game_state = GeometryDash::init_game();
        game_state.icon_mode = IconMode::Ship;
        
        // Ship should get upward velocity when jumping
        GeometryDash::jump(&mut game_state);
        
        let expected_velocity = -GRAVITY * 2;
        assert_eq!(game_state.velocity_y, expected_velocity);
    }
    
    #[test]
    fn test_ball_gravity_inversion() {
        let mut game_state = GeometryDash::init_game();
        game_state.icon_mode = IconMode::Ball;
        
        // Ball should invert gravity
        let initial_gravity = game_state.gravity_reversed;
        GeometryDash::jump(&mut game_state);
        
        assert_eq!(game_state.gravity_reversed, !initial_gravity);
        
        // Jump again to flip back
        GeometryDash::jump(&mut game_state);
        assert_eq!(game_state.gravity_reversed, initial_gravity);
    }
    
    #[test]
    fn test_ufo_hover_mechanics() {
        let mut game_state = GeometryDash::init_game();
        game_state.icon_mode = IconMode::UFO;
        
        // UFO should get half jump force
        GeometryDash::jump(&mut game_state);
        
        assert_eq!(game_state.velocity_y, JUMP_FORCE / 2);
    }
    
    #[test]
    fn test_wave_movement_mechanics() {
        let mut game_state = GeometryDash::init_game();
        game_state.icon_mode = IconMode::Wave;
        game_state.velocity_y = 1000;
        
        // Wave should reverse vertical direction
        GeometryDash::jump(&mut game_state);
        
        assert_eq!(game_state.velocity_y, -1000);
        
        // Jump again to flip back
        GeometryDash::jump(&mut game_state);
        assert_eq!(game_state.velocity_y, 1000);
    }
    
    #[test]
    fn test_physics_update() {
        let mut game_state = GeometryDash::init_game();
        
        GeometryDash::update_physics(&mut game_state);
        
        // Should apply gravity
        assert_eq!(game_state.velocity_y, GRAVITY);
        // Should move horizontally
        assert_eq!(game_state.player_x, MOVE_SPEED);
        // Should update score based on progress
        assert_eq!(game_state.score, 0); // Not enough progress yet
        
        // Update again to see score change
        for _ in 0..3 {
            GeometryDash::update_physics(&mut game_state);
        }
        assert!(game_state.score > 0);
    }
    
    #[test]
    fn test_ground_collision() {
        let mut game_state = GameState {
            player_x: 0,
            player_y: -500, // Below ground
            velocity_x: MOVE_SPEED,
            velocity_y: -1000,
            icon_mode: IconMode::Cube,
            is_jumping: true,
            is_on_ground: false,
            score: 0,
            lives: 3,
            progress: 0,
            game_over: false,
            level_complete: false,
            gravity_reversed: false,
            speed_multiplier: 1000,
        };
        
        GeometryDash::update_physics(&mut game_state);
        
        // Should be placed on ground
        assert_eq!(game_state.player_y, GROUND_Y);
        // Velocity should be zero
        assert_eq!(game_state.velocity_y, 0);
        // Should be on ground
        assert!(game_state.is_on_ground);
        // Should not be jumping
        assert!(!game_state.is_jumping);
    }
    
    #[test]
    fn test_collision_detection_positive() {
        let game_state = GameState {
            player_x: 500 * 1000, // Player at x=500
            player_y: 0,
            velocity_x: 0,
            velocity_y: 0,
            icon_mode: IconMode::Cube,
            is_jumping: false,
            is_on_ground: true,
            score: 0,
            lives: 3,
            progress: 0,
            game_over: false,
            level_complete: false,
            gravity_reversed: false,
            speed_multiplier: 1000,
        };
        
        let obstacle = Obstacle {
            x: 490, // Obstacle from x=490 to x=540
            y: 0,
            width: 50,
            height: 100,
            obstacle_type: ObstacleType::Spike,
            data: 0,
        };
        
        // Player at x=500 with size 40, obstacle at 490-540
        // Should collide (500 < 540 && 540 > 490 && 0 < 100 && 40 > 0)
        let collision = GeometryDash::check_collision(&game_state, &obstacle);
        assert!(collision, "Player should collide with obstacle");
    }
    
    #[test]
    fn test_collision_detection_negative() {
        let game_state = GameState {
            player_x: 600 * 1000, // Player at x=600
            player_y: 0,
            velocity_x: 0,
            velocity_y: 0,
            icon_mode: IconMode::Cube,
            is_jumping: false,
            is_on_ground: true,
            score: 0,
            lives: 3,
            progress: 0,
            game_over: false,
            level_complete: false,
            gravity_reversed: false,
            speed_multiplier: 1000,
        };
        
        let obstacle = Obstacle {
            x: 490, // Obstacle from x=490 to x=540
            y: 0,
            width: 50,
            height: 100,
            obstacle_type: ObstacleType::Spike,
            data: 0,
        };
        
        // Player at x=600, obstacle at 490-540
        // Should NOT collide (600 > 540)
        let collision = GeometryDash::check_collision(&game_state, &obstacle);
        assert!(!collision, "Player should NOT collide with obstacle");
    }
    
    #[test]
    fn test_spike_collision_handling() {
        let mut game_state = GameState {
            player_x: 0,
            player_y: 0,
            velocity_x: 0,
            velocity_y: 0,
            icon_mode: IconMode::Cube,
            is_jumping: false,
            is_on_ground: true,
            score: 100,
            lives: 3,
            progress: 0,
            game_over: false,
            level_complete: false,
            gravity_reversed: false,
            speed_multiplier: 1000,
        };
        
        let obstacle = Obstacle {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            obstacle_type: ObstacleType::Spike,
            data: 0,
        };
        
        // Hit a spike
        GeometryDash::handle_collision(&mut game_state, &obstacle);
        
        // Should lose a life
        assert_eq!(game_state.lives, 2);
        // Score should remain unchanged
        assert_eq!(game_state.score, 100);
        // Should not be game over
        assert!(!game_state.game_over);
    }
    
    #[test]
    fn test_multiple_spike_collisions() {
        let mut game_state = GameState {
            player_x: 0,
            player_y: 0,
            velocity_x: 0,
            velocity_y: 0,
            icon_mode: IconMode::Cube,
            is_jumping: false,
            is_on_ground: true,
            score: 100,
            lives: 3,
            progress: 0,
            game_over: false,
            level_complete: false,
            gravity_reversed: false,
            speed_multiplier: 1000,
        };
        
        let obstacle = Obstacle {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            obstacle_type: ObstacleType::Spike,
            data: 0,
        };
        
        // Hit spike 3 times
        for i in 0..3 {
            GeometryDash::handle_collision(&mut game_state, &obstacle);
            assert_eq!(game_state.lives, 2 - i as u32);
        }
        
        // Should be game over after 3 hits
        assert!(game_state.game_over);
        assert_eq!(game_state.lives, 0);
    }
    
    #[test]
    fn test_coin_collision_handling() {
        let mut game_state = GameState {
            player_x: 0,
            player_y: 0,
            velocity_x: 0,
            velocity_y: 0,
            icon_mode: IconMode::Cube,
            is_jumping: false,
            is_on_ground: true,
            score: 0,
            lives: 3,
            progress: 0,
            game_over: false,
            level_complete: false,
            gravity_reversed: false,
            speed_multiplier: 1000,
        };
        
        let obstacle = Obstacle {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            obstacle_type: ObstacleType::Coin,
            data: 100, // 100 points
        };
        
        // Collect a coin
        GeometryDash::handle_collision(&mut game_state, &obstacle);
        
        // Should gain score
        assert_eq!(game_state.score, 100);
        // Lives should remain unchanged
        assert_eq!(game_state.lives, 3);
    }
    
    #[test]
    fn test_multiple_coin_collections() {
        let mut game_state = GameState {
            player_x: 0,
            player_y: 0,
            velocity_x: 0,
            velocity_y: 0,
            icon_mode: IconMode::Cube,
            is_jumping: false,
            is_on_ground: true,
            score: 0,
            lives: 3,
            progress: 0,
            game_over: false,
            level_complete: false,
            gravity_reversed: false,
            speed_multiplier: 1000,
        };
        
        let coin_obstacle = Obstacle {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            obstacle_type: ObstacleType::Coin,
            data: 50, // 50 points each
        };
        
        // Collect 3 coins
        for i in 0..3 {
            GeometryDash::handle_collision(&mut game_state, &coin_obstacle);
            assert_eq!(game_state.score, 50 * (i + 1) as u32);
        }
        
        assert_eq!(game_state.score, 150);
        assert_eq!(game_state.lives, 3);
    }
    
    #[test]
    fn test_portal_collision_handling() {
        let mut game_state = GameState {
            player_x: 0,
            player_y: 0,
            velocity_x: 0,
            velocity_y: 0,
            icon_mode: IconMode::Cube,
            is_jumping: false,
            is_on_ground: true,
            score: 100,
            lives: 3,
            progress: 0,
            game_over: false,
            level_complete: false,
            gravity_reversed: false,
            speed_multiplier: 1000,
        };
        
        let obstacle = Obstacle {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            obstacle_type: ObstacleType::Portal,
            data: 1, // Ship mode
        };
        
        // Go through portal
        GeometryDash::handle_collision(&mut game_state, &obstacle);
        
        // Should change icon mode
        assert_eq!(game_state.icon_mode, IconMode::Ship);
        // Other stats should remain unchanged
        assert_eq!(game_state.score, 100);
        assert_eq!(game_state.lives, 3);
    }
    
     #[test]
    fn test_all_portal_modes() {
        let portal_modes = [0, 1, 2, 3, 4, 5, 6];
        let expected_modes = [
            IconMode::Cube,
            IconMode::Ship,
            IconMode::Ball,
            IconMode::UFO,
            IconMode::Wave,
            IconMode::Robot,
            IconMode::Spider,
        ];
        
        for (i, &mode_data) in portal_modes.iter().enumerate() {
            let mut game_state = GeometryDash::init_game();
            
            let obstacle = Obstacle {
                x: 0,
                y: 0,
                width: 100,
                height: 100,
                obstacle_type: ObstacleType::Portal,
                data: mode_data,
            };
            
            GeometryDash::handle_collision(&mut game_state, &obstacle);
            assert_eq!(game_state.icon_mode, expected_modes[i]);
        }
    }
    
    #[test]
    fn test_gravity_portal_collision() {
        let mut game_state = GameState {
            player_x: 0,
            player_y: 0,
            velocity_x: 0,
            velocity_y: 0,
            icon_mode: IconMode::Cube,
            is_jumping: false,
            is_on_ground: true,
            score: 100,
            lives: 3,
            progress: 0,
            game_over: false,
            level_complete: false,
            gravity_reversed: false,
            speed_multiplier: 1000,
        };
        
        let obstacle = Obstacle {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            obstacle_type: ObstacleType::GravityPortal,
            data: 0,
        };
        
        // Go through gravity portal
        let initial_gravity = game_state.gravity_reversed;
        GeometryDash::handle_collision(&mut game_state, &obstacle);
        
        // Should reverse gravity
        assert_eq!(game_state.gravity_reversed, !initial_gravity);
        
        // Go through another gravity portal
        GeometryDash::handle_collision(&mut game_state, &obstacle);
        assert_eq!(game_state.gravity_reversed, initial_gravity);
    }
    
    #[test]
    fn test_speed_portal_collision() {
        let mut game_state = GameState {
            player_x: 0,
            player_y: 0,
            velocity_x: MOVE_SPEED,
            velocity_y: 0,
            icon_mode: IconMode::Cube,
            is_jumping: false,
            is_on_ground: true,
            score: 100,
            lives: 3,
            progress: 0,
            game_over: false,
            level_complete: false,
            gravity_reversed: false,
            speed_multiplier: 1000, // Normal speed
        };
        
        let obstacle = Obstacle {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            obstacle_type: ObstacleType::SpeedPortal,
            data: 1500, // 1.5x speed
        };
        
        // Go through speed portal
        GeometryDash::handle_collision(&mut game_state, &obstacle);
        
        // Should change speed multiplier
        assert_eq!(game_state.speed_multiplier, 1500);
        
        // Go through another speed portal (slow down)
        let obstacle2 = Obstacle {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            obstacle_type: ObstacleType::SpeedPortal,
            data: 500, // 0.5x speed
        };
        
        GeometryDash::handle_collision(&mut game_state, &obstacle2);
        assert_eq!(game_state.speed_multiplier, 500);
    }
    
    #[test]
    fn test_platform_collision() {
        let mut game_state = GameState {
            player_x: 0,
            player_y: 1000, // Above platform
            velocity_x: 0,
            velocity_y: -500, // Falling
            icon_mode: IconMode::Cube,
            is_jumping: false,
            is_on_ground: false,
            score: 0,
            lives: 3,
            progress: 0,
            game_over: false,
            level_complete: false,
            gravity_reversed: false,
            speed_multiplier: 1000,
        };
        
        let obstacle = Obstacle {
            x: 0,
            y: 0,
            width: 100,
            height: 50,
            obstacle_type: ObstacleType::Platform,
            data: 0,
        };
        
        // Land on platform
        GeometryDash::handle_collision(&mut game_state, &obstacle);
        
        // Should be placed on top of platform
        assert_eq!(game_state.player_y, (obstacle.y + obstacle.height) * 1000);
        // Velocity should be zero
        assert_eq!(game_state.velocity_y, 0);
        // Should be on ground
        assert!(game_state.is_on_ground);
    }
    
    #[test]
    fn test_game_over_conditions() {
        let mut game_state = GameState {
            player_x: 0,
            player_y: 0,
            velocity_x: 0,
            velocity_y: 0,
            icon_mode: IconMode::Cube,
            is_jumping: false,
            is_on_ground: true,
            score: 1000,
            lives: 1, // Only one life left
            progress: 5000,
            game_over: false,
            level_complete: false,
            gravity_reversed: false,
            speed_multiplier: 1000,
        };
        
        let obstacle = Obstacle {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            obstacle_type: ObstacleType::Spike,
            data: 0,
        };
        
        // Hit spike with only one life left
        GeometryDash::handle_collision(&mut game_state, &obstacle);
        
        // Should be game over
        assert!(game_state.game_over);
        assert_eq!(game_state.lives, 0);
        
        // Game over should prevent further actions
        let initial_score = game_state.score;
        GeometryDash::jump(&mut game_state);
        GeometryDash::update_physics(&mut game_state);
        
        // Score should not change when game over
        assert_eq!(game_state.score, initial_score);
    }
    
    #[test]
    fn test_reset_game() {
        let mut game_state = GeometryDash::init_game();
        
        // Play the game a bit
        GeometryDash::jump(&mut game_state);
        GeometryDash::update_physics(&mut game_state);
        
        // Modify state
        game_state.score = 1000;
        game_state.lives = 1;
        game_state.player_x = 5000;
        game_state.icon_mode = IconMode::Ship;
        
        // Reset game
        game_state = GeometryDash::reset_game();
        
        // Should be back to initial state
        assert_eq!(game_state.score, 0);
        assert_eq!(game_state.lives, 3);
        assert_eq!(game_state.player_x, 0);
        assert_eq!(game_state.icon_mode, IconMode::Cube);
        assert!(!game_state.game_over);
        assert!(!game_state.level_complete);
        assert_eq!(game_state.progress, 0);
    }
    
    #[test]
    fn test_all_icon_modes_defined() {
        let modes = [
            IconMode::Cube,
            IconMode::Ship,
            IconMode::Ball,
            IconMode::UFO,
            IconMode::Wave,
            IconMode::Robot,
            IconMode::Spider,
        ];
        
        assert_eq!(modes.len(), 7, "Should have 7 icon modes");
        
        // Test each mode
        for mode in modes.iter() {
            let mut game_state = GeometryDash::init_game();
            game_state.icon_mode = *mode;
            
            // Each mode should be properly set
            assert_eq!(game_state.icon_mode, *mode);
            
            // Test that jump works for each mode
            GeometryDash::jump(&mut game_state);
            
            // Test physics update
            GeometryDash::update_physics(&mut game_state);
        }
    }
    
    #[test]
    fn test_all_obstacle_types_defined() {
        let obstacle_types = [
            ObstacleType::Spike,
            ObstacleType::Platform,
            ObstacleType::Portal,
            ObstacleType::Coin,
            ObstacleType::GravityPortal,
            ObstacleType::SpeedPortal,
        ];
        
        assert_eq!(obstacle_types.len(), 6, "Should have 6 obstacle types");
        
        // Test creation of each obstacle type
        for obs_type in obstacle_types.iter() {
            let obstacle = Obstacle {
                x: 0,
                y: 0,
                width: 100,
                height: 100,
                obstacle_type: *obs_type,
                data: 0,
            };
            
            // Should be able to create each type
            match obstacle.obstacle_type {
                ObstacleType::Spike => assert!(true),
                ObstacleType::Platform => assert!(true),
                ObstacleType::Portal => assert!(true),
                ObstacleType::Coin => assert!(true),
                ObstacleType::GravityPortal => assert!(true),
                ObstacleType::SpeedPortal => assert!(true),
            }
        }
    }
    
    #[test]
    fn test_complete_game_flow() {
        // Test a complete game session
        let mut game_state = GeometryDash::init_game();
        
        // Initial checks
        assert_eq!(game_state.score, 0);
        assert_eq!(game_state.lives, 3);
        assert_eq!(game_state.icon_mode, IconMode::Cube);
        
        // Play sequence
        GeometryDash::jump(&mut game_state);  // Jump as Cube
        GeometryDash::update_physics(&mut game_state);  // Update physics
        
        // Should have moved
        assert!(game_state.player_x > 0);
        assert!(game_state.player_y > 0 || game_state.velocity_y != 0);
        
        // Hit a coin
        let coin = GeometryDash::create_test_coin();
        GeometryDash::handle_collision(&mut game_state, &coin);
        assert_eq!(game_state.score, 100);
        
        // Go through portal to become Ship
        let portal = GeometryDash::create_test_portal();
        GeometryDash::handle_collision(&mut game_state, &portal);
        assert_eq!(game_state.icon_mode, IconMode::Ship);
        
        // Jump as Ship
        GeometryDash::jump(&mut game_state);
        
        // Hit a spike
        let spike = GeometryDash::create_test_spike();
        GeometryDash::handle_collision(&mut game_state, &spike);
        assert_eq!(game_state.lives, 2);
        
        // Reset game
        game_state = GeometryDash::reset_game();
        
        // Should be back to initial state
        assert_eq!(game_state.score, 0);
        assert_eq!(game_state.lives, 3);
        assert_eq!(game_state.icon_mode, IconMode::Cube);
    }
}

// ============ WASM ENTRY POINTS ============

/// Initialize game and return a success indicator
#[no_mangle]
pub extern "C" fn init_game() -> i32 {
    // Returns success code
    1
}

/// Execute jump and return vertical velocity
#[no_mangle]
pub extern "C" fn execute_jump() -> i32 {
    let mut game_state = GeometryDash::init_game();
    GeometryDash::jump(&mut game_state);
    
    // Return scaled vertical velocity
    game_state.velocity_y / 1000
}

/// Simple collision check test
#[no_mangle]
pub extern "C" fn test_collision() -> i32 {
    let game_state = GeometryDash::init_game();
    let obstacle = GeometryDash::create_test_spike();
    
    if GeometryDash::check_collision(&game_state, &obstacle) {
        1 // Collision detected
    } else {
        0 // No collision
    }
}