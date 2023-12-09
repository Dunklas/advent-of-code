pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    let histories = parse(input);
    histories.into_iter().map(|history| next(&history)).sum()
}

fn part2(input: &str) -> i64 {
    let histories = parse(input);
    histories.into_iter().map(|history| prev(&history)).sum()
}

fn next(history: &[i64]) -> i64 {
    if history.iter().all(|n| *n == 0) {
        return 0;
    }
    let diffs = history
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<_>>();
    history.last().unwrap() + next(&diffs)
}

fn prev(history: &[i64]) -> i64 {
    if history.iter().all(|n| *n == 0) {
        return 0;
    }
    let diffs = history
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<_>>();
    history.first().unwrap() - prev(&diffs)
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|raw| raw.parse::<_>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part1(input), 114);
    }

    #[test]
    fn part2_test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part2(input), 2);
    }
}
