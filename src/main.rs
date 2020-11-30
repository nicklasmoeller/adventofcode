use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use structopt::StructOpt;

mod template;

#[derive(StructOpt)]
struct Opt {
    /// The year to run a solution for. Defaults to the latest year
    #[structopt(short, long, default_value = "2020")]
    year: usize,

    /// The day to run a solution for
    #[structopt(short, long)]
    day: usize,

    /// The part to run as a numeric value, both if not present
    #[structopt(short, long)]
    part: Option<usize>,

    /// A relative path to an input file, stdin if not present
    #[structopt(short, long, parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let part = match opt.part {
        Some(1) => Some((true, false)),
        Some(2) => Some((false, true)),
        None => Some((true, true)),
        _ => panic!("Invalid part selection"),
    };

    let mut input = String::new();

    if let Some(path) = &opt.file {
        let file = File::open(path).expect("Could not open file.");

        BufReader::new(file).read_to_string(&mut input).unwrap();
    } else {
        let stdin = io::stdin();
        let mut guard = stdin.lock();

        guard.read_to_string(&mut input).unwrap();
    };

    let normalized_day = format!("{}", opt.day).parse::<usize>().unwrap();

    let day: Box<dyn AdventOfCode> = match (opt.year, normalized_day) {
        // A template implementation that may be copied to other days. This is the gist of defining
        // a new day.
        (42, 42) => Box::new(template::Template {}),

        _ => panic!("Not implemented"),
    };

    match part {
        Some((true, false)) => println!("{}", day.part_one(&input)),
        Some((false, true)) => println!("{}", day.part_two(&input)),
        Some((true, true)) => {
            println!("{}", day.part_one(&input));
            println!("{}", day.part_two(&input));
        }
        _ => unreachable!(),
    }
}

pub trait AdventOfCode {
    fn part_one(&self, input: &String) -> String;
    fn part_two(&self, input: &String) -> String;
}