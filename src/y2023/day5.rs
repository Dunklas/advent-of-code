use std::ops::Range;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBERS: Regex = Regex::new(r"(\d+)").unwrap();
}

pub fn solve(input: &str) {
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

fn part1(input: &str) -> Option<u64> {
    let (seeds, resource_maps) = parse(input);
    seeds
        .into_iter()
        .map(|seed| seed..seed + 1)
        .flat_map(|seed| seed_to_location(seed, &resource_maps))
        .map(|range| range.start)
        .min()
}

fn part2(input: &str) -> Option<u64> {
    let (seeds, resource_maps) = parse(input);
    seeds
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .flat_map(|range| seed_to_location(range, &resource_maps))
        .map(|range| range.start)
        .min()
}

fn seed_to_location(input: Range<u64>, resource_maps: &[ResourceMap]) -> Vec<Range<u64>> {
    resource_maps
        .iter()
        .fold(vec![input], |next_src, resource_map| {
            next_src
                .into_iter()
                .flat_map(|range| expand_range(&range, resource_map))
                .map(translate)
                .collect()
        })
}

fn expand_range(current: &Range<u64>, next: &ResourceMap) -> Vec<(Range<u64>, isize)> {
    let mut next_src: Vec<(Range<u64>, isize)> = vec![];
    for (dst, src) in next {
        let diff = (dst.start as isize) - (src.start as isize);
        if src.contains(&current.start) && src.contains(&(current.end - 1)) {
            next_src.push((current.start..current.end, diff));
        } else if src.contains(&current.start) && !src.contains(&(current.end - 1)) {
            next_src.push((current.start..src.end, diff));
        } else if !src.contains(&current.start) && src.contains(&(current.end - 1)) {
            next_src.push((src.start..current.end, diff));
        }
    }
    if let Some((first, _)) = next_src.first() {
        if !first.contains(&current.start) {
            next_src.insert(0, (current.start..first.start, 0));
        }
    }
    if let Some((last, _)) = next_src.last() {
        if !last.contains(&(current.end - 1)) {
            next_src.push((last.end..current.end, 0));
        }
    }
    if next_src.is_empty() {
        next_src.push((current.start..current.end, 0));
    }
    next_src
}

fn translate((range, diff): (Range<u64>, isize)) -> Range<u64> {
    ((range.start as isize) + diff) as u64..((range.end as isize) + diff) as u64
}

fn parse(input: &str) -> (Vec<u64>, Vec<ResourceMap>) {
    let mut parts = input.split("\n\n");
    let seeds = NUMBERS
        .captures_iter(parts.next().unwrap())
        .map(|cap| cap[1].parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    (
        seeds,
        parts
            .map(|part| {
                let mut ranges = part
                    .lines()
                    .skip(1)
                    .map(|line| {
                        NUMBERS
                            .captures_iter(line)
                            .map(|cap| cap[1].parse::<u64>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .map(|numbers| {
                        (
                            numbers[0]..numbers[0] + numbers[2],
                            numbers[1]..numbers[1] + numbers[2],
                        )
                    })
                    .collect::<Vec<_>>();
                ranges.sort_by(|(_, a_src), (_, b_src)| a_src.start.cmp(&b_src.start));
                ranges
            })
            .collect(),
    )
}

type ResourceMap = Vec<(Range<u64>, Range<u64>)>;

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
        assert_eq!(part1(input), Some(35));
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
        assert_eq!(part2(input), Some(46));
    }
}
