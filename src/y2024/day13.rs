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
        .filter_map(|machine| find_prize(&machine))
        .for_each(|val| {
            println!("{:?}", val)
        });
    0
}

fn part2(input: &str) -> usize {
    0
}

fn find_prize(machine: &Machine) -> Option<(f64, f64)> {
    let a = machine.a.dx as f64;
    let b = machine.b.dy as f64;
    let c = machine.b.dx as f64;
    let d = machine.a.dy as f64;
    let main = determinant(a, b, c, d);
    if main == 0.0 {
        return None;
    }
    let y = determinant(machine.prize.x as f64, b, machine.prize.y as f64, d) / main;
    let x = determinant(a, machine.prize.x as f64, c, machine.prize.y as f64) / main;
    Some((y, x))
}

fn determinant(a: f64, b: f64, c: f64, d: f64) -> f64 {
    (a * d) - (b * c)
}

#[derive(Debug)]
struct Machine {
    a: Direction,
    b: Direction,
    prize: Coordinate
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
