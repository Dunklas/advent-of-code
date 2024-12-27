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
        match (self.dy, self.dx) {
            (-1, 0) => Direction::new(0, 1),
            (0, 1) => Direction::new(1, 0),
            (1, 0) => Direction::new(0, -1),
            (0, -1) => Direction::new(-1, 0),
            _ => unreachable!(),
        }
    }
}
