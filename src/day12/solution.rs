use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq)]
enum Spring {
    Working,
    Broken,
    Unknown,
}

pub fn part1() {
    let mut sum = 0;
    for line in INPUT.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let springs = parts[0];
        let broken = parts[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let springseq = springs
            .chars()
            .map(|x| match x {
                '.' => Spring::Working,
                '#' => Spring::Broken,
                '?' => Spring::Unknown,
                _ => unreachable!(),
            })
            .collect::<Vec<Spring>>();
        let arrangements = get_arrangements(springseq, broken);
        sum += arrangements;
    }
    println!("Part 1: {}", sum);
}

pub fn part2() {
    let mut sum = 0;
    for line in INPUT.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let springs = parts[0];
        let broken = parts[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut expanded = String::new();
        for _ in 0..4 {
            expanded += springs;
            expanded += "?";
        }
        expanded += springs;
        let expanded_broken = broken.repeat(5);

        let springseq = expanded
            .chars()
            .map(|x| match x {
                '.' => Spring::Working,
                '#' => Spring::Broken,
                '?' => Spring::Unknown,
                _ => unreachable!(),
            })
            .collect::<Vec<Spring>>();
        let arrangements = get_arrangements(springseq, expanded_broken);
        sum += arrangements;
    }
    println!("Part 2: {}", sum);
}

fn get_arrangements(springseq: Vec<Spring>, broken: Vec<usize>) -> usize {
    let mut cache: HashMap<(usize, usize, usize), usize> = HashMap::new();
    dfs(&mut cache, &springseq, &broken, 0, 0, 0)
}

fn dfs(
    cache: &mut HashMap<(usize, usize, usize), usize>, // ( from, group, size ) arrangements
    springseq: &Vec<Spring>,
    broken: &Vec<usize>,
    from: usize,
    group: usize,
    size: usize,
) -> usize {
    if from >= springseq.len() {
        if group >= broken.len() {
            return 1;
        }
        if group == broken.len() - 1 && broken[group] == size {
            return 1;
        }
        return 0;
    }

    match springseq[from] {
        Spring::Working => {
            // skip
            if size == 0 {
                return dfs(cache, springseq, broken, from + 1, group, size);
            }

            if group >= broken.len() || size != broken[group] {
                return 0;
            }

            dfs(cache, springseq, broken, from + 1, group + 1, 0)
        }
        Spring::Broken => {
            if group >= broken.len() || size + 1 > broken[group] {
                return 0;
            }

            dfs(cache, springseq, broken, from + 1, group, size + 1)
        }
        Spring::Unknown => {
            if let Some(v) = cache.get(&(from, group, size)).copied() {
                return v;
            }

            let mut sum = 0;
            if size == 0 {
                sum += dfs(cache, springseq, broken, from + 1, group, size);
            }

            if group < broken.len() && size < broken[group] {
                sum += dfs(cache, springseq, broken, from + 1, group, size + 1);
            }

            if group < broken.len() && size == broken[group] {
                sum += dfs(cache, springseq, broken, from + 1, group + 1, 0);
            }

            cache.insert((from, group, size), sum);
            sum
        }
    }
}
