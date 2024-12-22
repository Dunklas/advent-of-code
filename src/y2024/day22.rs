pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut init_secrets : Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    for _ in 0..2000 {
        for secret in init_secrets.iter_mut() {
            *secret = next_secret(*secret);
        }
    }
    init_secrets.into_iter().sum()
}

fn part2(input: &str) -> usize {
    0
}

fn next_secret(value: usize) -> usize {
    let tmp = ((value * 64) ^ value) % 16777216;
    let tmp = ((tmp / 32) ^ tmp) % 16777216;
    let tmp = ((tmp * 2048) ^ tmp) % 16777216;
    tmp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1
10
100
2024";
        assert_eq!(part1(input), 37327623);
    }
}
