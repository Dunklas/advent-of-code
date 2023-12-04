use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CARD: Regex = Regex::new(r"Card +(\d+): ([\d ]+) \| ([\d ]+)").unwrap();
}

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    parse(input)
        .into_iter()
        .map(|game| num_matching(&game))
        .map(|num_matching| (0..num_matching).fold(0, |score, _| (score * 2).max(1)))
        .sum()
}

fn part2(input: &str) -> u32 {
    let games = parse(input).into_iter().collect::<Vec<_>>();
    let mut card_counts = games
        .iter()
        .map(|game| (game.0, 1))
        .collect::<HashMap<_, _>>();
    for game in games.iter() {
        let num_cards = *card_counts.get(&game.0).unwrap();
        for n in 0..num_matching(game) {
            let id = game.0 + (n as u32) + 1;
            *card_counts.entry(id).or_default() += num_cards;
        }
    }
    card_counts.values().sum()
}

fn num_matching(card: &(u32, HashSet<u32>, Vec<u32>)) -> usize {
    card.2.iter().filter(|n| card.1.contains(n)).count()
}

fn parse(input: &str) -> Vec<(u32, HashSet<u32>, Vec<u32>)> {
    input
        .lines()
        .filter_map(|line| {
            CARD.captures(line).map(|cap| {
                (
                    cap[1].parse::<u32>().ok().unwrap(),
                    cap[2]
                        .split(' ')
                        .filter_map(|n| n.parse::<u32>().ok())
                        .collect(),
                    cap[3]
                        .split(' ')
                        .filter_map(|n| n.parse::<u32>().ok())
                        .collect(),
                )
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part2(input), 30);
    }
}
