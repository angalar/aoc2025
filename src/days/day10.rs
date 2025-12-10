use crate::SolutionPair;
use good_lp::*;
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct Machine {
    controls: u16,
    buttons: Vec<u16>,
    buttons_idx: Vec<Vec<usize>>,
    energy: Vec<u16>,
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        let controls = parts[0][1..parts[0].len() - 1]
            .bytes()
            .fold(0u16, |acc, b| (acc << 1) | if b == b'#' { 1 } else { 0 });
        let bit_len = parts[0].len() as u16 - 2;
        let buttons: Vec<u16> = parts
            .iter()
            .take(parts.len() - 1)
            .skip(1)
            .map(|line| {
                line[1..line.len() - 1]
                    .split(',')
                    .map(|b| b.parse::<u16>().unwrap())
                    .fold(0u16, |acc, b| acc | (1 << (bit_len - 1 - b)))
            })
            .collect();
        let mut buttons_idx: Vec<_> = parts
            .iter()
            .take(parts.len() - 1)
            .skip(1)
            .map(|line| {
                line[1..line.len() - 1]
                    .split(',')
                    .map(|b| b.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        buttons_idx.sort_by_key(|idxs: &Vec<usize>| idxs.len());
        let part_last_len = parts[parts.len() - 1].len();
        let energy: Vec<u16> = parts[parts.len() - 1][1..part_last_len - 1]
            .split(',')
            .map(|e| e.parse().unwrap())
            .collect();
        Ok(Machine {
            controls,
            buttons,
            buttons_idx,
            energy,
        })
    }
}

impl Machine {
    fn count_buttons_pressed(&self) -> usize {
        let mut visited = vec![false; u16::MAX as usize + 1];
        let mut queue = VecDeque::new();
        queue.push_back((0u16, 0usize));
        visited[0] = true;

        while let Some((state, pushes)) = queue.pop_front() {
            if state == self.controls {
                return pushes;
            }
            for &button in &self.buttons {
                let new_state = state ^ button;
                if !visited[new_state as usize] {
                    visited[new_state as usize] = true;
                    queue.push_back((new_state, pushes + 1));
                }
            }
        }

        unreachable!()
    }
    fn count_energy_buttons_pressed(&self) -> usize {
        let m = self.buttons_idx.len();
        let n = self.energy.len();
        let mut vars = variables!();
        let press_vars: Vec<_> = (0..m)
            .map(|_| vars.add(variable().min(0).integer()))
            .collect();
        let mut problem = highs(vars.minimise(press_vars.iter().sum::<Expression>()));
        let mut exprs = vec![0.into_expression(); n];
        for (i, pressed) in press_vars.iter().enumerate().take(m) {
            for &idx in &self.buttons_idx[i] {
                exprs[idx] += pressed;
            }
        }
        for (i, expr) in exprs.iter().enumerate() {
            problem.add_constraint(constraint!(expr.clone() == self.energy[i] as f64));
        }
        let solution = problem.solve().unwrap();
        press_vars.iter().map(|v| solution.value(*v) as usize).sum()
    }
}

fn read_and_parse(path: &str) -> Vec<Machine> {
    let input = read_to_string(path).expect("Failed to read input file");
    input
        .lines()
        .map(|line| line.parse::<Machine>().unwrap())
        .collect()
}
fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(Machine::count_buttons_pressed).sum()
}
fn part2(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(Machine::count_energy_buttons_pressed)
        .sum()
}

pub fn solve() -> SolutionPair {
    let machines = read_and_parse("inputs/day10.txt");
    let sol1 = part1(&machines);
    let sol2 = part2(&machines);

    (crate::Solution::from(sol1), crate::Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let machines = read_and_parse("test_inputs/day10.txt");
        assert_eq!(part1(&machines), 7);
    }
    #[test]
    fn test_part2() {
        let machines = read_and_parse("test_inputs/day10.txt");
        assert_eq!(part2(&machines), 33);
    }
}
