const INPUT: &str = include_str!("input.txt"); // example, expected: 405

#[derive(Debug)]
struct Mirror {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl Mirror {
    fn parse(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut rows = vec![0; height];
        let mut columns = vec![0; width];
        let mut bytes = input.bytes();

        let (mut x, mut y) = (0, 0);
        while let Some(b) = bytes.next() {
            match b {
                b'.' => x += 1,
                b'#' => {
                    rows[y] |= 0b1 << x; // set bit at position x
                    columns[x] |= 0b1 << y; // set bit at position y
                    x += 1;
                }
                b'\n' => {
                    x = 0;
                    y += 1;
                }
                _ => unreachable!(),
            }
        }
        Mirror { rows, cols: columns }
    }
}

pub fn part1() {
    let sum: usize = INPUT.split("\n\n").map(|mirror| solve(&Mirror::parse(mirror), false)).sum();
    println!("Part 1: {}", sum);
}

pub fn part2() {
    let sum: usize = INPUT.split("\n\n").map(|mirror| solve(&Mirror::parse(mirror), true)).sum();
    println!("Part 2: {}", sum);
}

fn solve(mirror: &Mirror, has_smudge: bool) -> usize {
    if !has_smudge {
        if let Some(i) = find_mirror(&mirror.rows, false) {
            return i * 100;
        }
        if let Some(i) = find_mirror(&mirror.cols, false) {
            return i;
        }
    } else {
        if let Some(i) = find_mirror(&mirror.rows, true) {
            return i * 100;
        }
        if let Some(i) = find_mirror(&mirror.cols, true) {
            return i;
        }
    }
    0
}

fn find_mirror(m: &Vec<usize>, has_smudge: bool) -> Option<usize> {
    let count = m.len();
    if !has_smudge {
        for i in 1..count {
            if is_mirror(&m[0..i], &m[i..]) {
                return Some(i);
            }
        }
    } else {
        for i in 1..count {
            if is_smudged_mirror(&m[0..i], &m[i..]) {
                return Some(i);
            }
        }
    }
    None
}

fn is_mirror(left: &[usize], right: &[usize]) -> bool {
    let mut left_iter = left.iter().rev();
    let mut right_iter = right.iter();

    while let Some(l) = left_iter.next() {
        if let Some(r) = right_iter.next() {
            if l != r {
                return false;
            }
        } else {
            break;
        }
    }
    true
}

fn is_smudged_mirror(left: &[usize], right: &[usize]) -> bool {
    let mut left_iter = left.iter().rev();
    let mut right_iter = right.iter();

    let mut smudges = 0;

    while let Some(l) = left_iter.next() {
        if let Some(r) = right_iter.next() {
            smudges += (l ^ r).count_ones(); // xor and then count the number of different bits
            if smudges > 1 {
                return false;
            }
        } else {
            break;
        }
    }

    smudges == 1
}
