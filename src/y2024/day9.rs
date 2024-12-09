use std::fmt::Formatter;
use std::ops::Index;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 1: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    let mut input = parse(input);
    move_blocks(&mut input);
    checksum(&input)
}

fn part2(input: &str) -> usize {
    let mut input = parse(input);
    move2(&mut input);
    for c in input {
        println!("{}", c);
    }
    0
}

fn move2(file_system: &mut Vec<i64>) {
    // TODO: use ranges?
    let clone = file_system.clone();
    let mut iter = file_system.iter().rev().peekable();
    let mut current_id = None;
    let mut current_size = 0;
    while let Some(current) = iter.next() {
        if *current == -1 {
            continue;
        }
        current_id = Some(*current);
        current_size += 1;
        if let Some(next_id) = iter.peek() {
            if Some(**next_id) == current_id {
                continue;
            }
        }
    }
}

fn move_blocks(file_system: &mut Vec<i64>) {
    for i in (0..file_system.len()).rev() {
        if file_system[i] != -1 {
            if let Some(left_most) = file_system.iter().enumerate().position(|(i, &b)| b == -1) {
                if left_most >= i {
                    break;
                }
                file_system.swap(i, left_most);
            }
        }
    }
}

fn checksum(file_system: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for i in 0..file_system.len() {
        if file_system[i] == -1 {
            continue;
        }
        let id = file_system[i];
        sum += (id * i as i64);
    }
    sum
}

fn parse(input: &str) -> Vec<i64> {
    let mut result = Vec::new();
    let mut id_seq = 0;
    input.chars().enumerate().for_each(|(i, c)| {
        if let Some(value) = c.to_digit(10) {
            if i % 2 == 0 {
                // File
                for _ in 0..value {
                    result.push(id_seq);
                }
                id_seq += 1;
            } else {
                // Free space
                for _ in 0..value {
                    result.push(-1);
                }
            }
        }
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    fn test_part1() {
        assert_eq!(part1(INPUT), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2858);
    }
}
