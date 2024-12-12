use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use itertools::Itertools;
use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
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
    rects.into_iter()
        .map(|r| perimeter(&r) * r.len())
        .sum()
}

fn part2(input: &str) -> usize {
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
    rects.into_iter()
        .map(|r| num_sides(&r) * r.len())
        .sum()
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

fn num_sides(rect: &HashSet<Coordinate>) -> usize {
    let sorted = rect.iter().sorted_by(|a, b| match a.y.cmp(&b.y) {
        Ordering::Equal => a.x.cmp(&b.x),
        ord => ord
    }).collect::<Vec<_>>();
    let min_y = sorted.iter().map(|c| c.y).min().unwrap();
    let max_y = sorted.iter().map(|c| c.y).max().unwrap();
    let min_x = sorted.iter().map(|c| c.x).min().unwrap();
    let max_x = sorted.iter().map(|c| c.x).max().unwrap();

    let mut vertical_sides: Vec<(isize, isize)> = vec![];
    let mut inside = false;
    for y in (min_y - 1)..(max_y + 2) {
        for x in (min_x - 1)..(max_x + 2) {
            let c = Coordinate::new(y, x);
            if rect.contains(&c) && !inside {
                // Entered
                inside = true;
                if let Some(mut aaa) = vertical_sides.iter_mut().find(|side| side.0 == x && side.1 == y - 1) {
                    aaa.1 = y;
                } else {
                    vertical_sides.push((x, y));
                }
            } else if !rect.contains(&c) && inside {
                // Left
                inside = false;
                if let Some(aaa) = vertical_sides.iter_mut().find(|side| side.0 == x && side.1 == y - 1) {
                    aaa.1 = y;
                } else {
                    vertical_sides.push((x, y));
                }
            }
        }
    }

    let mut horizontal: Vec<(isize, isize)> = vec![];
    let mut inside = false;
    for x in (min_x - 1)..(max_x + 2) {
        for y in (min_y - 1)..(max_y + 2) {
            let c = Coordinate::new(y, x);
            if rect.contains(&c) && !inside {
                inside = true;
                if let Some(mut aaa) = horizontal.iter_mut().find(|side| side.0 == y && side.1 == x - 1) {
                    aaa.1 = x;
                } else {
                    horizontal.push((y, x));
                }
            } else if !rect.contains(&c) && inside {
                // Left
                inside = false;
                if let Some(aaa) = horizontal.iter_mut().find(|side| side.0 == y && side.1 == x - 1) {
                    aaa.1 = x;
                } else {
                    horizontal.push((y, x));
                }
            }
        }
    }
    horizontal.len() + vertical_sides.len()
}

fn perimeter(rect: &HashSet<Coordinate>) -> usize {
    let mut rect_sum = 0;
    let directions = vec![Direction::new(-1, 0), Direction::new(0, 1), Direction::new(1, 0), Direction::new(0, -1)];
    for coord in rect.iter() {
        for direction in directions.iter() {
            let c = Coordinate::new(coord.y + direction.dy, coord.x + direction.dx);
            if !rect.contains(&c) {
                rect_sum += 1;
            }
        }
    }
    rect_sum
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
        assert_eq!(part1(SMALL), 140);
        assert_eq!(part1(LARGE), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SMALL), 80);
        assert_eq!(part2(LARGE), 1206);
    }
}
