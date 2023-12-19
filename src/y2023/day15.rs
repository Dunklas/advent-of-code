use std::ops::Not;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref STEP: Regex = Regex::new(r"([a-z]+)(-|=)(\d*)").unwrap();
}

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input.split(',').map(|s| hash_str(s) as u32).sum()
}

fn part2(input: &str) -> usize {
    let steps = parse_steps(input);
    let mut boxes: [Vec<(String, u8)>; 256] = [(); 256].map(|_| Vec::new());
    for (label, sign, focal_length) in steps {
        let lens_box = &mut boxes[hash_str(&label)];
        let slot_index = lens_box.iter().position(|(l, _)| *l == label);
        match sign {
            '-' => {
                if let Some(index) = slot_index {
                    lens_box.remove(index);
                }
            }
            '=' => match slot_index {
                Some(index) => {
                    lens_box[index] = (label, focal_length.unwrap());
                }
                None => {
                    lens_box.push((label, focal_length.unwrap()));
                }
            },
            _ => panic!("Unexpected sign"),
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(box_number, lens_box)| {
            lens_box
                .into_iter()
                .enumerate()
                .map(move |(slot, (_, focal_length))| {
                    (1 + box_number) * (1 + slot) * focal_length as usize
                })
        })
        .sum()
}

fn hash_str(input: &str) -> usize {
    input.chars().fold(0, hash)
}

fn hash(value: usize, c: char) -> usize {
    ((value + (c as usize)) * 17) % 256
}

fn parse_steps(input: &str) -> Vec<(String, char, Option<u8>)> {
    input
        .split(',')
        .filter_map(|raw_step| STEP.captures(raw_step))
        .map(|cap| {
            (
                cap[1].to_string(),
                cap[2].chars().next().unwrap(),
                cap[3]
                    .is_empty()
                    .not()
                    .then(|| cap[3].parse::<u8>().unwrap()),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(input), 1320);
    }

    #[test]
    fn test_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part2(input), 145);
    }
}
