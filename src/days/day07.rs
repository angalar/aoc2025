use crate::etc::grid::Grid;
use crate::etc::point::Point;
use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Start,
    Split,
}
use Cell::*;

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Empty,
            'S' => Start,
            '^' => Split,
            _ => panic!("Invalid character for Cell: {}", c),
        }
    }
}

fn read_and_parse(path: &str) -> (Grid<Cell>, Point) {
    let input = read_to_string(path).expect("Failed to read input file");
    let grid = Grid::map_from_str(&input, Cell::from_char);
    let start = grid
        .enumerate()
        .find_map(|(p, &c)| if c == Start { Some(p) } else { None })
        .expect("No start point found");
    (grid, start)
}

fn count_time_lines(grid: &Grid<Cell>, current: Point, cache: &mut HashMap<Point, usize>) -> usize {
    if grid.height() - 1 == current.y as usize {
        return 1;
    }
    if let Some(&cached) = cache.get(&current) {
        return cached;
    }
    let mut total = 0;
    if grid[current] == Split {
        for &next in &[current.left(), current.right()] {
            if grid.is_in_bounds(next) {
                total += count_time_lines(grid, next, cache);
            }
        }
    } else {
        let down = current.down();
        if grid.is_in_bounds(down) {
            total += count_time_lines(grid, down, cache);
        }
    }
    cache.insert(current, total);
    total
}

fn part1(grid: &Grid<Cell>, start: Point) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited.insert(start);
    let mut splits = 0;

    while let Some(current) = queue.pop_front() {
        if grid[current] == Split {
            splits += 1;
            for &next in &[current.left(), current.right()] {
                if grid.is_in_bounds(next) && visited.insert(next) {
                    queue.push_back(next);
                }
            }
        } else {
            let down = current.down();
            if grid.is_in_bounds(down) && visited.insert(down) {
                queue.push_back(down);
            }
        }
    }

    splits
}
fn part2(grid: &Grid<Cell>, start: Point) -> usize {
    let mut cache = HashMap::new();
    count_time_lines(grid, start, &mut cache)
}

pub fn solve() -> SolutionPair {
    let (grid, start) = read_and_parse("inputs/day07.txt");
    let sol1 = part1(&grid, start);
    let sol2 = part2(&grid, start);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (grid, start) = read_and_parse("test_inputs/day07.txt");
        assert_eq!(part1(&grid, start), 21);
    }
    #[test]
    fn test_part2() {
        let (grid, start) = read_and_parse("test_inputs/day07.txt");
        assert_eq!(part2(&grid, start), 40);
    }
}
