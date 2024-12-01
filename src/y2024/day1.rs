use std::error::Error;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let (first, second) = parse(input).unwrap();
    first.iter().zip(second.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn part2(input: &str) -> usize {
    let (first, second) = parse(input).unwrap();
    first.iter()
        .map(|n| {
            second.iter().filter(|n2| n == *n2).count() * n
        })
        .sum()
}

fn parse(input: &str) -> Result<(Vec<usize>, Vec<usize>), Box<dyn Error>> {
    let mut first = Vec::new();
    let mut second = Vec::new();

    input.lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            Some((
                parts.next()?.parse::<usize>().ok()?,
                parts.next()?.parse::<usize>().ok()?,
            ))
        })
        .for_each(|(f, s)| {
            first.push(f);
            second.push(s);
        });

    first.sort_unstable();
    second.sort_unstable();

    Ok((first, second))
}

#[cfg(test)]
mod tests {
    use crate::y2024::day1::{part1, part2};

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
    #[test]
    fn test_part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part2(input), 31);
    }
}