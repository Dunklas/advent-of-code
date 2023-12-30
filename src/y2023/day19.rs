use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
    str::FromStr,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PART: Regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    static ref WORKFLOW: Regex = Regex::new(r"([a-z]+)\{(.+)\}").unwrap();
    static ref CONDITION: Regex = Regex::new(r"([xmas])([<>])(\d+):([a-zA-Z]+)").unwrap();
}

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let (workflows, parts) = parse(input).unwrap();
    parts
        .into_iter()
        .filter(|part| {
            let mut next = "in";
            loop {
                let workflow = workflows.get(next).unwrap();
                match workflow.execute(part) {
                    "A" => return true,
                    "R" => return false,
                    id => {
                        next = id;
                    }
                }
            }
        })
        .map(|part| part.sum())
        .sum()
}

fn part2(input: &str) -> usize {
    let (workflows, _) = parse(input).unwrap();
    let range = PartRange::new();
    let mut to_investigate = vec![("in", range)].into_iter().collect::<VecDeque<_>>();
    let mut accepted = vec![];
    while let Some((id, part_range)) = to_investigate.pop_front() {
        let workflow = workflows.get(id).unwrap();
        let mut range = part_range;
        for condition in workflow.conditions.iter() {
            let (success, fail) = condition.apply_range(&range);
            if condition.target == "A" {
                accepted.push(success);
            } else if condition.target != "R" {
                to_investigate.push_back((&condition.target, success));
            }
            range = fail;
        }
        if workflow.fallback == "A" {
            accepted.push(range);
        } else if workflow.fallback != "R" {
            to_investigate.push_back((&workflow.fallback, range))
        }
    }
    accepted
        .into_iter()
        .map(|part_range| part_range.permutations())
        .sum()
}

fn parse(input: &str) -> Option<(HashMap<String, Workflow>, Vec<Part>)> {
    let (raw_workflows, raw_parts) = input.split_once("\n\n")?;
    Some((
        raw_workflows
            .lines()
            .flat_map(Workflow::from_str)
            .map(|workflow| (workflow.id.clone(), workflow))
            .collect(),
        raw_parts.lines().flat_map(Part::from_str).collect_vec(),
    ))
}

struct ParseConditionError;
#[derive(Debug)]
struct Condition {
    category: usize,
    operator: char,
    threshold: u32,
    target: String,
}

impl Condition {
    pub fn apply_range(&self, part: &PartRange) -> (PartRange, PartRange) {
        let mut success = PartRange::from(part);
        let mut fail = PartRange::from(part);
        let current = &part.ranges[self.category];
        let (s, f) = match self.operator {
            '>' => (
                current.start.max(self.threshold + 1)..(current.end.min(4001)),
                current.start.max(1)..current.end.min(self.threshold + 1),
            ),
            '<' => (
                current.start.max(1)..current.end.min(self.threshold),
                current.start.max(self.threshold)..(current.end.min(4001)),
            ),
            _ => panic!("Unexpected operator"),
        };
        success.ranges[self.category] = s;
        fail.ranges[self.category] = f;
        (success, fail)
    }
    pub fn apply(&self, part: &Part) -> Option<&str> {
        let value = part.values[self.category];
        match self.operator {
            '<' => value < self.threshold,
            '>' => value > self.threshold,
            _ => panic!("Unexpected operator"),
        }
        .then_some(&self.target)
    }
}
impl FromStr for Condition {
    type Err = ParseConditionError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let caps = CONDITION.captures(input).ok_or(ParseConditionError)?;
        Ok(Condition {
            category: match caps[1]
                .to_string()
                .chars()
                .next()
                .ok_or(ParseConditionError)?
            {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => panic!("Unexpected category"),
            },
            operator: caps[2]
                .to_string()
                .chars()
                .next()
                .ok_or(ParseConditionError)?,
            threshold: caps[3]
                .to_string()
                .parse()
                .map_err(|_| ParseConditionError)?,
            target: caps[4].to_string(),
        })
    }
}

struct ParseWorkflowError;
#[derive(Debug)]
struct Workflow {
    id: String,
    fallback: String,
    conditions: Vec<Condition>,
}

impl Workflow {
    pub fn execute(&self, part: &Part) -> &str {
        for condition in self.conditions.iter() {
            if let Some(target) = condition.apply(part) {
                return target;
            }
        }
        &self.fallback
    }
}
impl FromStr for Workflow {
    type Err = ParseWorkflowError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let workflow = WORKFLOW.captures(input).ok_or(ParseWorkflowError)?;
        let raw_conditions = workflow[2].split(',').collect_vec();
        Ok(Workflow {
            id: workflow[1].to_string(),
            fallback: raw_conditions.last().ok_or(ParseWorkflowError)?.to_string(),
            conditions: raw_conditions
                .into_iter()
                .flat_map(Condition::from_str)
                .collect(),
        })
    }
}

struct PartRange {
    ranges: [Range<u32>; 4],
}

impl PartRange {
    pub fn from(part: &PartRange) -> Self {
        PartRange {
            ranges: part.ranges.clone(),
        }
    }

    pub fn new() -> Self {
        PartRange {
            ranges: [1..4001, 1..4001, 1..4001, 1..4001],
        }
    }
    pub fn permutations(&self) -> usize {
        self.ranges.iter().map(|range| range.len()).product()
    }
}

struct ParsePartError;
#[derive(Debug)]
struct Part {
    values: [u32; 4],
}

impl Part {
    pub fn sum(&self) -> u32 {
        self.values.iter().sum()
    }
}

impl FromStr for Part {
    type Err = ParsePartError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        PART.captures(input)
            .and_then(|cap| {
                match (
                    cap[1].parse(),
                    cap[2].parse(),
                    cap[3].parse(),
                    cap[4].parse(),
                ) {
                    (Ok(x), Ok(m), Ok(a), Ok(s)) => Some(Part {
                        values: [x, m, a, s],
                    }),
                    _ => None,
                }
            })
            .ok_or(ParsePartError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(part1(input), 19114);
    }

    #[test]
    fn test_part2() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(part2(input), 167409079868000);
    }
}
