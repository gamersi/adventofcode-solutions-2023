use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

pub fn part1() {
    let mut sum: i64 = 0;
    let mut wi = 1; // while iterator
    for line in INPUT.lines() {
        let nums = line
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let mut diffs: HashMap<i64, Vec<i64>> = HashMap::new();
        diffs.insert(0, nums.clone());
        while !is_zeroes(&diffs[&(wi - 1)]) {
            let mut diff: Vec<i64> = Vec::new();
            let prev_iter = diffs[&(wi as i64 - 1)].clone();
            for i in 0..prev_iter.len() - 1 {
                diff.push(prev_iter[i + 1] - prev_iter[i]);
            }
            diffs.insert(wi, diff.clone());

            wi += 1;
        }
        wi = 1;
        // extrapolation
        let mut last_num = diffs.get(&(diffs.len() as i64 - 1)).unwrap()
            [diffs.get(&(diffs.len() as i64 - 1)).unwrap().len() - 1];
        let mut nums: Vec<i64> = Vec::new();
        nums.push(last_num);
        for i in (0..diffs.len() as i64 - 1).rev() {
            last_num += diffs.get(&i).unwrap()[diffs.get(&i).unwrap().len() - 1];
            nums.push(last_num);
        }
        sum += nums[nums.len() - 1];
    }
    println!("Part 1: {}", sum);
}

pub fn part2() {
    let mut sum: i64 = 0;
    let mut wi = 1; // while iterator
    for line in INPUT.lines() {
        let nums = line
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let mut diffs: HashMap<i64, Vec<i64>> = HashMap::new();
        diffs.insert(0, nums.clone());
        while !is_zeroes(&diffs[&(wi - 1)]) {
            let mut diff: Vec<i64> = Vec::new();
            let prev_iter = diffs[&(wi as i64 - 1)].clone();
            for i in 0..prev_iter.len() - 1 {
                diff.push(prev_iter[i + 1] - prev_iter[i]);
            }
            diffs.insert(wi, diff.clone());

            wi += 1;
        }
        wi = 1;
        // extrapolation back in time (left)
        let mut last_num = diffs.get(&(diffs.len() as i64 - 1)).unwrap()[0];
        let mut nums: Vec<i64> = Vec::new();
        nums.push(last_num);
        for i in (0..diffs.len() as i64 - 1).rev() {
            last_num = diffs.get(&i).unwrap()[0] - last_num;
            nums.push(last_num);
        }
        sum += nums[nums.len() - 1];
    }
    println!("Part 2: {}", sum);
}

fn is_zeroes(v: &Vec<i64>) -> bool {
    for i in v {
        if *i != 0 {
            return false;
        }
    }
    true
}
