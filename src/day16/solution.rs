use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

type Point = (usize, usize, char); // (x, y, direction)

pub fn part1() {
    let map: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let start_beam: Point = (0, 0, 'R');
    
    let mut cache = HashSet::new();
    let mut visited = vec![vec![0; map[0].len()]; map.len()];

    walk_map(&map, start_beam, &mut visited, &mut cache);
    let sum: i32 = visited
        .iter()
        .flatten()
        .sum();
    println!("Part 1: {}", sum);
}

pub fn part2() {
    let map: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();

    let configurations = find_configurations(&map);

    let max: i32 = configurations
        .iter()
        .max()
        .unwrap()
        .clone();
    println!("Part 2: {}", max);
}

fn find_configurations(
    map: &Vec<Vec<char>>
) -> Vec<i32> {
    let mut configurations = vec![];

    let width = map[0].len();
    let height = map.len();

    for y in 0..height {
        for x in 0..width {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                let direction;
                if y == 0 {
                    direction = 'D';
                } else if y == height - 1 {
                    direction = 'U';
                } else if x == 0 {
                    direction = 'R';
                } else {
                    direction = 'L';
                }
                let movement = (x, y, direction);

                let mut cache = HashSet::new();
                let mut visited = vec![vec![0; map[0].len()]; map.len()];
                walk_map(map, movement, &mut visited, &mut cache);
                let sum: i32 = visited
                    .iter()
                    .flatten()
                    .sum();
                configurations.push(sum);
            }
        }
    }

    configurations
}

fn walk_map(
    map: &Vec<Vec<char>>,
    movement: Point,
    visited: &mut Vec<Vec<i32>>,
    cache: &mut HashSet<Point>,
) {
    let (x, y, direction) = movement;

    if y >= map.len() || x >= map[0].len() {
        return;
    }

    if cache.contains(&movement) {
        return;
    }

    cache.insert(movement);
    visited[y][x] = 1;
    let tile = map[y][x];

    match tile {
        '|' => {
            walk_map(map, (x, y + 1, 'D'), visited, cache);
            walk_map(map, (x, y - 1, 'U'), visited, cache);
        }
        '-' => {
            walk_map(map, (x + 1, y, 'R'), visited, cache);
            walk_map(map, (x - 1, y, 'L'), visited, cache);
        }
        '\\' => match direction {
            'R' => walk_map(map, (x, y + 1, 'D'), visited, cache),
            'L' => walk_map(map, (x, y - 1, 'U'), visited, cache),
            'U' => walk_map(map, (x - 1, y, 'L'), visited, cache),
            'D' => walk_map(map, (x + 1, y, 'R'), visited, cache),
            _ => unreachable!(),
        },
        '/' => match direction {
            'R' => walk_map(map, (x, y - 1, 'U'), visited, cache),
            'L' => walk_map(map, (x, y + 1, 'D'), visited, cache),
            'U' => walk_map(map, (x + 1, y, 'R'), visited, cache),
            'D' => walk_map(map, (x - 1, y, 'L'), visited, cache),
            _ => unreachable!(),
        },
        '.' => match direction {
            'R' => walk_map(map, (x + 1, y, 'R'), visited, cache),
            'L' => walk_map(map, (x - 1, y, 'L'), visited, cache),
            'U' => walk_map(map, (x, y - 1, 'U'), visited, cache),
            'D' => walk_map(map, (x, y + 1, 'D'), visited, cache),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

