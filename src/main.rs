use std::time::Instant;
use aoc23::*;

fn main() {
    execute_day(1);
}

fn execute_day(day: u32) {
    let time = Instant::now();
    match day {
        1 => day01(),
        _ => println!("Day {} not implemented yet", day),
    }
    println!("Time: {}ms", time.elapsed().as_millis());
}

fn day01() {
    day01::solution::part1();
    day01::solution::part2();
}