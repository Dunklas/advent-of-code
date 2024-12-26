use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
use crate::util::grid::Grid;
use std::collections::HashSet;
use std::str::FromStr;

const DIRECTIONS: [Direction; 4] = [
    Direction::UP,
    Direction::DOWN,
    Direction::LEFT,
    Direction::RIGHT,
];

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let grid = Grid::<char>::from_str(input).unwrap();
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut price = 0;
    grid.coordinates().for_each(|coordinate| {
        if visited.contains(&coordinate) {
            return;
        }
        if let Some(plot) = find_plot(&grid, &coordinate) {
            price += perimeter(&plot) * plot.len();
            visited.extend(plot);
        }
    });
    price
}

fn part2(input: &str) -> usize {
    let grid = Grid::<char>::from_str(input).unwrap();
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut price = 0;
    grid.coordinates().for_each(|coordinate| {
        if visited.contains(&coordinate) {
            return;
        }
        if let Some(plot) = find_plot(&grid, &coordinate) {
            price += num_corners(&plot) * plot.len();
            visited.extend(plot.clone());
        }
    });
    price
}

fn find_plot(grid: &Grid<char>, start: &Coordinate) -> Option<HashSet<Coordinate>> {
    let value = grid.get(start)?;
    let mut visited = HashSet::new();
    let mut stack = Vec::from([*start]);
    while let Some(c) = stack.pop() {
        if visited.contains(&c) {
            continue;
        }
        if let Some(v) = grid.get(&c) {
            if v == value {
                visited.insert(c);
                DIRECTIONS.iter().for_each(|dir| {
                    stack.push(Coordinate::new(c.y + dir.dy, c.x + dir.dx));
                });
            }
        }
    }
    Some(visited)
}

fn num_corners(plot: &HashSet<Coordinate>) -> usize {
    plot.iter()
        .map(|c| {
            let adjacent = [
                c.offset(&Direction::UP),
                c.offset(&Direction::RIGHT),
                c.offset(&Direction::DOWN),
                c.offset(&Direction::LEFT),
            ];
            let diagonals = [
                c.offset(&Direction::TOP_RIGHT),
                c.offset(&Direction::BOTTOM_RIGHT),
                c.offset(&Direction::BOTTOM_LEFT),
                c.offset(&Direction::TOP_LEFT),
            ];
            (0..4)
                .filter(|&i| {
                    (!plot.contains(&adjacent[i]) && !plot.contains(&adjacent[(i + 1) % 4]))
                        || (plot.contains(&adjacent[i])
                            && plot.contains(&adjacent[(i + 1) % 4])
                            && !plot.contains(&diagonals[i]))
                })
                .count()
        })
        .sum()
}

fn perimeter(plot: &HashSet<Coordinate>) -> usize {
    plot.iter()
        .map(|c| {
            DIRECTIONS.iter().fold(0, |acc, dir| {
                match plot.contains(&Coordinate::new(c.y + dir.dy, c.x + dir.dx)) {
                    true => acc,
                    false => acc + 1,
                }
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL: &str = "AAAA
BBCD
BBCC
EEEC";

    const LARGE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SMALL), 140);
        assert_eq!(part1(LARGE), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SMALL), 80);
        assert_eq!(part2(LARGE), 1206);
    }
}
