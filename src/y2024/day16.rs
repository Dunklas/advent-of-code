use std::cmp::Ordering;
use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
use crate::util::grid::Grid;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::str::FromStr;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let maze = Grid::<char>::from_str(input).unwrap();
    let start = maze.find(&'S').unwrap();
    find_best_path(&maze, &start).unwrap()
}

fn part2(input: &str) -> usize {
    0
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pos: Coordinate,
    cost: usize,
    dir: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_best_path(maze: &Grid<char>, start: &Coordinate) -> Option<usize> {
    let mut distances: HashMap<(Coordinate, Direction), usize> = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(State { pos: *start, cost: 0usize, dir: Direction::new(0, 1)});
    distances.insert((*start, Direction::new(0, 1)), 0);

    let mut end_costs = Vec::<usize>::new();
    while let Some(state) = queue.pop() {
        let val = maze.get(&state.pos);
        if val == Some(&'E') {
            end_costs.push(state.cost);
            continue;
        }
        if val == Some(&'#') || state.cost > *distances.get(&(state.pos, state.dir)).unwrap_or(&usize::MAX) {
            continue;
        }
        distances.insert((state.pos, state.dir), state.cost);
        queue.push(State { pos: Coordinate::new(state.pos.y + state.dir.dy, state.pos.x + state.dir.dx), cost: state.cost + 1, dir: state.dir});
        let left = state.dir.rotated_left();
        queue.push(State { pos: Coordinate::new(state.pos.y + left.dy, state.pos.x + left.dx), cost: state.cost + 1000 + 1, dir: left});
        let right = state.dir.rotated_right();
        queue.push(State { pos: Coordinate::new(state.pos.y + right.dy, state.pos.x + right.dx), cost: state.cost + 1000 + 1, dir: right});
    }
    println!("Costs: {:?}", end_costs);
    end_costs.into_iter().min().or(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const INPUT2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 7036);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
