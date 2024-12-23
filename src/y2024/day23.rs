use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let graph = parse(input);
    let mut cycles = Vec::new();
    graph.nodes().into_iter()
        .flat_map(|id| graph.has_cycle_with_depth(id, 2))
        .filter(|cycles| cycles.len() > 0)
        .for_each(|path| {
            if !cycles.contains(&path) && path.len() > 2 {
                cycles.push(path);
            }
        });
    let mut cycles = cycles.into_iter()
        .filter(|x| x.iter().any(|n| n.starts_with("t")))
        .map(|mut c| {
        c.sort();
        c.join(",")
    }).collect::<Vec<_>>();
    cycles.sort();
    cycles.dedup();
    cycles.len()
}

fn part2(input: &str) -> usize {
    0
}

fn parse(input: &str) -> Graph {
    let mut graph = Graph::new();
    input.lines().for_each(|line| {
        let parts = line.split("-").collect::<Vec<_>>();
        graph.add_edge(parts[0], parts[1]);
    });
    graph
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Vec<String>>
}

impl Graph {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    pub fn nodes(&self) -> Vec<&String> {
        self.nodes.keys().collect::<Vec<_>>()
    }

    pub fn add_edge(&mut self, a: &str, b: &str) {
        self.nodes.entry(a.to_owned()).or_insert_with(Vec::new).push(b.to_owned());
        self.nodes.entry(b.to_owned()).or_insert_with(Vec::new).push(a.to_owned());
    }

    fn has_cycle_with_depth(&self, start: &str, max_depth: usize) -> Vec<Vec<String>> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        let mut cycles = Vec::new();
        self.find(start, None, &mut visited, &mut path, &mut cycles, 0, max_depth);
        cycles
    }

    fn find(&self, current: &str, parent: Option<&str>, visited: &mut HashSet<String>, path: &mut Vec<String>, cycles: &mut Vec<Vec<String>>, depth: usize, max_depth: usize) -> Option<Vec<String>> {
        if depth > max_depth {
            return None;
        }
        visited.insert(current.to_string());
        path.push(current.to_string());

        if let Some(neighbours) = self.nodes.get(current) {
            for neighbour in neighbours {
                if Some(neighbour) == parent.map(|p| p.to_owned()).as_ref() {
                    continue;
                }
                if visited.contains(neighbour) {
                    let cycle_start = path.iter().position(|n| n == neighbour).unwrap();
                    let cycle = path[cycle_start..].to_vec();

                    let mut normalized_cycle = cycle.clone();
                    normalized_cycle.sort();
                    if !cycles.iter().any(|c| {
                        let mut c_sorted = c.clone();
                        c_sorted.sort();
                        c_sorted == normalized_cycle
                    }) {
                        cycles.push(cycle);
                    }
                } else {
                    self.find(neighbour, parent, visited, path, cycles, depth + 1, max_depth);
                }
            }
        }

        visited.remove(current);
        path.pop();
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
