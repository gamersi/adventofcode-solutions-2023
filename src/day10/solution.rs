use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

const INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    Ground,
    Start,
}

impl Pipe {
    fn connectable_bottom(&self) -> bool {
        match self {
            Pipe::Vertical | Pipe::BottomRight | Pipe::BottomLeft => true,
            _ => false,
        }
    }

    fn connectable_top(&self) -> bool {
        match self {
            Pipe::Vertical | Pipe::TopRight | Pipe::TopLeft => true,
            _ => false,
        }
    }

    fn connectable_left(&self) -> bool {
        match self {
            Pipe::Horizontal | Pipe::TopLeft | Pipe::BottomLeft => true,
            _ => false,
        }
    }

    fn connectable_right(&self) -> bool {
        match self {
            Pipe::Horizontal | Pipe::TopRight | Pipe::BottomRight => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    point: Point,
    pipe: Pipe,
    visited: bool,
    distance: usize,
    is_main_loop: bool,
    contained: bool,
}

pub fn part1() {
    let (mut map, start) = build_map(INPUT);

    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(start);

    // BFS to find the node that is the furthest away from the start
    let mut max_distance = 0;
    while let Some(node) = queue.pop_front() {
        let (connection_points, distance) = {
            let node = &map[node.y][node.x];
            (find_connected_nodes(node), node.distance)
        };

        for point in connection_points {
            let visited_distance = distance_to_node(point, &mut map, &mut queue, distance);
            max_distance = max_distance.max(visited_distance);
        }
    }

    println!("Part 1: {}", max_distance);
}

pub fn part2() {
    let (mut map, start) = build_map(INPUT);

    mark_main_loop(&mut map, start);
    let count = raytrace(&mut map);
    println!("Part 2: {}", count);
}

fn build_map(input: &str) -> (Vec<Vec<Node>>, Point) {
    let mut start = Point::default();

    let mut map = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let pipe = match c {
                '|' => Pipe::Vertical,
                '-' => Pipe::Horizontal,
                'L' => Pipe::TopRight,
                'J' => Pipe::TopLeft,
                'F' => Pipe::BottomRight,
                '7' => Pipe::BottomLeft,
                '.' => Pipe::Ground,
                'S' => Pipe::Start,
                _ => unreachable!("No other grid characters"),
            };

            if pipe == Pipe::Start {
                start = Point { x, y };
            }

            row.push(Node {
                point: Point { x, y },
                pipe,
                visited: false,
                distance: 0,
                is_main_loop: false,
                contained: false,
            });
        }
        map.push(row);
    }
    replace_start(&mut map, start);
    (map, start)
}

fn replace_start(map: &mut Vec<Vec<Node>>, start: Point) {
    let top = match start.y.checked_sub(1) {
        Some(y) => map[y][start.x].pipe.connectable_bottom(),
        None => false,
    };
    let bottom = match start.y.checked_add(1) {
        Some(y) => map[y][start.x].pipe.connectable_top(),
        None => false,
    };
    let left = match start.x.checked_sub(1) {
        Some(x) => map[start.y][x].pipe.connectable_right(),
        None => false,
    };
    let right = match start.x.checked_add(1) {
        Some(x) => map[start.y][x].pipe.connectable_left(),
        None => false,
    };

    let pipe = match (top, bottom, left, right) {
        (true, true, false, false) => Pipe::Vertical,
        (false, false, true, true) => Pipe::Horizontal,
        (true, false, false, true) => Pipe::TopRight,
        (true, false, true, false) => Pipe::TopLeft,
        (false, true, false, true) => Pipe::BottomRight,
        (false, true, true, false) => Pipe::BottomLeft,
        _ => unreachable!(
            "Start node must have at least one connection: {:?}{:?}",
            start,
            (top, bottom, left, right)
        ),
    };

    map[start.y][start.x] = Node {
        point: start,
        pipe,
        visited: false,
        distance: 0,
        is_main_loop: false,
        contained: false,
    };
}

fn find_connected_nodes(node: &Node) -> [Point; 2] {
    let point = node.point;

    match node.pipe {
        Pipe::Vertical => [
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Point {
                x: point.x,
                y: point.y + 1,
            },
        ],
        Pipe::Horizontal => [
            Point {
                x: point.x - 1,
                y: point.y,
            },
            Point {
                x: point.x + 1,
                y: point.y,
            },
        ],
        Pipe::TopRight => [
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Point {
                x: point.x + 1,
                y: point.y,
            },
        ],
        Pipe::TopLeft => [
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Point {
                x: point.x - 1,
                y: point.y,
            },
        ],
        Pipe::BottomRight => [
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Point {
                x: point.x + 1,
                y: point.y,
            },
        ],
        Pipe::BottomLeft => [
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Point {
                x: point.x - 1,
                y: point.y,
            },
        ],
        _ => unreachable!("Only pipes with two connections can be used"),
    }
}

fn distance_to_node(
    point: Point,
    map: &mut Vec<Vec<Node>>,
    queue: &mut VecDeque<Point>,
    distance: usize,
) -> u64 {
    let node = &mut map[point.y][point.x];

    if node.visited {
        return node.distance as u64;
    }

    node.distance = distance + 1;
    node.visited = true;
    queue.push_back(point);

    node.distance as u64
}

fn mark_main_loop(map: &mut Vec<Vec<Node>>, start: Point) {
    let mut current = start;

    'outer_loop: loop {
        let node = &mut map[current.y][current.x];
        node.contained = true;

        let connection_points = find_connected_nodes(node);
        for point in connection_points {
            let node = &mut map[point.y][point.x];
            if !node.is_main_loop {
                node.is_main_loop = true;
                current = point;
                continue 'outer_loop;
            }
        }
        break;
    }
}

fn raytrace(map: &mut Vec<Vec<Node>>) -> u64 {
    map.iter_mut()
        .map(|row| {
            let mut state = false;
            let mut count = 0;

            row.iter_mut().for_each(|node| {
                if !node.is_main_loop && state {
                    node.contained = true;
                    count += 1;
                } else if node.is_main_loop
                    && (node.pipe == Pipe::TopLeft
                        || node.pipe == Pipe::TopRight
                        || node.pipe == Pipe::Vertical)
                {
                    state = !state;
                }
            });
            count
        })
        .sum()
}
