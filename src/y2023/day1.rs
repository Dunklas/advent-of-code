use std::{iter::Peekable, str::Chars};

const DIGITS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.chars().peekable())
        .flat_map(|mut line| parse_calibration_value(&mut line, false))
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.chars().peekable())
        .flat_map(|mut line| parse_calibration_value(&mut line, true))
        .sum()
}

fn parse_calibration_value(line: &mut Peekable<Chars<'_>>, with_letters: bool) -> Option<u32> {
    let first = match next_numeric(line, with_letters) {
        Some(numeric) => numeric,
        None => {
            return None;
        }
    };
    let mut second = first;
    while let Some(digit) = next_numeric(line, with_letters) {
        second = digit;
    }
    format!("{}{}", first, second).parse::<u32>().ok()
}

fn next_numeric(line: &mut Peekable<Chars<'_>>, with_letters: bool) -> Option<u32> {
    let mut letters = String::new();
    while let Some(c) = line.next() {
        if c.is_numeric() {
            return c.to_digit(10);
        }
        if !with_letters {
            continue;
        }
        letters.push(c);

        let tmp = match line.peek() {
            Some(next) => format!("{}{}", &letters, next),
            None => letters.clone(),
        };
        for (numeric_str, numeric) in DIGITS {
            if tmp.contains(numeric_str) {
                return Some(numeric);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping() {
        let mut test = "oneight".chars().peekable();
        assert_eq!(Some(1u32), next_numeric(&mut test, true));
        assert_eq!(Some(8u32), next_numeric(&mut test, true));
        assert_eq!(None, next_numeric(&mut test, true));
    }

    #[test]
    fn letters() {
        let mut test = "onetwothreefourfivesixseveneightnine".chars().peekable();
        assert_eq!(Some(1u32), next_numeric(&mut test, true));
        assert_eq!(Some(2u32), next_numeric(&mut test, true));
        assert_eq!(Some(3u32), next_numeric(&mut test, true));
        assert_eq!(Some(4u32), next_numeric(&mut test, true));
        assert_eq!(Some(5u32), next_numeric(&mut test, true));
        assert_eq!(Some(6u32), next_numeric(&mut test, true));
        assert_eq!(Some(7u32), next_numeric(&mut test, true));
        assert_eq!(Some(8u32), next_numeric(&mut test, true));
        assert_eq!(Some(9u32), next_numeric(&mut test, true));
    }

    #[test]
    fn part_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, part1(input));
    }

    #[test]
    fn part_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, part2(input));
    }
}
