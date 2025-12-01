#![allow(dead_code)]
#![allow(unused_imports)]

mod days;
mod etc;

use etc::solution::Solution;
use days::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
           day11, day12};
use std::env;
use std::time::Instant;

pub type SolutionPair = (Solution, Solution);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day number as an argument.");
    }
    let days: Vec<u8> = args[1..]
        .iter()
        .map(|arg| arg.parse::<u8>().expect("Invalid day number"))
        .collect();

    let mut runtime = 0.0;

    for day in days {
        let func = get_day_solver(day);

        let start = Instant::now();
        let (part1, part2) = func();
        let duration = start.elapsed().as_nanos() as f64 / 1_000_000.0;     
        println!("\n=== Day {:02} ===", day);
        println!("  . Part 1: {part1}");
        println!("  . Part 2: {part2}");
        println!("  . Time: {:.4} ms", duration);
        runtime += duration;
    }

    println!("Total time: {:.4} ms", runtime);
}

fn get_day_solver(day: u8) -> fn() -> SolutionPair {
    match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        4 => day04::solve,
        5 => day05::solve,
        6 => day06::solve,
        7 => day07::solve,
        8 => day08::solve,
        9 => day09::solve,
        10 => day10::solve,
        11 => day11::solve,
        12 => day12::solve,
        _ => unimplemented!(),
    }
}
