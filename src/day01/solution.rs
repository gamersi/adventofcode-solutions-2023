const INPUT: &str = include_str!("input.txt");

pub fn part1() {
    let mut sum = 0;
    for line in INPUT.lines() {
        let nums: Vec<i32> = extract_num(line, false);
        let mut num: i32 = nums[0];
        num *= 10;
        num += nums[nums.len() - 1];
        sum += num;
    }
    println!("{}", sum);
}

pub fn part2() {
    let mut sum = 0;
    for line in INPUT.lines() {
        let nums: Vec<i32> = extract_num(line, true);
        let mut num: i32 = nums[0];
        num *= 10;
        num += nums[nums.len() - 1];
        sum += num;
    }
    println!("{}", sum);
}

fn extract_num(line: &str, p2: bool) -> Vec<i32> {
    let mut new_line = line.to_string();
    if p2 {
        new_line = new_line.replace("one", "one1one");
        new_line = new_line.replace("two", "two2two");
        new_line = new_line.replace("three", "three3three");
        new_line = new_line.replace("four", "four4four");
        new_line = new_line.replace("five", "five5five");
        new_line = new_line.replace("six", "six6six");
        new_line = new_line.replace("seven", "seven7seven");
        new_line = new_line.replace("eight", "eight8eight");
        new_line = new_line.replace("nine", "nine9nine");
    }
    new_line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect()
}
