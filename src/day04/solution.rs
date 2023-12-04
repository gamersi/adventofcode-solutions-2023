use regex::Regex;

const INPUT: &str = include_str!("input.txt");

pub fn part1() {
    let mut sum: u32 = 0;
    let cardre = Regex::new(r"Card (\d+): ").unwrap();
    let multiws = Regex::new(r"[ \t]+").unwrap();
    for line in INPUT.lines() {
        let (winning_numbers, my_numbers) = parse_line(line, &cardre, &multiws);
        let mut current_points: u16 = 0;
        for number in winning_numbers {
            if my_numbers.contains(&number) {
                if current_points == 0 {
                    current_points = 1;
                } else {
                    current_points *= 2;
                }
            }
        }
        sum += current_points as u32;
    }
    println!("Part 1: {}", sum);
}

pub fn part2() {
    let mut num_scratchcards: u32 = 0;
    let cardre = Regex::new(r"Card (\d+): ").unwrap();
    let multiws = Regex::new(r"[ \t]+").unwrap();
    let mut card_number: u32 = 1;
    let mut copies: Vec<u32> = vec![0; INPUT.lines().count()];

    for line in INPUT.lines() {
        let (winning_numbers, my_numbers) = parse_line(line, &cardre, &multiws);
        let mut shared_numbers: Vec<u32> = Vec::new();
        for number in winning_numbers {
            if my_numbers.contains(&number) {
                shared_numbers.push(number);
            }
        }

        copies[card_number as usize - 1] += 1;
        num_scratchcards += 1;

        for _ in 0..copies[card_number as usize - 1] {
            for j in card_number..card_number + shared_numbers.len() as u32 {
                copies[j as usize] += 1;
                num_scratchcards += 1;
            }
        }

        card_number += 1;
    }
    // println!("{:?}", copies);
    println!("Part 2: {}", num_scratchcards);
}

fn parse_line(line: &str, cardre: &Regex, multiws: &Regex) -> (Vec<u32>, Vec<u32>) {
    let parsed_line = cardre
        .replace_all(&multiws.replace_all(&line, " "), "")
        .to_string()
        .trim()
        .to_string(); // replace all whitespace with single space, then remove the Card X: part and trim
    let parts: Vec<&str> = parsed_line.split(" | ").collect();
    let winning_numbers: Vec<u32> = parts[0]
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let my_numbers: Vec<u32> = parts[1]
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    (winning_numbers, my_numbers)
}
