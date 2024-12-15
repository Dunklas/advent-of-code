use std::str::FromStr;
use crate::util::grid::Grid;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let (grid, moves) = parse(input);
    println!("Grid: {:?}", grid);
    println!("Moves: {:?}", moves);
    0
}

fn part2(input: &str) -> usize {
    0
}

fn parse(input: &str) -> (Grid<char>, Vec<char>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let grid = Grid::<char>::from_str(parts[0]).unwrap();
    let moves = parts[1].lines()
        .flat_map(|l| l.chars())
        .collect::<Vec<_>>();
    (grid, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SMALL), 2028)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SMALL), 0)
    }
}
