use std::borrow::ToOwned;
use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut stones = parse(input);
    let mut count = 0;
    let mut cache: HashMap<(String, usize), usize> = HashMap::new();

    for stone in stones {
        count += blink3(stone, 25, &mut cache);
    }
    count
}

fn part2(input: &str) -> usize {
    let mut stones = parse(input);
    let mut count = 0;
    let mut cache: HashMap<(String, usize), usize> = HashMap::new();

    for stone in stones {
        count += blink3(stone, 75, &mut cache);
    }
    count
}

fn blink3(stone: String, remaining_blinks: usize, cache: &mut HashMap<(String, usize), usize>) -> usize {
    if remaining_blinks == 0 {
        return 1;
    }
    let res = match stone.as_str() {
        "0" => {
            vec!["1".to_owned()]
        }
        stone if stone.len() % 2 == 0 => {
            let (left, right) = stone.split_at(stone.len() / 2);
            vec![left.parse::<usize>().unwrap().to_string(), right.parse::<usize>().unwrap().to_string()]
        }
        _ => {
            let old_n = stone.parse::<usize>().unwrap();
            vec![(old_n * 2024).to_string()]
        }
    };
    if remaining_blinks == 1 {
        return res.len();
    }
    res.into_iter()
        .map(|x| {
            match cache.get(&(x.clone(), remaining_blinks - 1)) {
                Some(&n) => n,
                None => {
                    let res = blink3(x.clone(), remaining_blinks - 1, cache);
                    cache.insert((x, remaining_blinks - 1), res);
                    res
                }
            }
        })
        .sum()
}

fn parse(input: &str) -> Vec<String> {
    input.split_whitespace().map(|c| c.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 55312);
    }
}
