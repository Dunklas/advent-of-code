use std::ops::Index;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let mut input = parse(input);
    println!("Original: {:?}", input);
    move_blocks(&mut input);
    println!("Moved: {:?}", input);
    0
}

fn part2(input: &str) -> usize {
    0
}

fn move_blocks(file_system: &mut Vec<char>) {
    for i in (0..file_system.len()).rev() {
        if file_system[i] != '.' {
            if let Some(left_most) = file_system.iter().enumerate().position(|(i, &b)|b == '.') {
                if left_most >= i {
                    break;
                }
                println!("Moving {:?} at {:?} to {:?}", file_system[i], i, left_most);
                file_system.swap(i, left_most);
            }
        }
    }
}

fn checksum(file_system: &Vec<char>) -> usize {
    let mut sum = 0;
    for i in 0..file_system.len() {
        if file_system[i] == '.' {
            continue;
        }
        let id = file_system[i].to_digit(10).unwrap() as usize;
        sum += (id * i);
    }
    sum
}

fn parse(input: &str) -> Vec<char> {
    let mut result = Vec::new();
    let mut id_seq = 0;
    input.chars().enumerate().for_each(|(i, c)| {
        let value = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            // File
            for _ in 0..value {
                println!("char: {:?}", id_seq);
                result.push(char::from_digit(id_seq, 10).unwrap());
            }
            id_seq += 1;
        } else {
            // Free space
            for _ in 0..value {
                result.push('.');
            }
        }
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn mini_test() {
        let input = "12345";
        assert_eq!(parse(input).into_iter().collect::<String>(), "0..111....22222".to_owned());
        let mut input2 = parse("12345");
        move_blocks(&mut input2);
        assert_eq!(input2.into_iter().collect::<String>(), "022111222......".to_owned());
    }

    #[test]
    fn medium_test() {
        let input = "2333133121414131402";
        assert_eq!(parse(input).into_iter().collect::<String>(), "00...111...2...333.44.5555.6666.777.888899".to_owned());
        let mut input2 = parse("2333133121414131402");
        move_blocks(&mut input2);
        assert_eq!(input2.into_iter().collect::<String>(), "0099811188827773336446555566..............".to_owned());
    }

    fn test_part1() {
        assert_eq!(part1(INPUT), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
