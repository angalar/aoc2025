use crate::etc::grid::Grid;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

#[derive(Debug)]
enum Cell {
    Empty,
    Start(usize),
    Split(usize),
    Beam(usize),
}
use Cell::*;

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Empty,
            'S' => Start(1),
            '^' => Split(0),
            _ => panic!("Invalid character for Cell: {}", c),
        }
    }
    fn is_split(&self) -> bool {
        matches!(self, Split(_))
    }
    fn is_empty(&self) -> bool {
        matches!(self, Empty)
    }
    fn get_value(&self) -> usize {
        match self {
            Empty => 0,
            Start(v) | Split(v) | Beam(v) => *v,
        }
    }
    fn add_value(&mut self, v: usize) {
        match self {
            Start(val) | Split(val) | Beam(val) => *val += v,
            Empty => {}
        }
    }
}

fn read_and_parse(path: &str) -> Grid<Cell> {
    let input = read_to_string(path).expect("Failed to read input file");
    Grid::map_from_str(&input, Cell::from_char)
}

fn part1(grid: &mut Grid<Cell>) -> usize {
    let mut split_count = 0;
    for y in 0..grid.height() - 1 {
        for x in 0..grid.width() {
            if !grid[(x, y)].is_split() {
                continue;
            }
            let val = grid[(x, y)].get_value();
            if val > 0 {
                split_count += 1;
            }
            if grid[(x-1, y)].is_empty() {
                grid[(x-1, y)] = Beam(val);
            } else {
                grid[(x-1, y)].add_value(val);
            }
            if grid[(x+1, y)].is_empty() {
                grid[(x+1, y)] = Beam(val);
            } else {
                grid[(x+1, y)].add_value(val);
            }
        }
        for x in 0..grid.width() {
            let cell = &grid[(x,y)];
            if cell.is_empty() || cell.is_split() {
                continue;
            }
            let val = cell.get_value();
            if grid[(x, y + 1)].is_empty() {
                grid[(x, y + 1)] = Beam(val);
            } else {
                grid[(x, y + 1)].add_value(val);
            }
        }
    }

    split_count
}

fn part2(grid: &mut Grid<Cell>) -> usize {
    let last_row = grid.height() - 1;
    (0..grid.width())
        .map(|x| grid[(x, last_row)].get_value())
        .sum()
}

pub fn solve() -> SolutionPair {
    let mut grid = read_and_parse("inputs/day07.txt");
    let sol1 = part1(&mut grid);
    let sol2 = part2(&mut grid);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut grid = read_and_parse("test_inputs/day07.txt");
        assert_eq!(part1(&mut grid), 21);
    }
    #[test]
    fn test_part2() {
        let mut grid = read_and_parse("test_inputs/day07.txt");
        part1(&mut grid);
        assert_eq!(part2(&mut grid), 40);
    }
}
