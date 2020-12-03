use crate::AdventOfCode;

pub struct Day03 {}

impl AdventOfCode for Day03 {
    fn part_one(&self, input: &String) -> String {
        input
            .lines()
            .fold((0, 0), |(horizontal_steps, passed_trees), line: &str| {
                let wrapped_position = (line.len() + horizontal_steps) % line.len();

                (
                    horizontal_steps + 3,
                    if line.chars().nth(wrapped_position).unwrap() == '#' {
                        passed_trees + 1
                    } else {
                        passed_trees
                    },
                )
            })
            .1
            .to_string()
    }

    fn part_two(&self, input: &String) -> String {
        let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        slopes
            .iter()
            .map(
                |(horizontal_steps, vertical_steps)| {
                    input
                        .lines()
                        .enumerate()
                        .filter_map(|(index, value)| {
                            match (vertical_steps + index) % vertical_steps {
                                0 => Some(value),
                                _ => None,
                            }
                        })
                        .fold(
                            (0, 0),
                            |(horizontal_steps_taken, passed_trees), line: &str| {
                                let wrapped_position =
                                    (line.len() + horizontal_steps_taken) % line.len();

                                (
                                    horizontal_steps_taken + horizontal_steps,
                                    if line.chars().nth(wrapped_position).unwrap() == '#' {
                                        passed_trees + 1
                                    } else {
                                        passed_trees
                                    },
                                )
                            },
                        )
                        .1
                },
            )
            .product::<usize>()
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
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        );
        assert_eq!((Day03 {}).part_one(&input), "7");
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/03.txt").expect("Could not read input file");
        assert_eq!((Day03 {}).part_one(&input), "270");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        );
        assert_eq!((Day03 {}).part_two(&input), "336");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/03.txt").expect("Could not read input file");
        assert_eq!((Day03 {}).part_two(&input), "2122848000");
    }
}
