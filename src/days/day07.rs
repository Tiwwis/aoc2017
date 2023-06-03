use crate::helpers;
use crate::helpers::{DayString, Solution};

struct Disk<'a> {
    name: &'static str,
    weight: u16,
    above:Vec<&'a Disk<'a>>,
}

fn parse_input(s: DayString) -> Input {
    unimplemented!();
}

fn solve_part1(input: &Input) -> usize {
    unimplemented!();
}

fn solve_part2(input: &Input) -> usize {
    unimplemented!();
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(&parsed).to_string();
    let sol2 = solve_part2(&parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(n))
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

