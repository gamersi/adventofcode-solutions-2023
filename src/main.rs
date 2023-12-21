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
        3 => day03(),
        4 => day04(),
        5 => day05(),
        6 => day06(),
        7 => day07(),
        8 => day08(),
        9 => day09(),
        10 => day10(),
        11 => day11(),
        12 => day12(),
        13 => day13(),
        14 => day14(),
        15 => day15(),
        16 => day16(),
        17 => day17(),
        18 => day18(),
        19 => day19(),
        20 => day20(),
        21 => day21(),
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

fn day03() {
    day03::solution::part1();
    day03::solution::part2();
}

fn day04() {
    day04::solution::part1();
    day04::solution::part2();
}

fn day05() {
    day05::solution::part1();
    day05::solution::part2();
}

fn day06() {
    day06::solution::part1();
    day06::solution::part2();
}

fn day07() {
    day07::solution::part1();
    day07::solution::part2();
}

fn day08() {
    day08::solution::part1();
    day08::solution::part2();
}

fn day09() {
    day09::solution::part1();
    day09::solution::part2();
}

fn day10() {
    day10::solution::part1();
    day10::solution::part2();
}

fn day11() {
    day11::solution::part1();
    day11::solution::part2();
}

fn day12() {
    day12::solution::part1();
    day12::solution::part2();
}

fn day13() {
    day13::solution::part1();
    day13::solution::part2();
}

fn day14() {
    day14::solution::part1();
    day14::solution::part2();
}

fn day15() {
    day15::solution::part1();
    day15::solution::part2();
}

fn day16() {
    day16::solution::part1();
    day16::solution::part2();
}

fn day17() {
    day17::solution::part1();
    day17::solution::part2();
}

fn day18() {
    day18::solution::part1();
    day18::solution::part2();
}

fn day19() {
    day19::solution::part1();
    day19::solution::part2();
}

fn day20() {
    day20::solution::solve();
}

fn day21() {
    day21::solution::part1();
    day21::solution::part2();
}
