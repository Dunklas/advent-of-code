use crate::util::coordinate::Coordinate;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl<T: Copy + PartialEq<T>> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        assert!(grid.iter().map(|row| row.len()).all_equal());
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
        self.iter_coordinates()
            .find(|coordinate| self.get(coordinate) == Some(value))
    }

    pub fn iter_coordinates(&self) -> GridIterator<T> {
        GridIterator {
            grid: self,
            y: 0,
            x: 0,
        }
    }

    pub fn get_segment(&self, start: &Coordinate, dx: isize, dy: isize, len: usize) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = Coordinate::new(start.y, start.x);

        for _ in 0..len {
            if !self.contains(&current) {
                break;
            }
            result.push(self.grid[current.y as usize][current.x as usize]);
            current = Coordinate::new(current.y + dy, current.x + dx);
        }

        result.into_iter().collect()
    }

    fn y_len(&self) -> usize {
        self.grid.len()
    }

    fn x_len(&self) -> usize {
        self.grid[0].len()
    }
}

#[derive(Debug)]
pub struct ParseGridError {}

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

pub struct GridIterator<'a, T>
where
    T: Copy + PartialEq<T>,
{
    grid: &'a Grid<T>,
    y: usize,
    x: usize,
}

impl<'a, T> Iterator for GridIterator<'a, T>
where
    T: Copy + PartialEq<T>,
{
    type Item = Coordinate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.grid.x_len() {
            let next = Coordinate::new(self.y as isize, self.x as isize);
            self.x += 1;
            return Some(next);
        }
        self.y += 1;
        if self.y < self.grid.y_len() {
            self.x = 0;
            return self.next();
        }
        None
    }
}
