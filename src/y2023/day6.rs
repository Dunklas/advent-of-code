use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBERS: Regex = Regex::new(r"(\d+)").unwrap();
}

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let parsed = parse(input);
    let races: Vec<_> = (0..parsed[0].len())
        .map(|i| (parsed[0][i], parsed[1][i]))
        .collect();
    races.iter().map(num_wins).product()
}

fn part2(input: &str) -> u64 {
    let race: Vec<_> = parse(input)
        .into_iter()
        .map(|l| {
            l.into_iter()
                .fold(String::new(), |s, x| format!("{}{}", s, x))
                .parse::<u64>()
                .unwrap()
        })
        .collect();
    num_wins(&(race[0], race[1]))
}

fn num_wins(race: &(u64, u64)) -> u64 {
    let (race_length, record) = race;
    (1..*race_length).fold(0, |num_wins, press_duration| {
        num_wins
            + match (race_length - press_duration) * press_duration > *record {
                true => 1,
                false => 0,
            }
    })
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            NUMBERS
                .captures_iter(line)
                .flat_map(|cap| cap.get(1).map(|n| n.as_str().parse::<u64>().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn part2_test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part2(input), 71503);
    }
}
