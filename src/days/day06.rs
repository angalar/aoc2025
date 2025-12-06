use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

enum Operation {
    Add,
    Multiply,
}
use Operation::*;

fn part1(data: &str) -> u64 {
    let table: Vec<&str> = data.lines().collect();
    let values: Vec<Vec<u64>> = table
        .iter()
        .take(table.len() - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();
    let operations: Vec<Operation> = table[table.len() - 1]
        .split_whitespace()
        .map(|op| match op {
            "+" => Add,
            "*" => Multiply,
            _ => panic!("Unknown operation"),
        })
        .collect();

    operations
        .iter()
        .enumerate()
        .map(|(i, op)| {
            let mut col_result = values[0][i];
            for row in &values[1..] {
                match op {
                    Add => col_result += row[i],
                    Multiply => col_result *= row[i],
                }
            }
            col_result
        })
        .sum()
}
fn part2(data: &str) -> u64 {
    let table: Vec<&[u8]> = data.lines().map(|line| line.as_bytes()).collect();
    let mut indexes: Vec<usize> = (0..table[0].len())
        .filter(|&i| table.iter().all(|row| row[i] == b' '))
        .collect();
    indexes.push(table[0].len());

    let operations: Vec<Operation> = table[table.len() - 1]
        .split(|&b| b == b' ')
        .filter(|op| !op.is_empty())
        .map(|op| match op {
            b"+" => Add,
            b"*" => Multiply,
            _ => panic!("Unknown operation"),
        })
        .collect();
    let mut result = 0;
    let mut start = 0;
    for (i, &end) in indexes.iter().enumerate() {
        let mut col_result = 0u64;
        for j in start..end {
            let mut num = 0u64;
            for row in &table[0..table.len() - 1] {
                if row[j] != b' ' {
                    num = num * 10 + (row[j] - b'0') as u64;
                }
            }
            if j == start {
                col_result = num;
            } else {
                match operations[i] {
                    Add => col_result += num,
                    Multiply => col_result *= num,
                }
            }
        }
        result += col_result;
        start = end + 1;
    }

    result
}

pub fn solve() -> SolutionPair {
    let data = read_to_string("inputs/day06.txt").unwrap();
    let sol1 = part1(&data);
    let sol2 = part2(&data);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("test_inputs/day06.txt").unwrap();
        assert_eq!(part1(&data), 4277556);
    }
    #[test]
    fn test_part2() {
        let data = read_to_string("test_inputs/day06.txt").unwrap();
        assert_eq!(part2(&data), 3263827);
    }
}
