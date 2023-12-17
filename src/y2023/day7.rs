use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

const CARDS: &str = "23456789TJQKA";

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut hands = parse(input, false);
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut hands = parse(input, true);
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
        .sum()
}

fn parse(input: &str, with_joker: bool) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').expect("No separator found");
            Hand::new(
                hand,
                bid.parse::<u64>().expect("Not able to parse bid as number"),
                with_joker,
            )
        })
        .collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    hand_type: HandType,
    card_score: Vec<u32>,
    pub bid: u64,
}

impl Hand {
    fn new(hand: &str, bid: u64, with_joker: bool) -> Self {
        let cards = hand.chars();
        Hand {
            hand_type: hand_type(hand, with_joker),
            card_score: cards.map(|card| card_score(card, with_joker)).collect_vec(),
            bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.card_score.cmp(&other.card_score),
            ordering => ordering,
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
            && self.card_score == other.card_score
            && self.bid == other.bid
    }
}
impl Eq for Hand {}

fn hand_type(hand: &str, with_joker: bool) -> HandType {
    let mut cards = hand.to_owned();
    if with_joker {
        let replacement = joker_replacement(&cards);
        cards = cards.replace('J', &replacement.to_string());
    }
    let mut cards = cards.chars().collect::<Vec<_>>();
    cards.sort();
    match cards.iter().collect::<HashSet<_>>().len() {
        1 => HandType::FiveOfAKind,
        2 => match cards[0] != cards[1] || cards[3] != cards[4] {
            true => HandType::FourOfAKind,
            false => HandType::FullHouse,
        },
        3 => match cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4] {
            true => HandType::ThreeOfAKind,
            false => HandType::TwoPair,
        },
        4 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn card_score(card: char, with_joker: bool) -> u32 {
    if card == 'J' && with_joker {
        return 0;
    }
    CARDS.chars().position(|c| c == card).unwrap() as u32 + 1
}

fn joker_replacement(hand: &str) -> char {
    let mut counts = HashMap::<char, usize>::new();
    for c in hand.chars().filter(|c| *c != 'J') {
        *counts.entry(c).or_insert(0) += 1;
    }
    if counts.is_empty() {
        return 'A';
    }
    let max_count = *counts.values().max().unwrap();
    counts
        .into_iter()
        .filter(|(_, count)| *count == max_count)
        .map(|(card, _)| (card, card_score(card, true)))
        .max_by(|(_, a_score), (_, b_score)| a_score.cmp(b_score))
        .map(|(card, _)| card)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part1(input), 6440);
    }

    #[test]
    fn part2_test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part2(input), 5905);
    }
}
