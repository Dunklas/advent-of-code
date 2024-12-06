pub struct Coordinate {
    y: isize,
    x: isize,
}

impl Coordinate {
    pub fn new(y: isize, x: isize) -> Coordinate {
        Self { y, x }
    }
}
