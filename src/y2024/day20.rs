use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
use crate::util::grid::Grid;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

const DIRECTIONS: [Direction; 4] = [
    Direction::UP,
    Direction::RIGHT,
    Direction::DOWN,
    Direction::LEFT,
];

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input, 100));
    println!("Part 2: {}", part2(input, 100));
}

fn part1(input: &str, min_cost_reduction: usize) -> usize {
    let track = Grid::<char>::from_str(input).unwrap();
    identify_cheats(&track, 2, min_cost_reduction).len()
}

fn part2(input: &str, min_cost_reduction: usize) -> usize {
    let track = Grid::<char>::from_str(input).unwrap();
    identify_cheats(&track, 20, min_cost_reduction).len()
}

fn identify_cheats(track: &Grid<char>, cheat_len: isize, min_cost_reduction: usize) -> Vec<usize> {
    let end = track.find(&'E').unwrap();
    let start = track.find(&'S').unwrap();
    let path = path_distances(track, &start, &end).unwrap();
    let cheat_positions = manhattan_destinations(cheat_len);
    let mut cheats = Vec::new();
    for (coord, cost) in path.iter() {
        if *coord == end {
            continue;
        }
        for (cheat_dir, len) in cheat_positions.iter() {
            let dst_coord = coord.offset(cheat_dir);
            if !path.contains_key(&dst_coord) {
                continue;
            }
            let cost_reduction = path.get(&dst_coord).unwrap().saturating_sub(cost + len);
            if cost_reduction >= min_cost_reduction {
                cheats.push(cost_reduction);
            }
        }
    }
    cheats
}

fn manhattan_destinations(max_distance: isize) -> Vec<(Direction, usize)> {
    let mut destinations = Vec::new();
    for current_distance in 1..=max_distance {
        for x_offset in -current_distance..=current_distance {
            let y_offset_abs = current_distance - x_offset.abs();
            destinations.push((
                Direction::new(y_offset_abs, x_offset),
                current_distance.unsigned_abs(),
            ));
            if y_offset_abs != 0 {
                destinations.push((
                    Direction::new(-y_offset_abs, x_offset),
                    current_distance.unsigned_abs(),
                ));
            }
        }
    }
    destinations
}

fn path_distances(
    grid: &Grid<char>,
    start: &Coordinate,
    end: &Coordinate,
) -> Option<HashMap<Coordinate, usize>> {
    let mut dist = HashMap::new();
    let mut queue = VecDeque::new();
    dist.insert(*start, 0);
    queue.push_back((*start, 0));
    while let Some((position, cost)) = queue.pop_front() {
        if position == *end {
            return Some(dist);
        }
        for dir in &DIRECTIONS {
            let next = (position.offset(dir), cost + 1);
            let next_val = grid.get(&next.0);
            if !dist.contains_key(&next.0) && next_val.is_some() && next_val != Some(&'#') {
                dist.insert(next.0, cost + 1);
                queue.push_back(next);
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

    #[test]
    fn test_part2() {
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
        assert_eq!(part2(input, 50), 285);
    }
}
