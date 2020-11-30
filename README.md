## yet another adventofcode impl in Rust

This is the home for my take on [Advent of Code](https://adventofcode.com) ⭐🎄⭐

Although not necessary for writing solutions for Advent of Code in Rust, I've
written a utility for structuring the solutions, along with a small CLI for
running your solvers. Feel free to fork this repository or use whatever you
like, in your own repositories.

I'm by no means an expert Rust programmer, but have been playing with it for a
couple of years.

### Add a new solution

To add a new implementation for a day, you need to do a couple of things. The
documented procedure here is a bit oppinionated about declaring each day as a
seperate module, split into different modules for each year. You don't need to
do that, but can just implement a day however you like, and ensure that it's
imported into `src/main.rs`.

First, you need to write a new file. The file needs to export a struct that
implements `AdventOfCode`. The trait is to be found in `src/main.rs` and just
requires you to implement two methods, one for each part.

To make it easier, there's a template implementation, which can be copied to
whereever you want it to be placed in the project.

Ensure the folder we're copying into exists:

```bash
mkdir src/years/year2020
```


```bash
cp src/template/template.rs src/years/year2020/day01.rs
```

After that is done, you can rename the struct to `Day01` in the file you just
copied.

You need to also export it in `src/years/year2020/mod.rs`. The following code
would do that for you.

```
mod day01;
pub use day01::Day01;
```

Ensure that you also export the module itself. In `src/years/mod.rs` add
```
pub mod year2020;
```

Not we need to ensure availability for this module in `src/main.rs`. This can
be done by adding the following in the top of the file.

```
mod years;
```

And last, but not least, the CLI needs to call the implementation, given the
desired inputs. In the match statement for the variable `day` in `src/main.rs`,
add a new arm that matches against the following.Make sure to also call

```
(2020, 1) => Box::new(years::year2020::Day01 {}),
```

After that's done, you're able run the CLI, which will in turn run the solver
for the provided day.

### Usage

Given you've followed the instructions in the previous section, you'll now be
able to execute your solutions with the CLI.

You can either pass a path to a file, or pipe stdin into the CLI. If you don't
provide a path, the CLI will assume that it can read from stdin.

You can pass a path to a file, by prepending `--file data/myfile.txt` to the
arguments list.

Otherwise you may pipe your clipboard contents into the CLI by the way your
operating system allows you to do so.

For MacOS that would be `pbpaste | ...`, and for Linux (assuming xclip), that
would be `xclip -o | ...`. There's plenty of other possibilites, so do
whatever you're comfortable with

For solving both days, run
```bash
cargo run -- --year 2020 --day 1
```

For solving part 1 only, run
```
cargo run -- --year 2020 --day 1 --part 1
```

For solving part 2 only, run
```
cargo run -- --year 2020 --day 1 --part 2
```
