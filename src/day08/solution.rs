use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone)] // Debug is needed for the println! macro and Clone for the .clone() method
struct Node {
    identifier: String,
    left: String,
    right: String,
    left_index: usize,
    right_index: usize,
}

const INPUT: &str = include_str!("input.txt");

pub fn part1() {
    let (instructions, network, _, start_index) = parse_input();

    let steps = get_steps_to_node(network, start_index, instructions.clone(), false);

    println!("Part 1: {}", steps);
}

pub fn part2() {
    let (instructions, network, start_nodes, _) = parse_input();

    let mut steps: Vec<u64> = Vec::new();

    for start in start_nodes {
        steps.push(get_steps_to_node(
            network.clone(),
            start,
            instructions.clone(),
            true,
        ));
    }

    println!("Part 2: {}", lcm_of_vec(steps));
}

fn parse_input() -> (Vec<String>, Vec<Node>, Vec<usize>, usize) {
    let mut indices: HashMap<&str, usize> = HashMap::new();
    let mut instructions: Vec<String> = Vec::new();
    let mut network: Vec<Node> = Vec::new();
    let mut start_nodes: Vec<usize> = Vec::new();
    let mut start_index: usize = 0;

    let parenthesisre = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();

    for (i, line) in INPUT.lines().enumerate() {
        if i == 0 {
            instructions = line
                .chars()
                .filter(|&c| c.is_alphabetic())
                .map(|c| c.to_string())
                .collect();
        } else if !line.is_empty() {
            let caps = parenthesisre.captures(line).unwrap();
            let identifier = &caps.get(1).unwrap().as_str();
            let left = caps.get(2).unwrap().as_str().to_string();
            let right = caps.get(3).unwrap().as_str().to_string();
            if identifier.ends_with("A") == true {
                start_nodes.push(i - 2);
            }
            indices.insert(identifier, i - 2);
            network.push(Node {
                identifier: identifier.to_string(),
                left,
                right,
                left_index: 0,
                right_index: 0,
            });
            if identifier == &"AAA" {
                start_index = i - 2;
            }
        }
    }

    for node in network.iter_mut() {
        node.left_index = *indices.get(node.left.as_str()).unwrap();
        node.right_index = *indices.get(node.right.as_str()).unwrap();
    }

    return (instructions, network, start_nodes, start_index);
}

fn get_steps_to_node(
    network: Vec<Node>,
    start: usize,
    instructions: Vec<String>,
    part2: bool,
) -> u64 {
    let mut steps = 0;
    let mut current_node: usize = start;
    for (_, nav) in instructions.iter().cycle().enumerate() {
        steps += 1;
        let mut node = &network[current_node];
        if nav == "R" {
            current_node = node.right_index;
        } else {
            current_node = node.left_index;
        }
        node = &network[current_node];
        if node.identifier == "ZZZ" && !part2 {
            break;
        }
        if node.identifier.ends_with("Z") && part2 {
            break;
        }
    }
    steps
}

fn lcm(a: u64, b: u64) -> u64 {
    // Least common multiple
    (a * b) / gcd(a, b)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    // Euclid's algorithm
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn lcm_of_vec(vec: Vec<u64>) -> u64 {
    let mut result = vec[0];
    for i in 1..vec.len() {
        result = lcm(result, vec[i]);
    }
    return result;
}
