use std::collections::HashSet;

use crate::AdventOfCode;

#[derive(Clone, Copy)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

type BootCode = Vec<Instruction>;

#[derive(Clone, Default)]
struct Device {
    boot_code: BootCode,
    pc: usize,
    accumulator: isize,
}

impl Device {
    fn from_string(input: &str) -> Self {
        Self {
            boot_code: input
                .lines()
                .map(|line| {
                    let mut instruction = line.splitn(2, ' ');

                    let operation = instruction.next().expect("No operation found");
                    let argument = instruction
                        .next()
                        .expect("No argument found")
                        .parse::<isize>()
                        .expect("Argument not a number");

                    match operation {
                        "acc" => Instruction::Acc(argument),
                        "jmp" => Instruction::Jmp(argument),
                        "nop" => Instruction::Nop(argument),
                        _ => panic!("Invalid operation"),
                    }
                })
                .collect(),
            ..Default::default()
        }
    }

    fn run_instruction(&mut self) {
        match self.boot_code[self.pc] {
            Instruction::Acc(argument) => {
                self.accumulator += argument;
                self.pc += 1;
            }
            Instruction::Jmp(argument) => {
                self.pc = (self.pc as isize + argument) as usize;
            }
            Instruction::Nop(_) => {
                self.pc += 1;
            }
        }
    }

    fn run(&mut self) -> Result<(), ()> {
        let mut history: HashSet<usize> = HashSet::new();

        while !history.contains(&self.pc) && self.pc < self.boot_code.len() {
            history.insert(self.pc);
            self.run_instruction();
        }

        if self.pc >= self.boot_code.len() - 1 {
            Ok(())
        } else {
            Err(())
        }
    }
}

pub struct Day08 {}

impl AdventOfCode for Day08 {
    fn part_one(&self, input: &String) -> String {
        let mut device = Device::from_string(input);

        match device.run() {
            Err(_) => device.accumulator,
            _ => panic!("Didn't run forever"),
        }
        .to_string()
    }

    fn part_two(&self, input: &String) -> String {
        let device = Device::from_string(input);

        device
            .boot_code
            .iter()
            .enumerate()
            .filter(|(_, &instruction)| match instruction {
                Instruction::Jmp(_) => true,
                Instruction::Nop(_) => true,
                _ => false,
            })
            .find_map(|(pc, &instruction)| {
                let mut new_device = device.clone();

                new_device.boot_code[pc] = match instruction {
                    Instruction::Jmp(argument) => Instruction::Nop(argument),
                    Instruction::Nop(argument) => Instruction::Jmp(argument),
                    op => op,
                };

                match new_device.run() {
                    Ok(()) => Some(new_device.accumulator),
                    Err(()) => None,
                }
            })
            .expect("No operation replacement resulted in a succesful run")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_examples_part_one() {
        let input = String::from(
            "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6",
        );
        assert_eq!((Day08 {}).part_one(&input), "5");
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/08.txt").expect("Could not read input file");
        assert_eq!((Day08 {}).part_one(&input), "1600");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from(
            "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6",
        );
        assert_eq!((Day08 {}).part_two(&input), "8");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/08.txt").expect("Could not read input file");
        assert_eq!((Day08 {}).part_two(&input), "1543");
    }
}
