use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");

pub fn part1() {
    let mut sum = 0;
    let map = INPUT.lines().map(|l| l.chars().collect()).collect();
    let expanded_map = expand_map(map);

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (y, row) in expanded_map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'#' {
                galaxies.push((x, y));
            }
        }
    }

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let dist = find_path(&expanded_map, galaxies[j], galaxies[i]);
            sum += dist;
        }
    }

    println!("Part 1: {}", sum)
}

pub fn part2() {
    let map = INPUT.lines().map(|l| l.chars().collect()).collect();
    let (empty_rows, empty_cols) = find_empty_rows_cols(&map);

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'#' {
                galaxies.push((x, y));
            }
        }
    }

    let sum = find_path_2(&galaxies, &empty_rows, &empty_cols, 1000000);

    println!("Part 2: {}", sum)
}

fn expand_map(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map: Vec<Vec<char>> = map.clone();
    let (empty_rows, empty_cols) = find_empty_rows_cols(&map);

    for &i in empty_rows.iter().rev() {
        new_map.insert(i, vec!['.'; map[0].len()]);
    }

    for &i in empty_cols.iter().rev() {
        for row in new_map.iter_mut() {
            row.insert(i, '.');
        }
    }

    new_map
}

fn find_empty_rows_cols(map: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows = vec![];
    let mut empty_cols = vec![];

    for i in 0..map.len() {
        if map[i].iter().all(|c| c != &'#') {
            empty_rows.push(i);
        }
    }

    for i in 0..map[0].len() {
        if map.iter().all(|row| row[i] != '#') {
            empty_cols.push(i);
        }
    }

    (empty_rows, empty_cols)
}

fn find_path(map: &Vec<Vec<char>>, start: (usize, usize), target: (usize, usize)) -> u32 {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    let mut queue: VecDeque<((usize, usize), u32)> = VecDeque::new();
    queue.push_back((start, 0));
    visited[start.1][start.0] = true;

    while let Some((node, steps)) = queue.pop_front() {
        if node.0 == target.0 && node.1 == target.1 {
            return steps;
        }

        for (i, j) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_x = node.0 as i32 + i;
            let new_y = node.1 as i32 + j;
            if new_x >= 0 && new_y >= 0 && new_x < map[0].len() as i32 && new_y < map.len() as i32 {
                let x = new_x as usize;
                let y = new_y as usize;
                if !visited[y][x] {
                    visited[y][x] = true;
                    queue.push_back(((x, y), steps + 1));
                }
            }
        }
    }
    0
}

fn find_path_2(
    galaxies: &Vec<(usize, usize)>,
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
    expansion: usize,
) -> usize {
    let mut sum = 0;
    let num_galaxies = galaxies.len();

    for i in 0..num_galaxies {
        for j in i + 1..num_galaxies {
            let galaxy1 = galaxies[i];
            let galaxy2 = galaxies[j];
            let mut dist = (galaxy1.1 as isize - galaxy2.1 as isize).abs() as usize
                + (galaxy1.0 as isize - galaxy2.0 as isize).abs() as usize;
            let empty_row_dist = empty_rows
                .iter()
                .filter(|&&r| is_in_between(r, galaxy1.1, galaxy2.1))
                .count();
            let empty_col_dist = empty_cols
                .iter()
                .filter(|&&c| is_in_between(c, galaxy1.0, galaxy2.0))
                .count();
            dist += (expansion - 1) * empty_row_dist + (expansion - 1) * empty_col_dist;
            sum += dist;
        }
    }
    sum
}

fn is_in_between(value: usize, bound1: usize, bound2: usize) -> bool {
    let min_bound = bound1.min(bound2);
    let max_bound = bound1.max(bound2);
    value > min_bound && value < max_bound
}
