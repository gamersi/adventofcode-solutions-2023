use aoc23::*;
use std::time::Instant;

fn main() {
    let mut input = String::new();
    println!("Enter the day to execute:");
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim().is_empty() {
        println!("No day entered, exiting");
        return;
    }
    let day: u32 = input.trim().parse().unwrap();
    execute_day(day);
}

fn execute_day(day: u32) {
    let time = Instant::now();
    match day {
        1 => day01(),
        2 => day02(),
        _ => println!("Day {} not implemented yet", day),
    }
    println!("Time: {}ms", time.elapsed().as_millis());
}

fn day01() {
    day01::solution::part1();
    day01::solution::part2();
}

fn day02() {
    day02::solution::part1();
    day02::solution::part2();
}
