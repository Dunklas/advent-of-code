use std::cmp::Ordering;
use crate::util::coordinate::Coordinate;
use crate::util::grid::Grid;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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
    let result = shortest_path(&track, &start, &end).unwrap();
    println!("{:?}", result.get(&end).unwrap());
    0
}

fn part2(input: &str) -> usize {
    0
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Coordinate,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(grid: &Grid<char>, start: &Coordinate, end: &Coordinate) -> Option<HashMap<Coordinate, usize>> {
    let mut dist = grid.coordinates().map(|c| (c, usize::MAX)).collect::<HashMap<_, _>>();
    let mut heap = BinaryHeap::new();
    dist.insert(*start, 0);
    heap.push(State { cost: 0, position: *start });
    while let Some(State { cost, position }) = heap.pop() {
        if position == *end {
            return Some(dist);
        }
        if let Some('#') = grid.get(&position) {
            continue;
        }
        if let Some(prev_cost) = dist.get(&position) {
            if cost > *prev_cost {
                continue;
            }
        }
        for (dy, dx) in &DIRECTIONS {
            let next = Coordinate::new(position.y + dy, position.x + dx);
            let next = State { cost: cost + 1, position: next};
            if let Some(prev_next_cost) = dist.get(&next.position) {
                if next.cost < *prev_next_cost {
                    heap.push(next);
                    dist.insert(next.position, next.cost);
                }
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
