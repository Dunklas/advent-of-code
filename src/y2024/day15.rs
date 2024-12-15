use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
use crate::util::grid::Grid;
use std::collections::VecDeque;
use std::str::FromStr;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> isize {
    let (mut grid, mut moves) = parse(input, false);

    let mut current = grid.find(&'@').unwrap();
    while let Some(movement) = moves.pop_front() {
        let dir = match movement {
            '^' => Direction::new(-1, 0),
            '>' => Direction::new(0, 1),
            'v' => Direction::new(1, 0),
            '<' => Direction::new(0, -1),
            _ => unreachable!(),
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
        }
        Some('.') => {
            let curr = grid.get(source).unwrap();
            grid.replace(&next, *curr);
            grid.replace(source, '.');
            Some(next)
        }
        _ => unreachable!(),
    }
}

fn part2(input: &str) -> isize {
    let (mut grid, mut moves) = parse(input, true);
    let mut current = grid.find(&'@').unwrap();
    while let Some(movement) = moves.pop_front() {
        let dir = match movement {
            '^' => Direction::new(-1, 0),
            '>' => Direction::new(0, 1),
            'v' => Direction::new(1, 0),
            '<' => Direction::new(0, -1),
            _ => unreachable!(),
        };
        if let Some(next) = move_robot(&current, &dir, &mut grid) {
            current = next;
        }
    }
    grid.iter()
        .filter(|(coord, &value)| value == '[')
        .map(|(coord, _)| coord.y * 100 + coord.x)
        .sum()
}

fn move_robot(source: &Coordinate, dir: &Direction, grid: &mut Grid<char>) -> Option<Coordinate> {
    let next = Coordinate::new(source.y + dir.dy, source.x + dir.dx);
    match grid.get(&next) {
        Some('#') => None,
        Some('.') => {
            let curr = grid.get(source).unwrap();
            grid.replace(&next, *curr);
            grid.replace(source, '.');
            Some(next)
        }
        Some('[') | Some(']') => {
            if let Some(_) = move_box(&next, dir, grid) {
                move_robot(source, dir, grid)
            } else {
                None
            }
        }
        _ => unreachable!(),
    }
}

fn is_movable_box(source: &Coordinate, direction: &Direction, grid: &Grid<char>) -> bool {
    match direction.dy {
        0 => {
            let past_box =
                Coordinate::new(source.y + direction.dy * 2, source.x + direction.dx * 2);
            match grid.get(&past_box) {
                Some('#') => false,
                Some('.') => true,
                Some('[') | Some(']') => is_movable_box(&past_box, direction, grid),
                None => false,
                _ => unreachable!(),
            }
        }
        1 | -1 => {
            let (left, right) = match grid.get(&source) {
                Some('[') => (*source, Coordinate::new(source.y, source.x + 1)),
                Some(']') => (Coordinate::new(source.y, source.x - 1), *source),
                _ => unreachable!(),
            };
            let behind_left = Coordinate::new(left.y + direction.dy, left.x);
            let behind_right = Coordinate::new(right.y + direction.dy, right.x);
            if let (Some(left_val), Some(right_val)) =
                (grid.get(&behind_left), grid.get(&behind_right))
            {
                if *left_val == '.' && *right_val == '.' {
                    return true;
                } else if *left_val == ']' && *right_val == '[' {
                    return is_movable_box(&behind_left, direction, grid)
                        && is_movable_box(&behind_right, direction, grid);
                } else if *left_val == '[' && *right_val == ']' {
                    return is_movable_box(&behind_right, direction, grid);
                } else if *left_val == ']' && *right_val == '.' {
                    return is_movable_box(&behind_left, direction, grid);
                } else if *left_val == '.' && *right_val == '[' {
                    return is_movable_box(&behind_right, direction, grid);
                }
            }
            false
        }
        _ => unreachable!(),
    }
}

fn move_box(
    source: &Coordinate,
    direction: &Direction,
    grid: &mut Grid<char>,
) -> Option<Coordinate> {
    if !is_movable_box(source, direction, grid) {
        return None;
    }
    match direction.dy {
        0 => {
            let past_box =
                Coordinate::new(source.y + direction.dy * 2, source.x + direction.dx * 2);
            match grid.get(&past_box) {
                Some('#') => None,
                Some('.') => {
                    let second_part =
                        Coordinate::new(source.y + direction.dy, source.x + direction.dx);
                    grid.swap(&second_part, &past_box);
                    grid.swap(source, &second_part);
                    Some(second_part)
                }
                Some('[') | Some(']') => {
                    if let Some(_) = move_box(&past_box, direction, grid) {
                        move_box(source, direction, grid)
                    } else {
                        None
                    }
                }
                None => None,
                _ => unreachable!(),
            }
        }
        1 | -1 => {
            let (left, right) = match grid.get(&source) {
                Some('[') => (*source, Coordinate::new(source.y, source.x + 1)),
                Some(']') => (Coordinate::new(source.y, source.x - 1), *source),
                _ => unreachable!(),
            };
            let past_box1 = Coordinate::new(left.y + direction.dy, left.x);
            let past_box2 = Coordinate::new(right.y + direction.dy, right.x);
            if let (Some(left_val), Some(right_val)) = (grid.get(&past_box1), grid.get(&past_box2))
            {
                if *left_val == '.' && *right_val == '.' {
                    grid.swap(&past_box1, &left);
                    grid.swap(&past_box2, &right);
                    Some(Coordinate::new(source.y + direction.dy, source.x))
                } else if *left_val == '[' && *right_val == ']' {
                    if let Some(_) = move_box(&past_box1, direction, grid) {
                        move_box(source, direction, grid)
                    } else {
                        None
                    }
                } else if *left_val == ']' && *right_val == '.' {
                    if let Some(_) = move_box(&past_box1, direction, grid) {
                        move_box(source, direction, grid)
                    } else {
                        None
                    }
                } else if *left_val == '.' && *right_val == '[' {
                    if let Some(_) = move_box(&past_box2, direction, grid) {
                        move_box(source, direction, grid)
                    } else {
                        None
                    }
                } else if *left_val == ']' && *right_val == '[' {
                    let left_res = move_box(&past_box1, direction, grid);
                    let right_res = move_box(&past_box2, direction, grid);
                    match (left_res, right_res) {
                        (Some(_), Some(_)) => move_box(source, direction, grid),
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => unreachable!(),
    }
}

fn parse(input: &str, double: bool) -> (Grid<char>, VecDeque<char>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let mut map_raw = parts[0].to_string();
    if double {
        let mut tmp = String::new();
        parts[0].chars().for_each(|c| match c {
            '#' => {
                tmp.push('#');
                tmp.push('#');
            }
            'O' => {
                tmp.push('[');
                tmp.push(']');
            }
            '.' => {
                tmp.push('.');
                tmp.push('.');
            }
            '@' => {
                tmp.push('@');
                tmp.push('.');
            }
            '\n' => {
                tmp.push('\n');
            }
            _ => unreachable!(),
        });
        map_raw = tmp;
    }
    let grid = Grid::<char>::from_str(&map_raw).unwrap();
    let moves = parts[1]
        .lines()
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

    const SMALL_ALT: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

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
        assert_eq!(part2(LARGE), 9021);
    }
}
