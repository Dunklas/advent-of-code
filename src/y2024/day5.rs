use std::cmp::Ordering;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let (ordering_rules, manual_updates) = parse(input);
    manual_updates
        .into_iter()
        .filter(|update| update.is_sorted(&ordering_rules))
        .map(|update| update.middle_number())
        .sum()
}

fn part2(input: &str) -> usize {
    let (ordering_rules, manual_updates) = parse(input);
    manual_updates
        .into_iter()
        .filter(|update| !update.is_sorted(&ordering_rules))
        .map(|mut update| {
            update.sort(&ordering_rules);
            update.middle_number()
        })
        .sum()
}

fn parse(input: &str) -> (Vec<PageOrderingRule>, Vec<ManualUpdate>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    (
        parts[0]
            .lines()
            .map(PageOrderingRule::from)
            .collect::<Vec<_>>(),
        parts[1].lines().map(ManualUpdate::from).collect::<Vec<_>>(),
    )
}

struct PageOrderingRule {
    left: usize,
    right: usize,
}

impl From<&str> for PageOrderingRule {
    fn from(value: &str) -> Self {
        let parts = value
            .split("|")
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<_>>();
        Self {
            left: parts[0],
            right: parts[1],
        }
    }
}

struct ManualUpdate {
    pages: Vec<usize>,
}

impl ManualUpdate {
    fn middle_number(&self) -> usize {
        self.pages[(self.pages.len() - 1) / 2]
    }
    fn is_sorted(&self, ordering_rules: &[PageOrderingRule]) -> bool {
        ordering_rules.iter().all(|rule| {
            match (
                self.pages.iter().position(|&x| x == rule.left),
                self.pages.iter().position(|&x| x == rule.right),
            ) {
                (Some(a), Some(b)) => a <= b,
                _ => true,
            }
        })
    }
    fn sort(&mut self, ordering_rules: &[PageOrderingRule]) {
        self.pages.sort_by(|a, b| {
            match &ordering_rules
                .iter()
                .find(|rule| rule.left == *b && rule.right == *a)
            {
                Some(_) => Ordering::Greater,
                _ => Ordering::Less,
            }
        })
    }
}

impl From<&str> for ManualUpdate {
    fn from(value: &str) -> Self {
        Self {
            pages: value
                .split(",")
                .filter_map(|n| n.parse().ok())
                .collect::<Vec<_>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::y2024::day5::{part1, part2};

    const INPUT: &str = "47|53
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 123);
    }
}
