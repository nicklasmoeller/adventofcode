use crate::AdventOfCode;

pub struct Day01 {}

impl AdventOfCode for Day01 {
    fn part_one(&self, input: &String) -> String {
        let lines = input
            .lines()
            .map(|line| { line.parse::<usize>().expect("Not a number") })
            .collect::<Vec<usize>>();

        lines.iter().find_map(|first| {
            match lines.iter().find(|second| {
                first + *second == 2020
            }) {
                Some(matching) => Some(first * matching),
                None => None
            }
        }).expect("No matches").to_string()
    }

    fn part_two(&self, input: &String) -> String {
        let lines = input
            .lines()
            .map(|line| { line.parse::<usize>().expect("Not a number") })
            .collect::<Vec<usize>>();

        lines.iter().find_map(|first| {
            lines.iter().find_map(|second| {
                match lines.iter().find(|third| {
                    first + *second + *third == 2020
                }) {
                    Some(matching) => Some(first * second * matching),
                    None => None
                }
            })
        }).expect("No matches").to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_examples_part_one() {
        let input = String::from("1721\n979\n366\n299\n675\n1456");
        assert_eq!((Day01 {}).part_one(&input), "514579");
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/01.txt").expect("Could not read input file");
        assert_eq!((Day01 {}).part_one(&input), "876459");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from("1721\n979\n366\n299\n675\n1456");
        assert_eq!((Day01 {}).part_two(&input), "241861950");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/01.txt").expect("Could not read input file");
        assert_eq!((Day01 {}).part_two(&input), "116168640");
    }
}