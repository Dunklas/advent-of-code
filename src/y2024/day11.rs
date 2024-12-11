use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let stones = parse(input).unwrap();
    observe_stones(stones, 25)
}

fn part2(input: &str) -> usize {
    let stones = parse(input).unwrap();
    observe_stones(stones, 75)
}

type MemoizationCache = HashMap<(usize, usize), usize>;

fn observe_stones(stones: Vec<usize>, num_blinks: usize) -> usize {
    let mut cache = MemoizationCache::new();
    stones
        .into_iter()
        .map(|stone| blink(stone, num_blinks, &mut cache))
        .sum()
}

fn blink(stone: usize, remaining_blinks: usize, cache: &mut MemoizationCache) -> usize {
    if remaining_blinks == 0 {
        return 1;
    }
    if let Some(result) = cache.get(&(stone, remaining_blinks)) {
        return *result;
    }
    let result = match stone {
        0 => blink(1, remaining_blinks - 1, cache),
        stone if num_digits(stone) % 2 == 0 => {
            let (left, right) = split_number(stone);
            blink(left, remaining_blinks - 1, cache) + blink(right, remaining_blinks - 1, cache)
        }
        _ => blink(stone * 2024, remaining_blinks - 1, cache),
    };
    cache.insert((stone, remaining_blinks), result);
    result
}

fn num_digits(num: usize) -> usize {
    (num.ilog10() + 1) as usize
}

fn split_number(num: usize) -> (usize, usize) {
    let divisor = 10_usize.pow((num_digits(num) / 2) as u32);
    (num / divisor, num % divisor)
}

fn parse(input: &str) -> Result<Vec<usize>, std::num::ParseIntError> {
    input
        .split_whitespace()
        .map(|c| c.parse::<usize>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 65601038650482);
    }
}
