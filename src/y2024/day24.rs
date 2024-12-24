use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

lazy_static! {
    static ref WIRE: Regex = Regex::new(r"^([0-9a-z]{3}): (1|0)$").unwrap();
    static ref GATE: Regex =
        Regex::new(r"^([0-9a-z]{3}) ([A-Z]{2,3}) ([0-9a-z]{3}) -> ([0-9a-z]{3})$").unwrap();
}
pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let (mut wires, gates) = parse(input);

    let mut queue = gates.into_iter().collect::<VecDeque<_>>();
    while let Some((op, a, b, out)) = queue.pop_front() {
        let (a_val, b_val) = (wires.get(&a), wires.get(&b));
        if a_val.is_none() || b_val.is_none() {
            queue.push_back((op, a, b, out));
            continue;
        }
        let value = match op.as_str() {
            "AND" => {
                if a_val == Some(&1) && b_val == Some(&1) {
                    1
                } else {
                    0
                }
            }
            "OR" => {
                if a_val == Some(&1) || b_val == Some(&1) {
                    1
                } else {
                    0
                }
            }
            "XOR" => {
                if (a_val == Some(&1) && b_val == Some(&0))
                    || (a_val == Some(&0) && b_val == Some(&1))
                {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        };
        *wires.entry(out.clone()).or_default() = value;
    }
    produce_number(&wires, "z")
}

fn part2(input: &str) -> String {
    let (input, gates) = parse(input);

    loop {
        let mut wires = input.clone();
        let mut queue = gates.iter().collect::<VecDeque<_>>();
        while let Some(gate) = queue.pop_front() {
            let (op, a, b, out) = gate;
            let (a_val, b_val) = (wires.get(a), wires.get(b));
            if a_val.is_none() || b_val.is_none() {
                queue.push_back(gate);
                continue;
            }
            let value = match op.as_str() {
                "AND" => {
                    if a_val == Some(&1) && b_val == Some(&1) {
                        1
                    } else {
                        0
                    }
                }
                "OR" => {
                    if a_val == Some(&1) || b_val == Some(&1) {
                        1
                    } else {
                        0
                    }
                }
                "XOR" => {
                    if (a_val == Some(&1) && b_val == Some(&0))
                        || (a_val == Some(&0) && b_val == Some(&1))
                    {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            };
            *wires.entry(out.clone()).or_default() = value;
        }

        let x = produce_number(&wires, "x");
        let y = produce_number(&wires, "y");
        let z = produce_number(&wires, "z");
        if x + y == z {
            break;
        }
        println!("{} + {} = {}", x, y, z);
    }
    String::new()
}

fn produce_number(wires: &HashMap<String, u8>, prefix: &str) -> usize {
    let mut output_wires = wires
        .keys()
        .filter(|id| id.starts_with(prefix))
        .collect::<Vec<_>>();
    output_wires.sort();

    let binary_output = output_wires
        .into_iter()
        .rev()
        .map(|wire| wires.get(wire).unwrap().to_string())
        .collect::<Vec<_>>()
        .join("");
    usize::from_str_radix(&binary_output, 2).unwrap()
}

fn parse(input: &str) -> (HashMap<String, u8>, Vec<(String, String, String, String)>) {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let wires = sections[0]
        .lines()
        .into_iter()
        .map(|line| {
            let cap = WIRE.captures(line).unwrap();
            (cap[1].to_string(), cap[2].parse::<u8>().unwrap())
        })
        .collect::<HashMap<_, _>>();

    let gates = sections[1]
        .lines()
        .into_iter()
        .map(|line| {
            let cap = GATE.captures(line).unwrap();
            let a = cap[1].to_string();
            let op = cap[2].to_string();
            let b = cap[3].to_string();
            let out = cap[4].to_string();
            (op, a, b, out)
        })
        .collect::<Vec<_>>();
    (wires, gates)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const LARGE: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SMALL), 4);
        assert_eq!(part1(LARGE), 2024);
    }

    #[test]
    fn test_part2() {
        let input = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";
        assert_eq!(part2(input), "z00,z01,z02,z05");
    }
}
