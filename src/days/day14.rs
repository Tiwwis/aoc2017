use crate::helpers;
use crate::helpers::{DayString, Solution};
use crate::day10::{knot_hash, knot_hash_str};

type Input = &'static str;
type Squares = [[u8; 16]; 128];

fn parse_input(s: DayString) -> Input {
    s
}

fn get_squares(original: DayString) -> Squares {
    let mut result = [[0; 16]; 128];
    let n:Vec<u8> = (0u8..=128u8).collect();
    for i in 0..=127 {
        let new_str = original.to_string()+"-"+&i.to_string();
        result[i] = knot_hash(new_str.as_bytes())
    }
    result
}

fn solve_part1(squares: &Squares) -> u32 {
    squares.iter().map(
        |arr| arr.iter().map(|num| num.count_ones()).sum::<u32>()
        ).sum()
}

fn solve_part2(input: &Squares) -> usize {
    0
}

fn solve_string(s: DayString) -> Solution {
    let squares = get_squares(parse_input(s));
    let sol1 = solve_part1(&squares).to_string();
    let sol2 = solve_part2(&squares).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(14))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(helpers::read_example("xxx"));
        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(helpers::read_example("xxx"));
        assert_eq!(solve_part2(&input), 3);
    }
}

