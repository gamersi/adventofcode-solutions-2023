use std::collections::VecDeque;

const INPUT: &str = include_str!("example.txt");

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
    print_galaxy(map);

}

pub fn part2() {}

fn expand_map(map: Vec<Vec<char>>, expansion: u8) -> Vec<Vec<char>> {
    let mut new_map = Vec::new();
    for row in &map {
        if row.iter().any(|&c| c == '#') {
            new_map.push(row.clone());
        } else {
            new_map.push(row.clone());
            for i in 0..expansion {
                new_map.push(vec!['.'; row.len()]);
            }
        }
    }
    let mut offset = 0;
    for col in 0..map[0].len() {
        if map.iter().all(|row| row[col] == '.') {
            offset += 1;
            for row in &mut new_map {
                for i in 0..expansion {
                    row.insert(col+offset, '.');
                }
            }
        }
    }
    new_map
}

fn find_path(map: Vec<Vec<char>>, start: Spot, target: Spot) {
    let mut queue: VecDeque<Spot> = VecDeque::new();
    queue.push_back(start);
    
}

fn print_galaxy(map: Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}