use crate::AdventOfCode;

pub struct Day02;

fn parse(entry: &str) -> (usize, usize) {
    let (them, you) = entry.split_once(' ').expect("Invalid guide entry");

    (
        usize::from(them.as_bytes()[0] - b'A'),
        usize::from(you.as_bytes()[0] - b'X'),
    )
}
impl AdventOfCode for Day02 {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .map(parse)
            .map(|(them, you)| (((4 + you - them) % 3) * 3) + (1 + you))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input
            .lines()
            .map(parse)
            .map(|(them, result)| (result * 3) + (1 + ((result + them + 2) % 3)))
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
A Y
B X
C Z",
        );

        assert_eq!((Day02 {}).part_one(&input), "15");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from(
            "\
A Y
B X
C Z",
        );

        assert_eq!((Day02 {}).part_two(&input), "12");
    }
}
