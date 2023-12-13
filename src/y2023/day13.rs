pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input, 1000000));
}

fn part1(input: &str) -> usize {
    let patterns = parse(input);
    for pattern in patterns.iter() {
        x(&pattern);
    }
    0
}

fn part2(input: &str, multiplier: usize) -> usize {
    0
}

fn x(pattern: &String) -> String {
    let lines = pattern.lines().collect::<Vec<_>>();
    if is_foldable_vertical(&lines, lines[0].len() / 2) || is_foldable_vertical(&lines, lines[0].len() / 2 + 1) {
        println!("Vertical!");
    }
    if is_foldable_horizontal(&lines, lines.len() / 2) || is_foldable_horizontal(&lines, lines.len() / 2 + 1) {
        println!("Horizontal!");
    }
    String::new()
}

fn is_foldable_vertical(lines: &Vec<&str>, i: usize) -> bool {
    let len = i.abs_diff(0).min(lines[0].len().abs_diff(i));
    lines
        .iter()
        .map(|line| (&line[i-len..i], &line[i..i+len]))
        .all(|(p1, p2)| p1.chars().zip(p2.chars().rev()).all(|(a, b)| a == b))
}

fn is_foldable_horizontal(lines: &Vec<&str>, i: usize) -> bool {
    let len = i.abs_diff(0).min(lines.len().abs_diff(i));
    let p1 = &lines[i-len..i];
    let p2 = &lines[i..i+len];

    p1.iter().zip(p2.iter().rev())
        .all(|(a, b)| **a == **b)
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
