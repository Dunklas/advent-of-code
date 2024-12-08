use crate::util::coordinate::Coordinate;
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq)]
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

    pub fn replace(&mut self, coordinate: &Coordinate, value: T) -> Option<T> {
        match self.contains(coordinate) {
            true => {
                let prev = self.grid[coordinate.y as usize][coordinate.x as usize];
                self.grid[coordinate.y as usize][coordinate.x as usize] = value;
                Some(prev)
            }
            false => None,
        }
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
        result
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

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for item in row {
                write!(f, "{:?}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for item in row {
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
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

impl<T> Iterator for GridIterator<'_, T>
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_valid_grid() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(grid.get(&Coordinate::new(0, 0)), Some(&1));
        assert_eq!(grid.get(&Coordinate::new(2, 2)), Some(&9));
    }

    #[test]
    #[should_panic]
    fn test_new_invalid_grid() {
        Grid::new(vec![vec![1, 2], vec![3, 4, 5]]);
    }

    #[test]
    fn test_get_valid_coordinate() {
        let grid = Grid::new(vec![vec!["a", "b"], vec!["c", "d"]]);
        assert_eq!(grid.get(&Coordinate::new(0, 1)), Some(&"b"));
    }

    #[test]
    fn test_get_invalid_coordinate() {
        let grid = Grid::new(vec![vec!["a", "b"], vec!["c", "d"]]);
        assert_eq!(grid.get(&Coordinate::new(2, 2)), None);
    }

    #[test]
    fn test_replace_valid_coordinate() {
        let mut grid = Grid::new(vec![vec![10, 20], vec![30, 40]]);
        let replaced = grid.replace(&Coordinate::new(0, 1), 25);
        assert_eq!(replaced, Some(20));
        assert_eq!(grid.get(&Coordinate::new(0, 1)), Some(&25));
    }

    #[test]
    fn test_replace_invalid_coordinate() {
        let mut grid = Grid::new(vec![vec![10, 20], vec![30, 40]]);
        let replaced = grid.replace(&Coordinate::new(3, 0), 50);
        assert_eq!(replaced, None);
    }

    #[test]
    fn test_contains_coordinate() {
        let grid = Grid::new(vec![vec!["x", "y"], vec!["z", "w"]]);
        assert!(grid.contains(&Coordinate::new(0, 1)));
        assert!(!grid.contains(&Coordinate::new(2, 0)));
    }

    #[test]
    fn test_find_value() {
        let grid = Grid::new(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(grid.find(&3), Some(Coordinate::new(1, 0)));
        assert_eq!(grid.find(&5), None);
    }

    #[test]
    fn test_iter_coordinates() {
        let grid = Grid::new(vec![vec![1, 2], vec![3, 4]]);
        let coordinates: Vec<Coordinate> = grid.iter_coordinates().collect();
        assert_eq!(
            coordinates,
            vec![
                Coordinate::new(0, 0),
                Coordinate::new(0, 1),
                Coordinate::new(1, 0),
                Coordinate::new(1, 1),
            ]
        );
    }

    #[test]
    fn test_get_segment() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let segment = grid.get_segment(&Coordinate::new(0, 0), 1, 1, 3);
        assert_eq!(segment, vec![1, 5, 9]);
    }

    #[test]
    fn test_get_segment_out_of_bounds() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let segment = grid.get_segment(&Coordinate::new(1, 1), 1, 1, 5);
        assert_eq!(segment, vec![5, 9]);
    }

    #[test]
    fn test_parse_grid_from_str() {
        let input = "12\n34";
        let grid: Grid<u32> = input.parse().unwrap();
        assert_eq!(grid.get(&Coordinate::new(0, 0)), Some(&1));
        assert_eq!(grid.get(&Coordinate::new(1, 1)), Some(&4));
    }

    #[test]
    fn test_parse_grid_from_str_invalid_characters() {
        let input = "1a\n34";
        let result: Result<Grid<u32>, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_debug_format() {
        let grid = Grid::new(vec![vec![1, 2], vec![3, 4]]);
        let debug_output = format!("{:?}", grid);
        assert!(debug_output.contains("1"));
        assert!(debug_output.contains("4"));
    }

    #[test]
    fn test_display_format() {
        let grid = Grid::new(vec![vec![1, 2], vec![3, 4]]);
        let display_output = format!("{}", grid);
        assert_eq!(display_output, "12\n34\n");
    }
}
