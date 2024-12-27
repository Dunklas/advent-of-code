use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut comp = Computer::from_str(input).unwrap();
    while comp.run().is_some() {}
    comp.out
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part2(input: &str) -> usize {
    let comp = Computer::from_str(input).unwrap();
    let initial_a = comp.registers[0];
    let expected_out = comp.instructions.to_vec();

    let mut matched = 0;
    let mut candidate = 1;
    loop {
        if candidate == initial_a {
            continue;
        }
        let mut comp = comp.clone();
        comp.registers[0] = candidate;
        while comp.run().is_some() {}
        if comp.out == expected_out {
            break;
        }
        let exp_tail = &expected_out[expected_out.len().saturating_sub(matched + 1)..];
        let act_tail = &comp.out[comp.out.len().saturating_sub(matched + 1)..];
        if exp_tail == act_tail {
            candidate *= 8;
            if matched < expected_out.len() {
                matched += 1;
            }
        } else {
            candidate += 1;
        }
    }
    candidate
}

#[derive(Debug, Clone)]
struct Computer {
    registers: [usize; 3],
    instructions: Vec<usize>,
    instruction_pointer: usize,
    out: Vec<usize>,
}

impl FromStr for Computer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split("\n\n").collect::<Vec<_>>();
        let registers = s[0].lines().collect::<Vec<_>>();
        let a = registers[0].replace("Register A: ", "").parse()?;
        let b = registers[1].replace("Register B: ", "").parse()?;
        let c = registers[2].replace("Register C: ", "").parse()?;
        let instructions = s[1]
            .replace("Program: ", "")
            .split(",")
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;
        Ok(Self {
            registers: [a, b, c],
            instructions,
            instruction_pointer: 0,
            out: Vec::new(),
        })
    }
}

impl Computer {
    pub fn run(&mut self) -> Option<()> {
        if let Some(&opcode) = self.instructions.get(self.instruction_pointer) {
            match OpCode::from(opcode) {
                OpCode::Adv => {
                    self.registers[0] /= 2usize.pow(self.combo()? as u32);
                    self.instruction_pointer += 2;
                }
                OpCode::Bxl => {
                    self.registers[1] ^= self.literal()?;
                    self.instruction_pointer += 2;
                }
                OpCode::Bst => {
                    self.registers[1] = self.combo()? % 8;
                    self.instruction_pointer += 2;
                }
                OpCode::Jnz => match self.registers[0] {
                    0 => {
                        self.instruction_pointer += 2;
                    }
                    _ => {
                        self.instruction_pointer = self.literal()?;
                    }
                },
                OpCode::Bxc => {
                    let _ = self.literal()?;
                    self.registers[1] ^= self.registers[2];
                    self.instruction_pointer += 2;
                }
                OpCode::Out => {
                    self.out.push(self.combo()? % 8);
                    self.instruction_pointer += 2;
                }
                OpCode::Bdv => {
                    self.registers[1] = self.registers[0] / 2usize.pow(self.combo()? as u32);
                    self.instruction_pointer += 2;
                }
                OpCode::Cdv => {
                    self.registers[2] = self.registers[0] / 2usize.pow(self.combo()? as u32);
                    self.instruction_pointer += 2;
                }
            }
            Some(())
        } else {
            None
        }
    }

    fn literal(&self) -> Option<usize> {
        self.instructions.get(self.instruction_pointer + 1).copied()
    }

    fn combo(&self) -> Option<usize> {
        let op = self.instructions.get(self.instruction_pointer + 1)?;
        Some(match *op {
            op if op <= 3 => op,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => unreachable!(),
        })
    }
}

enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<usize> for OpCode {
    fn from(value: usize) -> Self {
        match value {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => unreachable!(),
        }
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
