use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use crate::util::coordinate::Coordinate;
use crate::util::grid::Grid;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let grid = Grid::<char>::from_str(input).unwrap();
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut rects: Vec<HashSet<Coordinate>> = Vec::new();
    for coordinate in grid.coordinates() {
        if visited.contains(&coordinate) {
            continue;
        }
        if let Some(rect) = find_rect(&grid, &coordinate) {
            visited.extend(rect.clone());
            rects.push(rect);
        }
    }
    rects.len()
}

fn part2(input: &str) -> usize {
    0
}

fn find_rect(grid: &Grid<char>, start: &Coordinate) -> Option<HashSet<Coordinate>> {
    let value = grid.get(start)?;
    let mut visited = HashSet::new();
    let mut stack = VecDeque::from([*start]);
    while let Some(n) = stack.pop_front() {
        if visited.contains(&n) {
            continue;
        }
        if let Some(v) = grid.get(&n) {
            if v == value {
                visited.insert(n);
                stack.push_back(Coordinate::new(n.y - 1, n.x));
                stack.push_back(Coordinate::new(n.y, n.x + 1));
                stack.push_back(Coordinate::new(n.y + 1, n.x));
                stack.push_back(Coordinate::new(n.y, n.x - 1));
            }
        }
    }
    Some(visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL: &str = "AAAA
BBCD
BBCC
EEEC";

    const LARGE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        assert_eq!(part1(LARGE), 140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SMALL), 0);
    }
}
