use std::collections::{HashSet, VecDeque};
use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let machines = parse(input);
    for machine in machines {
        if let Some((a, b)) = find_prize(&machine) {
            if (a % 2 == 0) && b % 2 == 0 {

            }
            println!("{}, {}", a, b);
        }
    }
    0
}

fn part2(input: &str) -> usize {
    0
}

fn find_prize(machine: &Machine) -> Option<(f64, f64)> {
    // X * machine.a.dy + Y * machine.b.dy = prize.y
    // X * machine.a.dx + Y * machine.b.dx = prize.x
    // TODO: Solve X and Y
    // Solve X
    let tmp_price = machine.prize
    None
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
