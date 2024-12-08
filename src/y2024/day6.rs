use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
use crate::util::grid::Grid;
use std::collections::HashSet;
use std::str::FromStr;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let grid = Grid::<char>::from_str(input).unwrap();
    let guard = Guard::new(grid.find(&'^').unwrap(), Direction::new(-1, 0));
    let visited = find_path(&grid, guard);
    visited.len()
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::<char>::from_str(input).unwrap();
    let start = grid.find(&'^').unwrap();
    let start_dir = Direction::new(-1, 0);
    let visited = find_path(&grid, Guard::new(start, start_dir));

    let coordinates = grid.iter_coordinates().collect::<Vec<_>>();
    coordinates
        .iter()
        .filter(|coordinate| {
            if !visited.contains(coordinate) {
                return false;
            }
            let prev = grid.replace(coordinate, '#');
            let is_loop = is_infinite_loop(&grid, Guard::new(start, start_dir));
            grid.replace(coordinate, prev);
            is_loop
        })
        .count()
}

fn find_path(grid: &Grid<char>, mut guard: Guard) -> HashSet<Coordinate> {
    let mut visited = HashSet::new();
    loop {
        visited.insert(guard.current);
        if guard.walk(grid) == Movement::ExitingArea {
            break;
        }
    }
    visited
}

fn is_infinite_loop(grid: &Grid<char>, mut guard: Guard) -> bool {
    let mut visited = HashSet::new();
    loop {
        match guard.walk(grid) {
            Movement::Rotate => {
                if !visited.insert((guard.current, guard.dir)) {
                    return true;
                };
            }
            Movement::ExitingArea => {
                return false;
            }
            _ => {}
        }
    }
}

struct Guard {
    current: Coordinate,
    dir: Direction,
}

#[derive(PartialEq)]
enum Movement {
    Walk,
    Rotate,
    ExitingArea,
}

impl Guard {
    pub fn new(start: Coordinate, dir: Direction) -> Self {
        Self {
            current: start,
            dir,
        }
    }

    pub fn walk(&mut self, grid: &Grid<char>) -> Movement {
        let next = Coordinate::new(self.current.y + self.dir.dy, self.current.x + self.dir.dx);
        if next.y < 0
            || next.y >= grid.y_len() as isize
            || next.x < 0
            || next.x >= grid.x_len() as isize
        {
            return Movement::ExitingArea;
        }
        if *grid.get(&next).unwrap() == '#' {
            self.dir = self.dir.rotated_right();
            return Movement::Rotate;
        } else {
            self.current = next;
        }
        Movement::Walk
    }
}

#[cfg(test)]
mod tests {
    use crate::y2024::day6::{part1, part2};

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 6);
    }
}
