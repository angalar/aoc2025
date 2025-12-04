use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use crate::etc::grid::Grid;
use crate::etc::point::Point;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty, Roll(u8),
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {

            '.' => Cell::Empty,
            '@' => Cell::Roll(0),
            _ => panic!("Invalid cell character: {}", c),
        }
    }
}

fn fill_rolls(grid: &mut Grid<Cell>) {
    let grid_clone = grid.clone();
    for (pos, cell) in grid_clone.enumerate() {
        if *cell == Cell::Roll(0) {
            let mut count = 0;
            for neighbor in pos.neighbors_diagonal_in_bounds(&grid_clone) {
                if let Cell::Roll(_) = grid_clone[neighbor] {
                    count += 1;
                }
            }
            grid[pos] = Cell::Roll(count);
        }
    }
}

fn read_and_parse(path: &str) -> Grid<Cell> {
    let input = read_to_string(path).expect("Failed to read input file");
    Grid::map_from_str(&input, Cell::from_char)
}

fn part1(grid: &Grid<Cell>) -> usize {
    grid
        .enumerate()
        .filter(|(_, cell)| matches!(*cell, Cell::Roll(n) if *n < 4))
        .count()
}
fn part2(grid: &mut Grid<Cell>) -> usize {
    let mut queue = VecDeque::new();
    let mut count = 0;

    let mut to_empty = Vec::new();
    for (pos, cell) in grid.enumerate() {
        if matches!(*cell, Cell::Roll(n) if n < 4) {
            queue.push_back(pos);
            count += 1;
            to_empty.push(pos);
        }
    }
    for pos in to_empty {
        grid[pos] = Cell::Empty;
    }

    while let Some(pos) = queue.pop_front() {
        for neighbor in pos.neighbors_diagonal() {
            if grid.is_in_bounds(neighbor) 
                && let Cell::Roll(ref mut n) = grid[neighbor] {
                    *n -= 1;
                    if *n < 4 {
                        queue.push_back(neighbor);
                        count += 1;
                        grid[neighbor] = Cell::Empty;
                    }
            }
        }
    }
    count
}

pub fn solve() -> SolutionPair {
    let mut grid = read_and_parse("inputs/day04.txt");
    fill_rolls(&mut grid);
    let sol1 = part1(&grid);
    let sol2 = part2(&mut grid);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut grid = read_and_parse("test_inputs/day04.txt");
        fill_rolls(&mut grid);
        assert_eq!(part1(&grid), 13);
    }
    #[test]
    fn test_part2() {
        let mut grid = read_and_parse("test_inputs/day04.txt");
        fill_rolls(&mut grid);
        assert_eq!(part2(&mut grid), 43);
    }
}
