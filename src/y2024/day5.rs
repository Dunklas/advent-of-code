use std::ops::Index;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let (page_orderings, updates) = parse(input);

    updates.into_iter()
        .filter(|update| page_orderings.iter().filter_map(|(left, right)| {
            let left_res = update.iter().position(|n| n == left);
            let right_res = update.iter().position(|n| n == right);
            left_res.zip(right_res)
        }).all(|(left, right)| left < right))
        .map(|correct_update| correct_update[(correct_update.len() - 1) / 2])
        .sum()
}

fn part2(input: &str) -> usize {
    0
}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();

    let page_orderings = parts[0].lines()
        .map(|line| line.split("|").filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<_>>())
        .map(|x| (x[0], x[1]))
        .collect::<Vec<_>>();

    let updates = parts[1].lines()
        .map(|line| line.split(",").filter_map(|n| n.parse().ok()).collect::<Vec<_>>())
    .collect::<Vec<_>>();

    (page_orderings, updates)
}

#[cfg(test)]
mod tests {
    use crate::y2024::day5::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(part1(input), 143);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), 0);
    }
    }