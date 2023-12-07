use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn count_most_frequent(strings: Vec<String>, part2: bool) -> (String, u32) {
    let mut counts = HashMap::new();

    for string in &strings {
        if !part2 {
            *counts.entry(string).or_insert(0) += 1;
        } else {
            if string != "J" {
                *counts.entry(string).or_insert(0) += 1;
            }
        }
    }

    if counts.len() == 0 && part2 {
        return ("J".to_string(),5);
    }

    let (value, count) = counts.iter().max_by_key(|&(_, count)| *count).unwrap();

    if part2 {
        return (value.to_string(), count + strings.iter().filter(|&x| x == "J").count() as u32);
    }

    (value.to_string(), *count)
}

fn determine_strength(card: &str, part2: bool) -> u32 {
    match card {
        "A" => 14,
        "K" => 13,
        "Q" => 12,
        "J" => {
            if !part2 {
                11
            } else {
                1
            }
        },
        "T" => 10,
        "9" => 9,
        "8" => 8,
        "7" => 7,
        "6" => 6,
        "5" => 5,
        "4" => 4,
        "3" => 3,
        "2" => 2,
        _ => 0
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Hand {
    bet: u32,
    rank: u32,
    type_: u32,
    winnings: u32,
    part2: bool,
    cards: Vec<String>,
}

impl Hand {
    fn new(bet: u32, rank: u32, part2: bool, cards: Vec<String>) -> Hand {
        Hand {
            bet,
            rank,
            type_: 0,
            winnings: 0,
            part2,
            cards
        }
    }

    fn parse_str(input: &str, part2: bool) -> Hand {
        let mut cards: Vec<String> = Vec::new();
        let bet: u32;

        let split: Vec<&str> = input.split(" ").collect();
        let cards_str: Vec<&str> = split[0].split("").collect();
        for card in cards_str {
            if card != "" {
                cards.push(card.to_string());
            }
        }

        bet = split[1].parse::<u32>().unwrap();

        Hand::new(bet, 0, part2, cards)
    }

    fn determine_type(&self, part2: bool) -> u32 {
        let most_frequent = count_most_frequent(self.cards.clone(), part2);
        if most_frequent.1 == 5 { // Five of a kind
            return 6;
        } else if most_frequent.1 == 4 { // Four of a kind
            return 5;
        } else if most_frequent.1 == 3 && count_most_frequent(self.cards.iter().filter(|&x| x != &most_frequent.0 && ((part2 == true && x != "J") || part2==false)).map(|x| x.to_string()).collect(), part2).1 == 2 { // Full house
            return 4;
        } else if most_frequent.1 == 3 { // Three of a kind
            return 3;
        } else if most_frequent.1 == 2 && count_most_frequent(self.cards.iter().filter(|&x| x != &most_frequent.0 && ((part2 == true && x != "J") || part2==false)).map(|x| x.to_string()).collect(), part2).1 == 2 { // Two pair
            return 2;
        } else if most_frequent.1 == 2 { // One pair
            return 1;
        } else {
            return 0;
        }
    }

    fn compare_strength(&self, other: &Hand, part2: bool) -> u32 { // 0 = self wins, 1 = other wins
        for i in 0..self.cards.len() {
            if self.cards[i] == other.cards[i] {
                continue;
            } else {
                if determine_strength(&self.cards[i], part2) > determine_strength(&other.cards[i], part2) {
                    return 0;
                } else {
                    return 1;
                }
            }
        }
        3
    }
}

pub fn part1() {
    let mut hands: Vec<Hand> = Vec::new();

    for line in INPUT.lines() {
        let mut hand = Hand::parse_str(line, false);
        hand.type_ = hand.determine_type(false);
        hands.push(hand);
    }

    hands.sort_by_key(|hand| hand.type_);
    
    // compare hands with same type
    let mut i = 0;
    while i < hands.len() - 1 {
        let mut j = i + 1;
        while j < hands.len() && hands[i].type_ == hands[j].type_ {
            if hands[i].compare_strength(&hands[j], false) == 0 { 
                hands.swap(i, j);
                j += 1;
            } else {
                j += 1;
            }
        }
        i += 1;
    }

    for i in 0..hands.len() {
        hands[i].rank = (i + 1) as u32;
        hands[i].winnings = hands[i].bet * hands[i].rank;
    }

    let mut sum: u32 = 0;
    hands.iter().for_each(|hand| {sum += hand.winnings;});

    println!("Part 1: {}", sum);
}

pub fn part2() {
    let mut hands: Vec<Hand> = Vec::new();

    for line in INPUT.lines() {
        let mut hand = Hand::parse_str(line, true);
        hand.type_ = hand.determine_type(true);
        hands.push(hand);
    }
    hands.sort_by_key(|hand| hand.type_);
    
    // compare hands with same type
    let mut i = 0;
    while i < hands.len() - 1 {
        let mut j = i + 1;
        while j < hands.len() && hands[i].type_ == hands[j].type_ {
            if hands[i].compare_strength(&hands[j], true) == 0 { 
                hands.swap(i, j);
                j += 1;
            } else {
                j += 1;
            }
        }
        i += 1;
    }

    for i in 0..hands.len() {
        hands[i].rank = (i + 1) as u32;
        hands[i].winnings = hands[i].bet * hands[i].rank;
    }

    let mut sum: u32 = 0;
    hands.iter().for_each(|hand| {sum += hand.winnings;});

    println!("Part 2: {}", sum);
}