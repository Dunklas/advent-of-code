use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
use crate::util::grid::Grid;
use std::collections::{HashMap, HashSet, VecDeque};

const DIRECTIONS: [Direction; 4] = [
    Direction::UP,
    Direction::RIGHT,
    Direction::DOWN,
    Direction::LEFT,
];

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input, 71, 1024));
    println!("Part 2: {:?}", part2(input, 71));
}

fn part1(input: &str, size: usize, sim_len: usize) -> usize {
    let bytes = parse(input);
    let mut grid = Grid::<char>::new_with(size, size, '.');
    bytes.into_iter().take(sim_len).for_each(|byte_pos| {
        grid.replace(&byte_pos, '#');
    });
    let target = Coordinate::new(size as isize - 1, size as isize - 1);
    get_shortest_path(&grid, &target).unwrap().len()
}

fn part2(input: &str, size: usize) -> Option<Coordinate> {
    let bytes = parse(input);
    let mut grid = Grid::<char>::new_with(size, size, '.');
    let target = Coordinate::new(size as isize - 1, size as isize - 1);
    let mut shortest_path = get_shortest_path(&grid, &target)?;
    for byte in bytes {
        grid.replace(&byte, '#');
        if shortest_path.contains(&byte) {
            match get_shortest_path(&grid, &target) {
                Some(new_path) => shortest_path = new_path,
                None => return Some(byte),
            }
        }
    }
    None
}

fn get_shortest_path(grid: &Grid<char>, target: &Coordinate) -> Option<HashSet<Coordinate>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut predecessors = HashMap::new();
    queue.push_back(Coordinate::new(0, 0));
    visited.insert(Coordinate::new(0, 0));
    while let Some(current) = queue.pop_front() {
        if current == *target {
            let mut path = HashSet::new();
            let mut current = current;
            while let Some(prev) = predecessors.get(&current) {
                path.insert(current);
                current = *prev;
            }
            return Some(path);
        }
        match grid.get(&current) {
            Some('#') | None => continue,
            _ => {}
        }
        for dir in DIRECTIONS.iter() {
            let next = current.offset(dir);
            if visited.contains(&next) {
                continue;
            }
            visited.insert(next);
            predecessors.insert(next, current);
            queue.push_back(next);
        }
    }
    None
}

fn parse(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(",").collect::<Vec<_>>();
            Coordinate::new(parts[1].parse().unwrap(), parts[0].parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5,4
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT, 7, 12), 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT, 7), Some(Coordinate::new(1, 6)));
    }
}
