use std::borrow::ToOwned;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut stones = parse(input);
    for i in 0..25 {
        let even = stones.iter().filter(|x| x.len() % 2 == 0).count();
        let odd = stones.iter().filter(|x| x.len() % 2 != 0).count();
        stones = blink(stones);
        println!("Predicted: {}, Actual: {}", even * 2 + odd, stones.len());
    }
    stones.len()
}

fn part2(input: &str) -> usize {
    let mut stones = parse(input);
    stones.len()
}

fn blink(stones: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for (i, stone) in stones.into_iter().enumerate() {
        match stone.as_str() {
            "0" => {
                result.push("1".to_owned());
            },
            stone if stone.len() % 2 == 0 => {
                let (left, right) = stone.split_at(stone.len() / 2);
                result.push(left.parse::<usize>().unwrap().to_string());
                result.push(right.to_string().parse::<usize>().unwrap().to_string());
            },
            _ => {
                let old_n = stone.parse::<usize>().unwrap();
                result.push((old_n * 2024).to_string());
            }
        }
    }
    result
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
