#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Direction {
    pub dy: isize,
    pub dx: isize,
}

impl Direction {
    pub const UP: Direction = Direction { dy: -1, dx: 0 };
    pub const DOWN: Direction = Direction { dy: 1, dx: 0 };
    pub const LEFT: Direction = Direction { dy: 0, dx: -1 };
    pub const RIGHT: Direction = Direction { dy: 0, dx: 1 };
    pub const TOP_RIGHT: Direction = Direction { dx: 1, dy: -1 };
    pub const BOTTOM_RIGHT: Direction = Direction { dx: 1, dy: 1 };
    pub const BOTTOM_LEFT: Direction = Direction { dx: -1, dy: 1 };
    pub const TOP_LEFT: Direction = Direction { dx: -1, dy: -1 };

    pub fn new(dy: isize, dx: isize) -> Direction {
        Self { dy, dx }
    }

    pub fn rotated_right(&self) -> Direction {
        match *self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            _ => unreachable!(),
        }
    }

    pub fn rotated_left(&self) -> Direction {
        match *self {
            Direction::UP => Direction::LEFT,
            Direction::LEFT => Direction::DOWN,
            Direction::DOWN => Direction::RIGHT,
            Direction::RIGHT => Direction::UP,
            _ => unreachable!(),
        }
    }
}
