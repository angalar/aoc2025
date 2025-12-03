use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::str::FromStr;

struct BatteryBank {
    batteries: Vec<u8>,
}

impl FromStr for BatteryBank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        Ok(BatteryBank { batteries })
    }
}

impl BatteryBank {
    fn get_biggest_number(&self, num_len: u8) -> u64 {
        let mut stack = Vec::new();
        let mut remove = self.batteries.len() as u8 - num_len;

        for &b in &self.batteries {
            while let Some(&top) = stack.last() {
                if remove > 0 && b > top {
                    stack.pop();
                    remove -= 1;
                } else {
                    break;
                }
            }
            stack.push(b);
        }
        stack
            .iter()
            .take(num_len as usize)
            .fold(0u64, |acc, &d| acc * 10 + d as u64)
    }
}

fn read_and_parse(path: &str) -> Vec<BatteryBank> {
    let input = read_to_string(path).expect("Failed to read input file");
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part1(banks: &[BatteryBank]) -> u64 {
    banks
        .iter()
        .map(|bank| bank.get_biggest_number(2))
        .sum()
}
fn part2(banks: &[BatteryBank]) -> u64 {
    banks
        .iter()
        .map(|bank| bank.get_biggest_number(12))
        .sum()
}

pub fn solve() -> SolutionPair {
    let banks = read_and_parse("inputs/day03.txt");
    let sol1 = part1(&banks);
    let sol2 = part2(&banks);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let banks = read_and_parse("test_inputs/day03.txt");
        assert_eq!(part1(&banks), 357);
    }
    #[test]
    fn test_part2() {
        let banks = read_and_parse("test_inputs/day03.txt");
        assert_eq!(part2(&banks), 3121910778619);
    }
}
