use crate::util::coordinate::Coordinate;
use crate::util::grid::Grid;
use itertools::{all, iproduct, Itertools};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;
use std::str::FromStr;

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut cache = HashMap::new();
    let mut complexity = 0;
    let keypads = vec![keypad(), keypad(), numpad()];
    for line in input.lines() {
        let num = line.replace("A", "");
        let mut line = line.clone().to_owned();
        line.insert(0, 'A');
        let mut result = 0;
        for pair in line.chars().collect::<Vec<_>>().windows(2) {
            let part = find_best(&keypads, pair[0], pair[1], 2, &mut cache).unwrap();
            result += part;
        }
        let num = num.parse::<usize>().unwrap();
        complexity += num * result;
    }
    complexity
}

fn part2(input: &str) -> usize {
    let mut cache = HashMap::new();
    let mut complexity = 0;
    let mut keypads = vec![numpad()];
    for i in 0..25 {
        keypads.insert(0, keypad());
    }
    for line in input.lines() {
        let num = line.replace("A", "");
        let mut line = line.clone().to_owned();
        line.insert(0, 'A');
        let mut result = 0;
        for pair in line.chars().collect::<Vec<_>>().windows(2) {
            let part = find_best(&keypads, pair[0], pair[1], 25, &mut cache).unwrap();
            result += part;
        }
        let num = num.parse::<usize>().unwrap();
        complexity += num * result;
    }
    complexity
}


fn find_best(keypads: &Vec<ButtonPanel>, a: char, b: char, depth: usize, cache: &mut HashMap<(char, char, usize), usize>) -> Option<usize> {
    if let Some(val) = cache.get(&(a, b, depth)) {
        return Some(val.clone());
    }
    let keypad = match keypads.get(depth) {
        Some(keypad) => keypad,
        None => unreachable!(""),
    };
    let mut paths = keypad.get_key_presses(a, b);
    paths.iter_mut().for_each(|p| {
        p.push('A');
    });
    if (depth == 0) {
        return Some(paths.first().unwrap().len());
    }
    let return_val = paths
        .into_iter()
        .map(|mut s| {
            s.insert_str(0, "A");
            let mut tmp = 0;
            for c in s.chars().collect::<Vec<_>>().windows(2) {
                tmp += find_best(keypads, c[0], c[1], depth -1, cache).unwrap();
            }
            tmp
        })
        .min();
    if let Some(return_val) = return_val {
        cache.insert((a, b, depth), return_val);
        return Some(return_val);
    }
    None
}

fn to_button_sequence(presses: Vec<Vec<char>>) -> String {
    let mut tmp = presses
        .into_iter()
        .map(|p| p.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("A");
    tmp.push('A');
    tmp
}

fn numpad() -> ButtonPanel {
    ButtonPanel::new("789\n456\n123\n.0A")
}

fn keypad() -> ButtonPanel {
    ButtonPanel::new(".^A\n<v>")
}

struct ButtonPanel {
    keys: Grid<char>,
    start_pos: Coordinate,
}

impl ButtonPanel {
    pub fn new(keys: &str) -> Self {
        let keys = Grid::from_str(keys).unwrap();
        let start = keys.find(&'A').unwrap();
        Self {
            keys,
            start_pos: start,
        }
    }

    pub fn get_pos(&self, key: char) -> Option<Coordinate> {
        self.keys.find(&key)
    }

    pub fn get_key_presses(&self, source: char, key: char) -> Vec<String> {
        let source = self.keys.find(&source).unwrap();
        let paths = self.find_paths(&source, key);
        let paths = paths
            .into_iter()
            .map(|path| {
                path.windows(2)
                    .into_iter()
                    .map(
                        |pair| match (pair[0].y - pair[1].y, pair[0].x - pair[1].x) {
                            (0, 0) => unreachable!(),
                            (-1, 0) => 'v',
                            (1, 0) => '^',
                            (0, -1) => '>',
                            (0, 1) => '<',
                            _ => unreachable!(),
                        },
                    )
                    .collect()
            })
            .collect();
        paths
    }

    fn find_paths(&self, source: &Coordinate, target: char) -> Vec<Vec<Coordinate>> {
        let mut paths: Vec<Vec<Coordinate>> = Vec::new();
        let mut visisted = HashSet::new();
        let mut stack = VecDeque::new();
        stack.push_back(vec![*source]);
        while let Some(path) = stack.pop_front() {
            let last = path.last().unwrap();
            match self.keys.get(last) {
                Some(key) if key == &target => {
                    if let Some(prev_path) = paths.iter().next() {
                        if path.len() > prev_path.len() {
                            continue;
                        }
                    }
                    paths.push(path);
                    continue;
                }
                Some(_) => {
                    visisted.insert(*last);
                    for (dy, dx) in DIRECTIONS.iter() {
                        let next = Coordinate::new(last.y + dy, last.x + dx);
                        if visisted.contains(&next) {
                            continue;
                        }
                        if let Some('.') = self.keys.get(&next) {
                            continue;
                        }
                        let mut new_path = path.clone();
                        new_path.push(next);
                        stack.push_back(new_path);
                    }
                }
                None => {
                    continue;
                }
            }
        }
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 126384);
    }
}
