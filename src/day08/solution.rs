use regex::Regex;

#[derive(Debug, Clone)] // Debug is needed for the println! macro and Clone for the .clone() method
struct Node {
    name: String,
    left: String,
    right: String,
}

const INPUT: &str = include_str!("input.txt");

pub fn part1() {
    let (instructions, network, _) = parse_input();

    let steps = get_steps_to_node(network, "AAA".to_string(), instructions.clone(), false);

    println!("Part 1: {}", steps);
}

pub fn part2() {
    let (instructions, network, start_nodes) = parse_input();

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

fn parse_input() -> (Vec<String>, Vec<Node>, Vec<String>) {
    let mut instructions: Vec<String> = Vec::new();
    let mut network: Vec<Node> = Vec::new();
    let mut start_nodes: Vec<String> = Vec::new();

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
            let name = caps.get(1).unwrap().as_str().to_string();
            let left = caps.get(2).unwrap().as_str().to_string();
            let right = caps.get(3).unwrap().as_str().to_string();
            if &name.ends_with("A") == &true {
                start_nodes.push(name.clone());
            }
            network.push(Node { name, left, right });
        }
    }

    return (instructions, network, start_nodes);
}

fn get_steps_to_node(
    network: Vec<Node>,
    start: String,
    instructions: Vec<String>,
    part2: bool,
) -> u64 {
    let mut steps = 0;
    let mut current_node: String = start;

    loop {
        for (_, nav) in instructions.iter().enumerate() {
            steps += 1;
            let node = network
                .iter()
                .find(|&n| n.name == current_node)
                .unwrap()
                .clone();
            if nav == "R" {
                current_node = node.right;
            } else {
                current_node = node.left;
            }
            if current_node == "ZZZ" && !part2 {
                return steps;
            }
            if current_node.ends_with("Z") && part2 {
                return steps;
            }
        }
    }
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