const INPUT: &str = include_str!("example.txt");

pub fn part1() {
    let mut sum = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut num_galaxies = 0;
    for line in INPUT.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
            if c == '#' {
                num_galaxies += 1;
            }
        }
        map.push(row);
    }
    map = expand_map(map);
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

pub fn part2() {}

fn expand_map(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = Vec::new();
    for row in &map {
        if row.iter().any(|&c| c == '#') {
            new_map.push(row.clone());
        } else {
            new_map.push(row.clone());
            new_map.push(vec!['.'; row.len()]);
        }
    }
    for col in 0..map[0].len() {
        if map.iter().all(|row| row[col] == '.') {
            println!("Adding column {}", col);
            for row in &mut new_map {
                row.insert(col, '.');
            }
        }
    }
    new_map
}