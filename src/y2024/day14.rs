use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
use crate::util::grid::Grid;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input, 103, 101));
    println!("Part 2: {}", part2(input, 103, 101));
}

fn part1(input: &str, y_len: usize, x_len: usize) -> usize {
    let grid = Grid::new_with(y_len, x_len, '.');
    let mut guards = input
        .lines()
        .map(Guard::from_str)
        .collect::<Result<Vec<Guard>, ParseIntError>>()
        .unwrap();
    guards.iter_mut().for_each(|g| {
        simulate(g, 100, &grid);
    });
    let mid_y = (y_len / 2) as isize;
    let mid_x = (x_len / 2) as isize;
    guards
        .into_iter()
        .map(|g| g.pos)
        .fold([0; 4], |mut quadrants, pos| {
            if let Some(i) = determine_quadrant(mid_y, mid_x, &pos) {
                quadrants[i] += 1;
            }
            quadrants
        })
        .iter()
        .product()
}

fn part2(input: &str, y_len: usize, x_len: usize) -> usize {
    let mut grid = Grid::new_with(y_len, x_len, '.');
    let mut guards = input
        .lines()
        .map(Guard::from_str)
        .collect::<Result<Vec<Guard>, ParseIntError>>()
        .unwrap();
    let mut second = 1;
    loop {
        guards.iter_mut().for_each(|g| {
            simulate(g, 1, &grid);
        });
        if mean_distance(&guards.iter().map(|g| g.pos).collect::<Vec<_>>()) < 800 {
            break;
        }
        second += 1;
    }
    guards.iter().for_each(|g| {
        grid.replace(&g.pos, '#');
    });
    println!("{}", grid);
    second
}

fn simulate(guard: &mut Guard, seconds: usize, grid: &Grid<char>) {
    let new_y = ((guard.pos.y + guard.vel.dy * seconds as isize) % grid.y_len() as isize
        + grid.y_len() as isize)
        % grid.y_len() as isize;
    let new_x = ((guard.pos.x + guard.vel.dx * seconds as isize) % grid.x_len() as isize
        + grid.x_len() as isize)
        % grid.x_len() as isize;
    guard.pos = Coordinate::new(new_y, new_x);
}

fn determine_quadrant(mid_y: isize, mid_x: isize, pos: &Coordinate) -> Option<usize> {
    if pos.y == mid_y || pos.x == mid_x {
        return None;
    }
    Some(match (pos.y > mid_y, pos.x > mid_x) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (false, false) => 3,
    })
}

fn mean_distance(coordinates: &[Coordinate]) -> isize {
    let n = coordinates.len() as isize;
    let sum_x = coordinates.iter().map(|c| c.x).sum::<isize>();
    let sum_y = coordinates.iter().map(|c| c.y).sum::<isize>();
    let mean = Coordinate::new(sum_y / n, sum_x / n);

    coordinates
        .iter()
        .map(|c| {
            let dx = c.x - mean.x;
            let dy = c.y - mean.y;
            dx * dx + dy * dy
        })
        .sum::<isize>()
        / n
}

#[derive(Debug)]
struct Guard {
    pos: Coordinate,
    vel: Direction,
}

impl FromStr for Guard {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let raw_pos = parts[0].replace("p=", "");
        let raw_pos = raw_pos.split(",").collect::<Vec<_>>();
        let raw_vel = parts[1].replace("v=", "");
        let raw_vel = raw_vel.split(",").collect::<Vec<_>>();
        Ok(Self {
            pos: Coordinate::new(raw_pos[1].parse()?, raw_pos[0].parse()?),
            vel: Direction::new(raw_vel[1].parse()?, raw_vel[0].parse()?),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT, 7, 11), 12)
    }
}
