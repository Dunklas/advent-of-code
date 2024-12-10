use crate::util::coordinate::Coordinate;
use crate::util::grid::Grid;
use std::collections::HashSet;
use std::str::FromStr;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> usize {
    let grid = Grid::<i8>::from_str(input).unwrap();
    grid.find_all(&0)
        .map(|start| {
            find_destinations(&grid, start)
                .into_iter()
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let grid = Grid::<i8>::from_str(input).unwrap();
    grid.find_all(&0)
        .map(|start| find_destinations(&grid, start).len())
        .sum()
}

fn find_destinations(grid: &Grid<i8>, start: Coordinate) -> Vec<Coordinate> {
    let mut stack = vec![start];
    let mut destinations = Vec::new();
    while let Some(current) = stack.pop() {
        match grid.get(&current) {
            Some(&9) => destinations.push(current),
            Some(&height) => {
                for (dy, dx) in DIRECTIONS {
                    let next_coord = Coordinate::new(current.y + dy, current.x + dx);
                    if let Some(&next_height) = grid.get(&next_coord) {
                        if next_height - height == 1 {
                            stack.push(next_coord);
                        }
                    }
                }
            }
            None => {}
        }
    }
    destinations
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL: &str = "0123
1234
8765
9876";

    const LARGE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SMALL), 1);
        assert_eq!(part1(LARGE), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(LARGE), 81);
    }
}
