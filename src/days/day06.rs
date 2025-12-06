use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

enum Operation {
    Add,
    Multiply,
}
use Operation::*;

impl Operation {
    fn evaluate<I: Iterator<Item = u64>>(&self, iter: I) -> u64 {
        match self {
            Add => iter.sum(),
            Multiply => iter.product(),
        }
    }
}

fn read_and_parse(path: &str) -> Vec<(Vec<String>, Operation)> {
    let input = read_to_string(path).expect("Failed to read input file");
    let table: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let indexes: Vec<usize> = (0..table[0].len())
        .filter(|&i| table.iter().all(|row| row[i] == ' '))
        .collect();
    let mut strings: Vec<Vec<String>> = Vec::new();
    let mut start = 0;
    for &end in &indexes {
        let mut group = Vec::new();
        for row in &table[..table.len() - 1] {
            let s: String = row[start..end].iter().collect();
            group.push(s);
        }
        strings.push(group);
        start = end + 1;
    }
    strings.push({
        let mut group = Vec::new();
        for row in &table[..table.len() - 1] {
            let s: String = row[start..].iter().collect();
            group.push(s);
        }
        group
    });

    let mut operations = Vec::new();
    for &c in &table[table.len() - 1] {
        match c {
            '+' => operations.push(Add),
            '*' => operations.push(Multiply),
            _ => (),
        }
    }

    strings.into_iter().zip(operations).collect()
}

fn get_value(s: &[String], index: usize) -> u64 {
    s.iter()
        .map(|line| line.chars().nth(index).unwrap())
        .fold(0, |acc, c| {
            if c != ' ' {
                acc * 10 + (c as u64 - '0' as u64)
            } else {
                acc
            }
        })
}

fn part1(data: &[(Vec<String>, Operation)]) -> u64 {
    data.iter()
        .map(|(strings, operation)| {
            let iter = strings.iter().map(|s| s.trim().parse().unwrap());
            operation.evaluate(iter)
        })
        .sum()
}
fn part2(data: &[(Vec<String>, Operation)]) -> u64 {
    data.iter()
        .map(|(strings, operation)| {
            let iter = (0..strings[0].len()).map(|i| get_value(strings, i));
            operation.evaluate(iter)
        })
        .sum()
}

pub fn solve() -> SolutionPair {
    let data = read_and_parse("inputs/day06.txt");
    let sol1 = part1(&data);
    let sol2 = part2(&data);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_and_parse("test_inputs/day06.txt");
        assert_eq!(part1(&data), 4277556);
    }
    #[test]
    fn test_part2() {
        let data = read_and_parse("test_inputs/day06.txt");
        assert_eq!(part2(&data), 3263827);
    }
}
