use std::collections::HashSet;

const VALID_NORTH: [char; 3] = ['|', '7', 'F'];
const VALID_SOUTH: [char; 3] = ['|', 'L', 'J'];
const VALID_WEST: [char; 3] = ['-', 'L', 'F'];
const VALID_EAST: [char; 3] = ['-', 'J', '7'];

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let (tiles, origin) = parse_tiles(input);
    find_path(&tiles, origin).unwrap().len() / 2
}

fn part2(input: &str) -> i64 {
    let (mut tiles, origin) = parse_tiles(input);
    let path = find_path(&tiles, origin)
        .unwrap()
        .into_iter()
        .collect::<HashSet<_>>();
    for (y, row) in tiles.iter_mut().enumerate() {
        for (x, tile) in row.iter_mut().enumerate() {
            if !path.contains(&(y, x)) {
                *tile = '.';
            }
        }
    }
    let mut num_encompassing = 0;
    for (y, row) in tiles.iter().enumerate() {
        let mut prev = None;
        let mut inside = false;
        for (x, tile) in row.iter().enumerate() {
            if *tile == '.' && inside {
                num_encompassing += 1;
                continue;
            }
            if !path.contains(&(y, x)) || *tile == '-' {
                continue;
            }
            prev = match prev {
                Some('L') if *tile == '7' => {
                    continue;
                }
                Some('F') if *tile == 'J' => {
                    continue;
                }
                _ => Some(*tile),
            };
            inside = !inside;
        }
    }
    num_encompassing
}

fn find_path(map: &[Vec<char>], origin: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let first = match map[origin.0][origin.1] {
        '|' | 'L' | 'J' => Some((origin.0 - 1, origin.1)),
        '7' | 'F' => Some((origin.0 + 1, origin.1)),
        '-' => Some((origin.0, origin.1 + 1)),
        _ => None,
    }?;
    let mut path = vec![origin, first];
    while let Some(next) = next_pipe(map, &path[path.len() - 2..]) {
        if next == origin {
            break;
        }
        path.push(next);
    }
    Some(path)
}

fn next_pipe(map: &[Vec<char>], prev: &[(usize, usize)]) -> Option<(usize, usize)> {
    let yv = (prev[1].0 as isize) - (prev[0].0 as isize);
    let xv = (prev[1].1 as isize) - (prev[0].1 as isize);
    let pipe = map.get(prev[1].0)?.get(prev[1].1)?;
    match pipe {
        '|' | 'L' | 'J' if (yv < 0 || xv != 0) => Some((prev[1].0 - 1, prev[1].1)),
        '|' | 'F' | '7' if (yv > 0 || xv != 0) => Some((prev[1].0 + 1, prev[1].1)),
        '-' | '7' | 'J' if (xv < 0 || yv != 0) => Some((prev[1].0, prev[1].1 - 1)),
        '-' | 'L' | 'F' if (xv > 0 || yv != 0) => Some((prev[1].0, prev[1].1 + 1)),
        _ => None,
    }
}

fn parse_tiles(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut map: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let origin = map
        .iter()
        .enumerate()
        .filter_map(|(y, row)| row.iter().position(|e| *e == 'S').map(|x| (y, x)))
        .next()
        .unwrap();
    map[origin.0][origin.1] = start_pipe(&map, &origin);
    (map, origin)
}

fn start_pipe(map: &[Vec<char>], origin: &(usize, usize)) -> char {
    let nexts = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(yv, xv)| {
            match (
                (origin.0 as isize).checked_add(yv),
                (origin.1 as isize).checked_add(xv),
            ) {
                (Some(y), Some(x)) => Some(((y as usize, x as usize), (yv, xv))),
                _ => None,
            }
        })
        .filter_map(|((y, x), v)| map.get(y).and_then(|row| row.get(x)).map(|pipe| (pipe, v)))
        .filter(|(pipe, v)| match v {
            (-1, 0) => VALID_NORTH.contains(pipe),
            (1, 0) => VALID_SOUTH.contains(pipe),
            (0, -1) => VALID_WEST.contains(pipe),
            (0, 1) => VALID_EAST.contains(pipe),
            _ => false,
        })
        .map(|(_, v)| v)
        .collect::<Vec<_>>();
    match nexts.as_slice() {
        [(-1, 0), (1, 0)] => '|',
        [(0, -1), (0, 1)] => '-',
        [(-1, 0), (0, 1)] => 'L',
        [(-1, 0), (0, -1)] => 'J',
        [(1, 0), (0, -1)] => '7',
        [(1, 0), (0, 1)] => 'F',
        _ => panic!("No start pipe found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_small() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn part1_longer() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn part2_test() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(part2(input), 4);
    }

    #[test]
    fn part2_test_medium() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(part2(input), 8);
    }

    #[test]
    fn part2_test_larger() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(part2(input), 10);
    }
}
