#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    DiagonalUpRight,
    DiagonalDownRight,
    DiagonalUpLeft,
    DiagonalDownLeft,
}

impl Direction {
    // Order is clockwise starting from right
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::Right,
            Direction::DiagonalDownRight,
            Direction::Down,
            Direction::DiagonalDownLeft,
            Direction::Left,
            Direction::DiagonalUpLeft,
            Direction::Up,
            Direction::DiagonalUpRight,
        ]
    }

    pub fn cardinal() -> Vec<Direction> {
        vec![
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ]
    }

    pub fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::DiagonalUpRight => (-1, 1),
            Direction::DiagonalDownRight => (1, 1),
            Direction::DiagonalUpLeft => (-1, -1),
            Direction::DiagonalDownLeft => (1, -1),
        }
    }
}
