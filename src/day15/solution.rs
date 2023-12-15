use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

pub fn part1() {
    let sum: usize = INPUT
        .split(",")
        .map(|hash_seq| hash(hash_seq))
        .collect::<Vec<usize>>()
        .iter()
        .sum();
    println!("Part 1: {}", sum);
}

pub fn part2() {
    let mut hash_map: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();
    let _: Vec<()> = INPUT
        .split(",")
        .map(|hash_seq| hashmap(hash_seq, &mut hash_map))
        .collect();
    let sum: usize = focusing_power(&hash_map);
    println!("Part 2: {}", sum);
}

fn hash(input: &str) -> usize {
    let mut current_value: usize = 0;
    input.chars().enumerate().for_each(|(_, c)| {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    });
    current_value
}

fn hashmap<'a>(input: &'a str, hash_map: &mut HashMap<usize, Vec<(&'a str, usize)>>) {
    let mut label: &str = "";
    let mut operation: usize = 0; // 0 = -, 1..9 = =1..=9
    if input.contains("-") {
        let mut split = input.split("-");
        label = split.next().unwrap();
    }
    if input.contains("=") {
        let mut split = input.split("=");
        label = split.next().unwrap();
        operation = split.next().unwrap().parse::<usize>().unwrap();
    }
    let key = hash(label);
    let mut current_value = hash_map.entry(key).or_insert(Vec::new());
    if operation == 0 {
        current_value.retain(|&x| x.0 != label);
    } else if operation > 0 &&  operation < 10{
        if !current_value.iter().any(|(x, _)| x == &label) {
            current_value.push((label, operation));
        } else {
            // overwrite it keeping the position in the vector
            let index = current_value.iter().position(|(x, _)| x == &label).unwrap();
            current_value[index] = (label, operation);
        }
    } else {
        println!("{} {} {}", input, label, operation);
        panic!("Invalid operation");
    }
}

fn focusing_power(hash_map: &HashMap<usize, Vec<(&str, usize)>>) -> usize {
    let mut sum: usize = 0;
    for (key, value) in hash_map {
        if !value.is_empty() {
            println!("{:?} {}", value, value.len());
            value.iter().enumerate().for_each(|(i, (_, operation))| {
                sum += (key + 1) * (i + 1) * operation;
            });
        }
    }
    sum
}