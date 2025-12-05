use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    fn overlaps(&self, other: &Interval) -> bool {
        !(self.end < other.start || other.end < self.start)
    }
    fn merge(&self, other: &Interval) -> Interval {
        Interval {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
    fn len(&self) -> usize {
        (self.end - self.start + 1) as usize
    }
}

fn read_and_parse(path: &str) -> (Vec<Interval>, Vec<u64>) {
    let input = read_to_string(path).expect("Failed to read input file");
    let (first, second) = input.split_once("\n\n").unwrap();
    let intervals: Vec<Interval> = first
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let values: Vec<u64> = second
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    (intervals, values)
}

fn merge_intervals(intervals: &mut [Interval]) -> Vec<Interval> {
    intervals.sort();
    let mut merged: Vec<Interval> = Vec::new();
    for interval in intervals.iter() {
        if let Some(last) = merged.last_mut() && last.overlaps(interval) {
                *last = last.merge(interval);
                continue;
        }
        merged.push(interval.clone());
    }
    merged
}

fn part1(intervals: &[Interval], values: &[u64]) -> usize {
    values
        .iter()
        .filter(|&&value| {
            intervals.binary_search_by(|interval| {
                if interval.end < value {
                    std::cmp::Ordering::Less
                } else if interval.start > value {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }).is_ok()
        })
        .count()
}
fn part2(intervals: &[Interval]) -> usize {
    intervals
        .iter()
        .map(|interval| interval.len())
        .sum()

}

pub fn solve() -> SolutionPair {
    let (mut intervals, values) = read_and_parse("inputs/day05.txt");
    intervals = merge_intervals(&mut intervals);
    let sol1 = part1(&intervals, &values);
    let sol2 = part2(&intervals);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (intervals, values) = read_and_parse("test_inputs/day05.txt");
        assert_eq!(part1(&intervals, &values), 3);
    }
    #[test]
    fn test_part2() {
        let (mut intervals, _) = read_and_parse("test_inputs/day05.txt");
        intervals = merge_intervals(&mut intervals);
        assert_eq!(part2(&intervals), 14);
    }
}
