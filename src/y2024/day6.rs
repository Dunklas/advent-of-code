use std::collections::HashSet;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let mut pos = None;
    let mut dir = (0isize, 0isize);
    let mut visited = HashSet::<(usize, usize)>::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '^' {
                pos = Some((y as isize, x as isize));
                dir.0 = -1;
            }
        }
    }

    while true {
        if let Some((y, x)) = pos {
            visited.insert((y as usize, x as usize));
            println!("Visited: {:?}", (y, x));
            let (next_y, next_x) = (y + dir.0, x + dir.1);
            if next_y < 0 || next_y >= map.len() as isize || next_y < 0 || next_x >= map[0].len() as isize {
                println!("Outside of bounds!");
                break;
            }
            if map[next_y as usize][next_x as usize] == '#' {
                // Rotate
                match dir {
                    (-1, 0) => {
                        dir = (0, 1);
                    },
                    (0, 1) => {
                        dir = (1, 0);
                    },
                    (1, 0) => {
                        dir = (0, -1);
                    }
                    (0, -1) => {
                        dir = (-1, 0);
                    },
                    _ => unreachable!(),
                }
                continue;
            }
            pos = Some((next_y, next_x));
        }
    }
    visited.len()
}
fn part2(input: &str) -> usize {
    0
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

struct Guard {
    y: usize,
    x: usize,
    dir: char
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
        let input = "";
        assert_eq!(part2(input), 0);
    }
}