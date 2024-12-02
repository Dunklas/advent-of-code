use itertools::Itertools;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let levels = parse(input);
    levels.into_iter()
        .filter(|report| is_safe(report))
        .count()
}
fn part2(input: &str) -> usize {
    let levels = parse(input);
    let mut safe_count = 0;
    for level in levels {
        if is_safe(&level) {
            safe_count += 1;
            continue;
        }

        let skipped: Vec<Vec<usize>> = (0..level.len())
            .map(|i| {
                level.iter()
                    .enumerate()
                    .filter(|&(index, _)| index != i)
                    .map(|(_, &value)| value)
                    .collect()
            })
            .collect();

        if skipped.into_iter().any(|r| is_safe(&r)) {
           safe_count += 1;
        }
    }
    safe_count
}

fn is_safe(report: &Vec<usize>) -> bool {
    let mut x = report.clone();
    x.sort_unstable();
    let y = x.iter().cloned().rev().collect::<Vec<usize>>();
    if x != *report && y != *report {
        return false;
    }
    for pair in report.windows(2) {
        let diff = pair[0].abs_diff(pair[1]);
        if diff == 0 || diff > 3 {
            return false;
        }
    }
    println!("SAFE: {:?}", report);
    true
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|line| line.split_whitespace().map(|value| value.parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::y2024::day2::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part2(input), 4);
    }
}