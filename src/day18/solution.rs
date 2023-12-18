const INPUT: &str = include_str!("input.txt");
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Option<Direction> {
        match s {
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct CuM {
    // Cubic Meter
    x: usize,
    y: usize,
    is_part_of_lagoon: bool,
}

impl CuM {
    fn new(x: usize, y: usize, is_part_of_lagoon: bool) -> CuM {
        CuM {
            x,
            y,
            is_part_of_lagoon,
        }
    }

    fn parse_map(map: &mut Vec<(Direction, usize)>) -> (usize, usize, Vec<Vec<CuM>>) {
        let max_down = max_down(map);
        let max_up = max_up(map);
        let max_left = max_left(map);
        let max_right = max_right(map);

        let width = max_right + max_left;
        let height = max_down + max_up;
        let mut lagoon_map: Vec<Vec<CuM>> =
            vec![vec![CuM::new(0, 0, false); width as usize]; height as usize];

        let mut x = max_left as usize;
        let mut y = max_up as usize;

        // println!("x: {} y: {}", x, y);
        // println!("width: {} height:{}", width, height);
        // println!("max_down: {} max_up: {} max_left: {} max_right: {}", max_down, max_up, max_left, max_right);
        // // println!("{:?}", lagoon_map);

        for (direction, distance) in map {
            match direction {
                Direction::Up => {
                    for _ in 0..*distance {
                        lagoon_map[y][x].is_part_of_lagoon = true;
                        y -= 1;
                    }
                }
                Direction::Down => {
                    for _ in 0..*distance {
                        lagoon_map[y][x].is_part_of_lagoon = true;
                        y += 1;
                    }
                }
                Direction::Left => {
                    for _ in 0..*distance {
                        lagoon_map[y][x].is_part_of_lagoon = true;
                        x -= 1;
                    }
                }
                Direction::Right => {
                    for _ in 0..*distance {
                        lagoon_map[y][x].is_part_of_lagoon = true;
                        x += 1;
                    }
                }
            }
        }

        lagoon_map.iter_mut().enumerate().for_each(|(x, row)| {
            row.iter_mut().enumerate().for_each(|(y, cum)| {
                cum.x = x;
                cum.y = y;
            })
        });

        (x, y, lagoon_map)
    }

    fn calculate_area(lagoon_map: &Vec<Vec<CuM>>) -> usize {
        return lagoon_map.iter().fold(0, |acc, row| {
            acc + row.iter().fold(
                0,
                |acc, cum| {
                    if cum.is_part_of_lagoon {
                        acc + 1
                    } else {
                        acc
                    }
                },
            )
        });
    }
}

pub fn part1() {
    let mut map: Vec<(Direction, usize)> = INPUT
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let direction: Direction = Direction::from_str(parts.next().unwrap()).unwrap();
            let distance: usize = parts.next().unwrap().parse().unwrap();
            // the color is ignored :)
            (direction, distance)
        })
        .collect();
    let (x, y, mut lagoon_map) = CuM::parse_map(&mut map);
    flood_fill(x + 1, y + 1, &mut lagoon_map);
    println!("Part 1: {} m2", CuM::calculate_area(&lagoon_map));
}

pub fn part2() {
    let lines = INPUT.lines().collect::<Vec<&str>>();

    let mut x: i128 = 0;
    let mut y: i128 = 0;
    let mut perimiter: i128 = 0;
    let mut inner_points: i128 = 0;

    lines.iter().for_each(|line| {
        let mut parts = line.split_whitespace();
        let color = parts
            .nth(2)
            .unwrap()
            .to_string()
            .replace("(#", "")
            .replace(")", "");

        let direction: Direction = match color.chars().last().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("invalid direction"),
        };

        let steps: i128 = i128::from_str_radix(&color[..5], 16).unwrap();
        perimiter += steps;

        let (dx, dy) = match direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_x = x + dx * steps;
        let new_y = y + dy * steps;

        // gaussian area formula
        inner_points += x * new_y - new_x * y;
        x = new_x;
        y = new_y;
    });

    let area = (perimiter.abs() + inner_points.abs()) / 2 + 1;
    println!("Part 2: {} m2", area);
}

fn flood_fill(pos_x: usize, pos_y: usize, lagoon_map: &mut Vec<Vec<CuM>>) {
    if (pos_x >= lagoon_map.len() || pos_y >= lagoon_map[0].len())
        || lagoon_map[pos_x][pos_y].is_part_of_lagoon
    {
        return;
    }

    lagoon_map[pos_x][pos_y].is_part_of_lagoon = true;

    flood_fill(pos_x + 1, pos_y, lagoon_map);
    flood_fill(pos_x - 1, pos_y, lagoon_map);
    flood_fill(pos_x, pos_y + 1, lagoon_map);
    flood_fill(pos_x, pos_y - 1, lagoon_map);

    return;
}

fn max_down(map: &Vec<(Direction, usize)>) -> i32 {
    let mut max = 0;
    let mut current: i32 = 1;
    for (direction, distance) in map {
        match direction {
            Direction::Down => {
                current += *distance as i32;
            }
            Direction::Up => {
                current -= *distance as i32;
            }
            _ => {}
        }
        if current > max {
            max = current;
        }
    }
    max
}

fn max_up(map: &Vec<(Direction, usize)>) -> i32 {
    let mut max = 0;
    let mut current: i32 = 0;
    for (direction, distance) in map {
        match direction {
            Direction::Up => {
                current += *distance as i32;
            }
            Direction::Down => {
                current -= *distance as i32;
            }
            _ => {}
        }
        if current > max {
            max = current;
        }
    }
    if max < 0 {
        max = 0;
    }
    max
}

fn max_left(map: &Vec<(Direction, usize)>) -> i32 {
    let mut max = 0;
    let mut current: i32 = 0;
    for (direction, distance) in map {
        match direction {
            Direction::Left => {
                current += *distance as i32;
            }
            Direction::Right => {
                current -= *distance as i32;
            }
            _ => {}
        }
        if current > max {
            max = current;
        }
    }
    if max < 0 {
        max = 0;
    }
    max
}

fn max_right(map: &Vec<(Direction, usize)>) -> i32 {
    let mut max = 0;
    let mut current: i32 = 1;
    for (direction, distance) in map {
        match direction {
            Direction::Right => {
                current += *distance as i32;
            }
            Direction::Left => {
                current -= *distance as i32;
            }
            _ => {}
        }
        if current > max {
            max = current;
        }
    }
    max
}

// fn print_map(map: &Vec<Vec<CuM>>, start_x: usize, start_y: usize) {
//     map.iter().for_each(|row| {
//         row.iter().for_each(|cum| {
//             if cum.x == start_x && cum.y == start_y {
//                 print!("X");
//             } else if cum.is_part_of_lagoon {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         });
//         println!();
//     });
// }
