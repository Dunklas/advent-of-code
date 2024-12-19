use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize{
    let (towels, designs) = parse(input);
    println!("TOWELS: {:?}", towels);
    let towel_sizes = towels.iter()
        .map(|t| t.len())
        .collect::<HashSet<usize>>();
    designs.into_iter()
        .filter(|design| is_possible(design, &towel_sizes, &towels))
        .count()
}
fn part2(input: &str) -> usize{
    0
}

fn is_possible(design: &str, sizes: &HashSet<usize>, towels: &HashSet<String>) -> bool {
    if design == "" {
        return true;
    }
    for size in sizes.iter() {
        if *size > design.len() {
            continue;
        }
        let part = &design[0..*size];
        if towels.contains(part) {
            let next_part = &design[*size..];
            if is_possible(next_part, sizes, towels) {
                return true;
            }
        }
    };
    false
}

fn parse(input: &str) -> (HashSet<String>, Vec<String>) {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let towels = sections[0].split(",").map(|s| s.trim().to_owned()).collect::<HashSet<_>>();
    let designs = sections[1].lines()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    (towels, designs)
}

#[derive(Default, Debug)]
struct Node {
    end: bool,
    children: HashMap<char, Node>
}

#[derive(Default, Debug)]
struct Trie {
    root: Node
}

impl Trie {
    pub fn new() -> Trie {
        Self { root: Node::default()}
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.end = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = &self.root;
        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        current_node.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 6);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(INPUT), 16);
    }
}
