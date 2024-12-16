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
    let paths = find_best_path(&maze, &start);
    let mut best_score = usize::MAX;
    for (score, _) in paths {
        if score < best_score {
            best_score = score;
        }
    }
    best_score
}

fn part2(input: &str) -> usize {
    let maze = Grid::<char>::from_str(input).unwrap();
    let start = maze.find(&'S').unwrap();
    let paths = find_best_path(&maze, &start);
    let mut best_score = usize::MAX;
    for (score, _) in paths.iter() {
        if *score < best_score {
            best_score = *score;
        }
    }
    paths.into_iter()
        .filter(|(score, path)| *score == best_score)
        .flat_map(|(_, path)| path)
        .collect::<HashSet<_>>()
        .len()
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    pos: Coordinate,
    cost: usize,
    dir: Direction,
    path: Vec<Coordinate>
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

fn find_best_path(maze: &Grid<char>, start: &Coordinate) -> Vec<(usize, Vec<Coordinate>)> {
    let mut distances: HashMap<(Coordinate, Direction), usize> = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(State { pos: *start, cost: 0usize, dir: Direction::new(0, 1), path: vec![*start]});
    distances.insert((*start, Direction::new(0, 1)), 0);

    let mut end_paths = Vec::<(usize, Vec<Coordinate>)>::new();
    while let Some(state) = queue.pop() {
        let val = maze.get(&state.pos);
        if val == Some(&'E') {
            end_paths.push((state.cost, state.path));
            continue;
        }
        if val == Some(&'#') || state.cost > *distances.get(&(state.pos, state.dir)).unwrap_or(&usize::MAX) {
            continue;
        }
        distances.insert((state.pos, state.dir), state.cost);
        let mut path = state.path.clone();
        path.push(Coordinate::new(state.pos.y + state.dir.dy, state.pos.x + state.dir.dx));
        queue.push(State { pos: Coordinate::new(state.pos.y + state.dir.dy, state.pos.x + state.dir.dx), cost: state.cost + 1, dir: state.dir, path });
        let left = state.dir.rotated_left();
        let mut left_path = state.path.clone();
        left_path.push(Coordinate::new(state.pos.y + left.dy, state.pos.x + left.dx));
        queue.push(State { pos: Coordinate::new(state.pos.y + left.dy, state.pos.x + left.dx), cost: state.cost + 1000 + 1, dir: left, path: left_path });

        let right = state.dir.rotated_right();
        let mut right_path = state.path.clone();
        right_path.push(Coordinate::new(state.pos.y + right.dy, state.pos.x + right.dx));
        queue.push(State { pos: Coordinate::new(state.pos.y + right.dy, state.pos.x + right.dx), cost: state.cost + 1000 + 1, dir: right, path: right_path });
    }
    end_paths
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
