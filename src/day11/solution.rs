use std::collections::VecDeque;

const INPUT: &str = include_str!("example.txt");

#[derive(Debug, Clone)]
struct Spot {
    x: usize,
    y: usize,
    visited: bool,
    galaxy: bool
}

pub fn part1() {
    let mut sum = 0;
    let mut map: Vec<Vec<Spot>> = Vec::new();
    let mut num_galaxies = 0;
    for (i, line) in INPUT.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let mut galaxy: bool = false;
            if c == '#' {
                galaxy = true;
                num_galaxies += 1;
            } 
            row.push(Spot {
                x: j,
                y: i,
                visited: false,
                galaxy
            });
        }
        map.push(row);
    }
    map = expand_map(map, 1);
    print_galaxy(map.clone());
    let start = Spot { x: 1, y: 6, visited: false, galaxy: true };
    let target = Spot { x: 5, y: 11, visited: false, galaxy: true };
    println!("{}", find_path(map, start, target))
}

pub fn part2() {}

fn expand_map(map: Vec<Vec<Spot>>, expansion: u8) -> Vec<Vec<Spot>> {
    let mut new_map: Vec<Vec<Spot>> = Vec::new();
    for (i, row) in map.iter().enumerate() {
        if row.iter().any(|c| c.galaxy) {
            new_map.push(row.clone());
        } else {
            new_map.push(row.clone());
            for _ in 0..expansion {
                let mut new_row: Vec<Spot> = Vec::new();
                for j in 0..row.len() {
                    new_row.push(Spot { x: j, y: i, visited: false, galaxy: false })
                }
            }
        }
    }
    let mut offset = 0;
    for col in 0..map[0].len() {
        if map.iter().all(|row| !row[col].galaxy) {
            offset += 1;
            for (i, row) in new_map.iter().enumerate() {
                for _ in 0..expansion {
                    new_map[i].insert(col+offset, Spot { x: col+offset, y: i, visited: false, galaxy: false });
                }
            }
        }
    }
    new_map
}

fn find_path(map: Vec<Vec<Spot>>, start: Spot, target: Spot) -> u32 {
    let mut queue: VecDeque<Spot> = VecDeque::new();
    queue.push_back(start);
    
    while let Some(node) = queue.pop_front() {
        let mut record_x = usize::MAX;
        let mut record_y = usize::MAX;
        let mut steps: u32 = 0;

        for i in node.y-1..=node.y+1 {
            for j in node.x-1..=node.x+1 {
                if i > target.y && i - target.y < record_y {
                    record_y = i - target.y;
                    if i == target.y && j == target.x {
                        return steps;
                    }
                    steps += 1;
                } else if i < target.y && target.y - i < record_y {
                    record_y = target.y - i;
                    if i == target.y && j == target.x {
                        return steps;
                    }
                    steps += 1;
                }else if j > target.x && j - target.x < record_x {
                    record_x = i - target.x;
                    if i == target.y && j == target.x {
                        return steps;
                    }
                    steps += 1;
                } else if j < target.x && target.x - j < record_x {
                    record_x = target.x - j;
                    if i == target.y && j == target.x {
                        return steps;
                    }
                    steps += 1;
                } else {
                    panic!("That should not happen.")
                }
                queue.push_back(Spot {
                    x: target.x - record_x,
                    y: target.y - record_y,
                    galaxy: map[target.x - record_x][target.y - record_y].galaxy,
                    visited: map[target.x - record_x][target.y - record_y].visited,
                })
            }
        }
    };
    0
}

fn print_galaxy(map: Vec<Vec<Spot>>) {
    for row in map {
        for s in row {
            if s.galaxy {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}