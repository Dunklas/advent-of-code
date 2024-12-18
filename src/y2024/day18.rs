use crate::util::coordinate::Coordinate;
use crate::util::grid::Grid;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input, 71, 1024));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str, size: usize, sim_len: usize) -> usize {
    let bytes = parse(input);
    let mut grid = Grid::<char>::new_with(size, size, '.');
    bytes.into_iter().take(sim_len)
        .for_each(|byte_pos| {
            grid.replace(&byte_pos, '#');
        });
    println!("{:?}", grid);
    0
}

fn part2(input: &str) -> usize {
    0
}

fn parse(input: &str) -> Vec<Coordinate> {
    input.lines()
        .map(|line| {
            let parts = line.split(",").collect::<Vec<_>>();
            Coordinate::new(
                parts[1].parse().unwrap(),
                parts[0].parse().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(part1(input, 8, 12), 22);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), 0);
    }
}