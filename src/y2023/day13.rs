pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input, 1000000));
}

fn part1(input: &str) -> usize {
    let patterns = parse(input);
    let mut sum = 0;
    for pattern in patterns.iter() {
        let res = x(&pattern);
        sum += res;
    }
    sum
}

fn part2(input: &str, multiplier: usize) -> usize {
    let patterns = parse(input);
    let mut sum = 0;
    for pattern in patterns.iter() {
    }
    0
}

fn x(pattern: &String) -> usize {
    let lines = pattern.lines().collect::<Vec<_>>();
    for v in 1..lines[0].len() {
        if let Some(i) = is_foldable_vertical(&lines, v) {
            return i;
        }
    }
    for h in 1..lines.len() {
        if let Some(i) = is_foldable_horizontal(&lines, h) {
            return i * 100;
        }
    }
    0
}

fn is_foldable_vertical(lines: &Vec<&str>, i: usize) -> Option<usize> {
    let len = i.abs_diff(0).min(lines[0].len().abs_diff(i));
    match lines
        .iter()
        .map(|line| (&line[i-len..i], &line[i..i+len]))
        .all(|(p1, p2)| p1.chars().zip(p2.chars().rev()).all(|(a, b)| a == b)) {
            true => Some(i),
            false => None
        }
}

fn is_foldable_horizontal(lines: &Vec<&str>, i: usize) -> Option<usize> {
    let len = i.abs_diff(0).min(lines.len().abs_diff(i));
    let p1 = &lines[i-len..i];
    let p2 = &lines[i..i+len];

    match p1.iter().zip(p2.iter().rev())
        .all(|(a, b)| **a == **b) {
            true => Some(i),
            false => None
        }
}

fn parse(input: &str) -> Vec<String> {
    input.split("\n\n").map(|part| part.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(part1(input), 405);
    }
}
