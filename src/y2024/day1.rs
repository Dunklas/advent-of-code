pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let mut first = Vec::new();
    let mut second = Vec::new();
    input.lines().map(|line| line.split_whitespace().collect::<Vec<_>>())
        .for_each(|row| {
            first.push(row[0].parse::<usize>().unwrap());
            second.push(row[1].parse::<usize>().unwrap());
        });
    first.sort();
    second.sort();
    first.iter().zip(second.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::y2024::day1::part1;

    #[test]
    fn test_part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part1(input), 11);
    }
    fn test_part2() {

    }
}