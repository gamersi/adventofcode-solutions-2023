use std::{cmp::Reverse, collections::VecDeque};

const INPUT: &str = include_str!("input.txt");

const EMPTY: char = '.';
const CUBE: char = '#';
const ROUND: char = 'O';
const CYCLES: usize = 1_000_000_000; // only relevant for part 2

pub fn part1() {
    let mut dish = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    tilt_north(&mut dish);
    // output(&dish);
    println!("Part1: {}", calculate_load(&dish));
}

pub fn part2() {
    let mut dish = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // the solution of part 1 was sadly not performant enough for part 2 so I had to optimize it :(

    let mut cache: VecDeque<Vec<Vec<char>>> = VecDeque::new();
    let mut cycle_count = 0;

    while cycle_count < CYCLES {
        dish = cycle(dish);
        cycle_count += 1;

        if let Some(index) = cache.iter().position(|d| d == &dish) {
            let repeat_count = cache.len() - index;
            cycle_count = CYCLES - (CYCLES - cycle_count) % repeat_count;
        }

        cache.push_back(dish.clone());
        if cache.len() > 50 {
            cache.pop_front();
        }
    }

    // output(&dish);
    println!("Part2: {}", calculate_load(&dish));
}

fn tilt_north(dish: &mut Vec<Vec<char>>) {
    let mut new_dish = dish.clone();
    for (y, row) in dish.iter_mut().enumerate() {
        for (x, cube) in row.iter().enumerate() {
            if *cube != ROUND {
                continue;
            }

            let mut new_y = y;

            while new_y > 0 {
                new_y -= 1;
                if new_dish[new_y][x] == EMPTY {
                    new_dish[new_y][x] = ROUND;
                    new_dish[new_y + 1][x] = EMPTY;
                } else {
                    break;
                }
            }
        }
    }
    dish.clone_from(&new_dish);
}

fn calculate_load(dish: &[Vec<char>]) -> usize {
    dish.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row.iter().filter(|cube| **cube == ROUND).count() * (dish.len() - i)
    })
}

fn cycle(mut dish: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for _ in 0..2 {
        dish = transpose(&dish);
        dish.iter_mut().for_each(|row| {
            row.split_mut(|cube| *cube == CUBE).for_each(|part| {
                part.sort_unstable_by_key(|x| Reverse(*x));
            })
        });
    }
    for _ in 0..2 {
        dish = transpose(&dish);
        dish.iter_mut().for_each(|row| {
            row.split_mut(|cube| *cube == CUBE).for_each(|part| {
                part.sort_unstable();
            })
        });
    }
    dish
}

fn transpose(dish: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_dish = vec![vec!['.'; dish.len()]; dish[0].len()];
    for (y, row) in dish.iter().enumerate() {
        for (x, cube) in row.iter().enumerate() {
            new_dish[x][y] = *cube;
        }
    }
    new_dish
}

// fn output(dish: &[Vec<char>]) {
//     for row in dish {
//         for cube in row {
//             print!("{}", cube);
//         }
//         println!();
//     }
// }
