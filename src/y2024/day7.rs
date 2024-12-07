use std::error::Error;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    parse(input)
        .unwrap()
        .into_iter()
        .filter(|(result, operands)| evaluate(*result, operands, 0, false) >= 1)
        .map(|(result, _)| result)
        .sum()
}

fn part2(input: &str) -> usize {
    parse(input)
        .unwrap()
        .into_iter()
        .filter(|(result, operands)| evaluate(*result, operands, 0, true) >= 1)
        .map(|(result, _)| result)
        .sum()
}

fn evaluate(value: usize, operands: &[usize], current: usize, concat_enabled: bool) -> usize {
    let next = operands[0];
    let sum = current + next;
    let product = current * next;
    let concat = format!("{}{}", current, next).parse::<usize>().unwrap();
    if sum == value || product == value || (concat_enabled && concat == value) {
        return 1;
    }
    let next_operands = &operands[1..];
    if next_operands.is_empty() {
        return 0;
    }
    evaluate(value, next_operands, sum, concat_enabled)
        + evaluate(value, next_operands, product, concat_enabled)
        + match concat_enabled {
            true => evaluate(value, next_operands, concat, concat_enabled),
            false => 0,
        }
}

fn parse(input: &str) -> Result<Vec<(usize, Vec<usize>)>, Box<dyn Error>> {
    Ok(input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(":");
            let value = parts.next().and_then(|n| n.parse::<usize>().ok());
            let operands = parts.next().map(|raw| {
                raw.split_whitespace()
                    .filter_map(|w| w.parse::<usize>().ok())
                    .collect::<Vec<_>>()
            });
            value.zip(operands)
        })
        .collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use crate::y2024::day7::{part1, part2};
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 11387);
    }
}
