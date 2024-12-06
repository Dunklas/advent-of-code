use std::collections::HashSet;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let mut pos = None;
    let mut dir = (0isize, 0isize);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '^' {
                pos = Some((y as isize, x as isize));
                dir.0 = -1;
            }
        }
    }
    simulate_walk(&map, pos.unwrap(), dir)
}
fn part2(input: &str) -> usize {
    let map = parse(input);
    let mut pos = None;
    let mut dir = (0isize, 0isize);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '^' {
                pos = Some((y as isize, x as isize));
                dir.0 = -1;
            }
        }
    }

    let mut count = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if y == pos.unwrap().0 as usize && x == pos.unwrap().1 as usize {
                continue;
            }
            let mut map = map.clone();
            map[y][x] = '#';

            if simulate_walk_with_infinity_check(&map, pos.unwrap(), dir) {
                count += 1;
            }
        }
    }
    count
}

fn simulate_walk(map: &Vec<Vec<char>>, start: (isize, isize), dir: (isize, isize)) -> usize {
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut pos = start;
    let mut dir = dir;
    loop {
        visited.insert((pos.0 as usize, pos.1 as usize));
        let (next_y, next_x) = (pos.0 + dir.0, pos.1 + dir.1);
        if next_y < 0
            || next_y >= map.len() as isize
            || next_x < 0
            || next_x >= map[0].len() as isize
        {
            break;
        }
        if map[next_y as usize][next_x as usize] == '#' {
            // Rotate
            match dir {
                (-1, 0) => {
                    dir = (0, 1);
                }
                (0, 1) => {
                    dir = (1, 0);
                }
                (1, 0) => {
                    dir = (0, -1);
                }
                (0, -1) => {
                    dir = (-1, 0);
                }
                _ => unreachable!(),
            }
            continue;
        }
        pos = (next_y, next_x);
    }
    visited.len()
}

fn simulate_walk_with_infinity_check(
    map: &Vec<Vec<char>>,
    start: (isize, isize),
    dir: (isize, isize),
) -> bool {
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut pos = start;
    let mut dir = dir;
    let mut count_since_new = 0;
    loop {
        match visited.insert((pos.0 as usize, pos.1 as usize)) {
            true => {
                count_since_new = 0;
            }
            false => {
                count_since_new += 1;
            }
        };
        if count_since_new > visited.len() + 1 {
            return true;
        }
        let (next_y, next_x) = (pos.0 + dir.0, pos.1 + dir.1);
        if next_y < 0
            || next_y >= map.len() as isize
            || next_x < 0
            || next_x >= map[0].len() as isize
        {
            break;
        }
        if map[next_y as usize][next_x as usize] == '#' {
            // Rotate
            match dir {
                (-1, 0) => {
                    dir = (0, 1);
                }
                (0, 1) => {
                    dir = (1, 0);
                }
                (1, 0) => {
                    dir = (0, -1);
                }
                (0, -1) => {
                    dir = (-1, 0);
                }
                _ => unreachable!(),
            }
            continue;
        }
        pos = (next_y, next_x);
    }
    false
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

struct Guard {
    y: usize,
    x: usize,
    dir: char,
}

#[cfg(test)]
mod tests {
    use crate::y2024::day6::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(part1(input), 41);
    }

    #[test]
    fn test_part2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(part2(input), 6);
    }
}
