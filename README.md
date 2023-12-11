# Advent of Code 2023 Solutions by gamersi

This repository contains my solutions for the [Advent of Code 2023](https://adventofcode.com/2023) challenges. Advent of Code is an annual set of Christmas-themed programming puzzles that cover a variety of topics and difficulty levels.
I am using Rust to solve the puzzles and I am trying to solve them with the standard library and with as few dependencies as possible(only `regex` for now)

## Structure
Each day's solutions are organized into separate directories named `day01`, `day02`, etc. Each directory contains:

- The puzzle input in a text file, named `input.txt`.
- 1-2 Example input files, named `example.txt`, `example2.txt`, etc.
- The solution file, named `solution.rs`.
- The module file, named `mod.rs`.

I wrote a little script to generate the boilerplate for each day. It can be found at src/generate_day.sh.
It asks for the day number and then generates the directory and files for that day. Additionally, it adds the day to the lib.rs and main.rs file.

## Running the Solutions
The solutions are written in Rust and can be run using the standard Rust toolchain. To run a solution, navigate to the solution's directory and use the following command:

```bash
cargo run # for debug builds
or
cargo run --release # for optimized builds
```

## Progress
Here's my progress for the Advent of Code 2023:

- [x] Day 1: [Trebuchet?!](https://adventofcode.com/2023/day/1)
- [x] Day 2: [Cube Conundrum](https://adventofcode.com/2023/day/2)
- [x] Day 3: [Gear Ratios](https://adventofcode.com/2023/day/3)
- [x] Day 4: [Scratchcards](https://adventofcode.com/2023/day/4)
- [x] Day 5: [If You Give A Seed A Fertilizer](https://adventofcode.com/2023/day/5)
- [x] Day 6: [Wait For It](https://adventofcode.com/2023/day/6)
- [x] Day 7: [Camel Cards](https://adventofcode.com/2023/day/7)
- [x] Day 8: [Haunted Wasteland](https://adventofcode.com/2023/day/8)
- [x] Day 9: [Miarge Maintenance](https://adventofcode.com/2023/day/9)
- [x] Day 10: [Pipe Maze](https://adventofcode.com/2023/day/10)

## Acknowledgments
Thanks to the creators of Advent of Code for organizing this fun and educational event each year.

## License
This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.