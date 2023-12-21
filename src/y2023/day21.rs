pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input, 64));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str, steps: u32) -> u32 {
    let garden = parse(input);
    let start = start_pos(&garden).unwrap();
    0
}

fn part2(input: &str) -> u32 {
    0
}

fn num_plots(steps: u32, pos: (usize, usize), garden: &[Vec<char>]) {}

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
