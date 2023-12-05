use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");

pub trait CategoryMapper<T> {
    fn map(&self, value: T) -> T;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct CategoryMap {
    pub destination_range_start: u64,
    pub source_range_start: u64,
    pub range_length: u64,
}

impl CategoryMap {
    pub fn from_str(s: &str) -> CategoryMap {
        let mut iter = s.split(" ").flat_map(str::parse::<u64>);
        CategoryMap {
            destination_range_start: iter.next().unwrap(),
            source_range_start: iter.next().unwrap(),
            range_length: iter.next().unwrap(),
        }
    }

    pub fn is_in_range(&self, value: u64) -> bool { 
        value >= self.source_range_start && value < self.source_range_start + self.range_length
    }
}

impl CategoryMapper<u64> for CategoryMap {
    fn map(&self, value: u64) -> u64 {
        if !self.is_in_range(value) {
            panic!("{:?} does not map {}", self, value);
        }
        self.destination_range_start - self.source_range_start + value
    }
}

impl CategoryMapper<u64> for Vec<CategoryMap> {
    fn map(&self, value: u64) -> u64 {
        match self.iter().find(|category_map| category_map.is_in_range(value)) {
            Some(category_map) => category_map.map(value),
            None => value,
        }
    }
}

impl CategoryMapper<u64> for Vec<Vec<CategoryMap>> {
    fn map(&self, value: u64) -> u64 {
        self.iter().fold(value, |value, category_maps| {
            category_maps.map(value)
        })
    }
}

impl CategoryMapper<Vec<u64>> for Vec<CategoryMap> {
    fn map(&self, value: Vec<u64>) -> Vec<u64> {
        value.into_iter().map(|value| self.map(value)).collect()
    }
}

impl CategoryMapper<Vec<u64>> for Vec<Vec<CategoryMap>> {
    fn map(&self, value: Vec<u64>) -> Vec<u64> {
        value.into_iter().map(|value| self.map(value)).collect()
    }
}

pub fn part1() {
    let mut input = INPUT.lines();
    let seeds = input
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .flat_map(str::parse::<u64>)
        .collect::<Vec<u64>>(); // seeds
    let category_maps = input
        .filter(|s| !s.is_empty())
        .fold(Vec::new(), |mut almanac, line| {
            if line.contains("map") {
                almanac.push(Vec::new());
            } else {
                let last = almanac.len() - 1;
                almanac[last].push(CategoryMap::from_str(&line));
            }
            almanac
        });
    let min = category_maps.map(seeds.clone()).into_iter().min().unwrap();
    println!("Part1: {}", min);
}

// Solution for part 2 will follow soon
pub fn part2() {
}