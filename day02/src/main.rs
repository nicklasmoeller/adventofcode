use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("input.txt").expect("input.txt not found");

    let mut input = String::new();
    f.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

pub fn part1(_input: &str) -> isize {
    0
}

pub fn part2(_input: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(true, true);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(true, true);
    }
}
