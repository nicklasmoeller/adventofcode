use std::collections::HashMap;

use crate::AdventOfCode;

pub struct Day14 {}

impl AdventOfCode for Day14 {
    fn part_one(&self, input: &str) -> String {
        let mut memory: HashMap<usize, usize> = HashMap::new();
        let mut mask = "";

        input.lines().for_each(|line| {
            let mut command = line.splitn(2, " = ");

            let operation = command.next().unwrap();
            let argument = command.next().unwrap();

            if operation == "mask" {
                mask = argument;
            } else {
                let value = argument.parse::<usize>().expect("Not a number");
                let memory_address = operation
                    [operation.find('[').unwrap() + 1..operation.find(']').unwrap()]
                    .parse::<usize>()
                    .expect("Not a number");

                let masked_value = mask.bytes().rev().enumerate().fold(
                    value,
                    |value, (position, byte)| match byte {
                        b'X' => value,
                        b'0' => value & !(1 << position),
                        b'1' => value | (1 << position),
                        _ => panic!("Mask value at position {} is invalid: {}", position, byte),
                    },
                );

                memory.insert(memory_address, masked_value);
            }
        });

        memory.values().sum::<usize>().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut memory: HashMap<usize, usize> = HashMap::new();
        let mut mask = "";

        input.lines().for_each(|line| {
            let mut command = line.splitn(2, " = ");

            let operation = command.next().unwrap();
            let argument = command.next().unwrap();

            if operation == "mask" {
                mask = argument;
            } else {
                let value = argument.parse::<usize>().expect("Not a number");
                let memory_address = operation
                    [operation.find('[').unwrap() + 1..operation.find(']').unwrap()]
                    .parse::<usize>()
                    .expect("Not a number");

                let (floating_bits, memory_address) = mask.bytes().rev().enumerate().fold(
                    (Vec::new(), memory_address),
                    |(mut floating_bits, mut memory_address), (position, byte)| {
                        match byte {
                            b'0' => (),
                            b'X' => floating_bits.push(position),
                            b'1' => memory_address |= 1 << position,
                            _ => panic!("Mask value at position {} is invalid: {}", position, byte),
                        };

                        (floating_bits, memory_address)
                    },
                );

                for position in 0..1 << floating_bits.len() {
                    let mut new_memory_address = memory_address;

                    floating_bits.iter().enumerate().for_each(
                        |(floating_position, floating_bit)| match (position
                            & (1 << floating_position))
                            >> floating_position
                        {
                            0 => new_memory_address &= !(1 << floating_bit),
                            1 => new_memory_address |= 1 << floating_bit,
                            _ => unreachable!(),
                        },
                    );

                    memory.insert(new_memory_address, value);
                }
            }
        });

        memory.values().sum::<usize>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_examples_part_one() {
        let input = String::from(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0",
        );
        assert_eq!((Day14 {}).part_one(&input), "165");
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/14.txt").expect("Could not read input file");
        assert_eq!((Day14 {}).part_one(&input), "14862056079561");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from(
            "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1",
        );
        assert_eq!((Day14 {}).part_two(&input), "208");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/14.txt").expect("Could not read input file");
        assert_eq!((Day14 {}).part_two(&input), "3296185383161");
    }
}
