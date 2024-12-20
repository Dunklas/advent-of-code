use crate::util::coordinate::Coordinate;
use crate::util::grid::Grid;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input, 100));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str, save_cap: usize) -> usize {
    let mut track = Grid::<char>::from_str(input).unwrap();
    let end = track.find(&'E').unwrap();
    let start = track.find(&'S').unwrap();
    let base_shortest = shortest_path(&track, &start, &end).unwrap();
    let base_len = base_shortest.len();
    let mut cheats: usize = 0;
    let mut wall_removals = HashSet::new();
    for (i, coord) in base_shortest.iter().enumerate() {
        let mut cheat_pos = Vec::new();
        for (dy, dx) in DIRECTIONS.iter() {
            let candidate = Coordinate::new(coord.y + dy, coord.x + dx);
            if wall_removals.contains(&candidate) {
                continue;
            }
            let segment = track.get_segment(coord, *dx, *dy, 3);
            if segment.len() == 3 && segment[1] == '#' && (segment[2] == '.' || segment[2] == 'E') {
                cheat_pos.push(candidate);
            }
        };
        for pos in cheat_pos {
            wall_removals.insert(pos);
            track.replace(&pos, '.');
            if let Some(new_path_len) = shortest_path_len(&track, coord, &end) {
                let old = base_len - i - 1;
                if new_path_len < old {
                    if old - new_path_len >= save_cap {
                        cheats += 1;
                    }
                }
            }
            track.replace(&pos, '#');
        }
    }
    cheats
}

fn part2(input: &str) -> usize {
    0
}

fn shortest_path_len(grid: &Grid<char>, start: &Coordinate, end: &Coordinate) -> Option<usize> {
    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();
    stack.push_back((*start, 0));
    visited.insert(Coordinate::new(0, 0));
    while let Some((curr, path_len)) = stack.pop_front() {
        if curr == *end {
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

fn shortest_path(grid: &Grid<char>, start: &Coordinate, end: &Coordinate) -> Option<Vec<Coordinate>> {
    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();
    stack.push_back((vec![*start]));
    visited.insert(Coordinate::new(0, 0));
    while let Some(path) = stack.pop_front() {
        if let Some(curr) = path.last() {
            if curr == end {
                return Some(path);
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
                let mut path = path.clone();
                path.push(next);
                stack.push_back(path);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!(part1(input, 2), 44);
    }
}
