extern crate regex;

use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() {
    let mut f = File::open("input.txt").expect("input.txt not found");

    let mut input = String::new();
    f.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

pub fn part1(input: &str) -> usize {
    let claim = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let mut fabric = vec![vec![0usize; 1000]; 1000];

    for line in input.lines() {
        let matches = claim.captures(line).unwrap();
        // TODO: fix all this parsing and unwrapping
        for row in matches[3].parse::<usize>().unwrap()..matches[3].parse::<usize>().unwrap()+matches[5].parse::<usize>().unwrap() {
            for column in matches[2].parse::<usize>().unwrap()..matches[2].parse::<usize>().unwrap()+matches[4].parse::<usize>().unwrap() {
                fabric[row][column] += 1;
            }
        }
    }

    fabric.iter()
        .map(|row| row.iter().filter(|&c| c > &1).count())
        .sum()
}

pub fn part2(_input: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 4);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 3);
    }
}
