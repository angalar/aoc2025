use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

fn part1(path: &str) -> usize {
    let input = read_to_string(path).expect("Failed to read input file");
    let parts: Vec<&str> = input.split("\n\n").collect();
    parts
        .last()
        .unwrap()
        .lines()
        .filter_map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let mut splits = left.split('x');
            let x: u32 = splits.next().unwrap().parse().unwrap();
            let y: u32 = splits.next().unwrap().parse().unwrap();
            let gift_count: u32 = right
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .sum();
            if x / 3 * y / 3 >= gift_count {
                Some(())
            } else {
                None
            }
        })
        .count()
}
fn part2(_data: &str) -> u64 {
    0
}

pub fn solve() -> SolutionPair {
    let sol1 = part1("inputs/day12.txt");
    let sol2 = part2("");

    (Solution::from(sol1), Solution::from(sol2))
}
