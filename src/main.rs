mod days;
mod helpers;

use crate::helpers::Solution;
use days::*;

fn all_days() -> Vec<u8> {
    (1..26).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let day_string: Option<u8> = args.get(0).and_then(|x| x.parse().ok());
    let days: Vec<u8> = day_string.map_or_else(all_days, |x| vec![x]);

    let start_time = std::time::Instant::now();
    days.into_iter().for_each(run_day);
    let end_time = start_time.elapsed().as_millis();

    println!("\n=============");
    println!("Overall Time: {:.4} ms", end_time);
}

fn run_day(day: u8) {
    let solve = get_day_solver(day);
    let Some(solve) = solve else { return };
    let start = std::time::Instant::now();
    let [p1, p2] = solve();
    let elapsed_ms = start.elapsed().as_nanos();

    println!("\n=== Day {:02} ===", day);
    println!("  · Part 1: {}", p1);
    println!("  · Part 2: {}", p2);
    println!("  · Elapsed: {:.4} ns", elapsed_ms);
}

fn get_day_solver(day: u8) -> Option<fn() -> Solution> {
    match day {
        1 => Some(day01::solve),
        2 => Some(day02::solve),
        3 => Some(day03::solve),
        4 => Some(day04::solve),
        5 => Some(day05::solve),
        6 => Some(day06::solve),
        7 => Some(day07::solve),
        8 => Some(day08::solve),
        9 => Some(day09::solve),
        10 => Some(day10::solve),
        11 => Some(day11::solve),
        12 => Some(day12::solve),
        13 => Some(day13::solve),
        14 => Some(day14::solve),
        15 => Some(day15::solve),
        16 => Some(day16::solve),
        17 => Some(day17::solve),
        18 => Some(day18::solve),
        _ => None,
    }
}
