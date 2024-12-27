use crate::util::dir::Direction;
use std::cmp::Ordering;
use std::ops::{Add, Neg, Sub};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
    pub y: isize,
    pub x: isize,
}

impl Coordinate {
    pub fn new(y: isize, x: isize) -> Coordinate {
        Self { y, x }
    }

    pub fn offset(&self, offset: &Direction) -> Coordinate {
        Coordinate::new(self.y + offset.dy, self.x + offset.dx)
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> Ordering {
        other.y.cmp(&self.y).then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Coordinate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Coordinate {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coordinate::new(self.y + rhs.y, self.x + rhs.x)
    }
}

impl Sub for Coordinate {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate::new(self.y - rhs.y, self.x - rhs.x)
    }
}

impl Neg for Coordinate {
    type Output = Coordinate;
    fn neg(self) -> Self::Output {
        Coordinate::new(-self.y, -self.x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Coordinate::new(1, 2);
        let b = Coordinate::new(3, 4);
        assert_eq!(a + b, Coordinate::new(4, 6));
    }

    #[test]
    fn test_sub() {
        let a = Coordinate::new(1, 2);
        let b = Coordinate::new(3, 4);
        assert_eq!(a - b, Coordinate::new(-2, -2));
    }

    #[test]
    fn test_neg() {
        let a = Coordinate::new(1, 2);
        assert_eq!(-a, Coordinate::new(-1, -2));
    }
}
