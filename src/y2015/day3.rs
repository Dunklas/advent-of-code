use std::collections::HashSet;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part2(input: &str) -> u32 {
    let (santa, robot): (Vec<_>, Vec<_>) = input.chars()
        .enumerate()
        .partition(|(i, _c)| i % 2 == 0);
    deliver_presents(santa.into_iter().map(|(_i, c)| c))
        .union(&deliver_presents(robot.into_iter().map(|(_i, c)| c)))
        .count() as u32
}

fn part1(input: &str) -> u32 {
    deliver_presents(input.chars()).len() as u32
}

fn deliver_presents<I>(instructions: I) -> HashSet<(i32, i32)>
    where I: Iterator<Item = char> {
    let mut visited = vec![(0, 0)].into_iter()
        .collect::<HashSet<(i32, i32)>>();
    let mut pos = (0, 0);
    for c in instructions {
        match c {
            '^' => pos = (pos.0, pos.1 + 1),
            'v' => pos = (pos.0, pos.1 - 1),
            '>' => pos = (pos.0 + 1, pos.1),
            '<' => pos = (pos.0 - 1, pos.1),
            _ => {}
        }
        visited.insert(pos);
    }
    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_houses_at_least_once() {
        assert_eq!(2, part1(">"));
        assert_eq!(4, part1("^>v<"));
        assert_eq!(2, part1("^v^v^v^v^v"));
    }

    #[test]
    fn with_robot() {
        assert_eq!(3, part2("^v"));
        assert_eq!(3, part2("^>v<"));
        assert_eq!(11, part2("^v^v^v^v^v"));
    }
}