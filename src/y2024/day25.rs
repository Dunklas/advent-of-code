use crate::util::coordinate::Coordinate;
use crate::util::grid::{Grid, ParseGridError};
use std::cmp::PartialEq;
use std::str::FromStr;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let schematics = parse(input).unwrap();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for schematic in schematics {
        match schematic.t {
            SchematicType::Key => {
                keys.push(schematic);
            }
            SchematicType::Lock => {
                locks.push(schematic);
            }
        }
    }

    locks
        .into_iter()
        .map(|lock| keys.iter().filter(|key| lock.fits(key)).count())
        .sum()
}

fn part2(input: &str) -> usize {
    0
}

fn parse(input: &str) -> Result<Vec<Schematic>, ParseGridError> {
    input
        .split("\n\n")
        .map(Schematic::from_str)
        .collect::<Result<Vec<_>, ParseGridError>>()
}

#[derive(Debug, PartialEq, Eq)]
enum SchematicType {
    Lock,
    Key,
}

struct Schematic {
    grid: Grid<char>,
    t: SchematicType,
}

impl Schematic {
    pub fn height(&self) -> Vec<usize> {
        let mut heights = Vec::new();
        match self.t {
            SchematicType::Lock => {
                for x in 0..self.grid.x_len() {
                    let mut height = 0;
                    for y in 1..self.grid.y_len() {
                        let c = Coordinate::new(y as isize, x as isize);
                        if self.grid.get(&c) == Some(&'#') {
                            height += 1;
                        } else {
                            break;
                        }
                    }
                    heights.push(height);
                }
            }
            SchematicType::Key => {
                for x in 0..self.grid.x_len() {
                    let mut height = 0;
                    for y in (0..self.grid.y_len() - 1).rev() {
                        let c = Coordinate::new(y as isize, x as isize);
                        if self.grid.get(&c) == Some(&'#') {
                            height += 1;
                        }
                    }
                    heights.push(height);
                }
            }
        }
        heights
    }

    pub fn fits(&self, other: &Schematic) -> bool {
        assert!(self.t == SchematicType::Lock && other.t == SchematicType::Key);
        self.height()
            .iter()
            .zip(other.height().iter())
            .all(|(h1, h2)| h1 + h2 <= 5)
    }
}

impl FromStr for Schematic {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        let t = match grid.get(&Coordinate::new(0, 0)) {
            Some('#') => SchematicType::Lock,
            Some('.') => SchematicType::Key,
            _ => unreachable!(),
        };
        Ok(Self { grid, t })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
