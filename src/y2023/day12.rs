pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let x = parse(input);
    println!("{:?}", x);
    0
}

fn part2(input: &str) -> usize {
    0
}

fn parse(input: &str) -> Vec<(String, Vec<u8>)> {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(row, conditions)| {
            (
                row.to_owned(),
                conditions
                    .split(',')
                    .into_iter()
                    .map(|n| n.parse::<u8>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part1(input), 21);
    }
}
