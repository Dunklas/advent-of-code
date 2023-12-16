use std::ops::Range;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBERS: Regex = Regex::new(r"(\d+)").unwrap();
}

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let parsed = parse(input);
    let numbers = numbers(&parsed);
    symbol_coordinates(&parsed)
        .iter()
        .map(|(symbol, coordinate)| (symbol, adjacent_numbers(&numbers, coordinate)))
        .map(|(_, numbers)| numbers.into_iter().sum::<u32>())
        .sum()
}

fn part2(input: &str) -> u32 {
    let parsed = parse(input);
    let numbers = numbers(&parsed);
    symbol_coordinates(&parsed)
        .iter()
        .map(|(symbol, coordinate)| (symbol, adjacent_numbers(&numbers, coordinate)))
        .filter(|(symbol, numbers)| **symbol == '*' && numbers.len() == 2)
        .map(|(_, numbers)| numbers.into_iter().product::<u32>())
        .sum()
}

fn adjacent_numbers(
    number_ranges: &[Vec<(u32, Range<usize>)>],
    symbol_pos: &Coordinate,
) -> Vec<u32> {
    number_ranges
        .iter()
        .take(symbol_pos.0 + 2)
        .skip(symbol_pos.0 - 1)
        .flat_map(|ranges| {
            ranges
                .iter()
                .filter(|(_, n_range)| {
                    (symbol_pos.1 - 1..symbol_pos.1 + 2).any(|x| n_range.contains(&x))
                })
                .map(|(n, _)| *n)
        })
        .collect()
}

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

fn symbol_coordinates(schematic: &[String]) -> Vec<(char, Coordinate)> {
    schematic
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_numeric() && *c != '.')
                .map(|(x, c)| (c, (y, x)))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn numbers(schematic: &[String]) -> Vec<Vec<(u32, Range<usize>)>> {
    schematic.iter().map(|row| number_ranges(row)).collect()
}

fn number_ranges(row: &str) -> Vec<(u32, Range<usize>)> {
    NUMBERS
        .captures_iter(row)
        .flat_map(|captures| captures.get(1))
        .map(|m| (m.as_str().parse::<u32>().unwrap(), m.range()))
        .collect()
}

type Coordinate = (usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn part2_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part2(input), 467835);
    }
}
