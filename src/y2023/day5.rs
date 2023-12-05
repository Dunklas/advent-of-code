use std::{ops::Range, collections::HashSet};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBERS: Regex = Regex::new(r"(\d+)\D?").unwrap();
}

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let (seeds, resource_maps) = parse(input);
    let mut lowest = u64::MAX;
    for seed in seeds {
        let mut resource_id = seed;
        for resource_map in resource_maps.iter() {
            resource_id = next(&resource_id, resource_map);
        }
        if resource_id < lowest {
            lowest = resource_id;
        }
    }
    lowest
}

fn part2(input: &str) -> u64 {
    let (seed_ranges, resource_maps) = parse(input);
    tmp(&resource_maps);
    0
}

fn tmp(map: &Vec<ResourceMap>) {
    let mut prev: Option<&(Range<u64>, Range<u64>)> = None;
    let mut x = map.iter().rev();
    let lowest_location = x.next().unwrap().ranges.iter().min_by(|a, b| a.1.start.cmp(&b.1.start)).unwrap();
    // Iterate backwards, find overlapping ranges to reduce search space?
}

fn next(id: &u64, resource_map: &ResourceMap) -> u64 {
    let mut result = *id;
    for (source, destination) in resource_map.ranges.iter() {
        if source.contains(id) {
            let offset = id - source.start;
            result = destination.start + offset;
            break;
        }
    }
    result
}

fn parse(input: &str) -> (Vec<u64>, Vec<ResourceMap>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let seeds = NUMBERS
        .captures_iter(parts[0])
        .map(|cap| cap[1].parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut resource_maps = Vec::new();
    for i in 1..parts.len() {
        let mut lines = parts[i].lines();
        let header_raw = lines.next().unwrap().replace(" map:", "");
        let header_parts = header_raw.split("-to-").collect::<Vec<_>>();
        let ranges = lines
            .map(|line| {
                NUMBERS
                    .captures_iter(line)
                    .map(|cap| cap[1].parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|numbers| {
                (
                    Range {
                        start: numbers[1],
                        end: numbers[1] + numbers[2],
                    },
                    Range {
                        start: numbers[0],
                        end: numbers[0] + numbers[2],
                    },
                )
            })
            .collect::<Vec<_>>();
        resource_maps.push(ResourceMap {
            source: header_parts[0].to_owned(),
            destination: header_parts[1].to_owned(),
            ranges,
        })
    }
    (seeds, resource_maps)
}

#[derive(Debug)]
struct ResourceMap {
    source: String,
    destination: String,
    ranges: Vec<(Range<u64>, Range<u64>)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part1(input), 35);
    }

    #[test]
    fn test_part2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part2(input), 46);
    }
}
