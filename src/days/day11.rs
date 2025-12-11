use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::read_to_string;

fn read_and_parse(path: &str) -> HashMap<String, Vec<String>> {
    let input = read_to_string(path).expect("Failed to read input file");
    input
        .lines()
        .map(|line| {
            let (start, rest) = line.split_once(": ").unwrap();
            let ends = rest.split_whitespace().map(String::from).collect();
            (start.to_string(), ends)
        })
        .collect()
}
fn paths_count(map: &HashMap<String, Vec<String>>, start: &str, end: &str) -> usize {
    let mut cache = HashMap::new();
    dfs(map, start, end, &mut cache)
}
fn dfs(
    map: &HashMap<String, Vec<String>>,
    current: &str,
    end: &str,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if current == end {
        return 1;
    }
    if let Some(&cached) = cache.get(current) {
        return cached;
    }
    let mut total_paths = 0;
    if let Some(neighbors) = map.get(current) {
        for neighbor in neighbors {
            total_paths += dfs(map, neighbor, end, cache);
        }
    }
    cache.insert(current.to_string(), total_paths);
    total_paths
}
fn part1(map: &HashMap<String, Vec<String>>) -> usize {
    paths_count(map, "you", "out")
}
fn part2(map: &HashMap<String, Vec<String>>) -> usize {
    let dac_fft_paths = paths_count(map, "dac", "fft");
    let fft_dac_paths = paths_count(map, "fft", "dac");
    if dac_fft_paths != 0 {
        let srv_dac_paths = paths_count(map, "srv", "dac");
        let fft_out_paths = paths_count(map, "fft", "out");
        srv_dac_paths * dac_fft_paths * fft_out_paths
    } else {
        let srv_fft_paths = paths_count(map, "srv", "fft");
        let dac_out_paths = paths_count(map, "dac", "out");
        srv_fft_paths * fft_dac_paths * dac_out_paths
    }
}

pub fn solve() -> SolutionPair {
    let map = read_and_parse("inputs/day11.txt");
    let sol1 = part1(&map);
    let sol2 = part2(&map);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let map = read_and_parse("test_inputs/day11.txt");
        assert_eq!(part1(&map), 5);
    }
    #[test]
    fn test_part2() {
        let map = read_and_parse("test_inputs/day11_2.txt");
        assert_eq!(part2(&map), 2);
    }
}
