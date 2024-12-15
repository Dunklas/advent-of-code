use std::collections::VecDeque;
use std::str::FromStr;
use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
use crate::util::grid::Grid;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> isize {
    let (mut grid, mut moves) = parse(input);

    let mut current = grid.find(&'@').unwrap();
    while let Some(movement) = moves.pop_front() {
        let dir = match movement {
            '^' => Direction::new(-1, 0),
            '>' => Direction::new(0, 1),
            'v' => Direction::new(1, 0),
            '<' => Direction::new(0, -1),
            _ => unreachable!()
        };
        if let Some(next) = move_thing(&current, &dir, &mut grid) {
            current = next;
        }
    }

    grid.iter()
        .filter(|(coord, &value)| value == 'O')
        .map(|(coord, _)| coord.y * 100 + coord.x)
        .sum()
}

fn move_thing(source: &Coordinate, dir: &Direction, grid: &mut Grid<char>) -> Option<Coordinate> {
    let next = Coordinate::new(source.y + dir.dy, source.x + dir.dx);
    match grid.get(&next) {
        Some('#') => None,
        Some('O') => {
            if let Some(x) = move_thing(&next, dir, grid) {
                move_thing(source, dir, grid)
            } else {
                None
            }
        },
        Some('.') => {
            let curr = grid.get(source).unwrap();
            grid.replace(&next, *curr);
            grid.replace(source, '.');
            Some(next)
        },
        _ => unreachable!()
    }
}

fn part2(input: &str) -> usize {
    0
}

fn parse(input: &str) -> (Grid<char>, VecDeque<char>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let grid = Grid::<char>::from_str(parts[0]).unwrap();
    let moves = parts[1].lines()
        .flat_map(|l| l.chars())
        .collect::<VecDeque<_>>();
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

    const LARGE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SMALL), 2028);
        assert_eq!(part1(LARGE), 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SMALL), 0)
    }
}
