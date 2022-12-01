use std::collections::BinaryHeap;

use crate::AdventOfCode;

pub struct Day01;

impl AdventOfCode for Day01 {
    fn part_one(&self, input: &str) -> String {
        input
            .split("\n\n")
            .map(|entry| {
                entry
                    .lines()
                    .map(|calories| calories.parse::<usize>().expect("Failed to parse calories"))
                    .into_iter()
                    .sum()
            })
            .max()
            .unwrap_or(0)
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input
            .split("\n\n")
            .map(|entry| {
                entry
                    .lines()
                    .map(|calories| calories.parse::<usize>().expect("Failed to parse calories"))
                    .into_iter()
                    .sum()
            })
            .collect::<BinaryHeap<usize>>()
            .iter()
            .take(3)
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part_one() {
        let input = String::from(
            "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );

        assert_eq!((Day01 {}).part_one(&input), "24000");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from(
            "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );

        assert_eq!((Day01 {}).part_two(&input), "45000");
    }
}
