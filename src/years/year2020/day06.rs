use std::collections::HashSet;

use crate::AdventOfCode;

pub struct Day06 {}

impl AdventOfCode for Day06 {
    fn part_one(&self, input: &String) -> String {
        input
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .flat_map(|answers| answers.chars())
                    .collect::<HashSet<char>>()
                    .len()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &String) -> String {
        input
            .split("\n\n")
            .map(|group| {
                let all_group_answers = group
                    .lines()
                    .flat_map(|answers| answers.chars())
                    .collect::<HashSet<char>>();

                group
                    .lines()
                    .map(|answers| answers.chars().collect::<HashSet<char>>())
                    .fold(all_group_answers, |answers, group| {
                        answers.intersection(&group).cloned().collect()
                    })
                    .len()
            })
            .sum::<usize>()
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
            "abc

a
b
c

ab
ac

a
a
a
a

b",
        );
        assert_eq!((Day06 {}).part_one(&input), "11");
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/06.txt").expect("Could not read input file");
        assert_eq!((Day06 {}).part_one(&input), "6782");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from(
            "abc

a
b
c

ab
ac

a
a
a
a

b",
        );
        assert_eq!((Day06 {}).part_two(&input), "6");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/06.txt").expect("Could not read input file");
        assert_eq!((Day06 {}).part_two(&input), "3596");
    }
}
