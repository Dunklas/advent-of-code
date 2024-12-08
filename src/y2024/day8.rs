use crate::util::coordinate::Coordinate;
use crate::util::grid::Grid;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let grid = Grid::<char>::from_str(input).unwrap();
    antennas_by_frequency(&grid)
        .values()
        .flat_map(|antennas| {
            antennas.iter().combinations(2).flat_map(|pair| {
                let delta = *pair[0] - *pair[1];
                vec![*pair[0] + delta, *pair[1] - delta]
            })
        })
        .filter(|antenna| grid.contains(antenna))
        .unique()
        .count()
}

fn part2(input: &str) -> usize {
    let grid = Grid::<char>::from_str(input).unwrap();
    antennas_by_frequency(&grid)
        .values()
        .flat_map(|antennas| {
            antennas.iter().combinations(2).flat_map(|pair| {
                let delta = *pair[0] - *pair[1];
                [
                    trajectory(*pair[0], delta, &grid),
                    trajectory(*pair[1], -delta, &grid),
                ]
                .concat()
            })
        })
        .unique()
        .count()
}

fn trajectory(start: Coordinate, direction: Coordinate, grid: &Grid<char>) -> Vec<Coordinate> {
    let mut coordinates = Vec::new();
    let mut current = start;
    while grid.contains(&current) {
        coordinates.push(current);
        current = current + direction;
    }
    coordinates
}

fn antennas_by_frequency(grid: &Grid<char>) -> HashMap<char, Vec<Coordinate>> {
    let mut by_frequency: HashMap<char, Vec<Coordinate>> = HashMap::new();
    for y in 0..grid.y_len() {
        for x in 0..grid.x_len() {
            let coord = Coordinate::new(y as isize, x as isize);
            if let Some(&freq) = grid.get(&coord).filter(|&ch| *ch != '.') {
                by_frequency.entry(freq).or_default().push(coord);
            }
        }
    }
    by_frequency
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 34);
    }
}
