use crate::util::coordinate::Coordinate;
use std::str::FromStr;

#[derive(Debug)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

#[derive(Debug)]
pub struct ParseGridError {}

impl<T: Copy + PartialEq<T>> Grid<T> {
    fn new(grid: Vec<Vec<T>>) -> Self {
        Self { grid }
    }

    pub fn get(&self, coordinate: &Coordinate) -> Option<&T> {
        self.grid
            .get(coordinate.y as usize)?
            .get(coordinate.x as usize)
    }

    pub fn replace(&mut self, coordinate: &Coordinate, value: T) -> T {
        let prev = self.grid[coordinate.y as usize][coordinate.x as usize];
        self.grid[coordinate.y as usize][coordinate.x as usize] = value;
        prev
    }

    pub fn contains(&self, coordinate: &Coordinate) -> bool {
        coordinate.y < self.grid.len() as isize
            && coordinate.y >= 0
            && coordinate.x < self.grid[0].len() as isize
            && coordinate.x >= 0
    }

    pub fn find(&self, value: &T) -> Option<Coordinate> {
        let mut result = None;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x] == *value {
                    result = Some(Coordinate::new(y as isize, x as isize));
                }
            }
        }
        result
    }

    pub fn get_segment(&self, start: &Coordinate, dx: isize, dy: isize, len: usize) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = Coordinate::new(start.y, start.x);

        for _ in 0..len {
            if !self.contains(&current) {
                break;
            }
            result.push(&self.grid[current.y as usize][current.x as usize]);
            current = Coordinate::new(current.y + dy, current.x + dx);
        }

        result.into_iter().copied().collect()
    }

    pub fn y_len(&self) -> usize {
        self.grid.len()
    }

    pub fn x_len(&self) -> usize {
        self.grid[0].len()
    }
}

impl<T> FromStr for Grid<T>
where
    T: FromStr + Copy + PartialEq<T>,
{
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Result<Vec<Vec<T>>, ParseGridError> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| T::from_str(&c.to_string()).map_err(|_| ParseGridError {}))
                    .collect()
            })
            .collect();
        grid.map(Grid::new)
    }
}
