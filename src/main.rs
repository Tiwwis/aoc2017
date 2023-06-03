mod days;
mod helpers;

use crate::helpers::Solution;
use days::*;

fn all_days() -> Vec<u8> {
    (1..26).collect()
}

fn main() {
    let args:Vec<String> = std::env::args().skip(1).collect();

    let day_string:Option<u8> = args.get(0).and_then(|x| x.parse().ok());
    let days:Vec<u8> = day_string.map_or_else(all_days, |x| vec![x]);

    days.into_iter().for_each(run_day);
}

fn run_day(day: u8) {
    let solve = get_day_solver(day);


    let start = std::time::Instant::now();
    let [p1, p2] = solve();
    let elapsed_ms = start.elapsed().as_nanos();

    println!("\n=== Day {:02} ===", day);
    println!("  · Part 1: {}", p1);
    println!("  · Part 2: {}", p2);
    println!("  · Elapsed: {:.4} ns", elapsed_ms);
}

fn get_day_solver(day: u8) -> fn() -> Solution {
    match day {
         1 => day01::solve,
         2 => day02::solve,
         3 => day03::solve,
         4 => day04::solve,
         5 => day05::solve,
         6 => day06::solve,
         7 => day07::solve,
         _ => unimplemented!(),
    }
}
