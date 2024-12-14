use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
use crate::util::grid::Grid;
use itertools::Itertools;
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
        simulate2(g, 100, &grid);
    });
    let mid_y = (y_len / 2) as isize;
    let mid_x = (x_len / 2) as isize;
    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;
    for guard in guards {
        if guard.pos.y == mid_y || guard.pos.x == mid_x {
            continue;
        }
        if guard.pos.y - mid_y > 0 {
            if guard.pos.x - mid_x > 0 {
                br += 1;
            } else {
                bl += 1;
            }
        } else {
            if guard.pos.x - mid_x > 0 {
                tr += 1;
            } else {
                tl += 1;
            }
        }
    }
    tl * tr * bl * br
}

fn part2(input: &str, y_len: usize, x_len: usize) -> usize {
    let grid = Grid::new_with(y_len, x_len, '.');
    let mut guards = input
        .lines()
        .map(Guard::from_str)
        .collect::<Result<Vec<Guard>, ParseIntError>>()
        .unwrap();

    for second in 1..10000 {
        guards.iter_mut().for_each(|g| {
            simulate2(g, 1, &grid);
        });
        let counts: HashMap<Coordinate, usize> = guards.iter().fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x.pos).or_default() += 1;
            acc
        });

        let mut str = String::new();
        for y in 0..grid.y_len() {
            for x in 0..grid.x_len() {
                str.push(match counts.get(&Coordinate::new(y as isize, x as isize)) {
                    Some(_) => {
                        '#'
                    },
                    None => '.'
                });
            }
            str.push('\n');
        }

        // 278 is wrong but I can see the tree :(
        if str.contains("############") {
            println!("{}", str);
            println!("Secs: {}", second);
        }
    }
    0
}

pub fn simulate2(guard: &mut Guard, seconds: usize, grid: &Grid<char>) {
    let new_y = ((guard.pos.y + guard.vel.dy * seconds as isize) % grid.y_len() as isize
                 + grid.y_len() as isize) % grid.y_len() as isize;
    let new_x = ((guard.pos.x + guard.vel.dx * seconds as isize) % grid.x_len() as isize
                 + grid.x_len() as isize) % grid.x_len() as isize;
    guard.pos = Coordinate::new(new_y, new_x);
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT, 7, 11), 0)
    }
}
