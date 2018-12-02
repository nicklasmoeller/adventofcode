use std::collections::HashSet;
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

pub fn part1(input: &str) -> isize {
    // TODO: maybe rewrite to fold?
    input
        .split_whitespace()
        .map(|l| {
            l.parse().unwrap_or(0)
        })
        .sum()
}

pub fn part2(input: &str) -> isize {
    let mut freq: isize = 0;
    let mut past_freqs = HashSet::new();
    past_freqs.insert(freq);
    let changes: Vec<isize> = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    loop {
        for change in changes.iter() {
            freq += change;
            if past_freqs.contains(&freq) {
                return freq
            }
            past_freqs.insert(freq);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("+1\n+1\n+1"), 3);
        assert_eq!(part1("+1\n+1\n-2"), 0);
        assert_eq!(part1("-1\n-2\n-3"), -6);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2("+1\n-1"), 0);
        assert_eq!(part2("+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(part2("-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(part2("+7\n+7\n-2\n-7\n-4"), 14);
    }
}
