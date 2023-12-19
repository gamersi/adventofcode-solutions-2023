use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum Variable {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Comparison {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    else_case: String,
}

#[derive(Debug)]
struct Rule {
    var: Variable,
    comparison: Comparison,
    value: usize,
    switch: String,
}

impl Workflow {
    fn parse_workflows(input: &str) -> HashMap<String, Workflow> {
        let mut workflows: HashMap<String, Workflow> = HashMap::new();

        for line in input.lines() {
            let name = line.split("{").nth(0).unwrap().to_string();
            let mut workflow = Workflow {
                name,
                rules: Vec::new(),
                else_case: line
                    .split("}")
                    .nth(0)
                    .unwrap()
                    .to_string()
                    .split(",")
                    .last()
                    .unwrap()
                    .to_string(),
            };
            let line = line
                .trim()
                .split("{")
                .nth(1)
                .unwrap()
                .to_string()
                .split("}")
                .nth(0)
                .unwrap()
                .to_string();

            let mut components = line.split(",").collect::<Vec<&str>>();
            let _ = components.pop();
            for component in components {
                let var: Variable = match component.chars().nth(0).unwrap() {
                    'x' => Variable::X,
                    'm' => Variable::M,
                    'a' => Variable::A,
                    's' => Variable::S,
                    _ => unreachable!("Invalid variable"),
                };
                let comparison: Comparison = match component.chars().nth(1).unwrap() {
                    '<' => Comparison::LessThan,
                    '>' => Comparison::GreaterThan,
                    _ => unreachable!("Invalid comparison"),
                };
                let value: usize = component
                    .split(":")
                    .nth(0)
                    .unwrap()
                    .to_string()
                    .split_at(2)
                    .1
                    .parse()
                    .unwrap();
                let switch: String = component.split(":").nth(1).unwrap().to_string();
                workflow.rules.push(Rule {
                    var,
                    comparison,
                    value,
                    switch,
                });
            }
            workflows.insert(workflow.name.clone(), workflow);
        }
        workflows
    }
}

pub fn part1() {
    let parts: Vec<&str> = INPUT.split("\n\n").collect();
    let workflows: HashMap<String, Workflow> = Workflow::parse_workflows(parts[0]);
    let mut sum: usize = 0;
    let components: Vec<(usize, usize, usize, usize)> = parts[1]
        .split("\n")
        .map(|line| {
            let components = line
                .replace("{", "")
                .replace("}", "")
                .split(",")
                .map(|component| {
                    component
                        .split("=")
                        .nth(1)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                })
                .collect::<Vec<usize>>();
            (components[0], components[1], components[2], components[3])
        })
        .collect();
    for (x, m, a, s) in components {
        let name = follow(&workflows, x, m, a, s);

        if name == "A" {
            sum += x + m + a + s;
        }
    }
    println!("Part 1: {}", sum);
}

pub fn part2() {
    let parts: Vec<&str> = INPUT.split("\n\n").collect();
    let workflows: HashMap<String, Workflow> = Workflow::parse_workflows(parts[0]);
    // lets use BFS because "bruteforce" would take way too long. If we bruteforce we dont need to fix the snow as it will melt by the heat of my PC lol
    let mut queue: Vec<(
        String,
        (i32, i32),
        (i32, i32),
        (i32, i32),
        (i32, i32),
    )> = Vec::new();
    let mut visited: HashSet<(
        String,
        (i32, i32),
        (i32, i32),
        (i32, i32),
        (i32, i32),
    )> = HashSet::new();
    let mut count: u128 = 0; // i used u128 just to be sure we dont overflow
    queue.push(("in".to_string(), (1, 4000), (1, 4000), (1, 4000), (1, 4000)));
    while let Some((name, x_range, m_range, a_range, s_range)) = queue.pop() {
        if !visited.insert((name.clone(), x_range, m_range, a_range, s_range)) {
            continue;
        }
        if name == "A" {
            count += (x_range.1 - x_range.0 + 1) as u128
                * (m_range.1 - m_range.0 + 1) as u128
                * (a_range.1 - a_range.0 + 1) as u128
                * (s_range.1 - s_range.0 + 1) as u128;
            continue;
        }
        
        if name == "R" {
            continue;
        }

        if x_range.0 > x_range.1
            || m_range.0 > m_range.1
            || a_range.0 > a_range.1
            || s_range.0 > s_range.1
        {
            continue;
        }

        let mut x_range = x_range.clone();
        let mut m_range = m_range.clone();
        let mut a_range = a_range.clone();
        let mut s_range = s_range.clone();

        let workflow = workflows.get(&name).unwrap();
        for rule in &workflow.rules {
            if x_range.0 > x_range.1
                || m_range.0 > m_range.1
                || a_range.0 > a_range.1
                || s_range.0 > s_range.1
            {
                continue;
            }
            match rule.var {
                Variable::X => match rule.comparison {
                    Comparison::LessThan => {
                        if rule.value as i32 >= x_range.0 {
                            let mut x_range_clone = x_range.clone();
                            x_range_clone.1 = min(rule.value as i32 - 1, x_range.1);
                            queue.push((
                                rule.switch.clone(),
                                x_range_clone,
                                m_range.clone(),
                                a_range.clone(),
                                s_range.clone(),
                            ));
                        }
                        x_range.0 = max(rule.value as i32, x_range.0);
                    }
                    Comparison::GreaterThan => {
                        if rule.value as i32 <= x_range.1 {
                            let mut x_range_clone = x_range.clone();
                            x_range_clone.0 = max(rule.value as i32 + 1, x_range.0);
                            queue.push((
                                rule.switch.clone(),
                                x_range_clone,
                                m_range.clone(),
                                a_range.clone(),
                                s_range.clone(),
                            ));
                        }
                        x_range.1 = min(rule.value as i32, x_range.1);
                    }
                },
                Variable::M => match rule.comparison {
                    Comparison::LessThan => {
                        if rule.value as i32 >= m_range.0 {
                            let mut m_range_clone = m_range.clone();
                            m_range_clone.1 = min(rule.value as i32 - 1, m_range.1);
                            queue.push((
                                rule.switch.clone(),
                                x_range.clone(),
                                m_range_clone,
                                a_range.clone(),
                                s_range.clone(),
                            ));
                        }
                        m_range.0 = max(rule.value as i32, m_range.0);
                    }
                    Comparison::GreaterThan => {
                        if rule.value as i32 <= m_range.1 {
                            let mut m_range_clone = m_range.clone();
                            m_range_clone.0 = max(rule.value as i32 + 1, m_range.0);
                            queue.push((
                                rule.switch.clone(),
                                x_range.clone(),
                                m_range_clone,
                                a_range.clone(),
                                s_range.clone(),
                            ));
                        }
                        m_range.1 = min(rule.value as i32, m_range.1);
                    }
                },
                Variable::A => match rule.comparison {
                    Comparison::LessThan => {
                        if rule.value as i32 >= a_range.0 {
                            let mut a_range_clone = a_range.clone();
                            a_range_clone.1 = min(rule.value as i32 - 1, a_range.1);
                            queue.push((
                                rule.switch.clone(),
                                x_range.clone(),
                                m_range.clone(),
                                a_range_clone,
                                s_range.clone(),
                            ));
                        }
                        a_range.0 = max(rule.value as i32, a_range.0);
                    }
                    Comparison::GreaterThan => {
                        if rule.value as i32 <= a_range.1 {
                            let mut a_range_clone = a_range.clone();
                            a_range_clone.0 = max(rule.value as i32 + 1, a_range.0);
                            queue.push((
                                rule.switch.clone(),
                                x_range.clone(),
                                m_range.clone(),
                                a_range_clone,
                                s_range.clone(),
                            ));
                        }
                        a_range.1 = min(rule.value as i32, a_range.1);
                    }
                },
                Variable::S => match rule.comparison {
                    Comparison::LessThan => {
                        if rule.value as i32 >= s_range.0 {
                            let mut s_range_clone = s_range.clone();
                            s_range_clone.1 = min(rule.value as i32 - 1, s_range.1);
                            queue.push((
                                rule.switch.clone(),
                                x_range.clone(),
                                m_range.clone(),
                                a_range.clone(),
                                s_range_clone,
                            ));
                        }
                        s_range.0 = max(rule.value as i32, s_range.0);
                    }
                    Comparison::GreaterThan => {
                        if rule.value as i32 <= s_range.1 {
                            let mut s_range_clone = s_range.clone();
                            s_range_clone.0 = max(rule.value as i32 + 1, s_range.0);
                            queue.push((
                                rule.switch.clone(),
                                x_range.clone(),
                                m_range.clone(),
                                a_range.clone(),
                                s_range_clone,
                            ));
                        }
                        s_range.1 = min(rule.value as i32, s_range.1);
                    }
                },
            }
        }
        queue.push((
            workflow.else_case.clone(),
            x_range,
            m_range,
            a_range,
            s_range,
        ));
    }

    println!("Part 2: {}", count);
}

fn follow(workflows: &HashMap<String, Workflow>, x: usize, m: usize, a: usize, s: usize) -> String {
    let mut name = "in".to_string();
    while name != "A" && name != "R" {
        let workflow = workflows.get(&name).unwrap();
        let mut found = false;
        for rule in &workflow.rules {
            match rule.var {
                Variable::X => match rule.comparison {
                    Comparison::LessThan => {
                        if x < rule.value {
                            name = rule.switch.clone();
                            found = true;
                            break;
                        }
                    }
                    Comparison::GreaterThan => {
                        if x > rule.value {
                            name = rule.switch.clone();
                            found = true;
                            break;
                        }
                    }
                },
                Variable::M => match rule.comparison {
                    Comparison::LessThan => {
                        if m < rule.value {
                            name = rule.switch.clone();
                            found = true;
                            break;
                        }
                    }
                    Comparison::GreaterThan => {
                        if m > rule.value {
                            name = rule.switch.clone();
                            found = true;
                            break;
                        }
                    }
                },
                Variable::A => match rule.comparison {
                    Comparison::LessThan => {
                        if a < rule.value {
                            name = rule.switch.clone();
                            found = true;
                            break;
                        }
                    }
                    Comparison::GreaterThan => {
                        if a > rule.value {
                            name = rule.switch.clone();
                            found = true;
                            break;
                        }
                    }
                },
                Variable::S => match rule.comparison {
                    Comparison::LessThan => {
                        if s < rule.value {
                            name = rule.switch.clone();
                            found = true;
                            break;
                        }
                    }
                    Comparison::GreaterThan => {
                        if s > rule.value {
                            name = rule.switch.clone();
                            found = true;
                            break;
                        }
                    }
                },
            }
        }
        if !found {
            name = workflow.else_case.clone();
        }
    }
    name
}
