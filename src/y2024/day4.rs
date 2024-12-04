pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let grid = parse(input);

    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if hit(y, x, &grid) {
               count += 1;
            }
        }
    }
    count
}

fn part2(input: &str) -> usize {
    0
}

fn hit(y: usize, x: usize, grid: &Vec<Vec<char>>) -> bool {
    if grid[y][x] != 'X' && grid[y][x] != 'M' && grid[y][x] != 'A' && grid[y][x] != 'S' {
        return false;
    }
    // Horizontal
    if x + 3 < grid[y].len() {
        if grid[y][x] == 'X' && grid[y][x+1] == 'M' && grid[y][x+2] == 'A' && grid[y][x+3] == 'S' {
            return true;
        }
        if grid[y][x] == 'S' && grid[y][x+1] == 'A' && grid[y][x+2] == 'M' && grid[y][x+3] == 'X' {
            return true;
        }
    }
    if y + 3 < grid.len() {
        if grid[y][x] == 'X' && grid[y+1][x] == 'M' && grid[y+2][x] == 'A' && grid[y+3][x] == 'S' {
            return true;
        }
        if grid[y][x] == 'S' && grid[y+1][x] == 'A' && grid[y+2][x] == 'M' && grid[y+3][x] == 'X' {
            return true;
        }
    }
    false
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().into_iter().collect::<Vec<char>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::y2024::day4::part1;

    #[test]
    fn test_small() {
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        assert_eq!(part1(input), 0);
    }
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
        let input = "";
        assert_eq!(part1(input), 0);
    }
}