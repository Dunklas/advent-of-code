use std::collections::{HashSet, VecDeque};
use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let machines = parse(input);
    machines.into_iter()
        .filter_map(|val| val.min_tokens())
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn part2(input: &str) -> usize {
    0
}

#[derive(Debug)]
struct Machine {
    a: Direction,
    b: Direction,
    prize: Coordinate
}

impl Machine {
    fn min_tokens(&self) -> Option<(usize, usize)> {
        let (a_x, a_y) = (self.a.dx as f64, self.a.dy as f64);
        let (b_x, b_y) = (self.b.dx as f64, self.b.dy as f64);
        let (p_x, p_y) = (self.prize.x as f64, self.prize.y as f64);

        let det = a_x * b_y - a_y * b_x;
        if det == 0.0 {
            return None;
        }

        let cramer_a = p_x * b_y - p_y * b_x;
        let cramer_b = a_x * p_y - a_y * p_x;
        let a = cramer_a / det;
        let b = cramer_b / det;

        if a.fract() != 0.0 || b.fract() != 0.0 {
            return None
        }
        Some((a as usize, b as usize))
    }
}

fn parse(input:&str) -> Vec<Machine> {
    let section = input.split("\n\n");
    section.into_iter().map(|section| {
        let section = section.lines().collect::<Vec<_>>();
        let a_raw = section[0].split(' ').collect::<Vec<_>>();
        let a_x = a_raw[2].replace('X', "").replace(",", "");
        let a_x = a_x.parse::<isize>().unwrap();
        let a_y = a_raw[3].replace('Y', "");
        let a_y = a_y.parse::<isize>().unwrap();

        let b_raw = section[1].split(' ').collect::<Vec<_>>();
        let b_x = b_raw[2].replace('X', "").replace(",", "");
        let b_x = b_x.parse::<isize>().unwrap();
        let b_y = b_raw[3].replace('Y', "");
        let b_y = b_y.parse::<isize>().unwrap();

        let prize_raw = section[2].split(' ').collect::<Vec<_>>();
        let price_x = prize_raw[1].replace("X=", "").replace(",", "").parse::<isize>().unwrap();
        let price_y = prize_raw[2].replace("Y=", "").replace(",", "").parse::<isize>().unwrap();

        Machine {
            a: Direction::new(a_y, a_x),
            b: Direction::new(b_y, b_x),
            prize: Coordinate::new(price_y, price_x)
        }
    })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
