use crate::helpers;
use crate::helpers::{DayString, Solution};

const A_FACTOR: usize = 16807;
const B_FACTOR: usize = 48271;

#[derive(Debug, Clone, Copy)]
struct Generator {
    factor: usize,
    remainder: usize,
    value: usize,
}

fn parse_input(s: DayString) -> [Generator; 2] {
    fn get_number(s: &str) -> usize {
        s.chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap()
    }
    let mut lines = s.lines();
    let gen_a_value = lines.next().map(get_number).unwrap_or_default();
    let gen_b_value = lines.next().map(get_number).unwrap_or_default();
    [
        Generator::new(A_FACTOR, gen_a_value),
        Generator::new(B_FACTOR, gen_b_value),
    ]
}

impl Generator {
    const REM: usize = 2147483647;
    fn new(factor: usize, value: usize) -> Self {
        Generator {
            factor,
            value,
            remainder: Self::REM,
        }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value * self.factor) % self.remainder;
        Some(self.value)
    }
}

fn run_judge<I1, I2>(a: I1, b: I2, n: usize) -> usize
where
    I1: Iterator<Item = usize>,
    I2: Iterator<Item = usize>,
{
    a.zip(b)
        .take(n)
        .filter(|(val_a, val_b)| {
            let mask = 0b1111_1111_1111_1111;
            (val_a & mask) == (val_b & mask)
        })
        .count()
}

fn solve_part1([a, b]: [Generator; 2]) -> usize {
    run_judge(a, b, 40000000)
}

fn solve_part2([a, b]: [Generator; 2]) -> usize {
    let a = a.filter(|n| n % 4 == 0);
    let b = b.filter(|n| n % 8 == 0);
    run_judge(a, b, 5000000)
}

fn solve_string(s: DayString) -> Solution {
    let parsed = parse_input(s);
    let sol1 = solve_part1(parsed).to_string();
    let sol2 = solve_part2(parsed).to_string();
    [sol1, sol2]
}

pub fn solve() -> Solution {
    solve_string(helpers::read_day(15))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(helpers::read_example("15"));
        assert_eq!(solve_part1(input), 588)
    }

    #[test]
    fn test_part2() {
        let input = parse_input(helpers::read_example("15"));
        assert_eq!(solve_part2(input), 309)
    }
}
