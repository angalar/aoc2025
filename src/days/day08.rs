use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Box {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Box {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dims: Vec<usize> = s
            .split(',')
            .map(|d| d.parse().unwrap())
            .collect();
        Ok(Box {
            x: dims[0],
            y: dims[1],
            z: dims[2],
        })
    }
}

impl Box {
    fn distance(&self, other: &Self) -> u64 {
        let dx = self.x as isize - other.x as isize;
        let dy = self.y as isize - other.y as isize;
        let dz = self.z as isize - other.z as isize;
        (dx * dx + dy * dy + dz * dz) as u64
    }
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
    fn find(&mut self, a: usize) -> usize {
        if self.parent[a] != a {
            self.parent[a] = self.find(self.parent[a]);
        }
        self.parent[a]
    }
    fn union(&mut self, a: usize, b: usize) {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return;
        }
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        self.size[a] += self.size[b];
    }
    fn network_count(&mut self, a: usize) -> usize {
        let p = self.find(a);
        self.size[p]
    }
}

fn read_and_parse(path: &str) -> Vec<Box> {
    let input = read_to_string(path).expect("Failed to read input file");
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn processing(boxes: &[Box], pair_count: usize) -> (Vec<(u64, usize, usize)>, Dsu) {
    let n = boxes.len();
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..(n - 1) {
        for j in (1 + i)..n {
            let dist = boxes[i].distance(&boxes[j]);
            edges.push((dist, i, j));
        }
    }
    edges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut dsu = Dsu::new(n);
    for &(_, a, b) in edges.iter().take(pair_count) {
        dsu.union(a, b);
    }
    (edges, dsu)
}

fn part1(dsu: &mut Dsu) -> usize {
    let n = dsu.parent.len();
    let mut sizes = vec![0; n];
    for i in 0..n {
        let p = dsu.find(i);
        sizes[p] += 1;
    }
    sizes.sort_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product()
}
fn part2(boxes: &[Box], pair_count: usize, edges: &[(u64, usize, usize)], mut dsu: Dsu) -> usize {
    let n = boxes.len();
    for &(_, a, b) in edges.iter().skip(pair_count) {
        dsu.union(a, b);
        if dsu.network_count(a) == n {
            return boxes[a].x * boxes[b].x;
        }
    }
    unreachable!()
}

pub fn solve() -> SolutionPair {
    let boxes = read_and_parse("inputs/day08.txt");
    let (edges, mut dsu) = processing(&boxes, 1000);
    let sol1 = part1(&mut dsu);
    let sol2 = part2(&boxes, 1000, &edges, dsu);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let boxes = read_and_parse("test_inputs/day08.txt");
        let (_, mut dsu) = processing(&boxes, 10);
        assert_eq!(part1(&mut dsu), 40);
    }
    
    #[test]
    fn test_part2() {
        let boxes = read_and_parse("test_inputs/day08.txt");
        let (edges, dsu) = processing(&boxes, 10);
        assert_eq!(part2(&boxes, 10, &edges, dsu), 25272);
    }
}
