use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NODE: Regex =
        Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();
}

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let (steps, nodes) = parse(input);
    let mut current = nodes.get("AAA").unwrap();
    let mut num_steps = 0;
    for step in steps.iter().cycle() {
        let next = match *step {
            'L' => &current.0,
            'R' => &current.1,
            _ => {
                panic!("Unexpected step");
            }
        };
        current = nodes.get(next).unwrap();
        num_steps += 1;
        if next == "ZZZ" {
            break;
        }
    }
    num_steps
}

fn part2(input: &str) -> usize {
    let (steps, nodes) = parse(input);
    nodes
        .keys()
        .filter(|id| id.ends_with('A'))
        .map(|start| find_step(start, &steps, &nodes))
        .fold(1usize, |res, n| lcm(n, res))
}

fn find_step(start: &str, steps: &[char], nodes: &HashMap<String, (String, String)>) -> usize {
    let mut current = nodes.get(start).unwrap();
    let mut end_indexes = vec![];
    for (i, step) in steps.iter().cycle().enumerate() {
        let (left, right) = current;
        let next = match *step {
            'L' => left,
            'R' => right,
            _ => panic!("Unexpected step!"),
        };
        current = nodes.get(next).unwrap();
        if next.ends_with('Z') {
            end_indexes.push(i + 1);
        }
        if end_indexes.len() == 2 {
            break;
        }
    }
    end_indexes[1] - end_indexes[0]
}

fn parse(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut parts = input.split("\n\n");
    let steps = parts.next().unwrap().chars().collect::<Vec<_>>();
    (
        steps,
        parts
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let caps = NODE.captures(line).unwrap();
                (caps[1].to_owned(), (caps[2].to_owned(), caps[3].to_owned()))
            })
            .collect::<HashMap<_, _>>(),
    )
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return a + b;
    }
    gcd(a.max(b) % a.min(b), a.min(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let input2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input), 2);
        assert_eq!(part1(input2), 6);
    }

    #[test]
    fn part2_test() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part2(input), 6);
    }
}
