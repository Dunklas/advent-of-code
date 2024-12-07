#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Direction {
    pub dy: isize,
    pub dx: isize,
}

impl Direction {
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
