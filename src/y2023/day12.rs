pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let x = parse(input);
    let (row, conditions) = x.first().unwrap();
    is_valid(row, conditions);
    solve_row(row.clone(), conditions);
    0
}

fn part2(input: &str) -> usize {
    0
}

fn solve_row(row: String, conditions: &Vec<u8>) -> usize {
    if !row.contains('?') {
        return 1;
    }
    solve_row(row.replacen('?', ".", 1), conditions)
        + solve_row(row.replacen('?', "#", 1), conditions)
}

fn is_valid(row: &str, conditions: &Vec<u8>) -> bool {
    let mut iter = row.chars();
    let mut states: Vec<(State, u8)> = Vec::new();
    let mut prev_state = None;
    let mut state_len = 0;
    while let Some(next) = iter.next() {
        let current_state = match next {
            '#' => State::Damaged,
            '.' => State::Operational,
            '?' => State::Unknown,
            _ => panic!("Unexpected char")
        };
        if let Some(prev) = prev_state {
            if prev != current_state {
                states.push((prev, state_len));
                state_len = 0;
            }
        }
        prev_state = Some(current_state);
        state_len += 1;
    }
    if let Some(prev) = prev_state {
        states.push((prev, state_len));
    }

    for condition in conditions {

    }

    println!("{:?}", states);
    true
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
    Operational
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
