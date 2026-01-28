use soroban_sdk::{contracttype, Bytes, Env};

/// ComponentTrait from cougr-core
/// Components must implement serialization for on-chain storage
pub trait ComponentTrait {
    fn serialize(&self, env: &Env) -> Bytes;
    fn deserialize(env: &Env, data: &Bytes) -> Option<Self>
    where
        Self: Sized;
}

/// Position component - represents a point on the grid
#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl ComponentTrait for Position {
    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        let x_bytes = Bytes::from_array(env, &self.x.to_be_bytes());
        let y_bytes = Bytes::from_array(env, &self.y.to_be_bytes());
        bytes.append(&x_bytes);
        bytes.append(&y_bytes);
        bytes
    }

    fn deserialize(_env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 8 {
            return None;
        }
        let x = i32::from_be_bytes([
            data.get(0).unwrap(),
            data.get(1).unwrap(),
            data.get(2).unwrap(),
            data.get(3).unwrap(),
        ]);
        let y = i32::from_be_bytes([
            data.get(4).unwrap(),
            data.get(5).unwrap(),
            data.get(6).unwrap(),
            data.get(7).unwrap(),
        ]);
        Some(Self { x, y })
    }
}

/// Direction enum for snake movement
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    pub fn to_u8(self) -> u8 {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Direction::Up),
            1 => Some(Direction::Down),
            2 => Some(Direction::Left),
            3 => Some(Direction::Right),
            _ => None,
        }
    }

    /// Check if two directions are opposite
    pub fn is_opposite(&self, other: &Direction) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }

    /// Get the delta movement for this direction
    pub fn delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

/// Direction component - stores the current movement direction
#[derive(Clone, Debug)]
pub struct DirectionComponent {
    pub direction: Direction,
}

impl DirectionComponent {
    pub fn new(direction: Direction) -> Self {
        Self { direction }
    }
}

impl ComponentTrait for DirectionComponent {
    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        bytes.append(&Bytes::from_array(env, &[self.direction.to_u8()]));
        bytes
    }

    fn deserialize(_env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 1 {
            return None;
        }
        let direction = Direction::from_u8(data.get(0).unwrap())?;
        Some(Self { direction })
    }
}

/// Snake head marker component - identifies the head entity
#[derive(Clone, Debug)]
pub struct SnakeHead;

impl ComponentTrait for SnakeHead {
    fn serialize(&self, env: &Env) -> Bytes {
        Bytes::from_array(env, &[1])
    }

    fn deserialize(_env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 1 {
            return None;
        }
        Some(Self)
    }
}

/// Snake body segment component - stores the segment index (0 = closest to head)
#[derive(Clone, Debug)]
pub struct SnakeSegment {
    pub index: u32,
}

impl SnakeSegment {
    pub fn new(index: u32) -> Self {
        Self { index }
    }
}

impl ComponentTrait for SnakeSegment {
    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        bytes.append(&Bytes::from_array(env, &self.index.to_be_bytes()));
        bytes
    }

    fn deserialize(_env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 4 {
            return None;
        }
        let index = u32::from_be_bytes([
            data.get(0).unwrap(),
            data.get(1).unwrap(),
            data.get(2).unwrap(),
            data.get(3).unwrap(),
        ]);
        Some(Self { index })
    }
}

/// Food marker component - identifies food entities
#[derive(Clone, Debug)]
pub struct Food;

impl ComponentTrait for Food {
    fn serialize(&self, env: &Env) -> Bytes {
        Bytes::from_array(env, &[1])
    }

    fn deserialize(_env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 1 {
            return None;
        }
        Some(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_serialization() {
        let env = Env::default();
        let pos = Position::new(10, 20);

        let serialized = pos.serialize(&env);
        let deserialized = Position::deserialize(&env, &serialized).unwrap();

        assert_eq!(pos.x, deserialized.x);
        assert_eq!(pos.y, deserialized.y);
    }

    #[test]
    fn test_direction_serialization() {
        let env = Env::default();
        let dir = DirectionComponent::new(Direction::Right);

        let serialized = dir.serialize(&env);
        let deserialized = DirectionComponent::deserialize(&env, &serialized).unwrap();

        assert_eq!(dir.direction, deserialized.direction);
    }

    #[test]
    fn test_direction_opposite() {
        assert!(Direction::Up.is_opposite(&Direction::Down));
        assert!(Direction::Down.is_opposite(&Direction::Up));
        assert!(Direction::Left.is_opposite(&Direction::Right));
        assert!(Direction::Right.is_opposite(&Direction::Left));
        assert!(!Direction::Up.is_opposite(&Direction::Left));
        assert!(!Direction::Up.is_opposite(&Direction::Right));
    }

    #[test]
    fn test_snake_head_serialization() {
        let env = Env::default();
        let head = SnakeHead;

        let serialized = head.serialize(&env);
        let deserialized = SnakeHead::deserialize(&env, &serialized).unwrap();

        assert!(matches!(deserialized, SnakeHead));
    }

    #[test]
    fn test_snake_segment_serialization() {
        let env = Env::default();
        let segment = SnakeSegment::new(5);

        let serialized = segment.serialize(&env);
        let deserialized = SnakeSegment::deserialize(&env, &serialized).unwrap();

        assert_eq!(segment.index, deserialized.index);
    }

    #[test]
    fn test_food_serialization() {
        let env = Env::default();
        let food = Food;

        let serialized = food.serialize(&env);
        let deserialized = Food::deserialize(&env, &serialized).unwrap();

        assert!(matches!(deserialized, Food));
    }
}
