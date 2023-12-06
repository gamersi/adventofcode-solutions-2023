use regex::Regex;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

pub fn part1() {
    let mut total_wins: u64 = 1;

    let lines: Vec<&str> = INPUT.split("\n").collect();

    let times: Vec<u64> = extract_numbers(lines[0]);
    let distances: Vec<u64> = extract_numbers(lines[1]);

    for i in 0..distances.len() {
        let mut valid_attempts = 0;
        for j in 0..=times[i] {
            let distance = j * (times[i] - j as u64);
            if distance > distances[i] {
                valid_attempts += 1;
            }
        }
        total_wins *= valid_attempts;
    }

    println!("Part 1: {}", total_wins);
}

pub fn part2() {
    let mut valid_attempts: u64 = 0;

    let lines: Vec<&str> = INPUT.split("\n").collect();

    // regex for everything except numbers
    let re = Regex::new(r"[^0-9]+").unwrap();

    let times: Vec<u64> = extract_numbers(re.replace_all(lines[0], "").as_ref());
    let distances: Vec<u64> = extract_numbers(re.replace_all(lines[1], "").as_ref());

    for i in 0..distances.len() {
        for j in 0..=times[i] {
            let distance = j * (times[i] - j as u64);
            if distance > distances[i] {
                valid_attempts += 1;
            }
        }
    }

    println!("Part 2: {}", valid_attempts);
}

fn extract_numbers(input: &str) -> Vec<u64> {
    let re = Regex::new(r"\d+").unwrap();
    re.captures_iter(input)
        .map(|cap| u64::from_str(&cap[0]).unwrap())
        .collect()
}
