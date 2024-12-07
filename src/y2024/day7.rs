use std::panic::resume_unwind;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref EQUATION: Regex = Regex::new(r"^(\d+): (\d+) (\d+)$").unwrap();
}

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|(result, operands)| calc(result + 0, &operands, 0, 0) >= 1)
        .map(|(result, _)| result)
        .map(|result| {
            println!("RESULT: {}", result);
            result
        })
        .sum()
}

fn part2(input: &str) -> usize {
    0
}

fn calc(result: usize, operands: &Vec<usize>, index: usize, sum: usize) -> usize {
    if let Some(a) = operands.get(index) {
        if a + sum == result {
            println!("Found result: {}+{}={}", a, sum, result);
            return 1;
        }
        if a * sum == result {
            println!("Found result: {}*{}={}", a, sum, result);
            return 1;
        }
        return calc(result, operands, index + 1, a + sum) + calc(result, operands, index + 1, a * sum);
    }
    0
}

fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(":").collect::<Vec<_>>();
            let operands = parts[1]
                .split_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>();
            (parts[0].parse().unwrap(), operands)
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::y2024::day7::part1;

    #[test]
    fn test_part1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(part1(input), 3749);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part1(input), 0);
    }
}
