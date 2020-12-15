use crate::AdventOfCode;

struct CountingGame {
    end_at: usize,
}

impl CountingGame {
    fn play(&self, start_numbers: Vec<usize>) -> usize {
        let mut spoken_numbers = vec![0; self.end_at];

        start_numbers
            .iter()
            .enumerate()
            .for_each(|(index, &number)| {
                spoken_numbers[number] = index + 1;
            });

        let last_spoken_start_number = start_numbers
            .last()
            .copied()
            .expect("The starting list was empty");

        (start_numbers.len()..self.end_at).fold(
            last_spoken_start_number,
            |last_spoken_number, current_turn| {
                let last_spoken_turn = spoken_numbers[last_spoken_number];
                spoken_numbers[last_spoken_number] = current_turn;

                if last_spoken_turn != 0 {
                    current_turn - last_spoken_turn
                } else {
                    0
                }
            },
        )
    }
}

pub struct Day15 {}

impl AdventOfCode for Day15 {
    fn part_one(&self, input: &str) -> String {
        let start_numbers: Vec<usize> = input
            .split(',')
            .map(|start_number| {
                start_number
                    .parse::<usize>()
                    .expect("Start number was not a number")
            })
            .collect();

        let game = CountingGame { end_at: 2020 };

        game.play(start_numbers).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let start_numbers: Vec<usize> = input
            .split(',')
            .map(|start_number| {
                start_number
                    .parse::<usize>()
                    .expect("Start number was not a number")
            })
            .collect();

        let game = CountingGame { end_at: 30_000_000 };

        game.play(start_numbers).to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_examples_part_one() {
        let input = String::from("1,3,2");
        assert_eq!((Day15 {}).part_one(&input), "1");

        let input = String::from("2,1,3");
        assert_eq!((Day15 {}).part_one(&input), "10");

        let input = String::from("1,2,3");
        assert_eq!((Day15 {}).part_one(&input), "27");

        let input = String::from("2,3,1");
        assert_eq!((Day15 {}).part_one(&input), "78");

        let input = String::from("3,2,1");
        assert_eq!((Day15 {}).part_one(&input), "438");

        let input = String::from("3,1,2");
        assert_eq!((Day15 {}).part_one(&input), "1836");
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/15.txt").expect("Could not read input file");
        assert_eq!((Day15 {}).part_one(&input), "234");
    }

    #[test]
    #[ignore = "Simply too slow to bother"]
    fn test_examples_part_two() {
        let input = String::from("0,3,6");
        assert_eq!((Day15 {}).part_two(&input), "175594");

        let input = String::from("1,3,2");
        assert_eq!((Day15 {}).part_two(&input), "2578");

        let input = String::from("2,1,3");
        assert_eq!((Day15 {}).part_two(&input), "3544142");

        let input = String::from("1,2,3");
        assert_eq!((Day15 {}).part_two(&input), "261214");

        let input = String::from("2,3,1");
        assert_eq!((Day15 {}).part_two(&input), "6895259");

        let input = String::from("3,2,1");
        assert_eq!((Day15 {}).part_two(&input), "18");

        let input = String::from("3,1,2");
        assert_eq!((Day15 {}).part_two(&input), "362");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/15.txt").expect("Could not read input file");
        assert_eq!((Day15 {}).part_two(&input), "8984");
    }
}
