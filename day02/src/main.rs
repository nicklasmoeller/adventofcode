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

pub fn part1(input: &str) -> usize {
    let mut twos: usize = 0;
    let mut threes: usize = 0;

    for line in input.lines() {
        let mut has_two: bool = false;
        let mut has_three: bool = false;

        for c in line.chars() {
            let count = line.matches(c).count();
            if count == 2 {
                has_two = true;
            } else if count == 3 {
                has_three = true;
            }
        }

        if has_two {
            twos += 1;
        }
        if has_three {
            threes += 1;
        }
    }

    twos * threes
}

pub fn part2(input: &str) -> String {
    // NOTE: what's more readable? hashmap or nested for loop?
    // going with nested for loop, since hashmap seems too easy
    let lines: Vec<_> = input.lines().collect();

    for (index, first_line) in lines.iter().enumerate() {
        for next_line in lines.iter().skip(index + 1) {
            if is_close(first_line, next_line) {
                return get_common_chars(first_line, next_line);
            }
        }
    }

    fn is_close(first: &str, second: &str) -> bool {
        if first.is_empty() {
            false
        } else if first.chars().nth(0) == second.chars().nth(0) {
            is_close(&first[1..], &second[1..])
        } else {
            first[1..] == second[1..]
        }
    }

    fn get_common_chars(first: &str, second: &str) -> String {
        if first.chars().nth(0) == second.chars().nth(0) {
            let mut result = String::from(&first[0..1]);
            result.push_str(&get_common_chars(&first[1..], &second[1..]));
            result
        } else {
            first[1..].to_string()
        }
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let input = "abcdef\nbababc\nabbcde\naabcdd\nabcccd\nabcdee\nababab";
        assert_eq!(part1(input), 12);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"), "fgij");
    }
}
