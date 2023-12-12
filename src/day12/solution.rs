const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq)]
enum Spring {
    Working,
    Broken,
    Unknown
}

pub fn part1() {
    let mut sum = 0;
    for line in INPUT.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let springs = parts[0];
        let broken = parts[2].split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let springseq = springs.chars().map(|x| match x {
            '.' => Spring::Working,
            '#' => Spring::Broken,
            '?' => Spring::Unknown,
            _ => unreachable!()
        }).collect::<Vec<Spring>>();
        let arrangements = get_arrangements(springseq, broken);
        sum += arrangements;
    }
    println!("Part 1: {}", sum);
}

pub fn part2() {}

fn get_arrangements(springseq: Vec<Spring>, broken: Vec<usize>) -> usize {
    let mut arrangements: usize = 0;
    let mut last = 0;
    // TODO: Implement this
    arrangements
}