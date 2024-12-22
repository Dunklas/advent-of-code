use itertools::{multizip, Itertools};
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut init_secrets: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    for _ in 0..2000 {
        for secret in init_secrets.iter_mut() {
            *secret = next_secret(*secret);
        }
    }
    init_secrets.into_iter().sum()
}

fn part2(input: &str) -> usize {
    let mut init_secrets: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    println!("Input read");
    let secret_sequences = init_secrets
        .into_iter()
        .map(|init| unnamed(init, 2000))
        .collect::<Vec<_>>();
    println!("Unnamed collected");
    let possible_sequences = secret_sequences.iter().flat_map(|x| x.1.keys().collect_vec()).collect_vec();
    let mut max = 0;
    for (i, seq) in possible_sequences.iter().enumerate() {
        println!("{} / {}", i, possible_sequences.len());
        let sum = secret_sequences
            .iter()
            .map(|x| x.1.get(*seq).unwrap_or(&0))
            .sum();
        if sum > max {
            max = sum;
        }
    }
    max
}

fn unnamed(secret: usize, n: usize) -> (Vec<usize>, HashMap<Vec<isize>, usize>) {
    let mut prices = Vec::new();
    prices.push(secret % 10);
    let mut tmp = secret;
    for i in 0..n {
        tmp = next_secret(tmp);
        prices.push(tmp % 10);
    }
    let mut price_changes = HashMap::new();
    prices.iter().enumerate().skip(1)
        .collect::<Vec<_>>()
        .windows(4)
        .for_each(|window| {
            let mut seq = Vec::new();
            window.into_iter().for_each(|&(i, val)| {
                seq.push((prices[i] as isize) - (prices[i - 1] as isize));
            });
            if !price_changes.contains_key(&seq) {
                price_changes.insert(seq.clone(), *window.last().unwrap().1);
            }
        });
    (prices, price_changes)
}

fn next_secrets(secret: usize, n: usize) -> Vec<usize> {
    let mut secrets = Vec::new();
    secrets.push(secret);
    for i in 0..n {
        secrets.push(next_secret(secrets[i]));
    }
    secrets
}

fn next_secret(value: usize) -> usize {
    let tmp = ((value * 64) ^ value) % 16777216;
    let tmp = ((tmp / 32) ^ tmp) % 16777216;
    let tmp = ((tmp * 2048) ^ tmp) % 16777216;
    tmp
}

fn find_sequences(secrets: &Vec<usize>) -> Vec<Vec<isize>> {
    let mut sequences = Vec::new();
    secrets
        .iter()
        .enumerate()
        .skip(1)
        .collect::<Vec<_>>()
        .windows(4)
        .for_each(|window| {
            let mut seq = Vec::new();
            window.into_iter().for_each(|&(i, val)| {
                seq.push((secrets[i] % 10) as isize - (secrets[i - 1] % 10) as isize)
            });
            sequences.push(seq);
        });
    sequences
}

fn find_sequence_val(secrets: &Vec<usize>, seq: &Vec<isize>) -> usize {
    let mut num_matched = 0;
    for i in 1..secrets.len() {
        let diff = (secrets[i] % 10) as isize - (secrets[i - 1] % 10) as isize;
        if diff == seq[num_matched] {
            num_matched += 1;
        } else {
            num_matched = 0;
        }

        if num_matched == seq.len() {
            return (secrets[i] % 10);
        }
    }
    0
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

    #[test]
    fn test_part2() {
        let input = "1
2
3
2024";
        assert_eq!(part2(input), 23);
    }

    fn test_lab() {
        let (a, b) = unnamed(123, 10);
        println!("A: {:?}", a);
        println!("B: {:?}", b);
        assert_eq!(true, false);
    }
}
