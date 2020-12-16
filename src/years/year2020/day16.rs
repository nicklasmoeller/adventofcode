use std::collections::{HashMap, HashSet};
use std::str::Lines;

use crate::AdventOfCode;

type Rules = HashMap<String, Vec<usize>>;

pub struct Day16 {}

impl Day16 {
    fn collect_rules(lines: Lines) -> Rules {
        lines
            .map(|line| {
                let mut new_line = line.split(": ");

                let rule_description = new_line
                    .next()
                    .expect("Expected to find a rule description")
                    .to_string();

                let all_viable_numbers = new_line
                    .next()
                    .map(|inner_rule| {
                        inner_rule
                            .split(" or ")
                            .flat_map(|in_rule| {
                                let bounds: Vec<usize> = in_rule
                                    .split('-')
                                    .map(|num| {
                                        num.parse::<usize>().expect("Could not parse number")
                                    })
                                    .collect();

                                (bounds[0]..=bounds[1]).collect::<Vec<usize>>()
                            })
                            .collect::<Vec<usize>>()
                    })
                    .unwrap();

                (rule_description, all_viable_numbers)
            })
            .collect()
    }

    fn get_valid_tickets(rules: &Rules, tickets: Vec<String>) -> Vec<Vec<usize>> {
        tickets
            .iter()
            .map(|line| {
                line.split(',')
                    .map(|number| number.parse::<usize>().expect("Not a number"))
                    .collect::<Vec<usize>>()
            })
            .filter(|ticket| {
                ticket.iter().all(|number| {
                    rules
                        .iter()
                        .any(|(_, valid_numbers)| valid_numbers.contains(number))
                })
            })
            .collect()
    }

    fn calculate_rule_positions(
        rules: &Rules,
        tickets: Vec<Vec<usize>>,
    ) -> Result<HashMap<String, usize>, ()> {
        let mut possible_positions: HashMap<usize, HashSet<String>> =
            tickets.iter().fold(HashMap::new(), |map, ticket| {
                ticket
                    .iter()
                    .enumerate()
                    .fold(map, |mut map, (position, field)| {
                        match map.get_mut(&position) {
                            Some(possible_rules) => {
                                let new_values = Self::get_valid_field_rules(rules, field);
                                let intersection = possible_rules.intersection(&new_values);

                                *possible_rules = intersection.cloned().collect();
                            }
                            None => {
                                map.insert(position, Self::get_valid_field_rules(rules, field));
                            }
                        };

                        map
                    })
            });

        let mut found_fields: HashMap<String, usize> = HashMap::new();

        // TODO: Determining which columns belongs to which rules could certainly be optimized
        for _ in 0..=rules.len() {
            if found_fields.len() >= rules.len() {
                break;
            }

            possible_positions
                .iter_mut()
                .for_each(|(&position, possible_rules)| {
                    if possible_rules.len() == 1 {
                        found_fields
                            .insert(possible_rules.iter().next().unwrap().to_owned(), position);
                    } else {
                        let found_rules = found_fields.keys().cloned().collect::<HashSet<String>>();
                        let diff = possible_rules.difference(&found_rules);

                        *possible_rules = diff.cloned().collect();
                    }
                });
        }

        match found_fields.len() {
            matched_fields if matched_fields > 0 => Ok(found_fields),
            _ => Err(()),
        }
    }

    fn get_valid_field_rules(rules: &Rules, field: &usize) -> HashSet<String> {
        rules
            .iter()
            .filter(|&(_, possible_positions)| possible_positions.contains(field))
            .map(|(name, _)| name.clone())
            .collect()
    }

    fn get_invalid_fields_from_all_tickets(rules: &Rules, tickets: Vec<String>) -> Vec<usize> {
        tickets
            .iter()
            .flat_map(|line| {
                line.split(',')
                    .map(|number| number.parse::<usize>().expect("Not a number"))
            })
            .filter(|number| {
                rules
                    .iter()
                    .all(|(_, valid_numbers)| !valid_numbers.contains(number))
            })
            .collect()
    }
}

impl AdventOfCode for Day16 {
    fn part_one(&self, input: &str) -> String {
        let mut sections = input.split("\n\n");

        let rules = Self::collect_rules(
            sections
                .next()
                .expect("Didn't find a rules section")
                .lines(),
        );

        let nearby_tickets: Vec<String> = sections
            .nth(1)
            .expect("Didn't find any nearby tickets")
            .lines()
            .skip(1)
            .map(|line| line.to_string())
            .collect();

        let invalid_numbers = Self::get_invalid_fields_from_all_tickets(&rules, nearby_tickets);

        invalid_numbers.iter().sum::<usize>().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut sections = input.split("\n\n");

        let rules = Self::collect_rules(
            sections
                .next()
                .expect("Didn't find a rules section")
                .lines(),
        );

        let your_ticket: Vec<usize> = sections
            .next()
            .expect("Didn't find your ticket")
            .lines()
            .skip(1)
            .flat_map(|line| {
                line.split(',')
                    .map(|number| number.parse::<usize>().expect("Not a number"))
            })
            .collect();

        let nearby_tickets: Vec<String> = sections
            .next()
            .expect("Didn't find any nearby tickets")
            .lines()
            .skip(1)
            .map(|line| line.to_string())
            .collect();

        let valid_tickets = Self::get_valid_tickets(&rules, nearby_tickets);

        match Self::calculate_rule_positions(&rules, valid_tickets) {
            Ok(rule_positions) => rule_positions
                .iter()
                .filter(|&(name, _)| name.starts_with("departure"))
                .map(|(_, position)| your_ticket.get(*position).unwrap())
                .product::<usize>()
                .to_string(),
            _ => panic!("Could not determine what rules your columns belonged to"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_examples_part_one() {
        let input = String::from(
            "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12",
        );
        assert_eq!((Day16 {}).part_one(&input), "71");
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/16.txt").expect("Could not read input file");
        assert_eq!((Day16 {}).part_one(&input), "32842");
    }

    #[test]
    fn test_examples_modified_part_two() {
        // Modified input to contain departure on two of the rules
        let input = String::from(
            "departure_class: 0-1 or 4-19
row: 0-5 or 8-19
departure_seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9",
        );

        assert_eq!((Day16 {}).part_two(&input), "156");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/16.txt").expect("Could not read input file");
        assert_eq!((Day16 {}).part_two(&input), "2628667251989");
    }
}
