use std::cmp::Ordering;
use std::collections::VecDeque;
use std::ops::Range;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 1: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let result = move_blocks(input);
    checksum(result)
}

fn part2(input: &str) -> usize {
    0
}

fn checksum(file_system: Vec<(Content, Range<usize>)>) -> usize {
    let mut sum = 0;
    let mut i = 0usize;
    for (c, range) in file_system {
        if let Content::File(id) = c {
            for _ in range {
                sum += id * i;
                i += 1;
            }
        }
    }
    sum
}

fn move_blocks(file_system: Vec<(Content, Range<usize>)>) -> Vec<(Content, Range<usize>)> {
    let mut initial: VecDeque<_> = file_system.into_iter().collect();
    let mut result = Vec::new();
    let mut current_end = 0usize;
    while let Some((c1, mut r1)) = initial.pop_front() {
        match c1 {
            Content::Free => {
                while let Some((c2, r2)) = initial.pop_back() {
                    if c2 == Content::Free || r2.is_empty() {
                        continue;
                    }
                    match r1.len().cmp(&r2.len()) {
                        Ordering::Less => {
                            result.push((c2, current_end..current_end + r1.len()));
                            current_end += r1.len();
                            initial.push_back((c2, r2.start..r2.end - r1.len()));
                            break;
                        }
                        Ordering::Equal => {
                            result.push((c2, current_end..current_end + r1.len()));
                            current_end += r1.len();
                            break;
                        }
                        Ordering::Greater => {
                            r1.start += r2.len();
                            result.push((c2, current_end..current_end + r2.len()));
                            current_end += r2.len();
                        }
                    }
                }
            }
            _ => {
                current_end += r1.len();
                result.push((c1, r1));
            }
        }
    }
    result
}

fn parse(input: &str) -> Vec<(Content, Range<usize>)> {
    let mut result = Vec::new();
    let mut current = 0;
    let mut id_seq = 0;
    input.chars().enumerate().for_each(|(i, c)| {
        if let Some(value) = c.to_digit(10) {
            if i % 2 == 0 {
                result.push((Content::File(id_seq), current..current + value as usize));
                id_seq += 1;
            } else {
                result.push((Content::Free, current..current + value as usize));
            }
            current += value as usize;
        }
    });
    result
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Content {
    File(usize),
    Free,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
