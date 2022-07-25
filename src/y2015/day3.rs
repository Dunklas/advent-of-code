use std::collections::HashSet;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut visited = HashSet::new();
    visited.insert((x, y));
    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => {}
        }
        visited.insert((x, y));
    }
    visited.len() as u32
}

fn part2(input: &str) -> u32 {
    let mut sx = 0;
    let mut sy = 0;
    let mut rx = 0;
    let mut ry = 0;
    let mut visited = HashSet::new();
    visited.insert((sx, sy));
    for (i, c) in input.chars().enumerate() {
        let y = match c {
            '^' => 1,
            'v' => -1,
            _ => 0
        };
        let x = match c {
            '>' => 1,
            '<' => -1,
            _ => 0
        };
        if i % 2 == 0 {
            sx += x;
            sy += y;
            visited.insert((sx, sy));
        }
        else {
            rx += x;
            ry += y;
            visited.insert((rx, ry));
        }
    }
    visited.len() as u32
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