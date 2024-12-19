use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let (towels, designs) = parse(input);
    let towel_sizes = towels.iter().map(|t| t.len()).collect::<HashSet<usize>>();
    let mut cache: HashMap<String, usize> = HashMap::new();
    designs
        .into_iter()
        .filter(|design| num_possible(design, &towel_sizes, &towels, &mut cache) > 0)
        .count()
}
fn part2(input: &str) -> usize {
    let (towels, designs) = parse(input);
    let towel_sizes = towels.iter().map(|t| t.len()).collect::<HashSet<usize>>();
    let mut cache: HashMap<String, usize> = HashMap::new();
    designs
        .into_iter()
        .map(|design| num_possible(&design, &towel_sizes, &towels, &mut cache))
        .sum()
}

fn num_possible(
    design: &str,
    towel_sizes: &HashSet<usize>,
    towels: &HashSet<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(num_possible) = cache.get(design) {
        return *num_possible;
    }
    let num_possible = towel_sizes
        .iter()
        .filter(|&size| *size <= design.len())
        .map(|size| (&design[0..*size], &design[*size..]))
        .filter(|(prefix, _)| towels.contains(*prefix))
        .map(|(_, next)| num_possible(next, towel_sizes, towels, cache))
        .sum();
    cache.insert(design.to_string(), num_possible);
    num_possible
}

fn parse(input: &str) -> (HashSet<String>, Vec<String>) {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    (
        sections[0]
            .split(",")
            .map(|s| s.trim().to_owned())
            .collect(),
        sections[1].lines().map(|s| s.to_owned()).collect(),
    )
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
        assert_eq!(part2(INPUT), 16);
    }
}
