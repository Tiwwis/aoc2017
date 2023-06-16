use crate::helpers;
use crate::helpers::{DayString, Solution};

use nom::character::complete as ccmp;
use nom::bytes::complete as ncmp;
use nom::Parser;
use nom::sequence::terminated;


type Wall = [u32; 2];
type Firewall = Vec<Wall>;

fn parse_input(s: DayString) -> Firewall {
    let mut parser = terminated(ccmp::u32::<&str, ()>, ncmp::tag(": ")).and(ccmp::u32);
    s.lines().map(|ln| { let (_, (a,b)) = parser.parse(ln).unwrap(); [a, b] }).collect()
}

fn solve_part1(input: &[Wall]) -> u32 {
    input.iter()
        .filter_map(
            |&[depth, range]| (range == 1 || (depth % (2*(range-1))) == 0)
            .then(|| depth*range)
        ).sum::<u32>()
}

// 0, 01, 01210, 012321
// 1,  2, 4, 6

fn solve_part2(walls: &[Wall]) -> u32 {

    let is_sneaky = |wait:&u32| -> bool {
        walls.iter().all(|&[depth, range]| range != 1 && ((depth + wait) % (2*(range-1)) != 0))
    };

    (0..u32::max_value()).find(is_sneaky).unwrap()
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(&parsed).to_string();
    let sol2 = solve_part2(&parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(13))
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

