use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut map = parse(input);
    tilt(&mut map);
    total_load(&map)
}

fn part2(input: &str) -> u64 {
    let mut map = parse(input);
    let mut hashes: HashMap<u64, isize> = HashMap::new();
    let mut cycle = None;
    for i in 0.. {
        if let Some(prev) = hashes.insert(calculate_hash(&map), i) {
            cycle = Some((prev, i));
            break;
        };
        map = spin_cycle(map);
    }
    let (cycle_start, cycle_end) = cycle.unwrap();
    let cycle_length = cycle_end - cycle_start;
    for _ in 0..((1000000000 - cycle_start) % cycle_length) {
        map = spin_cycle(map);
    }
    total_load(&map)
}

fn tilt(map: &mut [Vec<char>]) {
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x] != 'O' {
                continue;
            }
            if let Some((new_y, new_x)) = new_position(y, x, map) {
                map[y][x] = '.';
                map[new_y][new_x] = 'O';
            }
        }
    }
}

fn new_position(y: usize, x: usize, map: &[Vec<char>]) -> Option<(usize, usize)> {
    (0..y)
        .rev()
        .map(|y| (y, map[y][x]))
        .take_while(|(_, tile)| *tile == '.')
        .map(|(y, _)| (y, x))
        .last()
}

fn spin_cycle(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..4).fold(map, |mut prev, _| {
        tilt(&mut prev);
        rotate(prev)
    })
}

fn rotate(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let len = map[0].len();
    let mut iters: Vec<_> = map.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<char>>()
        })
        .map(|row| row.into_iter().rev().collect())
        .collect()
}

fn total_load(map: &[Vec<char>]) -> u64 {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().map(move |tile| (y, *tile)))
        .filter(|(_, tile)| *tile == 'O')
        .fold(0, |sum, (y, _)| sum + (map.len() - y) as u64)
}

fn calculate_hash(map: &Vec<Vec<char>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    map.hash(&mut hasher);
    hasher.finish()
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part1(input), 136);
    }

    #[test]
    fn part2_test() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part2(input), 64);
    }
}
