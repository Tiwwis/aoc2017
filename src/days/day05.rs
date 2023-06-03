use crate::helpers;
use crate::helpers::{DayString, Solution};

type Program = Vec<i32>;

fn parse_input(s: DayString) -> Program {
    s.lines().map(str::parse).flatten().collect()
}

fn solve_part1(input: &Program) -> usize {
    let mut program = input.clone();
    let mut pointer:i32 = 0;
    let mut counter = 0;
    while let Some(offset) = program.get_mut(pointer as usize) {
        counter += 1;
        pointer += *offset;
        *offset += 1;
    };
    counter
}

fn solve_part2(input: &Program) -> usize {
    let mut program = input.clone();
    let mut pointer:i32 = 0;
    let mut counter = 0;
    while let Some(offset) = program.get_mut(pointer as usize) {
        counter += 1;
        pointer += *offset;
        if *offset >= 3 {
            *offset -= 1
        } else {
            *offset += 1
        }
    };
    counter
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(&parsed).to_string();
    let sol2 = solve_part2(&parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(5))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(helpers::read_example("05"));
        assert_eq!(solve_part1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(helpers::read_example("05"));
        assert_eq!(solve_part2(&input), 10);
    }
}

