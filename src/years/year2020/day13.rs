use crate::AdventOfCode;

pub struct Day13 {}

impl Day13 {
    fn egcd(t: i64, id: i64) -> (i64, i64, i64) {
        if t == 0 {
            (id, 0, 1)
        } else {
            let (g, x, y) = Self::egcd(id % t, t);
            (g, y - (id / t) * x, x)
        }
    }

    fn mod_inv(t: i64, id: i64) -> Option<i64> {
        let (g, x, _) = Self::egcd(t, id);
        if g == 1 {
            Some((x % id + id) % id)
        } else {
            None
        }
    }

    fn first_consecutive_departure(residues: &[i64], modulii: &[i64]) -> Option<i64> {
        let prod = modulii.iter().product::<i64>();

        let mut sum = 0;

        for (&residue, &modulus) in residues.iter().zip(modulii) {
            let p = prod / modulus;
            sum += residue * Self::mod_inv(p, modulus)? * p
        }

        Some(sum % prod)
    }
}

impl AdventOfCode for Day13 {
    fn part_one(&self, input: &str) -> String {
        let mut splits = input.splitn(2, '\n');
        let arrival = splits
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("Not a number");

        let busses = splits
            .next()
            .unwrap()
            .split(',')
            .filter(|&bus| bus != "x")
            .map(|bus| bus.parse::<usize>().expect("Not a number"));

        let next_bus_after_arrival = busses
            .map(|bus| (bus, bus - arrival % bus))
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();

        (next_bus_after_arrival.0 * next_bus_after_arrival.1).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let departures: Vec<(usize, i64)> = input
            .splitn(2, '\n')
            .nth(1)
            .unwrap()
            .split(',')
            .enumerate()
            .filter(|&(_, bus)| bus != "x" && bus != "0")
            .map(|(index, bus)| (index, bus.parse::<i64>().expect("Not a number")))
            .collect();

        let mut modulii = Vec::new();
        let mut residues = Vec::new();
        for (i, time) in departures {
            modulii.push(time);
            residues.push(time - (i as i64 % time));
        }

        Self::first_consecutive_departure(residues.as_slice(), &modulii.as_slice())
            .expect("modulii not pairwise coprime")
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
            "939
7,13,x,x,59,x,31,19",
        );
        assert_eq!((Day13 {}).part_one(&input), "295");
    }

    #[test]
    fn test_input_part_one() {
        let input = read_to_string("data/2020/13.txt").expect("Could not read input file");
        assert_eq!((Day13 {}).part_one(&input), "2215");
    }

    #[test]
    fn test_examples_part_two() {
        let input = String::from(
            "939
7,13,x,x,59,x,31,19",
        );
        assert_eq!((Day13 {}).part_two(&input), "1068781");
    }

    #[test]
    fn test_input_part_two() {
        let input = read_to_string("data/2020/13.txt").expect("Could not read input file");
        assert_eq!((Day13 {}).part_two(&input), "1058443396696792");
    }
}
