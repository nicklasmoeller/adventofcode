use crate::AdventOfCode;

pub struct Day10 {}

impl AdventOfCode for Day10 {
    fn part_one(&self, input: &String) -> String {
        let mut adapters: Vec<usize> = input
            .lines()
            .map(|line| line.parse::<usize>().expect("Not a number"))
            .collect();

        adapters.push(0);
        adapters.sort_unstable();
        adapters.push(adapters.last().unwrap() + 3);

        let differences = adapters.windows(2).fold((0, 0), |(ones, threes), window| {
            match window[1] - window[0] {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => (ones, threes),
            }
        });

        (differences.0 * differences.1).to_string()
    }

    fn part_two(&self, input: &String) -> String {
        let mut adapters: Vec<usize> = input
            .lines()
            .map(|line| line.parse::<usize>().expect("Not a number"))
            .collect();

        adapters.sort_unstable();

        let mut memory = Vec::with_capacity(adapters.len() + 1);

        // Prefill our memory, just to get going. There's at least one viable path
        memory.push((0, 1usize));

        adapters.iter().for_each(|&adapter| {
            let paths = memory
                .iter()
                .rev()
                // There can be 1..=3 steps between each adapter, and we assume that
                // every adapter is unique
                .take(3)
                .take_while(|(possible_adapter_path, _)| {
                    // We only grab paths that have less than 3 steps in between
                    adapter <= possible_adapter_path + 3
                })
                .map(|&(_, sum_of_paths)| sum_of_paths)
                .sum();

            memory.push((adapter, paths));
        });

        memory.last().unwrap().1.to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_examples_part_one() {
        let input = String::from(
            "16
10
15
5
1
11
7
19
6
12
4",
        );
        assert_eq!((Day10 {}).part_one(&input), (5 * 7).to_string());

        let input = String::from(
            "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3",
        );
        assert_eq!((Day10 {}).part_one(&input), (22 * 10).to_string());
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/10.txt").expect("Could not read input file");
        assert_eq!((Day10 {}).part_one(&input), "2592");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from(
            "16
10
15
5
1
11
7
19
6
12
4",
        );
        assert_eq!((Day10 {}).part_two(&input), "8");

        let input = String::from(
            "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3",
        );
        assert_eq!((Day10 {}).part_two(&input), "19208");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/10.txt").expect("Could not read input file");
        assert_eq!((Day10 {}).part_two(&input), "198428693313536");
    }
}
