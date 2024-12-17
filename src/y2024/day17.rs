use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut comp = Computer::from_str(input).unwrap();
    while let Some(_) = comp.run() {}
    let x = comp
        .out
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    x.join(",")
}

fn part2(input: &str) -> usize {
    let mut comp = Computer::from_str(input).unwrap();
    let initial_a = comp.registers[0];
    let expected_out = comp
        .instructions
        .iter()
        .map(|x| *x as isize)
        .collect::<Vec<_>>();

    let mut matched = 0;
    let mut tmp = 1;
    loop {
        if tmp == (initial_a as usize) {
            continue;
        }
        let mut comp = Computer::from_str(input).unwrap();
        comp.registers[0] = tmp as isize;
        while let Some(_) = comp.run() {}
        let exp_tail = expected_out.iter().rev().take(matched + 1).collect::<Vec<_>>();
        let act_tail = comp.out.iter().rev().take(matched + 1).collect::<Vec<_>>();
        if comp.out == expected_out {
            break;
        }
        if exp_tail == act_tail {
            tmp *= 8;
            if matched < expected_out.len() {
                matched += 1;
            }
        } else {
            tmp += 1;
        }
    }
    tmp
}

#[derive(Debug, Clone)]
struct Computer {
    registers: Vec<isize>,
    instructions: Vec<usize>,
    p: usize,
    out: Vec<isize>,
}

impl FromStr for Computer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split("\n\n").collect::<Vec<_>>();
        let registers = s[0].lines().collect::<Vec<_>>();
        let a = registers[0].replace("Register A: ", "").parse::<isize>()?;
        let b = registers[1].replace("Register B: ", "").parse::<isize>()?;
        let c = registers[2].replace("Register C: ", "").parse::<isize>()?;
        let instructions = s[1]
            .replace("Program: ", "")
            .split(",")
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;
        Ok(Self {
            registers: vec![a, b, c],
            instructions,
            p: 0,
            out: Vec::new(),
        })
    }
}

impl Computer {
    pub fn run(&mut self) -> Option<()> {
        if let Some(op) = self.instructions.get(self.p) {
            match *op {
                0 => {
                    let num = self.registers[0];
                    let op = self.combo()?;
                    let denominator = 2isize.pow(op as u32);
                    self.registers[0] = num / denominator;
                    self.p += 2;
                }
                1 => {
                    let op = self.literal()?;
                    self.registers[1] = self.registers[1] ^ op as isize;
                    self.p += 2;
                }
                2 => {
                    let op = self.combo()?;
                    self.registers[1] = op % 8;
                    self.p += 2;
                }
                3 => {
                    let op = self.literal()?;
                    match self.registers[0] {
                        0 => {
                            self.p += 2;
                        }
                        _ => {
                            self.p = op;
                        }
                    }
                }
                4 => {
                    let _ = self.literal()?;
                    self.registers[1] = self.registers[1] ^ self.registers[2];
                    self.p += 2;
                }
                5 => {
                    let op = self.combo()?;
                    self.out.push(op % 8);
                    self.p += 2;
                }
                6 => {
                    let num = self.registers[0];
                    let op = self.combo()?;
                    let denominator = 2isize.pow(op as u32);
                    self.registers[1] = num / denominator;
                    self.p += 2;
                }
                7 => {
                    let num = self.registers[0];
                    let op = self.combo()?;
                    let denominator = 2isize.pow(op as u32);
                    self.registers[2] = num / denominator;
                    self.p += 2;
                }
                _ => unreachable!(),
            }
            Some(())
        } else {
            None
        }
    }

    fn literal(&self) -> Option<usize> {
        self.instructions.get(self.p + 1).copied()
    }

    fn reset(&mut self) {
        self.p = 0;
        self.out.clear();
    }

    fn combo(&self) -> Option<isize> {
        let op = self.instructions.get(self.p + 1)?;
        Some(match op {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(part1(input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(part2(input), 117440);
    }
}
