use std::collections::{HashSet, VecDeque};
use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
use crate::util::grid::Grid;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input, 71, 1024));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str, size: usize, sim_len: usize) -> usize {
    let bytes = parse(input);
    let mut grid = Grid::<char>::new_with(size, size, '.');
    bytes.into_iter().take(sim_len)
        .for_each(|byte_pos| {
            grid.replace(&byte_pos, '#');
        });
    let target = Coordinate::new(size as isize -1, size as isize- 1);
    let path = shortes_path(&grid, &target).unwrap();
    path
}

fn part2(input: &str) -> usize {
    0
}

fn shortes_path(grid: &Grid<char>, target: &Coordinate) -> Option<usize> {
    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();
    stack.push_back((Coordinate::new(0, 0), 0));
    visited.insert(Coordinate::new(0, 0));
    while let Some((curr, path_len)) = stack.pop_front() {
        if curr == *target {
            return Some(path_len);
        }
        match grid.get(&curr) {
            Some('#') | None => continue,
            _ => {}
        }
        for (dy, dx) in DIRECTIONS.iter() {
            let next = Coordinate::new(curr.y + dy, curr.x + dx);
            if visited.contains(&next) {
                continue;
            }
            visited.insert(next);
            stack.push_back((next, path_len + 1));
        }
    }
    None
}

fn parse(input: &str) -> Vec<Coordinate> {
    input.lines()
        .map(|line| {
            let parts = line.split(",").collect::<Vec<_>>();
            Coordinate::new(
                parts[1].parse().unwrap(),
                parts[0].parse().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(part1(input, 7, 12), 22);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), 0);
    }
}