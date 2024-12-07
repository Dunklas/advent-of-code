#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
    pub y: isize,
    pub x: isize,
}

impl Coordinate {
    pub fn new(y: isize, x: isize) -> Coordinate {
        Self { y, x }
    }
}
