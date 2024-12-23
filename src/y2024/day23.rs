use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let graph = parse(input);
    graph
        .nodes()
        .into_iter()
        .filter(|node| node.starts_with("t"))
        .flat_map(|node| graph.find_paths(node, node, 3))
        .unique()
        .count()
}

fn part2(input: &str) -> String {
    let graph = parse(input);
    let mut max_clique = graph.max_clique().unwrap();
    max_clique.sort();
    max_clique.into_iter().join(",")
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
    nodes: HashMap<String, HashSet<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn nodes(&self) -> Vec<&String> {
        self.nodes.keys().collect::<Vec<_>>()
    }

    pub fn find_paths(&self, start: &str, target: &str, depth: usize) -> HashSet<String> {
        let mut paths = HashSet::new();
        let mut stack = Vec::new();
        stack.push(vec![start]);

        while let Some(path) = stack.pop() {
            let prev = path.last().unwrap();
            if path.len() > depth {
                continue;
            }
            for n in self.nodes[*prev].iter() {
                let mut path = path.clone();
                if n == target && depth == path.len() {
                    path.sort();
                    let path = path.join(",");
                    paths.insert(path);
                    continue;
                }
                if !path.contains(&n.as_str()) {
                    path.push(n);
                    stack.push(path);
                }
            }
        }
        paths
    }

    pub fn max_clique(&self) -> Option<Vec<&String>> {
        let mut cliques = Vec::new();
        for start in self.nodes() {
            let mut clique = vec![start];
            for (n, _) in self.nodes.iter() {
                if n == start {
                    continue;
                }
                if clique.iter().all(|m| self.nodes[*m].contains(n)) {
                    clique.push(n);
                }
            }
            cliques.push(clique);
        }
        cliques.into_iter().max_by(|a, b| a.len().cmp(&b.len()))
    }

    pub fn add_edge(&mut self, a: &str, b: &str) {
        self.nodes
            .entry(a.to_owned())
            .or_default()
            .insert(b.to_owned());
        self.nodes
            .entry(b.to_owned())
            .or_default()
            .insert(a.to_owned());
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
        assert_eq!(part2(INPUT), "co,de,ka,ta");
    }
}
