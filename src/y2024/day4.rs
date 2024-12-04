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
    let grid = Grid::from(input);
    let mut count = 0;
    for y in 0..grid.y_len() {
        for x in 0..grid.x_len() {
            let pos = Pos::new(y, x);
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
    let grid = Grid::from(input);
    let mut count = 0;
    for y in 1..grid.y_len() - 1 {
        for x in 1..grid.x_len() - 1 {
            if is_xmas(&grid, &Pos::new(y, x)) {
                count += 1;
            }
        }
    }
    count
}

fn is_xmas(grid: &Grid, pos: &Pos) -> bool {
    if grid.get(pos) != 'A' {
        return false;
    }
    vec![
        (Pos::new(pos.y - 1, pos.x - 1), 1, 1),
        (Pos::new(pos.y - 1, pos.x + 1), -1, 1),
    ]
    .into_iter()
    .map(|(pos, dx, dy)| grid.get_segment(&pos, dx, dy, 3))
    .all(|result| result == MAS || result == SAM)
}

struct Grid {
    grid: Vec<Vec<char>>,
}

struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    fn new(y: usize, x: usize) -> Pos {
        Self { y, x }
    }
}

impl Grid {
    fn get(&self, pos: &Pos) -> char {
        self.grid[pos.y][pos.x]
    }

    fn get_segment(&self, start: &Pos, dx: isize, dy: isize, range: usize) -> Vec<char> {
        let mut result = Vec::new();
        let mut current_x = start.x;
        let mut current_y = start.y;

        for _ in 0..range {
            if current_y >= self.grid.len() || current_x >= self.grid[current_y].len() {
                break;
            }
            result.push(&self.grid[current_y][current_x]);
            current_x = (current_x as isize + dx) as usize;
            current_y = (current_y as isize + dy) as usize;
        }

        result.into_iter().copied().collect()
    }

    fn y_len(&self) -> usize {
        self.grid.len()
    }

    fn x_len(&self) -> usize {
        self.grid[0].len()
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self {
            grid: value
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect(),
        }
    }
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
