use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::str::FromStr;
use std::collections::HashSet;

struct Interval {
    start: u64,
    end: u64,
}

impl FromStr for Interval {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();
        Ok(Interval { start, end })
    }
}

impl Interval {
    fn spread(&self) -> Vec<Interval> {
        let mut intervals = Vec::new();
        let start_digits = num_digits(self.start);
        let end_digits = num_digits(self.end);

        for digits in start_digits..=end_digits {
            let lower_bound = if digits == start_digits {
                self.start
            } else {
                10u64.pow(digits - 1)
            };
            let upper_bound = if digits == end_digits {
                self.end
            } else {
                10u64.pow(digits) - 1
            };
            intervals.push(Interval {
                start: lower_bound,
                end: upper_bound,
            });
        }
        intervals
    }
    fn get_incorrect_numbers(&self, max_divisor: u8, results: &mut HashSet<u64>) {
        let start_len = num_digits(self.start);
        if start_len != num_digits(self.end) {
            for interval in self.spread() {
                interval.get_incorrect_numbers(max_divisor, results);
            }
        } else {
            for div in get_divisors(num_digits(self.start) as u8) {
                if div > max_divisor {
                    break;
                }
                let part_len = start_len / div as u32;
                let factor = (10u64.pow(start_len) - 1) / (10u64.pow(part_len) - 1);
                let mut min_x = self.start / factor;
                if !self.start.is_multiple_of(factor) {
                    min_x += 1;
                }
                let max_x = self.end / factor;
                for x in min_x..=max_x {
                    results.insert(x * factor);
                }
            }
        }
    }
}

fn read_and_parse(path: &str) -> Vec<Interval> {
    let input = read_to_string(path).expect("Failed to read input file");
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_divisors(n: u8) -> Vec<u8> {
    (2..=n).filter(|&d| n.is_multiple_of(d)).collect()
}

fn num_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn part1(intervals: &[Interval]) -> u64 {
    let mut incorrect_id = HashSet::new();
    for interval in intervals {
        interval.get_incorrect_numbers(2, &mut incorrect_id);
    }
    incorrect_id.iter().sum()
}
fn part2(intervals: &[Interval]) -> u64 {
    let mut incorrect_id = HashSet::new();
    for interval in intervals {
        interval.get_incorrect_numbers(10, &mut incorrect_id);
    }
    incorrect_id.iter().sum()
}

pub fn solve() -> SolutionPair {
    let intervals = read_and_parse("inputs/day02.txt");
    let sol1 = part1(&intervals);
    let sol2 = part2(&intervals);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let intervals = read_and_parse("test_inputs/day02.txt");
        assert_eq!(part1(&intervals), 1227775554);
    }
    #[test]
    fn test_part2() {
        let intervals = read_and_parse("test_inputs/day02.txt");
        assert_eq!(part2(&intervals), 4174379265);
    }
}
