use crate::etc::grid::Grid;
use crate::etc::point::Point;
use crate::{Solution, SolutionPair};
use std::collections::{BinaryHeap, VecDeque};
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
            'S' => Start(0),
            '^' => Split(0),
            _ => panic!("Invalid character for Cell: {}", c),
        }
    }
    fn is_split(&self) -> bool {
        matches!(self, Split(_))
    }
    fn is_start(&self) -> bool {
        matches!(self, Start(_))
    }
    fn is_beam(&self) -> bool {
        matches!(self, Beam(_))
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

fn read_and_parse(path: &str) -> (Grid<Cell>, Point) {
    let input = read_to_string(path).expect("Failed to read input file");
    let grid = Grid::map_from_str(&input, Cell::from_char);
    let start = grid
        .enumerate()
        .find_map(|(p, c)| if c.is_start() { Some(p) } else { None })
        .expect("No start point found");
    (grid, start)
}

fn process_cell(grid: &mut Grid<Cell>, next: Point, queue: &mut VecDeque<Point>) {
    if !grid.is_in_bounds(next) {
        return;
    }
    match grid[next] {
        Empty => {
            grid[next] = Beam(0);
            queue.push_back(next);
        }
        Split(_) => {
            queue.push_back(next);
        }
        _ => {}
    }
}

#[derive(Eq, PartialEq)]
struct HeapWrapper(Point, bool);
impl Ord for HeapWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.y.cmp(&other.0.y).then_with(|| self.1.cmp(&other.1))
    }
}
impl PartialOrd for HeapWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
struct Heap {
    data: BinaryHeap<HeapWrapper>,
}
impl Heap {
    fn new() -> Self {
        Heap {
            data: BinaryHeap::new(),
        }
    }
    fn push(&mut self, p: Point, have_priority: bool) {
        self.data.push(HeapWrapper(p, have_priority));
    }
    fn pop(&mut self) -> Option<Point> {
        self.data.pop().map(|hw| hw.0)
    }
}

fn part1(grid: &mut Grid<Cell>, start: Point) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut split_count = 0;

    while let Some(current) = queue.pop_front() {
        if grid[current].is_split() {
            split_count += 1;
            process_cell(grid, current.left(), &mut queue);
            process_cell(grid, current.right(), &mut queue);
        } else {
            process_cell(grid, current.down(), &mut queue);
        }
    }

    split_count
}

fn part2(grid: &mut Grid<Cell>) -> usize {
    let mut queue = Heap::new();
    let mut current_down = Point::new(0, grid.height() as isize - 1);
    while grid.is_in_bounds(current_down) {
        if let Beam(ref mut v) = grid[current_down] {
            *v = 1;
            queue.push(current_down, true);
        }
        current_down = current_down.right();
    }

    while let Some(current) = queue.pop() {
        let current_value = grid[current].get_value();
        match grid[current] {
            Beam(_) => {
                let up = current.up();
                if grid[up].is_beam() || grid[up].is_start() {
                    grid[up].add_value(current_value);
                    queue.push(up, true);
                }
                let left = current.left();
                let right = current.right();
                if grid.is_in_bounds(left) && grid[left].is_split() {
                    grid[left].add_value(current_value);
                    queue.push(left, false);
                }
                if grid.is_in_bounds(right) && grid[right].is_split() {
                    grid[right].add_value(current_value);
                }
            }
            Split(_) => {
                grid[current.up()].add_value(current_value);
                queue.push(current.up(), true);
            }
            Start(_) => {
                return current_value;
            }
            _ => {}
        }
    }

    unreachable!()
}

pub fn solve() -> SolutionPair {
    let (mut grid, start) = read_and_parse("inputs/day07.txt");
    let sol1 = part1(&mut grid, start);
    let sol2 = part2(&mut grid);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (mut grid, start) = read_and_parse("test_inputs/day07.txt");
        assert_eq!(part1(&mut grid, start), 21);
    }
    #[test]
    fn test_part2() {
        let (mut grid, start) = read_and_parse("test_inputs/day07.txt");
        part1(&mut grid, start);
        assert_eq!(part2(&mut grid), 40);
    }
}
