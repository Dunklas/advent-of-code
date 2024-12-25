use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref BUTTON: Regex = Regex::new(r"^Button [AB]: X\+(\d+), Y\+(\d+)$").unwrap();
    static ref PRIZE: Regex = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();
}

const EPSILON: f64 = 1e-9;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let machines = parse(input).unwrap();
    machines
        .into_iter()
        .filter_map(|val| val.min_tokens())
        .filter(|&(a, b)| a <= 100 && b <= 100)
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn part2(input: &str) -> usize {
    let mut machines = parse(input).unwrap();
    for machine in machines.iter_mut() {
        machine.prize.x += 10000000000000;
        machine.prize.y += 10000000000000;
    }
    machines
        .into_iter()
        .filter_map(|val| val.min_tokens())
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn parse(input: &str) -> Result<Vec<Machine>, ParseMachineError> {
    let section = input.split("\n\n");
    section.into_iter().map(Machine::from_str).collect()
}

#[derive(Debug)]
struct Machine {
    a: Direction,
    b: Direction,
    prize: Coordinate,
}

impl Machine {
    fn min_tokens(&self) -> Option<(usize, usize)> {
        let (a_x, a_y) = (self.a.dx as f64, self.a.dy as f64);
        let (b_x, b_y) = (self.b.dx as f64, self.b.dy as f64);
        let (p_x, p_y) = (self.prize.x as f64, self.prize.y as f64);

        let determinant = a_x * b_y - a_y * b_x;
        if determinant.abs() < EPSILON {
            return None;
        }

        let cramer_a = p_x * b_y - p_y * b_x;
        let cramer_b = a_x * p_y - a_y * p_x;
        let a = cramer_a / determinant;
        let b = cramer_b / determinant;

        if a.fract() != 0.0 || b.fract() != 0.0 {
            return None;
        }
        Some((a as usize, b as usize))
    }
}

#[derive(Debug)]
struct ParseMachineError {}
impl FromStr for Machine {
    type Err = ParseMachineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let a: (usize, usize) = BUTTON
            .captures(lines[0])
            .and_then(|cap| Some((cap[1].parse().ok()?, cap[2].parse().ok()?)))
            .ok_or(ParseMachineError {})?;
        let b: (usize, usize) = BUTTON
            .captures(lines[1])
            .and_then(|cap| Some((cap[1].parse().ok()?, cap[2].parse().ok()?)))
            .ok_or(ParseMachineError {})?;
        let prize: (usize, usize) = PRIZE
            .captures(lines[2])
            .and_then(|cap| Some((cap[1].parse().ok()?, cap[2].parse().ok()?)))
            .ok_or(ParseMachineError {})?;
        Ok(Self {
            a: Direction::new(a.1 as isize, a.0 as isize),
            b: Direction::new(b.1 as isize, b.0 as isize),
            prize: Coordinate::new(prize.1 as isize, prize.0 as isize),
        })
    }
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
        assert_eq!(part2(INPUT), 875318608908);
    }
}
