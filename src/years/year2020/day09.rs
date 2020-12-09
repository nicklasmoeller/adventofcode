use crate::AdventOfCode;

struct XMAS {
    preamble_size: usize,
}

impl Default for XMAS {
    fn default() -> Self {
        Self { preamble_size: 25 }
    }
}

impl XMAS {
    fn find_invalid_number(&self, list: &Vec<usize>) -> Option<usize> {
        list.iter().enumerate().skip(self.preamble_size).find_map(
            |(next_number_index, &next_number)| {
                let set = &list[next_number_index - self.preamble_size..next_number_index];

                if set
                    .iter()
                    .enumerate()
                    .find(|(other_index, &other)| {
                        if next_number < other {
                            return false;
                        }

                        let expected = next_number - other;

                        let lhs = &set[..*other_index];
                        let rhs = &set[*other_index + 1..];

                        lhs.contains(&expected) || rhs.contains(&expected)
                    })
                    .is_some()
                {
                    None
                } else {
                    Some(next_number)
                }
            },
        )
    }

    fn find_encryption_weakness(&self, list: &Vec<usize>) -> Option<usize> {
        match self.find_invalid_number(list) {
            Some(invalid) => list.iter().enumerate().find_map(|(number_index, _number)| {
                let set = &list[number_index..];

                let test =
                    set.iter()
                        .try_fold((0usize, 0usize, 0usize), |(sum, min, max), &value| {
                            let max = std::cmp::max(value, max);
                            let min = if min == 0 {
                                value
                            } else {
                                std::cmp::min(value, min)
                            };

                            match value {
                                _ if sum + value > invalid => Err(None),
                                _ if sum + value == invalid => Err(Some(min + max)),
                                _ => Ok((sum + value, min, max)),
                            }
                        });

                test.err()?
            }),
            None => None,
        }
    }
}

pub struct Day09;

impl Day09 {
    fn parse(input: &String) -> Vec<usize> {
        input
            .lines()
            .map(|x| x.parse::<usize>().expect("Line was not a number"))
            .collect()
    }
}

impl AdventOfCode for Day09 {
    fn part_one(&self, input: &String) -> String {
        let xmas = XMAS::default();

        let collection = Self::parse(input);

        xmas.find_invalid_number(&collection)
            .expect("No invalid number found")
            .to_string()
    }

    fn part_two(&self, input: &String) -> String {
        let xmas = XMAS::default();

        let collection = Self::parse(input);

        xmas.find_encryption_weakness(&collection)
            .expect("No encryption weakness found")
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
            "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
        );

        let collection = Day09::parse(&input);

        let xmas = XMAS { preamble_size: 5 };

        assert_eq!(xmas.find_invalid_number(&collection), Some(127));
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/09.txt").expect("Could not read input file");
        assert_eq!((Day09 {}).part_one(&input), "85848519");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from(
            "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
        );

        let collection = Day09::parse(&input);

        let xmas = XMAS { preamble_size: 5 };

        assert_eq!(xmas.find_encryption_weakness(&collection), Some(62));
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/09.txt").expect("Could not read input file");
        assert_eq!((Day09 {}).part_two(&input), "13414198");
    }
}
