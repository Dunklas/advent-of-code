use crate::util::coordinate::Coordinate;
use crate::util::dir::Direction;
use crate::util::grid::Grid;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

const DIRECTIONS: [Direction; 4] = [
    Direction::UP,
    Direction::RIGHT,
    Direction::DOWN,
    Direction::LEFT,
];

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let numpad = ButtonPanel::numpad();
    let keypad = ButtonPanel::keypad();
    let mut panels = (0..2).map(|_| &keypad).collect::<Vec<_>>();
    panels.push(&numpad);
    total_complexity(input, &panels)
}

fn part2(input: &str) -> usize {
    let numpad = ButtonPanel::numpad();
    let keypad = ButtonPanel::keypad();
    let mut panels = (0..25).map(|_| &keypad).collect::<Vec<_>>();
    panels.push(&numpad);
    total_complexity(input, &panels)
}

fn total_complexity(input: &str, panels: &Vec<&ButtonPanel>) -> usize {
    let mut cache = HashMap::new();
    input
        .lines()
        .map(|line| {
            min_sequence_of_word(
                &panels,
                line.chars().collect(),
                panels.len() - 1,
                &mut cache,
            ) * line.trim_end_matches("A").parse::<usize>().unwrap()
        })
        .sum()
}

fn min_sequence_of_word(
    panels: &Vec<&ButtonPanel>,
    mut word: Vec<char>,
    depth: usize,
    cache: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    word.insert(0, 'A');
    word.windows(2)
        .filter_map(|pair| min_sequence(panels, pair[0], pair[1], depth, cache))
        .sum()
}

fn min_sequence(
    panels: &Vec<&ButtonPanel>,
    a: char,
    b: char,
    depth: usize,
    cache: &mut HashMap<(char, char, usize), usize>,
) -> Option<usize> {
    if let Some(val) = cache.get(&(a, b, depth)) {
        return Some(*val);
    }
    let mut paths = panels.get(depth)?.path_between(a, b);
    paths.iter_mut().for_each(|p| {
        p.push('A');
    });
    if depth == 0 {
        return paths.first().map(|p| p.len());
    }
    let result = paths
        .into_iter()
        .map(|path| min_sequence_of_word(panels, path, depth - 1, cache))
        .min()?;
    cache.insert((a, b, depth), result);
    Some(result)
}

struct ButtonPanel {
    keys: Grid<char>,
}

impl ButtonPanel {
    fn numpad() -> ButtonPanel {
        ButtonPanel::new("789\n456\n123\n.0A")
    }
    fn keypad() -> ButtonPanel {
        ButtonPanel::new(".^A\n<v>")
    }
    fn new(keys: &str) -> Self {
        let keys = Grid::from_str(keys).unwrap();
        Self { keys }
    }
    pub fn path_between(&self, source: char, key: char) -> Vec<Vec<char>> {
        let source = self.keys.find(&source).unwrap();
        find_paths(&self.keys, &source, key)
            .into_iter()
            .map(to_key_presses)
            .collect()
    }
}

fn to_key_presses(path: Vec<Coordinate>) -> Vec<char> {
    path.windows(2)
        .map(
            |pair| match (pair[0].y - pair[1].y, pair[0].x - pair[1].x) {
                (-1, 0) => 'v',
                (1, 0) => '^',
                (0, -1) => '>',
                (0, 1) => '<',
                _ => unreachable!(),
            },
        )
        .collect()
}

fn find_paths(grid: &Grid<char>, source: &Coordinate, target: char) -> Vec<Vec<Coordinate>> {
    let mut paths: Vec<Vec<Coordinate>> = Vec::new();
    let mut visited = HashSet::new();
    let mut stack = VecDeque::new();
    stack.push_back(vec![*source]);
    while let Some(current_path) = stack.pop_front() {
        let last = current_path.last().unwrap();
        visited.insert(*last);
        match grid.get(last) {
            Some(&key) if key == target => {
                if !paths.is_empty() && current_path.len() > paths[0].len() {
                    continue;
                }
                paths.push(current_path);
            }
            Some('.') | None => {
                continue;
            }
            _ => {
                for dir in DIRECTIONS.iter() {
                    let next = last.offset(dir);
                    if !visited.contains(&next) {
                        let mut new_path = current_path.clone();
                        new_path.push(next);
                        stack.push_back(new_path);
                    }
                }
            }
        }
    }
    paths
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
