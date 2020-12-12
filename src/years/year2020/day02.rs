use crate::AdventOfCode;

pub struct Day02 {}

struct Rule<'a> {
    policy: &'a str,
    password: &'a str,
}

impl<'a> Rule<'a> {
    fn new_from_str(rule: &'a str) -> Self {
        let mut split = rule.split(':');
        let (policy, password) = (
            split.next().expect("No policy provided"),
            split.next().expect("No password provided").trim(),
        );

        Self { policy, password }
    }
}

struct Policy<'a> {
    letter: &'a str,
    min: usize,
    max: usize,
}

impl<'a> Policy<'a> {
    fn new_from_str(policy: &'a str) -> Self {
        let mut policy_details = policy.split_whitespace();
        let (range, letter) = (
            policy_details.next().expect("No range provided"),
            policy_details.next().expect("No letter provided"),
        );

        let mut count = range.split('-');
        let (min, max) = (
            count
                .next()
                .expect("No min value provided")
                .parse::<usize>()
                .expect("Min value not a number"),
            count
                .next()
                .expect("No max value provided")
                .parse::<usize>()
                .expect("Max value not a number"),
        );

        Self { letter, min, max }
    }
}

impl AdventOfCode for Day02 {
    fn part_one(&self, input: &str) -> String {
        let lines = input.lines();
        lines
            .filter(|&line| {
                let Rule { policy, password } = Rule::new_from_str(line);
                let Policy { letter, min, max } = Policy::new_from_str(policy);

                let occurences = password.matches(letter).count();

                occurences >= min && occurences <= max
            })
            .count()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let lines = input.lines();
        lines
            .filter(|&line| {
                let Rule { policy, password } = Rule::new_from_str(line);
                let Policy { letter, min, max } = Policy::new_from_str(policy);

                let password_char_at_first_pos = password.chars().nth(min - 1).unwrap();
                let password_char_at_last_pos = password.chars().nth(max - 1).unwrap();

                (password_char_at_first_pos.to_string() == letter)
                    ^ (password_char_at_last_pos.to_string() == letter)
            })
            .count()
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
            "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc",
        );
        assert_eq!((Day02 {}).part_one(&input), "2");
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/02.txt").expect("Could not read input file");
        assert_eq!((Day02 {}).part_one(&input), "600");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from(
            "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc",
        );
        assert_eq!((Day02 {}).part_two(&input), "1");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/02.txt").expect("Could not read input file");
        assert_eq!((Day02 {}).part_two(&input), "245");
    }
}
