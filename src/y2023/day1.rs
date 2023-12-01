use std::str::Chars;

const DIGITS: [(&str, u32); 9] = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    input.lines()
        .map(|line| {
            let mut x = line.chars();
            let first = next_numeric(&mut x, false).unwrap();
            let mut second = Option::None;
            while let Some(d) = next_numeric(&mut x, false) {
                second = Option::Some(d);
            }
            let b = format!("{}{}", first.to_string(), second.unwrap_or(first));
            return b;
        })
        .flat_map(|x| x.parse::<i32>())
        .sum()
}

fn part2(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let mut x = line.chars();
            let first = next_numeric(&mut x, true).unwrap();
            let mut second = Option::None;
            while let Some(digit) = next_numeric(&mut x, true) {
                second = Option::Some(digit);
            }
            let b = format!("{}{}", first.to_string(), second.unwrap_or(first));
            // println!("{}", b);
            return b;
        })
        .flat_map(|x| x.parse::<u32>())
        .sum()
}

fn next_numeric(c: &mut Chars<'_>, with_string: bool) -> Option<u32> {
    let mut x = String::new();
    while let Some(c) = c.next() {
        if c.is_numeric() {
            return c.to_digit(10);
        }
        if !with_string {
            continue;
        }
        x.push(c);
        for (d, numeric) in DIGITS {
            if x.contains(d) {
                return Option::Some(numeric);
            }
        }
    }
    Option::None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmp() {
        let mut test = "one2three4five6".chars();
        assert_eq!(Some(1u32), next_numeric(&mut test));
        assert_eq!(Some(2u32), next_numeric(&mut test));
        assert_eq!(Some(3u32), next_numeric(&mut test));
        assert_eq!(Some(4u32), next_numeric(&mut test));
        assert_eq!(Some(5u32), next_numeric(&mut test));
        assert_eq!(Some(6u32), next_numeric(&mut test));
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
 