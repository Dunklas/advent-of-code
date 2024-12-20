use std::cmp::Ordering;
use std::ops::{Add, Neg, Sub};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
    pub y: isize,
    pub x: isize,
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y)
            .then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Coordinate {
    pub fn new(y: isize, x: isize) -> Coordinate {
        Self { y, x }
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
