pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|report| report.is_safe())
        .count()
}
fn part2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|report| report.is_safe_problem_dampened())
        .count()
}

struct Report {
    levels: Vec<usize>,
}

impl Report {
    fn is_safe(&self) -> bool {
        is_safe_increasing(&self.levels) || is_safe_increasing(&reversed(&self.levels))
    }

    fn is_safe_problem_dampened(&self) -> bool {
        if self.is_safe() {
            return true;
        }
        (0..self.levels.len())
            .map(|i| {
                self.levels
                    .iter()
                    .enumerate()
                    .filter(|&(index, _)| index != i)
                    .map(|(_, &value)| value)
                    .collect::<Vec<_>>()
            })
            .any(|r| is_safe_increasing(&r) || is_safe_increasing(&reversed(&r)))
    }
}

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        Self {
            levels: value
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect(),
        }
    }
}

fn is_safe_increasing(input: &[usize]) -> bool {
    !input
        .windows(2)
        .map(|pair| pair[0] as isize - pair[1] as isize)
        .any(|diff| diff.is_positive() || diff == 0 || diff < -3)
}

fn reversed(report: &[usize]) -> Vec<usize> {
    report.iter().rev().copied().collect()
}

fn parse(input: &str) -> Vec<Report> {
    input.lines().map(Report::from).collect()
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
