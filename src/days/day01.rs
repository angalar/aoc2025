use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

fn read_and_parse(path: &str) -> Vec<i32> {
    let input = read_to_string(path).expect("Failed to read input file");
    input
        .lines()
        .map(|line| {
            let (direction, number) = line.split_at(1);
            let mut value: i32 = number.parse().unwrap();
            if direction == "L" { value *= -1; }
            value
        })
        .collect()
}

struct Safe {
    value: i32
}

impl Safe {
    fn new() -> Self {
        Self { value: 50 }
    }
    fn add_value(&mut self, value: i32) -> (i32, usize) {
        let mut count = if self.value != 0 && self.value + value <= 0 {1} else {0};
        self.value += value;
        count += (self.value / 100).unsigned_abs() as usize;
        self.value = self.value.rem_euclid(100);
        (self.value, count)
    }
}

fn part1(instructions: &[i32]) -> usize {
    let mut safe = Safe::new();
    instructions
        .iter()
        .filter(|&&ins| safe.add_value(ins).0 == 0)
        .count()
}
fn part2(instructions: &[i32]) -> usize {
    let mut safe = Safe::new();
    instructions
        .iter()
        .map(|&ins| safe.add_value(ins).1)
        .sum()
}

pub fn solve() -> SolutionPair {
    let instructions = read_and_parse("inputs/day01.txt");
    let sol1 = part1(&instructions);
    let sol2 = part2(&instructions);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let instructions = read_and_parse("test_inputs/day01.txt"); 
        assert_eq!(part1(&instructions), 3);
    }
    #[test]
    fn test_part2() {
        let instructions = read_and_parse("test_inputs/day01.txt");
        assert_eq!(part2(&instructions), 6);
    }
}

