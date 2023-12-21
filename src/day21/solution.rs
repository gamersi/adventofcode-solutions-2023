const INPUT: &str = include_str!("input.txt");

const STEPS: usize = 64;

pub fn part1() {
    let grid: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    let sum = expand_grid(&grid, STEPS);
    println!("Part 1: {}", sum);
}

pub fn part2() {
    let grid: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    let sum = expand_grid_with_infinite(&grid);
    println!("Part 2: {}", sum);
}

fn expand_grid(grid: &Vec<Vec<char>>, steps: usize) -> usize {
    let mut grid = grid.clone();
    for _ in 0..steps {
        let mut new_grid = grid.clone();
        for (y, row) in grid.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                let mut count = 0;
                if grid[y][x] == 'S' || grid[y][x] == 'O' {
                    for (x2, y2) in vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                        if x2 >= grid[0].len() || y2 >= grid.len() {
                            continue;
                        }
                        if grid[y2][x2] == '.' || grid[y2][x2] == 'S' {
                            count += 1;
                            new_grid[y2][x2] = 'O';
                        }
                        if count != 0 {
                            new_grid[y][x] = '.';
                        }
                    }
                } else {
                    continue;
                }
            }
        }
        grid = new_grid;
    }
    grid.iter()
        .map(|row| row.iter().filter(|&&c| c == 'O').count())
        .sum()
}

fn expand_grid_with_infinite(_grid: &Vec<Vec<char>>) -> i64 {
    unimplemented!("Part 2")
}
