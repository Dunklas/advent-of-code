use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MUL: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    static ref ALL: Regex = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();
}

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    find_all(input).into_iter()
        .map(|instruction| match instruction {
            Instruction::MUL(a, b) => a * b,
            _ => 0
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let instructions = find_all(input);
    let mut enabled = true;
    let mut result = 0;
    for instruction in instructions {
        match instruction {
            Instruction::MUL(a, b) => {
                if enabled {
                    result += a * b;
                }
            },
            Instruction::DO => {
                enabled = true;
            }
            Instruction::DONT => {
                enabled = false;
            }
        }
    }
    result
}

fn find_all(input: &str) -> Vec<Instruction> {
    ALL.find_iter(input).filter_map(|m| match MUL.captures(m.as_str()) {
        Some(cap) => Some(Instruction::MUL(cap.get(1).unwrap().as_str().parse().unwrap(), cap.get(2).unwrap().as_str().parse().unwrap())),
        None => {
            match m.as_str() {
                "do()" => Some(Instruction::DO),
                "don't()" => Some(Instruction::DONT),
                _ => None
            }
        }
    }).collect()
}

#[derive(Debug)]
enum Instruction {
    MUL(usize, usize),
    DO,
    DONT
}

#[cfg(test)]
mod tests {
    use crate::y2024::day3::{find_all, part1, part2};

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), 161);
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), 48);
    }
}