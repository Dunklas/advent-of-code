pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let grid = parse(input);

    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            count += hit(y, x, &grid);
        }
    }
    count
}

fn part2(input: &str) -> usize {
    0
}

fn hit(y: usize, x: usize, grid: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    if grid[y][x] != 'X' && grid[y][x] != 'M' && grid[y][x] != 'A' && grid[y][x] != 'S' {
        return 0;
    }
    // Horizontal
    if x + 3 < grid[y].len() {
        if grid[y][x] == 'X' && grid[y][x+1] == 'M' && grid[y][x+2] == 'A' && grid[y][x+3] == 'S' {
            count += 1;
        }
        if grid[y][x] == 'S' && grid[y][x+1] == 'A' && grid[y][x+2] == 'M' && grid[y][x+3] == 'X' {
            count += 1;
        }
    }
    // Vertical
    if y + 3 < grid.len() {
        if grid[y][x] == 'X' && grid[y+1][x] == 'M' && grid[y+2][x] == 'A' && grid[y+3][x] == 'S' {
            count += 1;
        }
        if grid[y][x] == 'S' && grid[y+1][x] == 'A' && grid[y+2][x] == 'M' && grid[y+3][x] == 'X' {
            count += 1;
        }
    }

    if x + 3 < grid[y].len() && y + 3 < grid.len() {
        if grid[y][x] == 'X' && grid[y+1][x+1] == 'M' && grid[y+2][x+2] == 'A' && grid[y+3][x+3] == 'S' {
            count += 1;
        }
    }

    if x + 3 < grid[y].len() && y > 2 {
        if grid[y][x] == 'X' && grid[y-1][x+1] == 'M' && grid[y-2][x+2] == 'A' && grid[y-3][x+3] == 'S' {
            count += 1;
        }
    }

    if x > 2 && y + 3 < grid.len() {
        if grid[y][x] == 'X' && grid[y+1][x-1] == 'M' && grid[y+2][x-2] == 'A' && grid[y+3][x-3] == 'S' {
            count += 1;
        }
    }

    if x > 2 && y > 2 {
        if grid[y][x] == 'X' && grid[y-1][x-1] == 'M' && grid[y-2][x-2] == 'A' && grid[y-3][x-3] == 'S' {
            count += 1;
        }
    }
    count
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
        assert_eq!(part1(input), 4);
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