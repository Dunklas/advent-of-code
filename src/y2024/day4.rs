use crate::util::coordinate::Coordinate;
use crate::util::grid::Grid;
use std::str::FromStr;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const XMAS: &[char] = &['X', 'M', 'A', 'S'];
const SAMX: &[char] = &['S', 'A', 'M', 'X'];
const MAS: &[char] = &['M', 'A', 'S'];
const SAM: &[char] = &['S', 'A', 'M'];
const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (1, 1), (-1, 1)];

fn part1(input: &str) -> usize {
    let grid = Grid::<char>::from_str(input).unwrap();
    let mut count = 0;
    for y in 0..grid.y_len() {
        for x in 0..grid.x_len() {
            let pos = Coordinate::new(y as isize, x as isize);
            count += DIRECTIONS
                .iter()
                .map(|(dx, dy)| grid.get_segment(&pos, *dx, *dy, 4))
                .filter(|result| result == XMAS || result == SAMX)
                .count();
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let grid = Grid::<char>::from_str(input).unwrap();
    let mut count = 0;
    for y in 1..grid.y_len() - 1 {
        for x in 1..grid.x_len() - 1 {
            if is_xmas(&grid, &Coordinate::new(y as isize, x as isize)) {
                count += 1;
            }
        }
    }
    count
}

fn is_xmas(grid: &Grid<char>, pos: &Coordinate) -> bool {
    if *grid.get(pos) != 'A' {
        return false;
    }
    vec![
        (Coordinate::new(pos.y - 1, pos.x - 1), 1, 1),
        (Coordinate::new(pos.y - 1, pos.x + 1), -1, 1),
    ]
    .into_iter()
    .map(|(pos, dx, dy)| grid.get_segment(&pos, dx, dy, 3))
    .all(|result| result == MAS || result == SAM)
}

#[cfg(test)]
mod tests {
    use crate::y2024::day4::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part1(input), 18);
    }

    #[test]
    fn test_part2() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!(part2(input), 9);
    }
}
