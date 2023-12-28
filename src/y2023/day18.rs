use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DIG_INSTRUCTION: Regex =
        Regex::new(r"([RDLU]{1}) (\d+) \((#[a-z0-9]{6})\)").unwrap();
}

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    let dig_plan = parse_dig_plan(input)
        .into_iter()
        .map(|(direction, distance, _)| (direction, distance))
        .collect_vec();
    let lagoon = Lagoon::new(dig_plan);
    lagoon.area() + (lagoon.perimeter / 2 + 1)
}

fn part2(input: &str) -> i64 {
    let dig_plan = parse_dig_plan(input)
        .into_iter()
        .map(|(_, _, hex)| {
            (
                match &hex[6..7] {
                    "0" => 'R',
                    "1" => 'D',
                    "2" => 'L',
                    "3" => 'U',
                    _ => panic!("Unexpected direction"),
                },
                i64::from_str_radix(&hex[1..6], 16).unwrap(),
            )
        })
        .collect_vec();
    let lagoon = Lagoon::new(dig_plan);
    lagoon.area() + (lagoon.perimeter / 2 + 1)
}

fn parse_dig_plan(input: &str) -> Vec<(char, i64, String)> {
    input
        .lines()
        .flat_map(|line| {
            DIG_INSTRUCTION.captures(line).map(|cap| {
                (
                    cap[1].chars().next().unwrap(),
                    cap[2].parse().unwrap(),
                    cap[3].to_string(),
                )
            })
        })
        .collect()
}

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
    pub fn add(&self, point: Self) -> Self {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}

struct Lagoon {
    polygon: Vec<Point>,
    perimeter: i64,
}

impl Lagoon {
    pub fn new(dig_plan: Vec<(char, i64)>) -> Self {
        let mut points = vec![Point::new(0, 0)];
        let mut perimeter = 0;
        for (direction, distance) in dig_plan {
            let point = match direction {
                'L' => Point::new(-distance, 0),
                'U' => Point::new(0, -distance),
                'R' => Point::new(distance, 0),
                'D' => Point::new(0, distance),
                _ => panic!("Unexpected direction"),
            };
            if let Some(prev) = points.last() {
                points.push(prev.add(point));
                perimeter += distance;
            }
        }
        Lagoon {
            polygon: points,
            perimeter,
        }
    }

    pub fn area(&self) -> i64 {
        self.polygon
            .iter()
            .tuple_windows()
            .map(|(a, b)| a.x * b.y - b.x * a.y)
            .sum::<i64>()
            .abs()
            / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(part1(input), 62);
    }

    #[test]
    fn test_part2() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(part2(input), 952408144115);
    }
}
