use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
use crate::util::grid::Grid;
use std::collections::VecDeque;
use std::str::FromStr;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let (mut map, mut moves) = parse(input);
    while let Some(movement) = moves.pop_front() {
        map.move_robot(movement);
    }
    map.score()
}

fn part2(input: &str) -> usize {
    let (mut map, mut moves) = parse(&double_grid(input));
    while let Some(movement) = moves.pop_front() {
        map.move_robot(movement);
    }
    map.score()
}

#[derive(Debug)]
struct WarehouseMap {
    grid: Grid<char>,
    robot: Coordinate,
}

impl WarehouseMap {
    pub fn move_robot(&mut self, dir: char) {
        let dir = match dir {
            '^' => Direction::UP,
            '>' => Direction::RIGHT,
            'v' => Direction::DOWN,
            '<' => Direction::LEFT,
            _ => unreachable!(),
        };
        let current = self.robot;
        if self.try_move_thing(&current, &dir, true) {
            self.robot = Coordinate::new(current.y + dir.dy, current.x + dir.dx);
        }
    }

    fn try_move_thing(&mut self, source: &Coordinate, dir: &Direction, apply: bool) -> bool {
        let next = Coordinate::new(source.y + dir.dy, source.x + dir.dx);
        let x = match self.grid.get(&next) {
            Some('.') => true,
            Some('O') => self.try_move_thing(&next, dir, apply),
            Some('#') => false,
            Some('[') => match dir {
                &Direction::LEFT | &Direction::RIGHT => self.try_move_thing(&next, dir, true),
                _ => self.try_move_big_box(&'[', &next, dir, apply),
            },
            Some(']') => match dir {
                &Direction::LEFT | &Direction::RIGHT => self.try_move_thing(&next, dir, true),
                _ => self.try_move_big_box(&']', &next, dir, apply),
            },
            _ => unreachable!(),
        };

        if x {
            if apply {
                self.grid.swap(source, &next);
            }
            true
        } else {
            false
        }
    }

    fn try_move_big_box(
        &mut self,
        part: &char,
        source: &Coordinate,
        dir: &Direction,
        apply: bool,
    ) -> bool {
        let other = match &part {
            '[' => Coordinate::new(source.y, source.x + 1),
            ']' => Coordinate::new(source.y, source.x - 1),
            _ => unreachable!(),
        };
        if self.try_move_thing(&other, dir, false) {
            self.try_move_thing(source, dir, apply) && self.try_move_thing(&other, dir, apply)
        } else {
            false
        }
    }

    pub fn score(&self) -> usize {
        self.grid
            .iter()
            .filter(|(_, &value)| value == 'O' || value == '[')
            .map(|(coord, _)| (coord.y * 100 + coord.x) as usize)
            .sum()
    }
}

impl From<&str> for WarehouseMap {
    fn from(value: &str) -> Self {
        let grid = Grid::<char>::from_str(value.trim()).unwrap();
        let robot = grid.find(&'@').unwrap();
        Self { grid, robot }
    }
}

fn parse(input: &str) -> (WarehouseMap, VecDeque<char>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let map = WarehouseMap::from(parts[0]);
    let moves = parts[1]
        .lines()
        .flat_map(|l| l.chars())
        .collect::<VecDeque<_>>();
    (map, moves)
}

fn double_grid(map_raw: &str) -> String {
    map_raw
        .chars()
        .map(|c| match c {
            '#' => "##".to_owned(),
            'O' => "[]".to_owned(),
            '.' => "..".to_owned(),
            '@' => "@.".to_owned(),
            c => c.to_string(),
        })
        .collect()
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
        assert_eq!(part2(SMALL_ALT), 618);
        assert_eq!(part2(LARGE), 9021);
    }
}
