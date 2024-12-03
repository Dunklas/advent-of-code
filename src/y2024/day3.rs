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
    MUL.captures_iter(input).into_iter()
        .map(|m| m.get(1).unwrap().as_str().parse::<usize>().unwrap() * m.get(2).unwrap().as_str().parse::<usize>().unwrap())
        .sum()
}

fn part2(input: &str) -> usize {
    let mut enabled = true;
    let mut result = 0;
    let c = ALL.captures_iter(input).into_iter().for_each(|m| {
        match m.get(0) {
            Some(m) => {
                let ins = m.as_str();
                if let(Some(x)) = MUL.captures(ins) {
                    if enabled {
                        result += (x.get(1).unwrap().as_str().parse::<usize>().unwrap() * x.get(2).unwrap().as_str().parse::<usize>().unwrap());
                    }
                }
                if ins == "do()" {
                    enabled = true;
                }
                if ins == "don't()" {
                   enabled = false;
                }
            }
            None => {}
        }
    });
    result
}

#[cfg(test)]
mod tests {
    use crate::y2024::day3::{part1, part2};

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