use crate::etc::point::Point;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

struct Rectangle {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Rectangle {
    fn new(a: Point, b: Point) -> Self {
        Rectangle {
            min_x: a.x.min(b.x),
            max_x: a.x.max(b.x),
            min_y: a.y.min(b.y),
            max_y: a.y.max(b.y),
        }
    }
    fn area(&self) -> u64 {
        ((self.max_x - self.min_x + 1) as u64) * ((self.max_y - self.min_y + 1) as u64)
    }
    fn exceeds_rect(&self, other: &Rectangle) -> bool {
        self.min_x < other.max_x
            && self.max_x > other.min_x
            && self.min_y < other.max_y
            && self.max_y > other.min_y
    }
}

fn read_and_parse(path: &str) -> Vec<Point> {
    let input = read_to_string(path).expect("Failed to read input file");
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(',');
            let x: isize = coords.next().unwrap().parse().unwrap();
            let y: isize = coords.next().unwrap().parse().unwrap();
            Point::new(x, y)
        })
        .collect()
}

fn part1(points: &[Point]) -> u64 {
    (0..points.len() - 1)
        .flat_map(|i| {
            (i + 1..points.len()).map(move |j| Rectangle::new(points[i], points[j]).area())
        })
        .max()
        .unwrap()
}

fn part2(points: &[Point]) -> u64 {
    let mut greens: Vec<_> = points
        .windows(2)
        .map(|w| Rectangle::new(w[0], w[1]))
        .collect();
    greens.push(Rectangle::new(
        points.first().cloned().unwrap(),
        points.last().cloned().unwrap(),
    ));

    (0..points.len() - 1)
        .flat_map(|i| (i + 1..points.len()).map(move |j| Rectangle::new(points[i], points[j])))
        .filter(|rect| greens.iter().all(|g| !rect.exceeds_rect(g)))
        .map(|rect| rect.area())
        .max()
        .unwrap()
}

pub fn solve() -> SolutionPair {
    let points = read_and_parse("inputs/day09.txt");
    let sol1 = part1(&points);
    let sol2 = part2(&points);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let points = read_and_parse("test_inputs/day09.txt");
        assert_eq!(part1(&points), 50);
    }
    #[test]
    fn test_part2() {
        let points = read_and_parse("test_inputs/day09.txt");
        assert_eq!(part2(&points), 24);
    }
}
