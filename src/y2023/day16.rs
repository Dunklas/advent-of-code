use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let contraption = parse(input);
    let mut beams = vec![((0usize, 0usize), (0isize, 1isize))]
        .into_iter()
        .collect::<VecDeque<_>>();
    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    while let Some(((start_y, start_x), (start_dy, start_dx))) = beams.pop_front() {
        let (mut y, mut x) = (start_y, start_x);
        let (mut dy, mut dx) = (start_dy, start_dx);
        loop {
            // Check if outside, then quit?
            match contraption[y][x] {
                '.' => {},
                '\\' if dx.is_negative() => {},
                '\\' if dx.is_positive() => {},
                '\\' if dy.is_negative() => {},
                '\\' if dy.is_positive() => {},
                '/' if dx.is_negative() => {},
                '/' if dx.is_positive() => {},
                '/' if dy.is_negative() => {},
                '/' if dy.is_positive() => {},
                '|' if dx != 0 => {},
                '|' if dy != 0 => {},
                '-' if dx != 0 => {},
                '-' if dy != 0 => {},
                _ => panic!("unexpected"),
            }
        }
    }
    0
}

fn part2(input: &str) -> u32 {
    0
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        assert_eq!(part1(input), 46);
    }
}
