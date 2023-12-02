use regex::Regex;

const INPUT: &str = include_str!("input.txt");

const MAX_AMOUNT_RED: u8 = 12;
const MAX_AMOUNT_GREEN: u8 = 13;
const MAX_AMOUNT_BLUE: u8 = 14;

pub fn part1() {
    let mut sum_ids = 0;
    let mut current_id = 1;
    let linere = Regex::new(r"(\d+)(red|green|blue)").unwrap();
    for line in INPUT.lines() {
        let parsed_line = parse_line(line);
        let mut valid = true;
        parsed_line.split(';').for_each(|subset| {
            let mut amount_red = 0;
            let mut amount_green = 0;
            let mut amount_blue = 0;
            linere.captures_iter(subset).for_each(|cap| {
                let amount = cap[1].parse::<u8>().unwrap();
                match &cap[2] {
                    "red" => amount_red = amount,
                    "green" => amount_green = amount,
                    "blue" => amount_blue = amount,
                    _ => panic!("Invalid color"),
                }
            });
            if amount_red > MAX_AMOUNT_RED
                || amount_green > MAX_AMOUNT_GREEN
                || amount_blue > MAX_AMOUNT_BLUE
            {
                valid = false;
            }
        });
        if valid {
            sum_ids += current_id;
        }
        current_id += 1;
    }
    println!("Solution part1: {}", sum_ids);
}

pub fn part2() {
    let mut sum_powers = 0;
    let linere = Regex::new(r"(\d+)(red|green|blue)").unwrap();
    for line in INPUT.lines() {
        let parsed_line = parse_line(line);
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        parsed_line.split(';').for_each(|subset| {
            linere.captures_iter(subset).for_each(|cap| {
                let amount = cap[1].parse::<usize>().unwrap();
                match &cap[2] {
                    "red" => max_red = max_red.max(amount),
                    "green" => max_green = max_green.max(amount),
                    "blue" => max_blue = max_blue.max(amount),
                    _ => panic!("Invalid color"),
                }
            });
        });
        sum_powers += max_red * max_green * max_blue;
    }
    println!("Solution part2: {}", sum_powers);
}

fn parse_line(line: &str) -> String {
    let gamere = Regex::new(r"Game(\d+):").unwrap();
    gamere.replace_all(&line.replace(" ", ""), "").to_string()
}
