use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
use crate::util::grid::Grid;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let maze = Grid::<char>::from_str(input).unwrap();
    let paths = find_best_paths(&maze);
    paths.first().map(|(score, _)| *score).unwrap()
}

fn part2(input: &str) -> usize {
    let maze = Grid::<char>::from_str(input).unwrap();
    let paths = find_best_paths(&maze);
    paths
        .into_iter()
        .flat_map(|(_, path)| path)
        .collect::<HashSet<_>>()
        .len()
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    dir: Direction,
    path: Vec<Coordinate>,
}

fn find_best_paths(maze: &Grid<char>) -> Vec<(usize, Vec<Coordinate>)> {
    let start = maze.find(&'S').unwrap();
    let mut distances: HashMap<(Coordinate, Direction), usize> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(State {
        cost: 0usize,
        dir: Direction::RIGHT,
        path: vec![start],
    });
    distances.insert((start, Direction::RIGHT), 0);

    let mut lowest_cost = usize::MAX;
    let mut best_paths = Vec::<(usize, Vec<Coordinate>)>::new();
    while let Some(state) = queue.pop_front() {
        let pos = state.path.last().unwrap();
        let val = maze.get(&pos);
        if val == Some(&'E') {
            if state.cost < lowest_cost {
                best_paths.clear();
            }
            if state.cost <= lowest_cost {
                best_paths.push((state.cost, state.path));
                lowest_cost = state.cost;
            }
            continue;
        }
        if val == Some(&'#')
            || state.cost > *distances.get(&(*pos, state.dir)).unwrap_or(&usize::MAX)
        {
            continue;
        }
        distances.insert((*pos, state.dir), state.cost);

        [
            state.dir,
            state.dir.rotated_left(),
            state.dir.rotated_right(),
        ]
        .into_iter()
        .for_each(|dir| {
            let rotation_penalty = if dir == state.dir { 0 } else { 1000 };
            let mut path = state.path.clone();
            let next = pos.offset(&dir);
            path.push(next);
            queue.push_back(State {
                cost: state.cost + rotation_penalty + 1,
                dir,
                path,
            });
        });
    }
    best_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST: &str = "###############
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

    const SECOND: &str = "#################
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
        assert_eq!(part1(FIRST), 7036);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(FIRST), 45);
        assert_eq!(part2(SECOND), 64);
    }
}
