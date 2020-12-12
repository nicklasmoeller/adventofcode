use std::collections::HashMap;

use crate::AdventOfCode;

type Bag = HashMap<String, usize>;
type Bags = HashMap<String, Bag>;
struct BagContainer {
    bags: Bags,
}

impl BagContainer {
    fn parse(input: &str) -> Self {
        let container = input
            .lines()
            .map(|line| {
                let mut bag_rules = line.split(" bags contain ");

                let outer_bag = bag_rules
                    .next()
                    .expect("Didnt find an outer bag")
                    .to_string();

                let contain_map = bag_rules
                    .next()
                    .expect("Didnt find any rules")
                    .split(", ")
                    .filter_map(|rule| {
                        let count = rule.split(' ').next().expect("No quantity provided");

                        let quantity = match count {
                            "no" => None,
                            number => number.parse::<usize>().ok(),
                        }?;

                        let remaining_traits = &rule[count.len() + 1..];

                        let bag_traits = remaining_traits
                            .rsplitn(2, ' ')
                            .nth(1)
                            .expect("No traits found")
                            .to_string();

                        (bag_traits, quantity).into()
                    })
                    .collect::<Bag>();

                (outer_bag, contain_map)
            })
            .collect::<Bags>();

        Self { bags: container }
    }

    fn contains_bag(&self, search_trait: &str, bag_trait: &str) -> bool {
        self.bags[bag_trait].keys().any(|inner_trait| {
            inner_trait == search_trait || self.contains_bag(search_trait, inner_trait)
        })
    }

    fn get_outer_bags_containing(&self, search_trait: &str) -> usize {
        self.bags
            .keys()
            .filter(|&bag_trait| self.contains_bag(search_trait, bag_trait))
            .count()
    }

    fn get_number_of_bags(&self, bag_trait: &str) -> usize {
        self.bags[bag_trait]
            .iter()
            .fold(1, |sum, (bag_trait, count)| {
                sum + (count * self.get_number_of_bags(bag_trait))
            })
    }
}

pub struct Day07 {}

impl AdventOfCode for Day07 {
    fn part_one(&self, input: &str) -> String {
        (BagContainer::parse(input).get_outer_bags_containing("shiny gold")).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        (BagContainer::parse(input).get_number_of_bags("shiny gold") - 1).to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_examples_part_one() {
        let input = String::from(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.",
        );
        assert_eq!((Day07 {}).part_one(&input), "4");
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/07.txt").expect("Could not read input file");
        assert_eq!((Day07 {}).part_one(&input), "131");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.",
        );
        assert_eq!((Day07 {}).part_two(&input), "32");

        let long_topology_input = String::from(
            "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.",
        );
        assert_eq!((Day07 {}).part_two(&long_topology_input), "126");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/07.txt").expect("Could not read input file");
        assert_eq!((Day07 {}).part_two(&input), "11261");
    }
}
