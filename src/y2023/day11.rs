use itertools::Itertools;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input, 1000000));
}

fn part1(input: &str) -> usize {
    sum_distances(input, 2)
}

fn part2(input: &str, multiplier: usize) -> usize {
    sum_distances(input, multiplier)
}

fn sum_distances(input: &str, multiplier: usize) -> usize {
    let universe = parse(input);
    let expansion_points = expansion_points(&universe);
    planet_positions(&universe)
        .into_iter()
        .combinations(2)
        .map(|pair| distance(pair[0], pair[1], &expansion_points, multiplier))
        .sum()
}

fn distance(
    a: (usize, usize),
    b: (usize, usize),
    expansion_points: &(Vec<usize>, Vec<usize>),
    multiplier: usize,
) -> usize {
    let mut dist = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
    for y in &expansion_points.0 {
        if *y > a.0.min(b.0) && *y < a.0.max(b.0) {
            dist += multiplier - 1;
        }
    }
    for x in &expansion_points.1 {
        if *x > a.1.min(b.1) && *x < a.1.max(b.1) {
            dist += multiplier - 1;
        }
    }
    dist
}

fn planet_positions(universe: &[Vec<char>]) -> Vec<(usize, usize)> {
    universe
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, tile)| **tile == '#')
                .map(move |(x, _)| (y, x))
        })
        .collect()
}

fn expansion_points(universe: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut rows = Vec::<usize>::new();
    for (y, row) in universe.iter().enumerate() {
        if row.iter().all(|tile| *tile != '#') {
            rows.push(y);
        }
    }
    let mut columns = Vec::<usize>::new();
    for x in 0..universe[0].len() {
        if (0..universe.len())
            .map(|y| (y, universe[y][x]))
            .all(|(_, tile)| tile != '#')
        {
            columns.push(x);
        }
    }
    (rows, columns)
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part1(input), 374);
    }

    #[test]
    fn part2_test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part2(input, 10), 1030);
        assert_eq!(part2(input, 100), 8410);
    }
}
