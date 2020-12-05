use crate::AdventOfCode;

pub struct Day05 {}

struct Seat;

impl Seat {
    fn search_for_pos(sequence: &str, length: usize) -> usize {
        sequence[0..length]
            .chars()
            .fold(
                (0usize, (2usize.pow(length as u32)) - 1),
                |(start, end), letter| {
                    let mid = (start + end) / 2;

                    match letter {
                        'F' | 'L' => (start, mid),
                        'B' | 'R' => (mid + 1, end),
                        _ => (start, end),
                    }
                },
            )
            .0
    }

    fn calculate_id(sequence: &str) -> usize {
        let row = Self::search_for_pos(&sequence[..7], 7);
        let column = Self::search_for_pos(&sequence[7..10], 3);

        row * 8 + column
    }
}

impl AdventOfCode for Day05 {
    fn part_one(&self, input: &String) -> String {
        input
            .lines()
            .map(|line| Seat::calculate_id(line))
            .max()
            .expect("No seats found")
            .to_string()
    }

    fn part_two(&self, input: &String) -> String {
        let mut seats = input
            .lines()
            .map(|line| Seat::calculate_id(line))
            .collect::<Vec<usize>>();

        seats.sort_unstable();

        seats
            .windows(2)
            .find_map(|seat_set| {
                if seat_set[1] - seat_set[0] == 2 {
                    Some(seat_set[0] + 1)
                } else {
                    None
                }
            })
            .expect("Didn't find any leftover seat")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_generate_seat_id_examples() {
        assert_eq!(Seat::calculate_id("FBFBBFFRLR"), 357);
        assert_eq!(Seat::calculate_id("BFFFBBFRRR"), 567);
        assert_eq!(Seat::calculate_id("FFFBBBFRRR"), 119);
        assert_eq!(Seat::calculate_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_examples_part_one() {
        let input = String::from("FBFBBFFRLR\nBFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL");
        assert_eq!((Day05 {}).part_one(&input), "820");
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/05.txt").expect("Could not read input file");
        assert_eq!((Day05 {}).part_one(&input), "915");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/05.txt").expect("Could not read input file");
        assert_eq!((Day05 {}).part_two(&input), "699");
    }
}
