use crate::AdventOfCode;

pub struct Day04 {}

struct Parser<'a> {
    field: &'a str,
    rules: Vec<Box<dyn Validate>>,
}

impl<'a> Parser<'a> {
    fn parse(&self, input: &'a str) -> bool {
        match input
            .split_whitespace()
            .into_iter()
            .find(|&key| key.contains(self.field))
        {
            Some(set) => {
                let value = set
                    .split(':')
                    .into_iter()
                    .nth(1)
                    .expect("Key doesn't have a value");

                self.rules.iter().any(|rule| rule.validate(value))
            }
            None => false,
        }
    }
}

trait Validate {
    fn validate(&self, input: &str) -> bool;
}

struct Range {
    min: usize,
    max: usize,
}

impl Validate for Range {
    fn validate(&self, input: &str) -> bool {
        let number = input.parse::<usize>().expect("Not a number");

        number >= self.min && number <= self.max
    }
}

struct SizedRange {
    size: usize,
    range: Range,
}

impl Validate for SizedRange {
    fn validate(&self, input: &str) -> bool {
        if input.len() != self.size {
            return false;
        }

        self.range.validate(input)
    }
}

struct SuffixedRange<'a> {
    suffix: &'a str,
    range: Range,
}

impl<'a> Validate for SuffixedRange<'a> {
    fn validate(&self, input: &str) -> bool {
        if !input.ends_with(self.suffix) {
            return false;
        }

        let number = &input[..input.len() - self.suffix.len()];

        self.range.validate(number)
    }
}

struct Hex;

impl Validate for Hex {
    fn validate(&self, input: &str) -> bool {
        input.len() == 7
            && input.chars().nth(0) == Some('#')
            && match u64::from_str_radix(&input[1..], 16) {
                Ok(_) => true,
                Err(_) => false,
            }
    }
}

struct OneOf<'a> {
    variants: Vec<&'a str>,
}

impl<'a> Validate for OneOf<'a> {
    fn validate(&self, input: &str) -> bool {
        self.variants.contains(&input)
    }
}

impl AdventOfCode for Day04 {
    fn part_one(&self, input: &String) -> String {
        let required_properties = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        let fields = input.split("\n\n");

        fields
            .fold(0, |number_of_valid_passports, passport| {
                if required_properties
                    .iter()
                    .all(|&property| passport.contains(property))
                {
                    number_of_valid_passports + 1
                } else {
                    number_of_valid_passports
                }
            })
            .to_string()
    }

    fn part_two(&self, input: &String) -> String {
        let required_properties = vec![
            (Parser {
                field: "byr",
                rules: vec![Box::new(Range {
                    min: 1920,
                    max: 2002,
                })],
            }),
            (Parser {
                field: "iyr",
                rules: vec![Box::new(Range {
                    min: 2010,
                    max: 2020,
                })],
            }),
            (Parser {
                field: "eyr",
                rules: vec![Box::new(Range {
                    min: 2020,
                    max: 2030,
                })],
            }),
            (Parser {
                field: "hgt",
                rules: vec![
                    Box::new(SuffixedRange {
                        suffix: "cm",
                        range: Range { min: 150, max: 193 },
                    }),
                    Box::new(SuffixedRange {
                        suffix: "in",
                        range: Range { min: 59, max: 76 },
                    }),
                ],
            }),
            (Parser {
                field: "hcl",
                rules: vec![Box::new(Hex {})],
            }),
            (Parser {
                field: "ecl",
                rules: vec![Box::new(OneOf {
                    variants: vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"],
                })],
            }),
            (Parser {
                field: "pid",
                rules: vec![Box::new(SizedRange {
                    size: 9,
                    range: Range {
                        min: 0,
                        max: 999999999,
                    },
                })],
            }),
        ];

        let fields = input.split("\n\n");

        fields
            .fold(0, |number_of_valid_passports, passport| {
                if required_properties
                    .iter()
                    .all(|parser| parser.parse(passport))
                {
                    number_of_valid_passports + 1
                } else {
                    number_of_valid_passports
                }
            })
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
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in",
        );
        assert_eq!((Day04 {}).part_one(&input), "2");
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/04.txt").expect("Could not read input file");
        assert_eq!((Day04 {}).part_one(&input), "264");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from(
            "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007",
        );
        assert_eq!((Day04 {}).part_two(&input), "0");

        let input2 = String::from(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        );
        assert_eq!((Day04 {}).part_two(&input2), "4");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/04.txt").expect("Could not read input file");
        assert_eq!((Day04 {}).part_two(&input), "224");
    }
}
