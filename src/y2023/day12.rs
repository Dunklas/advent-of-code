pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let x = parse(input);
    x.into_iter()
        .map(|(row, conditions)| solve_row(row, &conditions))
        .sum()
}

fn part2(input: &str) -> usize {
    0
}

fn solve_row(row: String, conditions: &Vec<u8>) -> usize {
    if !is_valid(&row, conditions) {
        return 0;
    }
    if !row.contains('?') {
        println!("{:?}", row);
        return 1;
    }
    solve_row(row.replacen('?', ".", 1), conditions)
        + solve_row(row.replacen('?', "#", 1), conditions)
}

fn is_valid(row: &str, conditions: &Vec<u8>) -> bool {
    let mut iter = row.chars();
    let mut condition_iter = conditions.iter();
    let mut prev_state = None;
    let mut state_len = 0;
    while let Some(next) = iter.next() {
        let current_state = match next {
            '#' => State::Damaged,
            '.' => State::Operational,
            '?' => {
                return true;
            }
            _ => panic!("Unexpected char"),
        };
        if let Some(prev) = prev_state {
            if prev != current_state {
                if prev == State::Damaged {
                    match condition_iter.next() {
                        Some(condition) => {
                            if state_len != *condition {
                                return false;
                            }
                        }
                        None => {
                            return false;
                        }
                    }
                }
                state_len = 0;
            }
        }
        prev_state = Some(current_state);
        state_len += 1;
    }
    if let Some(prev) = prev_state {
        if prev == State::Damaged {
            match condition_iter.next() {
                Some(condition) => {
                    if state_len > *condition {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }
    match condition_iter.next() {
        None => true,
        _ => false,
    }
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

#[derive(PartialEq, Copy, Clone, Debug)]
enum State {
    Unknown,
    Damaged,
    Operational,
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
