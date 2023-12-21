use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input, 64));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str, steps: u32) -> usize {
    let garden = parse(input);
    let start = start_pos(&garden).unwrap();
    num_plots(start, &garden, steps)
}

fn part2(input: &str) -> u32 {
    0
}

fn num_plots(start: (usize, usize), garden: &[Vec<char>], desired_steps: u32) -> usize {
    let mut to_visit = vec![(start, 0)].into_iter().collect::<VecDeque<_>>();
    let mut targets = HashSet::<(usize, usize)>::new();
    while let Some(((y, x), distance)) = to_visit.pop_front() {
        if distance == desired_steps {
            targets.insert((y, x));
            continue;
        }
        let mut neighbours = vec![];
        if let Some(left) = x.checked_sub(1) {
            neighbours.push((y, left));
        }
        if let Some(up) = y.checked_sub(1) {
            neighbours.push((up, x));
        }
        if x + 1 < garden[0].len() {
            neighbours.push((y, x + 1));
        }
        if y + 1 < garden.len() {
            neighbours.push((y + 1, x));
        }
        for (ny, nx) in neighbours {
            if garden[ny][nx] != '#' {
                to_visit.push_back(((ny, nx), distance + 1));
            }
        }
    }
    targets.len()
}

fn start_pos(garden: &[Vec<char>]) -> Option<(usize, usize)> {
    garden
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, tile)| **tile == 'S')
                .map(|(x, _)| (y, x))
                .collect::<Vec<_>>()
        })
        .flatten()
        .next()
}
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(part1(input, 6), 16);
    }
}
