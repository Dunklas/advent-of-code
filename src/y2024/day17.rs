use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut comp = Computer::from_str(input).unwrap();
    while let Some(_) = comp.run() {
        println!("HELO");
    }
    let x = comp.out.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    x.join(",")
}

fn part2(input: &str) -> usize {
    0
}

#[derive(Debug)]
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
            out: Vec::new()
        })
    }
}

impl Computer {
    pub fn run(&mut self) -> Option<()> {
        if let Some(op) = self.instructions.get(self.p) {
            println!("OP: {}", op);
            match *op {
                0 => {
                    let num = self.registers[0];
                    let op = self.combo()?;
                    let denominator = 2isize.pow(op as u32);
                    self.registers[0] = num / denominator;
                    self.p += 2;
                },
                1 => {
                    let op = self.literal()?;
                    self.registers[1] = self.registers[1] ^ op as isize;
                    self.p += 2;
                }
                2 => {
                    let op = self.combo()?;
                    self.registers[1] = op % 8;
                    self.p += 2;
                },
                3 => {
                    let op = self.literal()?;
                    match self.registers[0] {
                        0 => {
                            self.p += 2;
                        },
                        _ => {
                            self.p = op;
                        }
                    }
                },
                4 => {
                    let _ = self.literal()?;
                    self.registers[1] = self.registers[1] ^ self.registers[2];
                    self.p += 2;
                },
                5 => {
                    let op = self.combo()?;
                    self.out.push(op % 8);
                    self.p += 2;
                },
                6 => {
                    let num = self.registers[0];
                    let op = self.combo()?;
                    let denominator = 2isize.pow(op as u32);
                    self.registers[1] = num / denominator;
                    self.p += 2;
                },
                7 => {
                    let num = self.registers[0];
                    let op = self.combo()?;
                    let denominator = 2isize.pow(op as u32);
                    self.registers[2] = num / denominator;
                    self.p += 2;
                }
                _ => unreachable!()
            }
            Some(())
        } else {
            None
        }
    }

    fn literal(&self) -> Option<usize> {
        self.instructions.get(self.p + 1).copied()
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
            _ => unreachable!()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
