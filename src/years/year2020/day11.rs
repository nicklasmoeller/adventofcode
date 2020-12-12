use crate::AdventOfCode;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Floor,
    Occupied,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Tile::Empty => write!(f, "L"),
            Tile::Floor => write!(f, "."),
            Tile::Occupied => write!(f, "#"),
        }
    }
}

#[derive(Clone)]
struct WaitingArea {
    tiles: Vec<Tile>,
    width: u8,
    height: u8,
    include_floor: bool,
    tolerance: u8,
}

impl WaitingArea {
    fn new(input: &str, include_floor: bool, tolerance: u8) -> Self {
        Self {
            tiles: input
                .chars()
                .filter_map(|c| match c {
                    'L' => Some(Tile::Empty),
                    '.' => Some(Tile::Floor),
                    '#' => Some(Tile::Occupied),
                    '\n' => None,
                    l => panic!(
                        "Input invalid, found {:?} but expected 'L', '.', '#' or '\\n'",
                        l
                    ),
                })
                .collect::<Vec<Tile>>(),
            width: input.lines().next().unwrap().len() as u8,
            height: input.lines().count() as u8,
            include_floor,
            tolerance,
        }
    }

    fn get_valid_move_paths() -> Vec<(isize, isize)> {
        vec![
            // Left
            (-1, 0),
            // Up left
            (-1, -1),
            // Up
            (0, -1),
            // Up right
            (1, -1),
            // Right
            (1, 0),
            // Down right
            (1, 1),
            // Down
            (0, 1),
            // Down left
            (-1, 1),
        ]
    }

    fn get_occupied_seats(&self) -> usize {
        self.tiles
            .iter()
            .filter(|&&tile| tile == Tile::Occupied)
            .count()
    }

    fn get_adjacent_tile(
        &self,
        current_index: usize,
        move_x: isize,
        move_y: isize,
    ) -> Option<&Tile> {
        let index = current_index as isize;
        let width = self.width as isize;
        let height = self.height as isize;
        let size = width * height;

        // Calculate whether this would wrap to a new row when not only moving on the y axis
        let invalid_move = if move_x != 0 {
            // Ensure x axis moves doesn't wrap to another row
            let flat_x = index % width;
            let next_x = flat_x + move_x;

            next_x < 0 || next_x >= width
        } else {
            false
        };

        if invalid_move {
            return None;
        }

        let new_index = index + move_y * width + move_x;

        match new_index {
            // Only allow moves within the bounds of the grid
            index if new_index >= 0 && new_index < size => Some(&self.tiles[index as usize]),
            _ => None,
        }
    }

    fn get_adjacent_seat(
        &self,
        current_index: usize,
        move_x: isize,
        move_y: isize,
    ) -> Option<Tile> {
        match self.get_adjacent_tile(current_index, move_x, move_y) {
            Some(Tile::Floor) => {
                if self.include_floor {
                    Some(Tile::Floor)
                } else {
                    let next_x = if move_x > 0 {
                        move_x + 1
                    } else if move_x < 0 {
                        move_x - 1
                    } else {
                        move_x
                    };

                    let next_y = if move_y > 0 {
                        move_y + 1
                    } else if move_y < 0 {
                        move_y - 1
                    } else {
                        move_y
                    };

                    self.get_adjacent_seat(current_index, next_x, next_y)
                }
            }
            Some(tile) => Some(*tile),
            None => None,
        }
    }

    // If a seat is empty and there are no occupied seats adjacent
    // to it, the seat becomes occupied
    fn occupy_if_possible(&self, index: usize) -> Option<Tile> {
        match Self::get_valid_move_paths().iter().find_map(|&(x, y)| {
            match self.get_adjacent_seat(index, x, y) {
                Some(Tile::Occupied) => Some(Tile::Occupied),
                _ => None,
            }
        }) {
            Some(_) => None,
            None => Some(Tile::Occupied),
        }
    }

    // If a seat is occupied and four or more seats adjacent to it
    // are also occupied, the seat becomes empty
    fn leave_is_possible(&self, index: usize) -> Option<Tile> {
        match Self::get_valid_move_paths()
            .iter()
            .try_fold(0, |acc, &(x, y)| {
                match self.get_adjacent_seat(index, x, y) {
                    Some(Tile::Occupied) => {
                        let new_sum = acc + 1;
                        if new_sum >= self.tolerance {
                            Err(new_sum)
                        } else {
                            Ok(new_sum)
                        }
                    }
                    _ => Ok(acc),
                }
            }) {
            Err(_) => Some(Tile::Empty),
            _ => None,
        }
    }

    fn advance(&mut self) -> Result<(), ()> {
        let mut altered = false;
        let new_tiles = self
            .tiles
            .iter()
            .enumerate()
            .map(|(index, tile)| match tile {
                Tile::Empty => {
                    if let Some(occupied) = self.occupy_if_possible(index) {
                        altered = true;

                        occupied
                    } else {
                        *tile
                    }
                }
                Tile::Occupied => {
                    if let Some(leave) = self.leave_is_possible(index) {
                        altered = true;

                        leave
                    } else {
                        *tile
                    }
                }
                Tile::Floor => *tile,
            })
            .collect();

        if altered {
            self.tiles = new_tiles;

            Ok(())
        } else {
            Err(())
        }
    }
}

impl std::fmt::Display for WaitingArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    format!("{}", self.tiles[(y * self.width + x) as usize])
                )?;
            }

            if y < self.height - 1 {
                writeln!(f)?;
            }
        }

        write!(f, "")
    }
}

pub struct Day11 {}

impl AdventOfCode for Day11 {
    fn part_one(&self, input: &String) -> String {
        let mut waiting_area = WaitingArea::new(input, true, 4);

        while waiting_area.advance().is_ok() {}

        waiting_area.get_occupied_seats().to_string()
    }

    fn part_two(&self, input: &String) -> String {
        let mut waiting_area = WaitingArea::new(input, false, 5);

        while waiting_area.advance().is_ok() {}

        waiting_area.get_occupied_seats().to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    mod waiting_area {
        use super::WaitingArea;

        #[test]
        fn test_waiting_area_first_step() {
            let start_area = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

            let mut waiting_area = WaitingArea::new(start_area, true, 4);
            assert_eq!(waiting_area.advance().is_ok(), true);

            let expected_area = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

            assert_eq!(format!("{}", waiting_area), expected_area);
        }

        #[test]
        fn test_waiting_area_second_step() {
            let start_area = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

            let mut waiting_area = WaitingArea::new(start_area, true, 4);
            assert_eq!(waiting_area.advance().is_ok(), true);

            let expected_area = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";

            assert_eq!(format!("{}", waiting_area), expected_area);
        }

        #[test]
        fn test_waiting_area_third_step() {
            let start_area = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";

            let mut waiting_area = WaitingArea::new(start_area, true, 4);
            assert_eq!(waiting_area.advance().is_ok(), true);

            let expected_area = "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##";

            assert_eq!(format!("{}", waiting_area), expected_area);
        }

        #[test]
        fn test_waiting_area_fourth_step() {
            let start_area = "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##";

            let mut waiting_area = WaitingArea::new(start_area, true, 4);
            assert_eq!(waiting_area.advance().is_ok(), true);

            let expected_area = "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##";

            assert_eq!(format!("{}", waiting_area), expected_area);
        }

        #[test]
        fn test_waiting_area_fifth_step() {
            let start_area = "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##";

            let mut waiting_area = WaitingArea::new(start_area, true, 4);
            assert_eq!(waiting_area.advance().is_ok(), true);

            let expected_area = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";

            assert_eq!(format!("{}", waiting_area), expected_area);
        }

        #[test]
        fn test_waiting_area_sixth_step() {
            let start_area = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";

            let mut waiting_area = WaitingArea::new(start_area, true, 4);
            assert_eq!(waiting_area.advance().is_err(), true);

            assert_eq!(format!("{}", waiting_area), start_area);
        }

        #[test]
        fn test_waiting_area_first_step_exclude_floor() {
            let start_area = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

            let mut waiting_area = WaitingArea::new(start_area, false, 5);
            assert_eq!(waiting_area.advance().is_ok(), true);

            let expected_area = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

            assert_eq!(format!("{}", waiting_area), expected_area);
        }

        #[test]
        fn test_waiting_area_second_step_exclude_floor() {
            let start_area = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

            let mut waiting_area = WaitingArea::new(start_area, false, 5);
            assert_eq!(waiting_area.advance().is_ok(), true);

            let expected_area = "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#";

            assert_eq!(format!("{}", waiting_area), expected_area);
        }

        #[test]
        fn test_waiting_area_third_step_exclude_floor() {
            let start_area = "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#";

            let mut waiting_area = WaitingArea::new(start_area, false, 5);
            assert_eq!(waiting_area.advance().is_ok(), true);

            let expected_area = "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#";

            assert_eq!(format!("{}", waiting_area), expected_area);
        }

        #[test]
        fn test_waiting_area_fourth_step_exclude_floor() {
            let start_area = "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#";

            let mut waiting_area = WaitingArea::new(start_area, false, 5);
            assert_eq!(waiting_area.advance().is_ok(), true);

            let expected_area = "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#";

            assert_eq!(format!("{}", waiting_area), expected_area);
        }

        #[test]
        fn test_waiting_area_fifth_step_exclude_floor() {
            let start_area = "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#";

            let mut waiting_area = WaitingArea::new(start_area, false, 5);
            assert_eq!(waiting_area.advance().is_ok(), true);

            let expected_area = "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#";

            assert_eq!(format!("{}", waiting_area), expected_area);
        }

        #[test]
        fn test_waiting_area_sixth_step_exclude_floor() {
            let start_area = "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#";

            let mut waiting_area = WaitingArea::new(start_area, false, 5);
            assert_eq!(waiting_area.advance().is_ok(), true);

            let expected_area = "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#";

            assert_eq!(format!("{}", waiting_area), expected_area);
        }

        #[test]
        fn test_waiting_area_seventh_step_exclude_floor() {
            let start_area = "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#";

            let mut waiting_area = WaitingArea::new(start_area, false, 5);
            assert_eq!(waiting_area.advance().is_err(), true);

            assert_eq!(format!("{}", waiting_area), start_area);
        }
    }

    #[test]
    fn test_examples_part_one() {
        let input = String::from(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        );
        assert_eq!((Day11 {}).part_one(&input), "37");
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/11.txt").expect("Could not read input file");
        assert_eq!((Day11 {}).part_one(&input), "2468");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        );
        assert_eq!((Day11 {}).part_two(&input), "26");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/11.txt").expect("Could not read input file");
        assert_eq!((Day11 {}).part_two(&input), "2214");
    }
}
