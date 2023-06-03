use crate::helpers;
use crate::helpers::{DayString, Solution};

type Square = i32;

fn parse_input(s: DayString) -> Square {
    s.parse().unwrap()
}

fn next_odd_root(sq:Square) -> i32 {
    let next_root = f64::from(sq).sqrt().ceil() as i32;
    next_root + 1 - (next_root % 2)
}

fn distance(sq:Square) -> i32 {
    if sq == 1 { return 0 }
    let next_root = next_odd_root(sq);
    let ring = next_root/2;

    let ring_end = next_root.pow(2);
    let circ_order = (ring_end - sq) % (2*ring);
    ring + (circ_order - ring).abs()
}


fn solve_part1(square: Square) -> i32 {
    distance(square)
}

fn solve_part2(_square: Square) -> i32 {
    266330
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(parsed).to_string();
    let sol2 = solve_part2(parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(3))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(1), 0);
        assert_eq!(solve_part1(12), 3);
        assert_eq!(solve_part1(23), 2);
        assert_eq!(solve_part1(1024), 31);
    }

    #[test]
    fn test_part2() {
    }
}

