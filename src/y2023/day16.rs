use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let contraption = parse(input);
    num_energized(&contraption, ((0, 0), (0, 1)))
}

fn part2(input: &str) -> usize {
    let contraption = parse(input);
    let down = (0..contraption[0].len()).map(|x| ((0usize, x), (1isize, 0isize)));
    let right = (0..contraption.len()).map(|y| ((y, 0usize), (0isize, 1isize)));
    let left = (0..contraption.len()).map(|y| ((y, contraption[0].len() - 1), (0isize, -1isize)));
    let up = (0..contraption[0].len()).map(|x| ((contraption.len() - 1, x), (-1isize, 0isize)));
    down.chain(right)
        .chain(left)
        .chain(up)
        .map(|start| num_energized(&contraption, start))
        .max()
        .unwrap()
}

fn num_energized(contraption: &[Vec<char>], start: ((usize, usize), (isize, isize))) -> usize {
    let mut beams = vec![start].into_iter().collect::<VecDeque<_>>();
    let mut energized: HashSet<((usize, usize), (isize, isize))> = HashSet::new();
    while let Some(((start_y, start_x), (start_dy, start_dx))) = beams.pop_front() {
        let (mut y, mut x) = (start_y, start_x);
        let (mut dy, mut dx) = (start_dy, start_dx);
        loop {
            if !energized.insert(((y, x), (dy, dx))) {
                break;
            };
            (dy, dx) = match contraption[y][x] {
                '\\' if dx.is_negative() => (-1, 0),
                '\\' if dx.is_positive() => (1, 0),
                '\\' if dy.is_negative() => (0, -1),
                '\\' if dy.is_positive() => (0, 1),
                '/' if dx.is_negative() => (1, 0),
                '/' if dx.is_positive() => (-1, 0),
                '/' if dy.is_negative() => (0, 1),
                '/' if dy.is_positive() => (0, -1),
                _ => (dy, dx),
            };
            if contraption[y][x] == '|' && dx != 0 {
                if let Some((new_y, new_x)) = new_pos((y, x), (-1, 0), contraption) {
                    beams.push_back(((new_y, new_x), (-1, 0)));
                }
                if let Some((new_y, new_x)) = new_pos((y, x), (1, 0), contraption) {
                    beams.push_back(((new_y, new_x), (1, 0)));
                }
                break;
            }
            if contraption[y][x] == '-' && dy != 0 {
                if let Some((new_y, new_x)) = new_pos((y, x), (0, -1), contraption) {
                    beams.push_back(((new_y, new_x), (0, -1)));
                }
                if let Some((new_y, new_x)) = new_pos((y, x), (0, 1), contraption) {
                    beams.push_back(((new_y, new_x), (0, 1)));
                }
                break;
            }
            match new_pos((y, x), (dy, dx), contraption) {
                Some((new_y, new_x)) => {
                    y = new_y;
                    x = new_x;
                }
                None => {
                    break;
                }
            }
        }
    }
    energized
        .into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>()
        .len()
}

fn new_pos(
    (y, x): (usize, usize),
    (dy, dx): (isize, isize),
    contraption: &[Vec<char>],
) -> Option<(usize, usize)> {
    let y = match (y as isize) + dy {
        y if y < 0 || y >= contraption.len() as isize => None,
        y => Some(y),
    }? as usize;
    let x = match (x as isize) + dx {
        x if x < 0 || x >= contraption[0].len() as isize => None,
        x => Some(x),
    }? as usize;
    Some((y, x))
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        assert_eq!(part1(input), 46);
    }

    #[test]
    fn test_part2() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        assert_eq!(part2(input), 51);
    }
}
