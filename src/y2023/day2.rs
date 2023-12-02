use std::{collections::HashMap, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ROUND_PART: Regex = Regex::new(r"(\d+) (red|blue|green)").unwrap();
}

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(Game::from_str)
        .filter(|game| {
            game.max_of("red") <= 12 && game.max_of("green") <= 13 && game.max_of("blue") <= 14
        })
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .flat_map(Game::from_str)
        .map(|game| {
            vec![
                game.max_of("red"),
                game.max_of("green"),
                game.max_of("blue"),
            ]
            .into_iter()
            .product::<u32>()
        })
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

struct Game {
    id: u32,
    rounds: Vec<HashMap<String, u32>>,
}

impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(':').collect::<Vec<&str>>();
        let raw_title = parts.first().ok_or(ParseGameError)?.replace("Game ", "");
        let raw_rounds = parts.get(1).ok_or(ParseGameError)?.split(';');
        let rounds = raw_rounds
            .into_iter()
            .map(|raw_round| {
                ROUND_PART
                    .captures_iter(raw_round)
                    .map(|caps| (caps[2].to_owned(), caps[1].parse::<u32>().unwrap()))
                    .collect()
            })
            .collect::<Vec<HashMap<String, u32>>>();
        let id = raw_title.parse::<u32>().map_err(|_| ParseGameError)?;
        Ok(Game { id, rounds })
    }
}

impl Game {
    pub fn max_of(&self, color: &str) -> u32 {
        match self.rounds.iter().flat_map(|round| round.get(color)).max() {
            Some(max) => max.to_owned(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, part1(input));
    }

    #[test]
    fn part2_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, part2(input));
    }
}
