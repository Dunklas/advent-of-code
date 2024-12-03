use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CARD: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
}

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    CARD.captures_iter(input).into_iter()
        .map(|m| m.get(1).unwrap().as_str().parse::<usize>().unwrap() * m.get(2).unwrap().as_str().parse::<usize>().unwrap())
        .sum()
}

fn part2(input: &str) -> usize {
    0
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
        let input = "";
        assert_eq!(part2(input), 0);
    }
}