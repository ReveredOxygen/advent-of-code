use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use derivative::Derivative;
use itertools::Itertools;

#[derive(Clone, Default, Derivative)]
#[derivative(Debug)]
struct Processor {
    ra: usize,
    rb: usize,
    rc: usize,
    pc: usize,
    #[derivative(Debug = "ignore")]
    program: Vec<u8>,

    terminated: bool,
    output: Vec<usize>,
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Processor {
    let mut out = Processor::default();
    let lines: Vec<_> = input.lines().collect();

    out.ra = lines[0][12..].parse().unwrap();
    out.rb = lines[1][12..].parse().unwrap();
    out.rc = lines[2][12..].parse().unwrap();
    out.pc = 0;
    out.terminated = false;

    out.program = lines[4][9..]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    out
}

impl Processor {
    fn run(&mut self) {
        while !self.terminated {
            self.step();
        }
    }

    fn step(&mut self) {
        if self.pc >= self.program.len() || self.terminated {
            self.terminated = true;
        } else {
            let combo_operand = match self.program[self.pc + 1] {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => self.ra,
                5 => self.rb,
                6 => self.rc,
                _ => 0,
            };
            let literal_operand = self.program[self.pc + 1] as usize;

            match self.program[self.pc] {
                0 => {
                    // adv
                    self.ra = self.ra / 2_usize.pow(combo_operand.try_into().unwrap());
                    self.pc += 2;
                }
                1 => {
                    // bxl
                    self.rb = self.rb ^ literal_operand;
                    self.pc += 2;
                }
                2 => {
                    self.rb = combo_operand % 8;
                    self.pc += 2;
                }
                3 => {
                    // jnz
                    if self.ra != 0 {
                        self.pc = literal_operand as usize;
                    } else {
                        self.pc += 2;
                    }
                }
                4 => {
                    // bxc
                    self.rb = self.rb ^ self.rc;
                    self.pc += 2;
                }
                5 => {
                    // out
                    self.output.push(combo_operand % 8);
                    self.pc += 2;
                }
                6 => {
                    // bdv
                    self.rb = self.ra / 2_usize.pow(combo_operand.try_into().unwrap());
                    self.pc += 2;
                }
                7 => {
                    // cdv
                    self.rc = self.ra / 2_usize.pow(combo_operand.try_into().unwrap());
                    self.pc += 2;
                }
                _ => panic!("invalid opcode"),
            }
        }
    }
}

#[aoc(day17, part1)]
fn part1(input: &Processor) -> String {
    let mut proc = input.clone();

    proc.run();

    proc.output.iter().join(",")
}

#[aoc(day17, part2)]
fn part2(input: &Processor) -> usize {
    let program = input.program.as_slice();

    let mut candidates: HashSet<usize> = HashSet::from_iter(0..7);

    for term in 1..16_usize {
        candidates = candidates
            .drain()
            .map(|c| (0..(2 << 8)).map(move |new| c | new << (term * 3)))
            .flatten()
            .filter(|c| {
                let mut p = input.clone();
                p.ra = *c;
                p.run();

                p.output
                    .iter()
                    .take(term)
                    .map(|x| *x)
                    .eq(program.iter().take(term).map(|x| *x as usize))
            })
            .collect();

        dbg!(&candidates);
    }

    *candidates.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#
            )),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }
}
