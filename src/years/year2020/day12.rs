use std::convert::TryInto;
use std::ops::{AddAssign, SubAssign};

use crate::AdventOfCode;

#[derive(Default)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl AddAssign<Coordinate> for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl SubAssign<Coordinate> for Coordinate {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
enum Rotation {
    Right,
    Left,
}
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(&self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Right => match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            Rotation::Left => match self {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
        }
    }
}

struct Ship {
    coordinate: Coordinate,
    direction: Direction,
}

impl Ship {
    fn new() -> Self {
        Self {
            coordinate: Coordinate::default(),
            direction: Direction::East,
        }
    }

    fn get_manhattan_distance(&self) -> usize {
        (self.coordinate.x.abs() + self.coordinate.y.abs())
            .try_into()
            .unwrap()
    }

    fn advance(&mut self, action: &str, value: isize) {
        match action {
            "L" => {
                for _ in 0..(value / 90) {
                    self.direction = self.direction.rotate(Rotation::Left);
                }
            }
            "R" => {
                for _ in 0..(value / 90) {
                    self.direction = self.direction.rotate(Rotation::Right);
                }
            }

            "N" => self.coordinate += Coordinate { x: 0, y: value },
            "E" => self.coordinate += Coordinate { x: value, y: 0 },
            "S" => self.coordinate -= Coordinate { x: 0, y: value },
            "W" => self.coordinate -= Coordinate { x: value, y: 0 },

            "F" => {
                self.coordinate += match self.direction {
                    Direction::North => Coordinate { x: 0, y: value },
                    Direction::East => Coordinate { x: value, y: 0 },
                    Direction::South => Coordinate { x: 0, y: -value },
                    Direction::West => Coordinate { x: -value, y: 0 },
                }
            }

            _ => panic!("Unrecognized instruction"),
        };
    }
}

struct Waypoint {
    coordinate: Coordinate,
}

impl Waypoint {
    fn new(x: isize, y: isize) -> Self {
        Self {
            coordinate: Coordinate { x, y },
        }
    }
}

struct WaypointShip {
    coordinate: Coordinate,
    waypoint: Waypoint,
}

impl WaypointShip {
    fn new() -> Self {
        Self {
            coordinate: Coordinate::default(),
            waypoint: Waypoint::new(10, 1),
        }
    }

    fn get_manhattan_distance(&self) -> usize {
        (self.coordinate.x.abs() + self.coordinate.y.abs())
            .try_into()
            .unwrap()
    }

    fn advance(&mut self, action: &str, value: isize) {
        match action {
            "L" => match value {
                0 => {}
                90 => {
                    self.waypoint.coordinate = Coordinate {
                        x: -self.waypoint.coordinate.y,
                        y: self.waypoint.coordinate.x,
                    }
                }
                180 => {
                    self.waypoint.coordinate = Coordinate {
                        x: -self.waypoint.coordinate.x,
                        y: -self.waypoint.coordinate.y,
                    }
                }
                270 => {
                    self.waypoint.coordinate = Coordinate {
                        x: self.waypoint.coordinate.y,
                        y: -self.waypoint.coordinate.x,
                    }
                }
                _ => panic!("Unsupported degree"),
            },
            "R" => match value {
                0 => {}
                90 => {
                    self.waypoint.coordinate = Coordinate {
                        x: self.waypoint.coordinate.y,
                        y: -self.waypoint.coordinate.x,
                    }
                }
                180 => {
                    self.waypoint.coordinate = Coordinate {
                        x: -self.waypoint.coordinate.x,
                        y: -self.waypoint.coordinate.y,
                    }
                }
                270 => {
                    self.waypoint.coordinate = Coordinate {
                        x: -self.waypoint.coordinate.y,
                        y: self.waypoint.coordinate.x,
                    }
                }
                _ => panic!("Unsupported degree"),
            },

            "N" => self.waypoint.coordinate += Coordinate { x: 0, y: value },
            "E" => self.waypoint.coordinate += Coordinate { x: value, y: 0 },
            "S" => self.waypoint.coordinate -= Coordinate { x: 0, y: value },
            "W" => self.waypoint.coordinate -= Coordinate { x: value, y: 0 },

            "F" => {
                self.coordinate += Coordinate {
                    x: self.waypoint.coordinate.x * value,
                    y: self.waypoint.coordinate.y * value,
                }
            }

            _ => panic!("Unrecognized instruction"),
        };
    }
}

pub struct Day12 {}

impl AdventOfCode for Day12 {
    fn part_one(&self, input: &str) -> String {
        let mut ship = Ship::new();

        input.lines().for_each(|line| {
            let action = &line[0..1];
            let value = line[1..].parse::<isize>().expect("Not a number");

            ship.advance(action, value)
        });

        ship.get_manhattan_distance().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut ship = WaypointShip::new();

        input.lines().for_each(|line| {
            let action = &line[0..1];
            let value = line[1..].parse::<isize>().expect("Not a number");

            ship.advance(action, value)
        });

        ship.get_manhattan_distance().to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_examples_part_one() {
        let input = String::from(
            "F10
N3
F7
R90
F11",
        );
        assert_eq!((Day12 {}).part_one(&input), "25");
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/12.txt").expect("Could not read input file");
        assert_eq!((Day12 {}).part_one(&input), "879");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from(
            "F10
N3
F7
R90
F11",
        );
        assert_eq!((Day12 {}).part_two(&input), "286");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/12.txt").expect("Could not read input file");
        assert_eq!((Day12 {}).part_two(&input), "18107");
    }
}
