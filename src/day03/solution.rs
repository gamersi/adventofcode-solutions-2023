const INPUT: &str = include_str!("input.txt");

pub fn part1() {
    let line_len = INPUT.lines().next().unwrap().len();
    let line_count = INPUT.lines().count();
    // create a 2D String array of the correct size
    let mut grid: Vec<Vec<String>> = vec![vec![(&"").to_string(); line_len]; line_count];
    let mut used: Vec<Vec<bool>> = vec![vec![false; line_len]; line_count];
    // populate the grid
    for (i, line) in INPUT.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c.to_string();
        }
    }
    
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != "." && !str_is_digit(grid[i][j].as_str()) {
                // check if the surrounding positions are digits
                for k in i-1..i+2 {
                    for l in j-1..j+2 {
                        if k >= 0 && k < grid.len() && l >= 0 && l < grid[k].len() {
                            if str_is_digit(grid[k][l].as_str()) && !used[k][l] {
                                let mut num: String = grid[k][l].to_string();
                                used[k][l] = true;
                                let mut m = l as i32 - 1;

                                while m >= 0 && str_is_digit(grid[k][m as usize].as_str()) {
                                    // prepend the number when its before it
                                    num = grid[k][m as usize].to_string() + &num;
                                    used[k][m as usize] = true;
                                    m -= 1;
                                }
                                m = l as i32 + 1;
                                while m < grid.len() as i32 && str_is_digit(grid[k][m as usize].as_str()) {
                                    // append the number when its after it
                                    num = num + &grid[k][m as usize].to_string();
                                    used[k][m as usize] = true;
                                    m += 1;
                                }
                                sum += num.parse::<i32>().unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Part 1: {}", sum);
}

pub fn part2() {
    let line_len = INPUT.lines().next().unwrap().len();
    let line_count = INPUT.lines().count();
    // create a 2D String array of the correct size
    let mut grid: Vec<Vec<String>> = vec![vec![(&"").to_string(); line_len]; line_count];
    let mut used: Vec<Vec<bool>> = vec![vec![false; line_len]; line_count];
    // populate the grid
    for (i, line) in INPUT.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c.to_string();
        }
    }
    
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == "*" { // search for gears
                let mut nums: Vec<i16> = Vec::new();
                for k in i-1..i+2 {
                    for l in j-1..j+2 {
                        if k >= 0 && k < grid.len() && l >= 0 && l < grid[k].len() {
                            if str_is_digit(grid[k][l].as_str()) && !used[k][l] {
                                let mut num: String = grid[k][l].to_string();
                                used[k][l] = true;
                                let mut m = l as i32 - 1;

                                while m >= 0 && str_is_digit(grid[k][m as usize].as_str()) {
                                    // prepend the number when its before it
                                    num = grid[k][m as usize].to_string() + &num;
                                    used[k][m as usize] = true;
                                    m -= 1;
                                }
                                m = l as i32 + 1;
                                while m < grid.len() as i32 && str_is_digit(grid[k][m as usize].as_str()) {
                                    // append the number when its after it
                                    num = num + &grid[k][m as usize].to_string();
                                    used[k][m as usize] = true;
                                    m += 1;
                                }
                                nums.push(num.parse::<i16>().unwrap());
                            }
                        }
                    }
                }
                if nums.len() == 2 {
                    sum += nums[0] as i32 * nums[1] as i32;
                }
            }
        }
    }
    println!("Part 2: {}", sum);
}

fn str_is_digit(s: &str) -> bool {
    if s.len() != 1 {
        return false;
    }
    s.chars().next().unwrap().is_digit(10)
}